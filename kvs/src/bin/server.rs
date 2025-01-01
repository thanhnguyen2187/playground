use axum::routing::get;
use axum::Router;
use clap::{Parser, ValueEnum};
use env_logger;
use env_logger::Env;
use kvs::Result;
use log::{error, info};
use snafu::whatever;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::net::SocketAddr;
use std::path::Path;

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

#[derive(ValueEnum, PartialEq, Eq, Hash, Default, Debug, Clone)]
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

/// Checks for the existence of other engines' database files. For example, if we are using
/// `kvs`, then `sled` database file should not exist and vice versa.
fn check_engine_db_file(engine: &Engine) -> Result<()> {
    let engine_db_files: HashMap<Engine, _> = HashMap::from([
        (Engine::Kvs, Path::new(kvs::DEFAULT_FILE_NAME_KVS).exists()),
        (
            Engine::Sled,
            Path::new(kvs::DEFAULT_FILE_NAME_SLED).exists(),
        ),
    ]);
    for (engine_checking, db_file_exists) in engine_db_files {
        if engine_checking != *engine && db_file_exists {
            return whatever!(
                "Current engine is {} while database file for engine {} existed",
                engine,
                engine_checking,
            );
        }
    }

    Ok(())
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

    if let Err(err) = check_engine_db_file(&cli.engine) {
        error!(
            "Database file of engines other than {} already exists",
            cli.engine,
        );
        return Err(err);
    }

    let current_dir = env::current_dir().unwrap();
    let mut app = Router::new();
    match cli.engine {
        Engine::Kvs => {
            let _ = kvs::KvStoreV2::open(current_dir.as_path())?;
            //     app = app.route("/kvs/get/:key", get(kvs_get));
            //     app = app.route("/kvs/set/:key", get(kvs_set));
            //     app = app.route("/kvs/delete/:key", get(kvs_delete));
        }
        Engine::Sled => {
            let _ = kvs::SledStore::open(current_dir.as_path())?;
            // app = app.route("/sled/get/:key", get(sled_get));
            // app = app.route("/sled/set/:key", get(sled_set));
            // app = app.route("/sled/delete/:key", get(sled_delete));
        }
    }
    let listener = tokio::net::TcpListener::bind(cli.addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
