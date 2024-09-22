use clap::arg;
use clap::Parser;
use tower::ServiceBuilder;
use tower_abci::split;
use tower_abci::Server;

mod app;

use app::Application;

/// A simple KVStore example on cometbft.
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
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let args = Args::parse();
    let service = Application::default();

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
