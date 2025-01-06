use crate::err::Result;
use crate::KvsEngine;
use std::collections::HashMap;

pub struct MemStore {
    map: HashMap<String, String>,
}

impl MemStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl KvsEngine for MemStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    fn remove(&mut self, key: String) -> Result<Option<String>> {
        let value_opt = self.map.remove(&key);
        match value_opt {
            Some(value) => {
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}
