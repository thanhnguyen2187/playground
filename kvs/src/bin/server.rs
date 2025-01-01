use axum::routing::get;
use axum::Router;
use clap::{Parser, ValueEnum};
use env_logger;
use env_logger::Env;
use kvs::Result;
use log::{error, info};
use snafu::whatever;
use std::fmt::Display;
use std::net::SocketAddr;

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

fn validate_addr(addr: &str) -> Result<()> {
    match addr.parse::<SocketAddr>() {
        Ok(_) => Ok(()),
        Err(_) => whatever!(
            "Invalid binding address; expected [ip-v4-host]:[port]; got {}",
            addr,
        ),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    info!("Logger initialized!");
    info!("Current binary version: {:?}", env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();
    // TODO: validate engine by Clap instead of hard-coding
    validate_addr(&cli.addr)?;
    info!("Started server at: {:?}", cli.addr);
    info!("Chosen engine: {:?}", {
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
