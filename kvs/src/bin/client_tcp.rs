use clap::Parser;
use cli::parse_addr::parse_addr;
use log::info;
use snafu::ResultExt;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;

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
    let mut stream = TcpStream::connect(&cli.addr)
        .with_whatever_context(|_| format!("Unable to connect to server at {}", &cli.addr))?;

    match cli.command {
        Commands::Get { key } => {
            write!(stream, "GET {}", key)
                .with_whatever_context(|_| format!("Unable to write to stream at {}", &cli.addr))?;
            let mut response = String::new();
            // Shut down the write part of the stream to indicate that we are
            // done writing and that we want to read the response. Without this,
            // the client will not receive any response from the server.
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
            let parts = response.split_once(' ');
            match parts {
                Some(("OK", value)) => println!("{}", value),
                Some(("ERR", value)) => println!("{}", value),
                _ => {
                    eprintln!("Unknown response: {}", response);
                }
            }
        }
        Commands::Set { key, value } => {
            write!(stream, "SET {} {}", key, value)
                .with_whatever_context(|_| format!("Unable to write to stream at {}", &cli.addr))?;
        }
        Commands::Rm { key } => {
            write!(stream, "RM {}", key)
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
            if response.starts_with("ERR") {
                eprintln!("{}", response.strip_prefix("ERR ").unwrap());
                exit(1);
            }
        }
    }

    Ok(())
}
