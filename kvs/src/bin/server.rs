use axum::routing::get;
use axum::Router;
use clap::{Parser, ValueEnum};
use std::fmt::Display;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    /// The address of the server
    #[arg(long, default_value_t = String::from("127.0.0.1:4004"))]
    addr: String,

    /// The underlying engine to use
    #[arg(long, default_value_t)]
    engine: Engine,
}

#[derive(ValueEnum, Default, Debug, Clone)]
enum Engine {
    /// A custom key-value store
    #[default]
    Kvs,
    /// A key-value store using sled
    Sled,
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::Kvs => write!(f, "kvs"),
            Engine::Sled => write!(f, "sled"),
        }
    }
}

#[tokio::main]
async fn main() -> kvs::Result<()> {
    let cli = Cli::parse();

    eprintln!("Current version: {:?}", env!("CARGO_PKG_VERSION"));
    eprintln!("Started server at: {:?}", cli.addr);
    eprintln!("Used engine: {:?}", {
        match cli.engine {
            Engine::Kvs => "kvs",
            Engine::Sled => "sled",
        }
    });

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind(cli.addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
