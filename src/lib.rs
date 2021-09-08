#![deny(missing_docs)]
//! Simple in-memory key/value store that maps strings to strings
use std::collections::HashMap;

/// Key value store
pub struct KvStore {
    db: HashMap<String, String>,
}

impl KvStore {
    /// Returns a KvStore
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }

    /// Insert new value based on key
    ///
    /// # Arguments
    ///
    /// * `k` - A string that hold the key of the value
    /// * `v` - A string that hold the value of the key
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// ```
    pub fn set(&mut self, k: String, v: String) {
        self.db.insert(k, v);
    }

    /// Get value from key
    ///
    /// # Arguments
    ///
    /// * `k` - A string that hold the key of the value
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// ```
    pub fn get(&self, k: String) -> Option<String> {
        self.db.get(&k).map(|v| v.to_string())
    }

    /// Remove value based on key
    ///
    /// # Arguments
    ///
    /// * `k` - A string that hold the key of the value
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.remove("key1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), None);
    /// ```
    pub fn remove(&mut self, k: String) {
        self.db.remove(&k);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
