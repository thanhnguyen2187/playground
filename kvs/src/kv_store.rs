use crate::engine::KvsEngine;
use crate::err::{Result, Error, ResultExt};
use serde::{Deserialize, Serialize};
use snafu::whatever;
use std::collections::HashMap;
use std::fs;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub const DEFAULT_FILE_NAME: &str = "kvs.db";

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
pub enum Command {
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

pub fn convert_map_to_commands(store_underlying: &HashMap<String, String>) -> Vec<Command> {
    let mut commands = Vec::new();
    for (key, value) in store_underlying {
        commands.push(Command::Set {
            key: key.clone(),
            value: value.clone(),
        });
    }
    commands
}

pub struct KvStoreV2 {
    file_path: Option<PathBuf>,
    map: HashMap<String, String>,
    log_count: usize,
}

impl KvStoreV2 {
    pub fn new() -> Self {
        Self {
            file_path: None,
            map: HashMap::new(),
            log_count: 0,
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
        store.log_count = commands.len();
        for command in commands {
            apply_command(&command, &mut store.map)?;
        }

        Ok(store)
    }

    pub fn compact(&mut self) -> Result<()> {
        let log_path = self.file_path.as_ref().expect("file path not initialized");
        let commands = convert_map_to_commands(&self.map);
        let logs_content = serialize_commands(&commands)?;
        fs::write(log_path.as_path(), logs_content).with_whatever_context(|_| {
            format!("Couldn't write to file at {}", log_path.display())
        })?;
        self.log_count = self.map.len();
        Ok(())
    }
}

impl DerefMut for KvStoreV2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

impl Deref for KvStoreV2 {
    type Target = KvStoreV2;

    fn deref(&self) -> &Self::Target {
        self
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
        self.log_count += 1;
        self.map.insert(key, value);
        if self.log_count >= 2 * self.map.len() {
            self.compact()?;
        }
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let value = self.map.remove(&key);
        match value {
            Some(_) => {
                let command = Command::Rm { key: key.clone() };
                let file_path = self.file_path.as_ref().expect("file path not initialized");
                append_command(command, file_path)?;
                self.log_count += 1;
                if self.log_count >= 2 * self.map.len() {
                    self.compact()?;
                }
                Ok(())
            }
            None => Err(Error::KeyNotFound { key }),
        }
    }
}

#[cfg(test)]
mod tests_pure_fns {
    use super::*;

    mod initialize {
        use std::fs::File;
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

    mod convert_map_to_commands {
        use super::*;

        #[test]
        fn success() {
            let store = HashMap::from([
                ("key1".to_owned(), "value1".to_owned()),
                ("key2".to_owned(), "value2".to_owned()),
                ("key3".to_owned(), "value3".to_owned()),
            ]);
            let mut commands = convert_map_to_commands(&store);
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

    mod compact {
        use super::*;

        #[test]
        fn success() {
            let temp_dir =
                tempfile::tempdir().expect("unable to create temporary working directory");
            let mut store = KvStoreV2::open(temp_dir.path()).expect("unable to initialize file");

            store
                .set("key1".to_owned(), "value1".to_owned())
                .expect("unable to set key");
            store
                .set("key1".to_owned(), "value2".to_owned())
                .expect("unable to set key");
            store
                .set("key1".to_owned(), "value3".to_owned())
                .expect("unable to set key");
            store
                .set("key2".to_owned(), "value1".to_owned())
                .expect("unable to set key");
            store
                .set("key2".to_owned(), "value2".to_owned())
                .expect("unable to set key");

            store.compact().expect("unable to compact");

            assert_eq!(
                store.get("key1".to_owned()).expect("unable to get key"),
                Some("value3".to_owned()),
            );
            assert_eq!(
                store.get("key2".to_owned()).expect("unable to get key"),
                Some("value2".to_owned()),
            );
            assert_eq!(store.map.len(), 2,);
        }
    }
}

#[cfg(test)]
mod tests_kv_store {
    use super::*;

    mod open {
        use std::fs::File;
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
