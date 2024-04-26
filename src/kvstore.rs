use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use bytes::Bytes;

use clap::arg;
use clap::Parser;

use tower::Service;
use tower_abci::BoxError;

use cometbft::abci::v1::request::Request;
use cometbft::abci::v1::response;
use cometbft::abci::v1::response::Response;
use cometbft::abci::Event;
use cometbft::abci::EventAttributeIndexExt;

use cometbft::abci::v1::response::ExtendVote;
use cometbft::abci::v1::response::FinalizeBlock;
use cometbft::abci::v1::response::PrepareProposal;
use cometbft::abci::v1::response::ProcessProposal;
use cometbft::abci::v1::response::VerifyVoteExtension;

#[derive(Default)]
pub struct KVStore {
    store: HashMap<String, String>,

    // this is the height of the current block. Obviously this is just a key value store, so we're
    // not *really* storing blocks, but we need this parameter because comet needs it.
    height: u32,

    // TODO: I'm not really sure what this field is for, to be honest. Comet needs it for whatever
    // reason. Info on this would be nice.
    app_hash: [u8; 8],
}

// These are the functions that our KVStore struct implements.
impl KVStore {
    fn info(&self) -> response::Info {
        response::Info {
            data: String::from("498c-kvstore-example-data"),
            version: String::from("0.1.0"),
            app_version: 1,
            last_block_height: self.height.into(),
            last_block_app_hash: self.app_hash.to_vec().try_into().unwrap(),
        }
    }

    fn query(&self, query: Bytes) -> response::Query {
        let key = String::from_utf8(query.to_vec()).unwrap(); // TODO: no `unwrap`s in production!!

        let (value, log) = match self.store.get(&key) {
            Some(value) => (value.clone(), "value exists"), // NOTE: `clone` is cheap here
            None => (String::new(), "value does not exist"),
        };

        response::Query {
            log: log.to_string(),
            key: query,
            value: value.into_bytes().into(),
            ..Default::default()
        }
    }

    fn deliver_tx(&mut self, tx: Bytes) -> response::DeliverTx {
        let tx = String::from_utf8(tx.to_vec()).unwrap(); // TODO: no `unwrap`s in production!!

        let [key, value]: [&str; 2] = tx
            .split('=')
            .collect::<Vec<_>>()
            .try_into()
            .expect("was not of the form key value!"); // TODO: no panic in production!!

        self.store.insert(key.to_string(), value.to_string());

        response::DeliverTx {
            events: vec![Event::new(
                "app",
                vec![
                    ("key", key).index(),
                    ("index_key", "index is working").index(),
                    ("noindex_key", "index is working").no_index(),
                ],
            )],
            ..Default::default()
        }
    }

    fn commit(&mut self) -> response::Commit {
        let retain_height = self.height.into();

        self.app_hash = (self.store.len() as u64).to_be_bytes();
        self.height += 1;

        response::Commit {
            data: self.app_hash.to_vec().into(),
            retain_height,
        }
    }
}

// Recall that `tower` is about creating `Service`s. Here what we are doing is turning our
// `KVStore` struct into a `tower` service. This then allows us to compose it with the rest of the
// `tower` ecosystem, which we do later in `main()`.
impl Service<Request> for KVStore {
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
        println!("got {:?}", req);

        let res = match req {
            Request::Info(_) => Response::Info(Default::default()),
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
            Request::PrepareProposal(proposal) => Response::PrepareProposal(PrepareProposal { txs: proposal.txs }),
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
