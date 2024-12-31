mod err;
mod server;
mod engine;
mod store;

pub use err::{Error, Result};
pub use store::KvStoreV2;
pub use engine::KvsEngine;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct KvStore {
    underlying: HashMap<String, String>,
    log_path: Option<PathBuf>,
    log_count: usize,
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

        let command_str = serde_json::to_string(command).map_err(|err| Error::CommandSerialize {
            command: format!("{:?}", command),
            err_str: err.to_string(),
        });
        output.push_str(&command_str?);
        output.push('\n');
    }

    Ok(output)
}

pub fn deserialize_commands(text: &String) -> Result<Vec<Command>> {
    let lines = text.trim().lines();
    let mut commands = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let command: Command =
            serde_json::from_str(line).map_err(|err| Error::CommandDeserialize {
                command: line.to_owned(),
                err_str: err.to_string(),
            })?;
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

pub fn _append_log(line: String, path: &PathBuf) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path.clone())
        .map_err(|err| Error::FileOpen {
            path: path.to_string_lossy().to_string(),
            err_str: err.to_string(),
        })?;

    writeln!(file, "{}", line).map_err(|err| Error::CommandWriteLine {
        err_str: err.to_string(),
    })?;

    Ok(())
}

pub fn _restore(
    store_underlying: &mut HashMap<String, String>,
    commands: &[Command],
) -> Result<()> {
    for command in commands {
        match command {
            Command::Set { key, value } => {
                store_underlying.insert(key.clone(), value.clone());
            }
            Command::Rm { key } => {
                store_underlying.remove(key);
            }
            _ => unreachable!("invalid command {:?}", command),
        }
    }
    Ok(())
}

pub fn _store_underlying_to_commands(store_underlying: &HashMap<String, String>) -> Vec<Command> {
    let mut commands = Vec::new();
    for (key, value) in store_underlying {
        commands.push(Command::Set {
            key: key.clone(),
            value: value.clone(),
        });
    }
    commands
}

pub fn _save(store: &HashMap<String, String>, path: &PathBuf) -> Result<()> {
    let commands = _store_underlying_to_commands(store);
    let log_content = serialize_commands(&commands)?;
    fs::write(path, log_content).map_err(|err| Error::FileWrite {
        path: path.to_string_lossy().to_string(),
        err_str: err.to_string(),
    })?;
    // writeln!(file, "{}", log_content).map_err(|err| Error::FileWrite {
    //     path: path.to_string_lossy().to_string(),
    //     err_str: err.to_string(),
    // })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize_commands {
        use super::*;

        #[test]
        fn success() {
            let commands = vec![
                Command::Set {
                    key: "key1".to_owned(),
                    value: "value1".to_owned(),
                },
                Command::Rm {
                    key: "key1".to_owned(),
                },
            ];

            let serialized = serialize_commands(&commands).unwrap();
            let expected = vec![
                "{\"type\":\"Set\",\"key\":\"key1\",\"value\":\"value1\"}",
                "{\"type\":\"Rm\",\"key\":\"key1\"}",
            ]
            .join("\n")
                + "\n";
            assert_eq!(serialized, expected);
        }

        #[test]
        fn fail() {
            let commands = vec![Command::Get {
                key: "key2".to_owned(),
            }];

            let result = serialize_commands(&commands);

            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Couldn't serialize command Get"));
        }
    }

    mod deserialize_commands {
        use super::*;

        #[test]
        fn success() {
            let deserialized = vec![
                "{\"type\":\"Set\",\"key\":\"key1\",\"value\":\"value1\"}",
                "{\"type\":\"Rm\",\"key\":\"key1\"}",
            ]
            .join("\n")
                + "\n";

            let expected = vec![
                Command::Set {
                    key: "key1".to_owned(),
                    value: "value1".to_owned(),
                },
                Command::Rm {
                    key: "key1".to_owned(),
                },
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
            ]
            .join("\n")
                + "\n";

            let result = deserialize_commands(&deserialized);

            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Get command cannot be deserialized"));
        }
    }

    mod _append_log {
        use super::*;
        use std::fs::File;

        #[test]
        fn success() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let path = temp_dir.path().join("log.json");
            File::create(&path).unwrap();
            _append_log("test1".to_owned(), &path).unwrap();
            _append_log("test2".to_owned(), &path).unwrap();
            _append_log("test3".to_owned(), &path).unwrap();
            assert_eq!(read_to_string(path).unwrap(), "test1\ntest2\ntest3\n");
        }
    }

    mod _restore {
        use super::*;

        #[test]
        fn success() {
            let mut store = HashMap::new();
            let commands = vec![
                Command::Set {
                    key: "key1".to_owned(),
                    value: "value1".to_owned(),
                },
                Command::Set {
                    key: "key2".to_owned(),
                    value: "value2".to_owned(),
                },
                Command::Set {
                    key: "key3".to_owned(),
                    value: "value3".to_owned(),
                },
            ];
            _restore(&mut store, &commands).unwrap();
            assert_eq!(store.get("key1").unwrap(), "value1");
            assert_eq!(store.get("key2").unwrap(), "value2");
            assert_eq!(store.get("key3").unwrap(), "value3");
        }
    }

    mod _store_underlying_to_commands {
        use super::*;

        #[test]
        fn success() {
            let store = HashMap::from([
                ("key1".to_owned(), "value1".to_owned()),
                ("key2".to_owned(), "value2".to_owned()),
                ("key3".to_owned(), "value3".to_owned()),
            ]);
            let mut commands = _store_underlying_to_commands(&store);
            assert_eq!(commands.len(), 3);
            commands.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
            assert_eq!(
                commands[0],
                Command::Set {
                    key: "key1".to_owned(),
                    value: "value1".to_owned()
                }
            );
            assert_eq!(
                commands[1],
                Command::Set {
                    key: "key2".to_owned(),
                    value: "value2".to_owned()
                }
            );
            assert_eq!(
                commands[2],
                Command::Set {
                    key: "key3".to_owned(),
                    value: "value3".to_owned()
                }
            );
        }
    }

    mod _save {
        use super::*;
        use std::fs::File;

        #[test]
        fn success() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let path = temp_dir.path().join("log.json");
            let mut store = HashMap::new();
            store.insert("key1".to_owned(), "value1".to_owned());
            store.insert("key2".to_owned(), "value2".to_owned());
            store.insert("key3".to_owned(), "value3".to_owned());
            _save(&store, &path).unwrap();
            let expected = vec![
                "{\"type\":\"Set\",\"key\":\"key1\",\"value\":\"value1\"}",
                "{\"type\":\"Set\",\"key\":\"key2\",\"value\":\"value2\"}",
                "{\"type\":\"Set\",\"key\":\"key3\",\"value\":\"value3\"}",
            ];
            let binding = read_to_string(path).unwrap();
            let binding = binding.trim();
            let mut got = binding.split("\n").collect::<Vec<&str>>();
            got.sort();

            assert_eq!(got, expected);
        }
    }

    mod compact {
        use super::*;
        use tempfile::TempDir;

        #[test]
        fn success() {
            let temp_dir = TempDir::new().expect("unable to create temporary working directory");
            let path = temp_dir.path();
            let mut store = KvStore::open(path).unwrap();
            store.set("key1".to_owned(), "value1".to_owned()).unwrap();
            store.set("key1".to_owned(), "value2".to_owned()).unwrap();
            store.set("key1".to_owned(), "value3".to_owned()).unwrap();
            store.set("key2".to_owned(), "value1".to_owned()).unwrap();
            store.set("key2".to_owned(), "value2".to_owned()).unwrap();

            store.compact().unwrap();

            assert_eq!(
                store.get("key1".to_owned()).unwrap(),
                Some("value3".to_owned()),
            );
            assert_eq!(
                store.get("key2".to_owned()).unwrap(),
                Some("value2".to_owned()),
            );
            assert_eq!(store.log_count, 2);
        }
    }

    mod kv_store {
        use super::*;
        use std::fs::read_to_string;

        #[test]
        fn open() -> Result<()> {
            // let temp_dir = tempfile::tempdir()?;
            // let store = KvStore::open(temp_dir.path())?;
            // assert!(store.log_path.is_some());
            Ok(())
        }

        #[test]
        fn set() -> Result<()> {
            let temp_dir = tempfile::tempdir().map_err(|err| Error::FileInit {
                path: "".to_string(),
                err_str: err.to_string(),
            })?;
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
            log_count: 0,
        }
    }

    pub fn set_persistent(&mut self, key: String, value: String) -> Result<()> {
        let log_line = serde_json::to_string(&Command::Set {
            key: key.clone(),
            value: value.clone(),
        })
        .map_err(|err| Error::CommandSerialize {
            command: format!("{:?}", Command::Set { key, value }),
            err_str: err.to_string(),
        })?;

        let log_path = self.log_path.as_ref().expect("log path not initialized");
        _append_log(log_line, log_path)
    }

    pub fn set_underlying(&mut self, key: String, value: String) -> Result<()> {
        self.underlying.insert(key, value);
        Ok(())
    }

    pub fn compact(&mut self) -> Result<()> {
        let log_path = self.log_path.as_ref().expect("log path not initialized");
        _save(&self.underlying, log_path)?;
        self.log_count = self.underlying.len();
        Ok(())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.set_underlying(key.clone(), value.clone())?;
        self.set_persistent(key, value)?;
        self.log_count += 1;
        if self.log_count >= 2 * self.underlying.len() {
            self.compact()?;
        }
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.underlying.get(&key).cloned())
    }

    pub fn remove_persistent(&mut self, key: String) -> Result<()> {
        let command = Command::Rm { key: key.clone() };
        let log_line = serde_json::to_string(&command).map_err(|err| Error::CommandSerialize {
            command: format!("{:?}", command),
            err_str: err.to_string(),
        })?;

        let log_path = self.log_path.as_ref().expect("log path not initialized");
        _append_log(log_line, log_path)
    }

    pub fn remove_underlying(&mut self, key: String) -> Option<String> {
        self.underlying.remove(&key)
    }

    pub fn remove(&mut self, key: String) -> Result<Option<String>> {
        match self.remove_underlying(key.clone()) {
            Some(value) => {
                self.remove_persistent(key.clone())?;
                self.log_count += 1;
                if self.log_count >= 1000 {
                    self.compact()?;
                }
                Ok(Some(value))
            }
            None => Ok(None),
        }
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
            .map_err(|err| Error::FileInit {
                path: log_path.to_string_lossy().to_string(),
                err_str: err.to_string(),
            })?;

        let log_raw = read_to_string(log_path.clone()).map_err(|err| Error::FileRead {
            path: log_path.to_string_lossy().to_string(),
            err_str: err.to_string(),
        })?;
        let commands = deserialize_commands(&log_raw)?;

        _restore(&mut store.underlying, &commands)?;
        store.log_count = commands.len();

        Ok(store)
    }
}
