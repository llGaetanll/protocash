use std::collections::HashMap;
use std::future::Future;
use std::num::NonZeroU32;
use std::pin::Pin;

use bytes::Bytes;
use cometbft::abci::response::Echo;
use cometbft::abci::types::ExecTxResult;
use cometbft::abci::v1::request;
use cometbft::abci::v1::request::Request;
use cometbft::abci::v1::response;
use cometbft::abci::v1::response::ExtendVote;
use cometbft::abci::v1::response::Response;
use cometbft::abci::v1::response::VerifyVoteExtension;
use cometbft::abci::Code;
use cometbft::validator::Update;
use cometbft::PublicKey;
use sha2::Digest;
use sha2::Sha256;
use tower::Service;
use tower_abci::BoxError;

#[derive(Default)]
pub struct State {
    store: HashMap<i32, i32>,
    ongoingblock: HashMap<i32, i32>,
    size: u32,
    height: u32,
}

impl State {
    fn hash(&self) -> Vec<u8> {
        let hasher = Sha256::new();

        let bytes: Vec<u8> = self
            .store
            .iter()
            .flat_map(|(k, v)| k.to_be_bytes().into_iter().chain(v.to_be_bytes()))
            .collect();

        let hasher = hasher
            .chain_update(bytes.as_slice()) // add the store
            .chain_update(self.height.to_ne_bytes()) // add the height
            .chain_update(self.size.to_ne_bytes()); // add the size

        hasher.finalize().to_vec() // TODO: should be [u8; 32] for SHA256
    }
}

// according to cometbft, this is the first 20 bytes of `SHA256(public_key)`
pub type Addr = [u8; 20];

#[derive(Default)]
pub struct Application {
    /// The state of our application
    state: State,

    /// The number of blocks to retain after a commit.
    retain_blocks: i32,

    /// The list of staged transactions.
    staged_txs: Vec<Bytes>,

    /// We keep track of a list of updates for our validators.
    validator_upds: Vec<Update>,

    /// A set of validators, mapping addresses to public keys.
    validators: HashMap<Addr, PublicKey>,

    /// If true, the app will generate block events in BeginBlock. Used to test the event indexer
    /// Should be false by default to avoid generating too much data.
    gen_block_events: bool,
}

// These are the functions that our KVStore struct implements.
impl Application {
    // Info returns information about the state of the application. This is generally used
    // everytime a CometBFT instance begins and let's the application know what CometBFT
    // versions it's interacting with. Based from this information, CometBFT will ensure it is in
    // sync with the application by potentially replaying the blocks it has. If the Application
    // returns a 0 appBlockHeight, CometBFT will call InitChain to initialize the application
    // with consensus related data
    fn info(&self, info: request::Info) -> response::Info {
        tracing::debug!("Info");

        // TODO: CometBFT expects the application to persist validators.
        // On startup, we need to load them if they exist.
        tracing::trace!(info = ?info);

        let res = response::Info {
            data: String::from("protocash"),
            version: String::from("0.0.0"),
            app_version: 1,
            last_block_height: self.state.height.into(),
            last_block_app_hash: self.state.hash().try_into().unwrap(),
        };

        tracing::trace!(res = ?res);

        res
    }

    fn is_valid(tx: &Bytes) -> Code {
        match Self::parse_kv(tx) {
            Some(_) => Code::Ok,
            None => Code::Err(unsafe { NonZeroU32::new_unchecked(1) })
        }
    }

    fn parse_kv(tx: &Bytes) -> Option<(i32, i32)> {
        let tx_str = String::from_utf8_lossy(tx);

        tx_str
            .split('=')
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .and_then(|[k, v]: [&str; 2]| k.parse::<i32>().ok().zip(v.parse::<i32>().ok()))
    }

    fn query(&self, query: request::Query) -> response::Query {
        tracing::debug!("Query");

        tracing::trace!(?query);

        let request::Query {
            data,
            path: _path,
            height,
            prove: _prove,
        } = query;

        let data_slice: &[u8] = &data;
        let res = if let Ok(key_bytes) = TryInto::<[u8; 4]>::try_into(data_slice) {
            let key = i32::from_be_bytes(key_bytes);

            tracing::trace!(?key_bytes, key);

            match self.state.store.get(&key) {
                Some(value) => response::Query {
                    code: Code::Ok,
                    log: String::new(),
                    info: String::new(),
                    index: 0,
                    key: Bytes::copy_from_slice(&value.to_be_bytes()),
                    value: data,
                    proof: None,
                    height,
                    codespace: String::new(),
                },
                None => response::Query {
                    code: Code::Err(unsafe { NonZeroU32::new_unchecked(1) }),
                    log: format!("Key {} not found", key),
                    info: String::from("Key not found"),
                    index: 0,
                    key: Bytes::new(),
                    value: data,
                    proof: None,
                    height,
                    codespace: String::new(),
                },
            }
        } else {
            response::Query {
                code: Code::Err(unsafe { NonZeroU32::new_unchecked(1) }),
                log: String::from("Invalid query"),
                info: String::from("Invalid query"),
                index: 0,
                key: Bytes::new(),
                value: Bytes::new(),
                proof: None,
                height,
                codespace: String::new(),
            }
        };

        tracing::trace!(?res);

        res
    }

    fn check_tx(tx: request::CheckTx) -> response::CheckTx {
        let request::CheckTx {
            tx: data,
            kind: _kind,
        } = tx;

        let res = response::CheckTx {
            code: Self::is_valid(&data),
            data,
            ..Default::default()
        };

        tracing::trace!(?res);

        res
    }

    fn finalize_block(&mut self, block: request::FinalizeBlock) -> response::FinalizeBlock {
        tracing::debug!("Finalizing Block");

        let request::FinalizeBlock { txs, .. } = block;

        let mut tx_results: Vec<ExecTxResult> = Vec::with_capacity(txs.len());

        for tx in txs {
            let code = match Self::parse_kv(&tx) {
                Some((k, v)) => {
                    self.state.ongoingblock.insert(k, v);

                    Code::Ok
                }
                None => {
                    Code::Err(unsafe { NonZeroU32::new_unchecked(1) })
                }
            };

            tx_results.push(ExecTxResult {
                code,
                data: tx,
                ..Default::default()
            });
        }

        response::FinalizeBlock {
            tx_results,
            events: Default::default(),
            validator_updates: Default::default(),
            consensus_param_updates: Default::default(),
            app_hash: Default::default(), // TODO
        }
    }

    fn prepare_proposal(proposal: request::PrepareProposal) -> response::PrepareProposal {
        let request::PrepareProposal { txs, .. } = proposal;

        response::PrepareProposal { txs }
    }

    fn process_proposal(_proposal: request::ProcessProposal) -> response::ProcessProposal {
        response::ProcessProposal::Accept
    }

    fn commit(&mut self) -> response::Commit {
        tracing::debug!("Committing");

        tracing::trace!(?self.state.ongoingblock);

        self.state
            .store
            .extend(self.state.ongoingblock.drain());

        tracing::trace!(?self.state.store);

        response::Commit {
            data: Bytes::new(), // ignored since v0.38
            retain_height: 0u32.into(),
        }
    }
}

// Recall that `tower` is about creating `Service`s. Here what we are doing is turning our
// `KVStore` struct into a `tower` service. This then allows us to compose it with the rest of the
// `tower` ecosystem, which we do later in `main()`.
impl Service<Request> for Application {
    type Response = Response;

    type Error = BoxError;

    type Future = Pin<Box<dyn Future<Output = Result<Response, BoxError>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        tracing::debug!(?req);

        let res = match req {
            Request::Info(info) => Response::Info(self.info(info)),
            Request::Query(query) => Response::Query(self.query(query)),
            Request::CheckTx(check_tx) => Response::CheckTx(Self::check_tx(check_tx)),
            Request::PrepareProposal(proposal) => {
                Response::PrepareProposal(Self::prepare_proposal(proposal))
            }
            Request::ProcessProposal(proposal) => {
                Response::ProcessProposal(Self::process_proposal(proposal))
            }
            Request::FinalizeBlock(block) => Response::FinalizeBlock(self.finalize_block(block)),
            Request::Commit => Response::Commit(self.commit()),

            Request::Flush => Response::Flush,
            Request::InitChain(_) => Response::InitChain(Default::default()),
            Request::Echo(echo) => Response::Echo(Echo {
                message: echo.message,
            }),

            Request::ListSnapshots => Response::ListSnapshots(Default::default()),
            Request::OfferSnapshot(_) => Response::ListSnapshots(Default::default()),
            Request::LoadSnapshotChunk(_) => Response::LoadSnapshotChunk(Default::default()),
            Request::ApplySnapshotChunk(_) => Response::ApplySnapshotChunk(Default::default()),
            Request::ExtendVote(_) => Response::ExtendVote(ExtendVote {
                vote_extension: Bytes::new(),
            }),
            Request::VerifyVoteExtension(_) => {
                Response::VerifyVoteExtension(VerifyVoteExtension::Accept)
            }
        };

        Box::pin(async move { Ok(res) })
    }
}
