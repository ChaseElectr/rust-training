#![deny(missing_docs)]
//! A simple key/value store.

use failure::Fail;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// The type for storing key-value pairs. The key and the value are both String, and each key must be assigned with a value.
///
/// You can store the key-value pair by set() method, and get a key's value by get() method. This is all we support now.
pub struct KvStore {
    store: HashMap<String, String>,
    log: File,
}

/// The error type
#[derive(Debug, Fail)]
pub enum KvsError {
    /// The passing in command is invalid, either in wrong format or the command not support
    #[fail(display = "Invalid command {}", command)]
    InvalidCommand {
        /// Used to show the wrong command
        command: String,
    },
    /// There is a io::Error during the operation
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
}

impl From<std::io::Error> for KvsError {
    fn from(err: std::io::Error) -> Self {
        KvsError::Io(err)
    }
}

/// A specialized Result type for I/O operations.
pub type Result<T> = std::result::Result<T, KvsError>;

impl KvStore {
    /// Open a log file to create a KvStore
    pub fn open(path: impl AsRef<Path>) -> Result<KvStore> {
        let store = HashMap::new();
        let log = File::open(path)?;
        Ok(KvStore { store, log })
    }

    /// Store a key with it's value, this will store a key and it's value to the storage.
    /// If the key has already been exist, the value will be overwrited.
    ///
    /// # Example
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key".to_owned(), "value1".to_owned());
    /// assert_eq!(Some("value1".to_owned()), store.get("key".to_owned()));
    /// store.set("key".to_owned(), "value2".to_owned());
    /// assert_eq!(Some("value2".to_owned()), store.get("key".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
        Ok(())
    }

    /// Get a key's value.
    /// Return Some(_) if the key exists, return None otherwise.
    /// The return value is a copy of the stored value, so it won't delete the origin data.
    ///
    /// # Example
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key".to_owned(), "value".to_owned());
    /// assert_eq!(Some("value".to_owned()), store.get("key".to_owned()));
    /// assert_eq!(None, store.get("kkkk".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    /// Remove a key's value
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        Ok(())
    }
}
