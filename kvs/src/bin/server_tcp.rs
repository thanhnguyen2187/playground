use clap::Parser;
use cli::engine::{check_engine_db_file, Engine};
use cli::parse_addr::parse_addr;
use cli::server::Server;
use env_logger::Env;
use kvs::{
    evaluate_command, Command, CommandResponse, KvStoreV2, KvsEngine, MemStore, Result, SledStore,
};
use log::{error, info, warn};
use snafu::{whatever, ResultExt, Whatever};
use std::env;
use std::io::{BufRead, BufReader, BufWriter, Cursor, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::ops::DerefMut;
use tokio::io::AsyncWriteExt;

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
        .filter(|result| match result {
            Ok(word) => !word.is_empty(),
            Err(_) => true,
        })
        .collect::<Result<Vec<String>>>()?;

    Ok(words)
}

fn parse(words: Vec<String>) -> Result<Command> {
    match &words[..] {
        [command_str, key] if command_str.to_uppercase() == "GET" => {
            Ok(Command::Get { key: key.clone() })
        }
        [command_str, key, value] if command_str.to_uppercase() == "SET" => Ok(Command::Set {
            key: key.clone(),
            value: value.clone(),
        }),
        [command_str, key] if command_str.to_uppercase() == "RM" => {
            Ok(Command::Rm { key: key.clone() })
        }
        _ => whatever!("Invalid command"),
    }
}

fn respond<T: Write>(stream: &mut T, command_response: CommandResponse) -> Result<()> {
    match command_response {
        CommandResponse::Get { value } => {
            let mut buf_writer = BufWriter::new(stream);
            if let Some(value) = value {
                buf_writer
                    .write(format!("OK {}", value).as_bytes())
                    .with_whatever_context(|e| format!("Error happened writing to stream {}", e))?;
            } else {
                buf_writer
                    .write("ERR Key not found".as_bytes())
                    .with_whatever_context(|e| format!("Error happened writing to stream {}", e))?;
            }
            buf_writer
                .flush()
                .with_whatever_context(|e| format!("Error happened flushing {}", e))?;
        }
        CommandResponse::Set {} => {}
        CommandResponse::Rm { value } => {
            if value.is_none() {
                let mut buf_writer = BufWriter::new(stream);
                buf_writer
                    .write("ERR Key not found".as_bytes())
                    .with_whatever_context(|e| format!("Error happened writing to stream {}", e))?;
                buf_writer
                    .flush()
                    .with_whatever_context(|e| format!("Error happened flushing {}", e))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
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

    mod parse {
        use super::*;

        #[test]
        fn success() {
            let test_table = vec![
                (
                    "GET key1".to_string(),
                    Command::Get {
                        key: "key1".to_string(),
                    },
                ),
                (
                    "SET key1 value1".to_string(),
                    Command::Set {
                        key: "key1".to_string(),
                        value: "value1".to_string(),
                    },
                ),
                (
                    "RM key1".to_string(),
                    Command::Rm {
                        key: "key1".to_string(),
                    },
                ),
                (
                    "   GET   spaced-key-command    ".to_string(),
                    Command::Get {
                        key: "spaced-key-command".to_string(),
                    },
                ),
            ];

            for (input, expected) in test_table {
                let input_stream = Cursor::new(input.as_bytes());
                let words = tokenize(input_stream).unwrap();
                let got = parse(words).unwrap();
                assert_eq!(got, expected);
            }
        }
    }

    mod respond {
        use super::*;

        #[test]
        fn success() {
            let command_response = CommandResponse::Get {
                value: Some("value1".to_string()),
            };
            let mut stream = Cursor::new(Vec::new());

            respond(&mut stream, command_response).unwrap();
            let got = String::from_utf8(stream.into_inner()).unwrap();

            assert_eq!(got, "value1")
        }
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
    let current_dir = env::current_dir().unwrap();
    info!("Current directory: {:?}", &current_dir);

    if let Err(err) = check_engine_db_file(&cli.engine) {
        error!(
            "Database file of engines other than {} already exists",
            cli.engine,
        );
        return Err(err);
    }

    let mut store: Box<dyn KvsEngine> = match cli.engine {
        Engine::Kvs => Box::new(KvStoreV2::open(current_dir.as_path())?),
        Engine::Sled => Box::new(SledStore::open(current_dir.as_path())?),
        Engine::Mem => Box::new(MemStore::new()),
    };

    let listener = TcpListener::bind(cli.addr).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.with_whatever_context(|err| {
            format!("Failed to accept incoming connection: {}", err)
        })?;
        let words = tokenize(&stream)?;
        info!("Received words: {:?}", words);
        let command = parse(words)?;
        info!("Parsed command: {:?}", command);
        let command_response = evaluate_command(&command, store.deref_mut())?;
        info!("Response: {:?}", command_response);
        respond(&mut stream, command_response)?;
        info!("Sent response");
        // stream.shutdown(Shutdown::Both).with_whatever_context(
        //     |err| "Unable to shut down stream",
        // )?;
    }

    Ok(())
}
