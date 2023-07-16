#![deny(missing_docs)]
//!A simple key-value store.

use std::collections::HashMap;

/// KvStore is a key-value which is used to store & manage key-value pairs.
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Create a new instance of KvStore.
    pub fn new() -> KvStore {
        Self {
            store: HashMap::new(),
        }
    }

    /// Set the given key with the given value.
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Get the value for the given key. If it does not exist, then an empty option will be returned.
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Remove the given key from the store.
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
