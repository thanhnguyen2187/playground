use std::process::exit;
use clap::Parser;
use cli::parse_addr::parse_addr;
use snafu::ResultExt;

mod cli {
    pub mod parse_addr;
}

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// The address of the server
    #[arg(long, default_value_t = String::from("127.0.0.1:4004"), global = true)]
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
    parse_addr(&cli.addr)?;
    let client = reqwest::Client::new();
    let addr = if cli.addr.starts_with("http://") || cli.addr.starts_with("https://") {
        cli.addr
    } else {
        format!("http://{}", cli.addr)
    };

    match cli.command {
        Commands::Get { key } => {
            let resp = client
                .get(format!("{}/v1/get/{}", addr, key))
                .send()
                .await
                .with_whatever_context(|_| "Unable to connect to server")?
                .text()
                .await
                .with_whatever_context(|_| "Unable to read response from server")?;
            println!("{}", resp);
        }
        Commands::Set { key, value } => {
            let resp = client
                .post(format!("{}/v1/set/{}/{}", addr, key, value))
                .send()
                .await
                .with_whatever_context(|_| "Unable to connect to server")?;
            if resp.status().is_success() {
                print!(
                    "{}",
                    resp.text()
                        .await
                        .with_whatever_context(|_| "Unable to read response from server")?
                );
            } else {
                eprint!(
                    "{}",
                    resp.text()
                        .await
                        .with_whatever_context(|_| "Unable to read response from server")?
                );
                exit(1);
            }
        }
        Commands::Rm { key } => {
            let resp = client
                .post(format!("{}/v1/rm/{}", addr, key))
                .send()
                .await
                .with_whatever_context(|_| "Unable to connect to server")?;
            if resp.status().is_success() {
                let resp = resp
                    .text()
                    .await
                    .with_whatever_context(|_| "Unable to read response from server")?;
                print!("{}", resp);
            } else {
                let resp = resp
                    .text()
                    .await
                    .with_whatever_context(|_| "Unable to read response from server")?;
                eprint!("{}", resp);
                exit(1);
            }
        }
    }

    Ok(())
}
