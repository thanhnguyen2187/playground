use std::collections::HashMap;

pub struct KvStore {
    underlying: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            underlying: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.underlying.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.underlying.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.underlying.remove(&key);
    }
}