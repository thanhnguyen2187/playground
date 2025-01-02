use crate::err::Result;
use crate::KvsEngine;
use sled;
use snafu::ResultExt;
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

impl DerefMut for SledStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

impl Deref for SledStore {
    type Target = SledStore;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl KvsEngine for SledStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        todo!()
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        todo!()
    }

    fn remove(&mut self, key: String) -> Result<()> {
        todo!()
    }
}
