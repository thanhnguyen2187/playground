use std::path::Path;
use clap::{Parser};

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Get a value from the store
    Get {
        /// The key to be retrieved
        key: String
    },
    /// Set a value in the store
    Set {
        /// The key to be set
        key: String,
        /// The value to set
        value: String,
    },
    /// Remove a value from the store
    Rm {
        /// The key to be removed
        key: String
    },
}

fn main() -> kvs::Result<()> {
    let cli = Cli::parse();
    let mut store = kvs::KvStore::open(Path::new(env!("PWD")))?;

    match cli.command {
        Commands::Get { key } => {
            let result =
                store.get(key.clone());
            match result {
                Ok(Some(value)) => println!("{}", value),
                Ok(None) => println!("Key not found"),
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::Set { key, value } => {
            store.set(key.clone(), value.clone())?;
            // println!("Set key {} to value {}", key, value);
        }
        Commands::Rm { key: _ } => panic!("unimplemented"),
    }

    Ok(())
}
