use crate::err::Result;
use crate::KvsEngine;
use sled;
use snafu::{whatever, ResultExt};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub const DEFAULT_FILE_NAME: &str = "sled.db";

pub struct SledStore {
    file_path: Option<PathBuf>,
    db: Option<sled::Db>,
}

impl SledStore {
    pub fn new() -> Self {
        Self {
            file_path: None,
            db: None,
        }
    }

    pub fn open(working_dir: &Path) -> Result<Self> {
        let file_path = working_dir.join(DEFAULT_FILE_NAME);
        let db = sled::open(file_path.clone()).with_whatever_context(|_| {
            format!(
                "Couldn't open file at {} for Sled store",
                file_path.display()
            )
        })?;
        // initialize(&file_path)?;

        let mut store = SledStore::new();
        store.file_path = Some(file_path.clone());
        store.db = Some(db);

        Ok(store)
    }
}

impl KvsEngine for SledStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let Some(db) = self.db.as_ref() else {
            whatever!("Sled store not initialized");
        };
        db.insert(key.clone(), value.as_bytes())
            .with_whatever_context(|_| format!("Couldn't insert key {} into sled store", key))?;

        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        let Some(db) = self.db.as_ref() else {
            whatever!("Sled store not initialized");
        };

        let value_option = db
            .get(key.clone())
            .with_whatever_context(|_| format!("Couldn't get key {} from sled store", key))?;
        if let Some(value) = value_option {
            let value = String::from_utf8(value.to_vec()).with_whatever_context(|_| {
                format!("Couldn't convert value for key {} to UTF-8", key)
            })?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let Some(db) = self.db.as_ref() else {
            whatever!("Sled store not initialized");
        };

        db.remove(key.clone())
            .with_whatever_context(|_| format!("Couldn't remove key {} from sled store", key))?;

        Ok(())
    }
}
