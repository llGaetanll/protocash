use std::collections::BTreeSet;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use bytes::Bytes;

use rand::Rng;

use cometbft::validator::Update;
use cometbft::PublicKey;
use tower::Service;
use tower_abci::BoxError;

use cometbft::abci::v1::request::Request;
use cometbft::abci::v1::response;
use cometbft::abci::v1::response::Response;

use cometbft::abci::v1::response::ExtendVote;
use cometbft::abci::v1::response::FinalizeBlock;
use cometbft::abci::v1::response::PrepareProposal;
use cometbft::abci::v1::response::ProcessProposal;
use cometbft::abci::v1::response::VerifyVoteExtension;

use crate::types::Tx;
use crate::types::TxHash;
use crate::types::TxRequest;

#[derive(Default)]
pub struct State {
    /// a list of transactions. You might think that this should be a MerkleTree, but in fact
    /// ArkWorks' MerkleTree size is fixed at runtime, so we actually just store the txs in a
    /// list, and build the tree for each new tx. This sucks but idk how else to do it.
    txs: Vec<Tx>,

    /// We check in this set to see if a tx is already spent
    spents: BTreeSet<u64>,

    height: u32,
    size: u32,
}

pub enum TxError {
    AlreadySpent
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

    /// Add a tx to the state. Checks that the tx is not already spent.
    pub fn add_tx(&mut self, tx: TxRequest) -> Result<(), TxError> {
        // generate a random serial_number
        let mut rng = rand::thread_rng();
        let serial_number: TxHash = rng.gen();

        // verify that the tx is not already spent
        if self.spents.contains(&tx.prev_tx) {
            return Err(TxError::AlreadySpent)
        }

        self.spents.insert(tx.prev_tx);

        let tx = Tx {
            prev_tx: tx.prev_tx,
            to: tx.to,
            serial_number,
        };

        self.txs.push(tx);

        Ok(())
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
    fn info(&self) -> response::Info {
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

    fn query(&self, query: Bytes) -> response::Query {
        todo!()
    }

    fn deliver_tx(&mut self, tx: Bytes) -> response::DeliverTx {
        todo!()
    }

    fn commit(&mut self) -> response::Commit {
        todo!()
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
        // println!("got {:?}", req);

        let res = match req {
            Request::Info(_) => Response::Info(self.info()),
            Request::Query(_) => Response::Query(Default::default()),
            Request::Commit => Response::Commit(Default::default()),
            Request::Echo(_) => Response::Echo(Default::default()),
            Request::Flush => Response::Flush,
            Request::InitChain(_) => Response::InitChain(Default::default()),
            Request::CheckTx(_) => Response::CheckTx(Default::default()),
            Request::ListSnapshots => Response::ListSnapshots(Default::default()),
            Request::OfferSnapshot(_) => Response::ListSnapshots(Default::default()),
            Request::LoadSnapshotChunk(_) => Response::LoadSnapshotChunk(Default::default()),
            Request::ApplySnapshotChunk(_) => Response::ApplySnapshotChunk(Default::default()),
            Request::PrepareProposal(proposal) => {
                Response::PrepareProposal(PrepareProposal { txs: proposal.txs })
            }
            Request::ProcessProposal(_) => Response::ProcessProposal(ProcessProposal::Accept),
            Request::ExtendVote(_) => Response::ExtendVote(ExtendVote {
                vote_extension: Bytes::new(),
            }),
            Request::VerifyVoteExtension(_) => {
                Response::VerifyVoteExtension(VerifyVoteExtension::Accept)
            }
            Request::FinalizeBlock(_) => Response::FinalizeBlock(FinalizeBlock {
                events: vec![],
                tx_results: vec![],
                validator_updates: vec![],
                consensus_param_updates: None,
                app_hash: Default::default(),
            }),
        };

        Box::pin(async move { Ok(res) })
    }
}
