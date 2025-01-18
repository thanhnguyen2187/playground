use clap::Parser;
use cli::parse_addr::parse_addr;
use snafu::ResultExt;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use log::info;

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

fn main() -> kvs::Result<()> {
    let cli = Cli::parse();
    parse_addr(&cli.addr)?;
    let mut stream = TcpStream::connect(&cli.addr).with_whatever_context(|_| {
        format!("Unable to connect to server at {}", &cli.addr)
    })?;

    match cli.command {
        Commands::Get { key } => {
            write!(stream, "GET {}", key)
                .with_whatever_context(|_| format!("Unable to write to stream at {}", &cli.addr))?;
            let mut response = String::new();
            stream
                .shutdown(Shutdown::Write)
                .with_whatever_context(|err| {
                    format!("Unable to shut down stream at {}: {}", &cli.addr, err)
                })?;
            stream
                .read_to_string(&mut response)
                .with_whatever_context(|_| {
                    format!("Unable to read response from server at {}", &cli.addr)
                })?;
            println!("{}", response);
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
