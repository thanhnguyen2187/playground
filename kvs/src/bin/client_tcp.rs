use clap::Parser;
use cli::parse_addr::parse_addr;
use snafu::ResultExt;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::time::Duration;
use tokio::io::AsyncReadExt;

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
            // stream
            //     .set_read_timeout(Some(Duration::new(1, 0)))
            //     .with_whatever_context(|e| format!("Unable to set stream read timeout {}", e))?;
            write!(stream, "GET {}\n", key)
                .with_whatever_context(|_| format!("Unable to write to stream at {}", &cli.addr))?;
            let mut response = String::new();
            stream
                .read_to_string(&mut response)
                .with_whatever_context(|_| {
                    format!("Unable to read response from server at {}", &cli.addr)
                })?;
            drop(stream);
            // let mut vec = Vec::new();
            // buf_reader.read_to_end(&mut vec).with_whatever_context(|_| {
            //     format!("Unable to read response from server at {}", &cli.addr)
            // })?;
            // let response = String::from_utf8(vec).with_whatever_context(|_| {
            //     format!("Unable to parse response from server at {}", &cli.addr)
            // })?;
            println!("{}", response);
        }
        Commands::Set { key, value } => {
            unimplemented!()
        }
        Commands::Rm { key } => {
            unimplemented!()
        }
    }

    Ok(())
}
