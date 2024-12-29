mod err;

pub use err::{Result, Error};
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::Write;
use serde::{Serialize, Deserialize};

pub struct KvStore {
    underlying: HashMap<String, String>,
    log_path: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

pub fn serialize_commands(commands: &[Command]) -> Result<String> {
    let mut output = String::new();
    for command in commands {
        if let Command::Get { key: _ } = command {
            return Err(Error::CommandSerialize {
                command: format!("{:?}", command),
                err_str: "Get command cannot be serialized".to_owned(),
            });
        }

        let command_str = serde_json::to_string(command).map_err(
            |err| Error::CommandSerialize { command: format!("{:?}", command), err_str: err.to_string() }
        );
        output.push_str(&command_str?);
        output.push('\n');
    }

    Ok(output)
}

pub fn deserialize_commands(text: &String) -> Result<Vec<Command>> {
    let lines = text.trim().lines().collect::<Vec<&str>>();
    let mut commands = Vec::new();

    for line in lines {
        let command: Command = serde_json::from_str(line).map_err(
            |err| Error::CommandDeserialize { command: line.to_owned(), err_str: err.to_string() }
        )?;
        if let Command::Get { key: _ } = command {
            return Err(Error::CommandDeserialize {
                command: line.to_owned(),
                err_str: "Get command cannot be deserialized".to_owned(),
            });
        }
        commands.push(command);
    }

    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize_commands {
        use super::*;

        #[test]
        fn success() {
            let commands = vec![
                Command::Set { key: "key1".to_owned(), value: "value1".to_owned() },
                Command::Rm { key: "key1".to_owned() },
            ];

            let serialized = serialize_commands(&commands).unwrap();
            let expected = vec![
                "{\"type\":\"Set\",\"key\":\"key1\",\"value\":\"value1\"}",
                "{\"type\":\"Rm\",\"key\":\"key1\"}",
            ].join("\n") + "\n";
            assert_eq!(serialized, expected);
        }

        #[test]
        fn fail() {
            let commands = vec![
                Command::Get { key: "key2".to_owned() },
            ];

            let result = serialize_commands(&commands);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Couldn't serialize command Get"));
        }
    }

    mod deserialize_commands {
        use super::*;

        #[test]
        fn success() {
            let deserialized = vec![
                "{\"type\":\"Set\",\"key\":\"key1\",\"value\":\"value1\"}",
                "{\"type\":\"Rm\",\"key\":\"key1\"}",
            ].join("\n") + "\n";

            let expected = vec![
                Command::Set { key: "key1".to_owned(), value: "value1".to_owned() },
                Command::Rm { key: "key1".to_owned() },
            ];
            let commands = deserialize_commands(&deserialized).unwrap();

            assert_eq!(expected, commands);
        }

        #[test]
        fn fail() {
            let deserialized = vec![
                "{\"type\":\"Set\",\"key\":\"key1\",\"value\":\"value1\"}",
                "{\"type\":\"Rm\",\"key\":\"key1\"}",
                "{\"type\":\"Get\",\"key\":\"key2\"}",
            ].join("\n") + "\n";

            let result = deserialize_commands(&deserialized);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Get command cannot be deserialized"));
        }
    }

    mod kv_store {
        use std::fs::{read_to_string};
        use super::*;

        #[test]
        fn open() -> Result<()> {
            // let temp_dir = tempfile::tempdir()?;
            // let store = KvStore::open(temp_dir.path())?;
            // assert!(store.log_path.is_some());
            Ok(())
        }

        #[test]
        fn set() -> Result<()> {
            let temp_dir = tempfile::tempdir().map_err(
                |err| Error::FileInit { path: "".to_string(), err_str: err.to_string() },
            )?;
            let mut store = KvStore::open(temp_dir.path())?;
            let test_table = vec![
                ("key1", "value1", 1),
                ("key1", "value2", 2),
                ("key1", "value3", 3),
            ];
            for (key, value, line_count) in test_table {
                store.set(key.to_owned(), value.to_owned())?;
                assert_eq!(store.get(key.to_owned())?, Some(value.to_owned()));

                let log_path = store.log_path.as_ref().unwrap();
                let log_content = read_to_string(log_path).unwrap();
                assert_eq!(log_content.lines().count(), line_count);
            }

            Ok(())
        }
    }
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            underlying: HashMap::new(),
            log_path: None,
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.underlying.insert(key.clone(), value.clone());

        let log_line = serde_json::to_string(
            &Command::Set {
                key: key.clone(),
                value: value.clone(),
            }
        ).map_err(
            |err| Error::CommandSerialize {
                command: format!("{:?}", Command::Set { key, value }),
                err_str: err.to_string(),
            }
        )?;
        let log_path = self.log_path.as_ref().unwrap();
        let log_path_str = log_path.to_string_lossy().to_string();
        let mut file = OpenOptions::new()
            .append(true)
            .open(log_path)
            .map_err(|err| Error::FileOpen {
                path: log_path_str,
                err_str: err.to_string(),
            })?;

        writeln!(file, "{}", log_line).map_err(
            |err| Error::CommandWriteLine { err_str: err.to_string() }
        )?;

        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(
            self
                .underlying
                .get(&key)
                .cloned()
        )
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.underlying.remove(&key);

        Ok(())
    }

    pub fn open(path: &Path) -> Result<Self> {
        let mut store = KvStore::new();
        let log_path = path.join("log.json");

        store.log_path = Some(log_path.clone());
        // TODO: optimize this by reusing the opened file instead of using `read_to_string`.
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(log_path.clone())
            .map_err(
            |err| Error::FileInit {
                path: log_path.to_string_lossy().to_string(),
                err_str: err.to_string(),
            }
        )?;

        let log_raw = read_to_string(log_path.clone()).map_err(
            |err| Error::FileRead {
                path: log_path.to_string_lossy().to_string(),
                err_str: err.to_string(),
            }
        )?;
        let commands = deserialize_commands(&log_raw)?;

        for command in commands {
            match command {
                Command::Set { key, value } => store.set(key, value)?,
                Command::Rm { key } => store.remove(key)?,
                _ => unreachable!("invalid command {:?}", command),
            }
        }

        // TODO: log how many commands were restored.

        Ok(store)
    }
}