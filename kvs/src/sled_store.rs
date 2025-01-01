use std::path::{Path, PathBuf};
use crate::err::Result;
use sled;
use snafu::ResultExt;

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
        let db = sled::open(file_path.clone()).with_whatever_context(
            |_| format!("Couldn't open file at {} for Sled store", file_path.display()),
        )?;
        // initialize(&file_path)?;

        let mut store = SledStore::new();
        store.file_path = Some(file_path.clone());
        store.db = Some(db);

        Ok(store)
    }
}
