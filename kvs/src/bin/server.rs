use axum::routing::{get, post};
use axum::Router;
use clap::{Parser};
use cli::engine::{check_engine_db_file, Engine};
use cli::parse_addr::parse_addr;
use cli::server::Server;
use env_logger::Env;
use kvs::{KvStoreV2, KvsEngine, MemStore, Result, SledStore};
use log::{error, info};
use server::app_state::AppState;
use server::handlers;
use std::env;
use std::sync::{Arc, RwLock};

mod server {
    pub mod app_state;
    pub mod handlers;
}

mod cli {
    pub mod parse_addr;

    pub mod engine;
    pub mod server;
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    info!("Logger initialized!");
    info!("Current binary version: {:?}", env!("CARGO_PKG_VERSION"));

    let cli = Server::parse();
    // TODO: validate engine by Clap instead of hard-coding
    parse_addr(&cli.addr)?;
    info!("Started server at: {:?}", cli.addr);
    info!("Chosen engine: {:?}", {
        match cli.engine {
            Engine::Kvs => "kvs",
            Engine::Sled => "sled",
            Engine::Mem => "in-memory",
        }
    });

    if let Err(err) = check_engine_db_file(&cli.engine) {
        error!(
            "Database file of engines other than {} already exists",
            cli.engine,
        );
        return Err(err);
    }

    let current_dir = env::current_dir().unwrap();
    let shared_state = AppState {
        store: match cli.engine {
            Engine::Kvs => Arc::new(RwLock::new(KvStoreV2::open(current_dir.as_path())?)),
            Engine::Sled => Arc::new(RwLock::new(SledStore::open(current_dir.as_path())?)),
            Engine::Mem => Arc::new(RwLock::new(MemStore::new())),
        },
    };

    let app = Router::new()
        .route("/v1/get/{key}", get(handlers::get))
        .route("/v1/set/{key}/{value}", post(handlers::set))
        .route("/v1/rm/{key}", post(handlers::remove))
        .fallback(handlers::not_found)
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(cli.addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
