use kvs::err::Result;
use std::collections::HashMap;
use std::path::Path;
use clap_derive::ValueEnum;
use snafu::whatever;
use std::fmt::Display;

#[derive(ValueEnum, PartialEq, Eq, Hash, Default, Debug, Clone)]
pub enum Engine {
    /// A custom key-value store
    #[default]
    Kvs,
    /// A key-value store using sled
    Sled,
    /// A key-value store using in-memory
    Mem,
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Engine::Kvs => write!(f, "kvs"),
            Engine::Sled => write!(f, "sled"),
            Engine::Mem => write!(f, "in-memory"),
        }
    }
}

/// Checks for the existence of other engines' database files. For example, if we are using
/// `kvs`, then `sled` database file should not exist and vice versa.
pub fn check_engine_db_file(engine: &Engine) -> Result<()> {
    let engine_db_files: HashMap<Engine, _> = HashMap::from([
        (Engine::Kvs, Path::new(kvs::DEFAULT_FILE_NAME_KVS).exists()),
        (
            Engine::Sled,
            Path::new(kvs::DEFAULT_FILE_NAME_SLED).exists(),
        ),
    ]);
    for (engine_checking, db_file_exists) in engine_db_files {
        if engine_checking != *engine && db_file_exists {
            whatever!(
                "Current engine is {} while database file for engine {} existed",
                engine,
                engine_checking,
            );
        }
    }

    Ok(())
}