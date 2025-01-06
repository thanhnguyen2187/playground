use clap::Parser;
use snafu::ResultExt;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// The address of the server
    #[arg(default_value_t = String::from("127.0.0.1:4004"))]
    addr: String,
}

#[derive(Parser)]
enum Commands {
    /// Get a value from the store
    Get {
        /// The key to be retrieved
        key: String,
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
        key: String,
    },
}

#[tokio::main]
async fn main() -> kvs::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get { key: _ } => {
            let resp = reqwest::get(cli.addr).await.with_whatever_context(
                |_| "Unable to connect to server",
            )?.text().await.with_whatever_context(
                |_| "Unable to read response from server",
            );
            println!("{}", resp?);
        }
        Commands::Set { key: _, value: _ } => {
            unimplemented!()
        }
        Commands::Rm { key: _ } => {
            unimplemented!()
        }
    }

    Ok(())
}
