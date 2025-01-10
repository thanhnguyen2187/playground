use clap::Parser;
use cli::engine::{check_engine_db_file, Engine};
use cli::parse_addr::parse_addr;
use cli::server::Server;
use env_logger::Env;
use kvs::{Command, Result};
use log::{error, info};
use snafu::{ResultExt, Whatever};
use std::env;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};

mod cli {
    pub mod engine;
    pub mod parse_addr;
    pub mod server;
}

/// Turns the incoming stream into readable words represented as a vector of
/// strings.
fn tokenize<T: Read>(stream: T) -> Result<Vec<String>> {
    let buf_reader = BufReader::new(stream);
    let words = buf_reader
        .split(b' ')
        .map(|vec_result| {
            let vec: Vec<u8> = vec_result.with_whatever_context::<_, &str, kvs::Error>(|_| {
                "Failed to parse stream to u8 vector"
            })?;
            let word: String = String::from_utf8(vec)
                .with_whatever_context::<_, &str, kvs::Error>(|_| {
                    "Failed to parse u8 vector to UTF-8"
                })?;
            Ok::<String, kvs::Error>(word.trim().to_owned())
        })
        .collect::<Result<Vec<String>>>()?
        .into_iter()
        .filter(|word| !word.is_empty())
        .collect();

    Ok(words)
}

fn parse(words: Vec<String>) -> Result<Vec<Command>> {
    unimplemented!()
}

#[cfg(test)]
mod pure_fns {
    use super::*;

    mod tokenize {
        use super::*;

        #[test]
        fn success() {
            let test_table = vec![
                (
                    "word1 word2 word3",
                    vec![
                        "word1".to_string(),
                        "word2".to_string(),
                        "word3".to_string(),
                    ],
                ),
                (
                    "  word1   word2  word3    ",
                    vec![
                        "word1".to_string(),
                        "word2".to_string(),
                        "word3".to_string(),
                    ],
                ),
                (
                    "word1\nword2 word3",
                    vec!["word1\nword2".to_string(), "word3".to_string()],
                ),
            ];

            for (input, expected) in test_table {
                let stream = Cursor::new(input.as_bytes());
                let got = tokenize(stream).unwrap();
                assert_eq!(got, expected);
            }
        }
    }
}

fn response_connection(stream: TcpStream, response: Vec<String>) {
    let mut buf_writer = BufWriter::new(&stream);
    for line in response {
        buf_writer.write(line.as_bytes()).unwrap();
        buf_writer.write(b"\n").unwrap();
    }
}

fn main() -> Result<()> {
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

    // let current_dir = env::current_dir().unwrap();
    let listener = TcpListener::bind(cli.addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let lines = tokenize(stream.try_clone().unwrap())?;
        println!("Received request: {:?}", lines);
        response_connection(stream, lines);
    }

    Ok(())
}
