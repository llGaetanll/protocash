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
use tower::Service;
use tower_abci::BoxError;

#[derive(Default)]
pub struct State {
    store: HashMap<i32, i32>,
    ongoingblock: HashMap<i32, i32>,
    size: u32,
    height: u32,
    hash: Vec<u8>,
}

pub enum TxError {
    AlreadySpent,
}

impl State {
    fn hash(&self) -> Vec<u8> {
        todo!()

        // let hasher = Sha256::new();
        //
        // let bytes: Vec<u8> = self
        //     .store
        //     .iter()
        //     .flat_map(|(k, v)| k.bytes().chain(v.bytes()))
        //     .collect();
        //
        // let hasher = hasher
        //     .chain_update(bytes.as_slice()) // add the store
        //     .chain_update(self.height.to_ne_bytes()) // add the height
        //     .chain_update(self.size.to_ne_bytes()); // add the size
        //
        // hasher.finalize().to_vec() // TODO: should be [u8; 32] for SHA256
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
        let request::Info {
            version,
            block_version,
            p2p_version,
            abci_version,
        } = info;

        // CometBFT expects the application to persist validators. On startup, we need to load them
        // if they exist.
        // TODO

        response::Info {
            data: String::from("498c-protocash"),
            version: String::from("0.1.0"),
            app_version: 1,
            last_block_height: self.state.height.into(),
            last_block_app_hash: self.state.hash().try_into().unwrap(),
        }
    }

    fn is_valid(tx: &Bytes) -> Code {
        let tx_str = String::from_utf8_lossy(tx);

        match TryInto::<[&str; 2]>::try_into(tx_str.split('=').collect::<Vec<_>>()) {
            Ok([left, right]) => {
                if left.parse::<i32>().is_ok() && right.parse::<i32>().is_ok() {
                    Code::Ok
                } else {
                    Code::Err(unsafe { NonZeroU32::new_unchecked(1) })
                }
            }
            _ => Code::Err(unsafe { NonZeroU32::new_unchecked(1) }),
        }
    }

    fn query(&self, query: request::Query) -> response::Query {
        let request::Query {
            data: _data,
            path: _path,
            height,
            prove: _prove,
        } = query;

        let key = 1;
        let value = self.state.store.get(&key).unwrap_or(&1);

        response::Query {
            code: Code::Ok,
            log: String::from("exists"),
            info: String::new(),
            index: 0,
            key: Bytes::copy_from_slice(&key.to_be_bytes()),
            value: Bytes::copy_from_slice(&value.to_be_bytes()),
            proof: None,
            height,
            codespace: String::new(),
        }
    }

    fn check_tx(tx: request::CheckTx) -> response::CheckTx {
        let request::CheckTx {
            tx: data,
            kind: _kind,
        } = tx;

        response::CheckTx {
            code: Self::is_valid(&data),
            data,
            ..Default::default()
        }
    }

    fn finalize_block(&mut self, block: request::FinalizeBlock) -> response::FinalizeBlock {
        let request::FinalizeBlock { txs, .. } = block;

        let mut tx_results: Vec<ExecTxResult> = Vec::with_capacity(txs.len());

        for tx in txs {
            let code = Self::is_valid(&tx);

            if let Code::Ok = code {
                let s = String::from_utf8_lossy(&tx);

                // TODO: redundant logic with is_valid
                let [k, v]: [&str; 2] = s.split('=').collect::<Vec<_>>().try_into().unwrap(); // won't fail since valid

                // won't fail because of is_valid
                self.state
                    .store
                    .insert(k.parse().unwrap(), v.parse().unwrap());
            }

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
        self.state
            .store
            .extend(self.state.ongoingblock.iter().map(|(k, v)| (*k, *v)));

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
        let res = match req {
            Request::Info(info) => Response::Info(self.info(info)),
            Request::Query(query) => Response::Query(self.query(query)),
            Request::CheckTx(check_tx) => Response::CheckTx(Self::check_tx(check_tx)),
            Request::PrepareProposal(proposal) => Response::PrepareProposal(Self::prepare_proposal(proposal)),
            Request::ProcessProposal(proposal) => Response::ProcessProposal(Self::process_proposal(proposal)),
            Request::FinalizeBlock(block) => Response::FinalizeBlock(self.finalize_block(block)),
            Request::Commit => Response::Commit(self.commit()),

            Request::Flush => Response::Flush,
            Request::InitChain(_) => Response::InitChain(Default::default()),
            Request::Echo(echo) => Response::Echo(Echo { message: echo.message }),

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
