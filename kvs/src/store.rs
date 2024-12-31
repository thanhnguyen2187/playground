use crate::engine::KvsEngine;
use crate::err::{Result, ResultExt};
use crate::{Command, Error};
use serde::{Deserialize, Serialize};
use snafu::whatever;
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const DEFAULT_FILE_NAME: &str = "kvs.jsonl";

/// Make sure that the file exists and is writable.
pub fn initialize(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .with_whatever_context(|_| format!("Couldn't open file at {}", file_path.display()))?;
    }
    Ok(())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum KvCommand {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

pub fn deserialize_commands(text: &String) -> Result<Vec<Command>> {
    let lines = text.trim().lines();
    let mut commands = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let command: Command = serde_json::from_str(line)
            .with_whatever_context(|_| format!("Couldn't deserialize command {}", line))?;
        if let Command::Get { key: _ } = command {
            whatever!("Get command should not be deserialized");
        }
        commands.push(command);
    }

    Ok(commands)
}

pub fn apply_command(command: &Command, map: &mut HashMap<String, String>) -> Result<()> {
    match command {
        Command::Set { key, value } => {
            map.insert(key.clone(), value.clone());
        }
        Command::Rm { key } => {
            // TODO: check if key exists
            map.remove(key);
        }
        _ => whatever!("Invalid command {:?}", command),
    }
    Ok(())
}

pub fn serialize_command(command: &Command) -> Result<String> {
    if let Command::Get { key: _ } = command {
        whatever!("Get command should not be serialized");
    }
    let command_str = serde_json::to_string(command)
        .with_whatever_context(|_| format!("Couldn't serialize command {:?}", command))?;
    Ok(command_str)
}

pub fn serialize_commands(commands: &[Command]) -> Result<String> {
    let mut output = String::new();
    for command in commands {
        let command_str = serialize_command(command)?;
        output.push_str(&command_str);
        output.push('\n');
    }

    Ok(output)
}

pub fn append_command(command: Command, file_path: &PathBuf) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .with_whatever_context(|_| format!("Couldn't open file at {}", file_path.display()))?;
    let line = serialize_command(&command)?;

    writeln!(file, "{}", line)
        .with_whatever_context(|_| format!("Couldn't write command as new line: {}", line))
}

pub struct KvStoreV2 {
    file_path: Option<PathBuf>,
    map: HashMap<String, String>,
}

impl KvStoreV2 {
    pub fn new() -> Self {
        Self {
            file_path: None,
            map: HashMap::new(),
        }
    }

    pub fn open(working_dir: &Path) -> Result<Self> {
        let file_path = working_dir.join(DEFAULT_FILE_NAME);
        initialize(&file_path)?;

        let mut store = KvStoreV2::new();
        store.file_path = Some(file_path.clone());

        let commands =
            deserialize_commands(&read_to_string(&file_path).with_whatever_context(|_| {
                format!("Couldn't read content of file at {}", file_path.display())
            })?)?;
        for command in commands {
            apply_command(&command, &mut store.map)?;
        }

        Ok(store)
    }
}

impl KvsEngine for KvStoreV2 {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let file_path = self.file_path.as_ref().expect("file path not initialized");
        append_command(command, file_path)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::Rm { key: key.clone() };
        let file_path = self.file_path.as_ref().expect("file path not initialized");
        append_command(command, file_path)?;
        let value = self.map.remove(&key);
        match value {
            Some(_) => Ok(()),
            None => Err(Error::KeyNotFound { key }),
        }
    }
}

#[cfg(test)]
mod tests_pure_fns {
    use super::*;

    mod initialize {
        use super::*;

        #[test]
        fn success_file_new() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let file_path = temp_dir.path().join(DEFAULT_FILE_NAME);
            initialize(&file_path).expect("unable to initialize file");
            assert!(file_path.exists());
        }

        #[test]
        fn success_file_existing() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let file_path = temp_dir.path().join(DEFAULT_FILE_NAME);
            File::create(&file_path).expect("unable to create file");
            initialize(&file_path).expect("unable to initialize file");
            assert!(file_path.exists());
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
                .contains("Get command should not be deserialized"));
        }
    }

    mod apply_command {
        use super::*;

        #[test]
        fn success() {
            let map: HashMap<String, String> = HashMap::new();
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
                Command::Rm {
                    key: "key1".to_owned(),
                },
            ];
            let mut map = HashMap::new();
            for command in commands {
                apply_command(&command, &mut map).unwrap();
            }
            // assert_eq!(map.get("key1").unwrap(), "value1");
            assert_eq!(map.get("key2").unwrap(), "value2");
            assert_eq!(map.get("key3").unwrap(), "value3");
        }
    }

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
                .contains("Get command should not be serialized"));
        }
    }

    mod append_command {
        use super::*;

        #[test]
        fn success() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let file_path = temp_dir.path().join(DEFAULT_FILE_NAME);
            initialize(&file_path).expect("unable to initialize file");

            let command = Command::Set {
                key: "key1".to_owned(),
                value: "value1".to_owned(),
            };
            append_command(command, &file_path).expect("unable to append command");

            let file_content = read_to_string(file_path).expect("unable to read file content");
            assert_eq!(
                file_content,
                "{\"type\":\"Set\",\"key\":\"key1\",\"value\":\"value1\"}\n"
            );
        }
    }
}

#[cfg(test)]
mod tests_kv_store {
    use super::*;

    mod open {
        use super::*;
        use std::io::Write;

        #[test]
        fn success_file_new() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let file_path_expected = temp_dir.path().join(DEFAULT_FILE_NAME);
            let _ = KvStoreV2::open(temp_dir.path()).expect("unable to initialize file");

            assert!(file_path_expected.exists());
        }

        #[test]
        fn success_file_existing() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let file_path = temp_dir.path().join(DEFAULT_FILE_NAME);

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
            let commands_str = serialize_commands(&commands).expect("unable to serialize commands");

            writeln!(
                File::create(&file_path).expect("unable to create file"),
                "{}",
                commands_str,
            )
            .expect("unable to write to file");

            let store = KvStoreV2::open(temp_dir.path()).expect("unable to initialize file");

            assert_eq!(store.map.len(), commands.len());
            assert_eq!(store.map.get("key1").unwrap(), "value1");
            assert_eq!(store.map.get("key2").unwrap(), "value2");
            assert_eq!(store.map.get("key3").unwrap(), "value3");
        }
    }
}
