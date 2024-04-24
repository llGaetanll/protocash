use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use bytes::Bytes;

use clap::arg;
use clap::Parser;

use tower::Service;
use tower::ServiceBuilder;
use tower_abci::v037::{split, Server};
use tower_abci::BoxError;

use tendermint::abci::Event;
use tendermint::abci::EventAttributeIndexExt;
use tendermint::v0_37::abci::request::Request;
use tendermint::v0_37::abci::response;
use tendermint::v0_37::abci::response::Response;

#[derive(Default)]
struct KVStore {
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

        let [key, value]: [&str; 2] = tx.split('=').collect::<Vec<_>>().try_into().expect("was not of the form key value!"); // TODO: no panic in production!!

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
        let res = match req {
            // handled messages
            Request::Info(_) => Response::Info(self.info()),
            Request::Query(query) => Response::Query(self.query(query.data)),
            Request::DeliverTx(deliver_tx) => Response::DeliverTx(self.deliver_tx(deliver_tx.tx)),
            Request::Commit => Response::Commit(self.commit()),

            // unhandled messages
            Request::Flush => Response::Flush,
            Request::Echo(_) => Response::Echo(Default::default()),
            Request::InitChain(_) => Response::InitChain(Default::default()),
            Request::BeginBlock(_) => Response::BeginBlock(Default::default()),
            Request::CheckTx(_) => Response::CheckTx(Default::default()),
            Request::EndBlock(_) => Response::EndBlock(Default::default()),
            Request::ListSnapshots => Response::ListSnapshots(Default::default()),
            Request::OfferSnapshot(_) => Response::OfferSnapshot(Default::default()),
            Request::LoadSnapshotChunk(_) => Response::LoadSnapshotChunk(Default::default()),
            Request::ApplySnapshotChunk(_) => Response::ApplySnapshotChunk(Default::default()),

            // Note: https://github.com/tendermint/tendermint/blob/v0.37.x/spec/abci/abci%2B%2B_tmint_expected_behavior.md#adapting-existing-applications-that-use-abci
            Request::PrepareProposal(prepare_prop) => {
                println!("received: {:?}", prepare_prop);

                Response::PrepareProposal(response::PrepareProposal {
                    txs: prepare_prop.txs,
                })
            }
            Request::ProcessProposal(..) => {
                Response::ProcessProposal(response::ProcessProposal::Accept)
            }
        };

        Box::pin(async move { Ok(res) })
    }
}

/// A simple KVStore example on tendermint.
#[derive(Parser, Debug)]
struct Args {
    /// Binds the TCP server to this host
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value = "26658")]
    port: u16,

    /// Binds the UDS server to this path
    #[arg(long)]
    uds: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let service = KVStore::default();

    let (consensus, mempool, snapshot, info) = split::service(service, 1);

    // Hand those components to the ABCI server, but customize request behavior
    // for each category -- for instance, apply load-shedding only to mempool
    // and info requests, but not to consensus requests.
    let server_builder = Server::builder()
        .consensus(consensus)
        .snapshot(snapshot)
        .mempool(
            ServiceBuilder::new()
                .load_shed()
                .buffer(10)
                .service(mempool),
        )
        .info(
            ServiceBuilder::new()
                .load_shed()
                .buffer(100)
                .rate_limit(50, std::time::Duration::from_secs(1))
                .service(info),
        );

    let server = server_builder.finish().unwrap();

    if let Some(uds_path) = args.uds {
        server.listen_unix(uds_path).await.unwrap();
    } else {
        server
            .listen_tcp(format!("{}:{}", args.host, args.port))
            .await
            .unwrap();
    }
}
