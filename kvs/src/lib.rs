mod err;

pub use err::{Result};
use std::collections::HashMap;
use std::path::Path;

pub struct KvStore {
    underlying: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            underlying: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.underlying.insert(key, value);

        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(
            Some(
                self
                    .underlying
                    .get(&key)
                    .ok_or(err::Error::KeyNotFound { key })?
                    .clone()
            )
        )
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.underlying.remove(&key);

        Ok(())
    }

    pub fn open(path: &Path) -> Result<Self> {
        unimplemented!()
    }
}