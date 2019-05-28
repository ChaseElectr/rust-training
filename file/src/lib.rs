#![deny(missing_docs)]
//! A simple key/value store.

use failure::Fail;
use serde::{Deserialize, Serialize};
use serde_cbor::Deserializer;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// The type for storing key-value pairs. The key and the value are both String, and each key must be assigned with a value.
///
/// You can store the key-value pair by set() method, and get a key's value by get() method. This is all we support now.
#[derive(Debug)]
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
    /// The log file is invalid. Maybe it's modified by other application.
    #[fail(display = "Invalid file format: {}", _0)]
    InvalidFile(#[cause] serde_cbor::error::Error),
    /// There is a io::Error during the operation
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
}

impl From<std::io::Error> for KvsError {
    fn from(err: std::io::Error) -> Self {
        KvsError::Io(err)
    }
}

impl From<serde_cbor::error::Error> for KvsError {
    fn from (err: serde_cbor::error::Error) -> Self {
        KvsError::InvalidFile(err)
    }
}

/// A specialized Result type for I/O operations.
pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Debug, Serialize, Deserialize)]
enum Operation {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

impl KvStore {
    /// Open a log file to create a KvStore
    pub fn open(path: impl AsRef<Path>) -> Result<KvStore> {
        let store = HashMap::new();
        let log = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path.as_ref().join("kvs.db"))?;
        Ok(KvStore { store, log })
    }

    /// Save an operation into log file.
    fn log(&mut self, op: Operation) -> Result<()> {
        // Use CBOR as log format because it saves more spaces, and I can learn a
        // new data format, and it may be used in the network transfer.
        // Except this, I think JSON is the other data format I'll choose， as it's
        // human readable, extensible, and (maybe) converts faster than CBOR. More
        // importantly, it can be easily dealed with Linux command line tools.
        serde_cbor::to_writer(&mut self.log, &op).map_err(KvsError::InvalidFile)
    }

    ///  reads the entire log, one command at a time, recording the affected key and
    ///  file offset of the command to an in-memory key -> log pointer map
    fn rebuild(&mut self) -> Result<()> {
        for op in Deserializer::from_reader(&self.log).into_iter::<Operation>() {
            match op? {
                Operation::Set { key, value } => self.store.insert(key, value),
                Operation::Rm { key } => self.store.remove(&key),
                Operation::Get { .. } => None,
            };
        }
        Ok(())
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
        self.log(Operation::Set {
            key: key.clone(),
            value: value.clone(),
        })?;
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
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.rebuild()?;
        Ok(self.store.get(&key).cloned())
    }

    /// Remove a key's value
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.rebuild()?;
        self.store.remove(&key).ok_or_else(|| {
            println!("Key not found");
            KvsError::InvalidCommand {
                command: "Key not found".to_owned(),
            }
        })?;
        self.log(Operation::Rm { key })
    }
}
