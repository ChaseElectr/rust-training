#![deny(missing_docs)]
#![feature(seek_convenience)]
//! A simple key/value store.

use failure::Fail;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// The type for storing key-value pairs. The key and the value are both String, and each key must be assigned with a value.
///
/// You can store the key-value pair by set() method, and get a key's value by get() method. This is all we support now.
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, (usize, usize)>,
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
    InvalidFile(#[cause] serde_json::Error),
    /// A possible error value when converting a String from the file data.
    #[fail(display = "Invalid file data: {}", _0)]
    InvalidUtf8(#[cause] std::string::FromUtf8Error),
    /// There is a io::Error during the operation
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
}

impl From<std::io::Error> for KvsError {
    fn from(err: std::io::Error) -> Self {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> Self {
        KvsError::InvalidFile(err)
    }
}

impl From<std::string::FromUtf8Error> for KvsError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        KvsError::InvalidUtf8(err)
    }
}

/// A specialized Result type for I/O operations.
pub type Result<T> = std::result::Result<T, KvsError>;

/// A enum used to represent the operations. This struct is directly write
/// into log files, and deserialized directly.
#[derive(Debug, Serialize, Deserialize)]
enum Operation {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

impl KvStore {
    /// Open a log file to create a KvStore
    pub fn open(path: impl AsRef<Path>) -> Result<KvStore> {
        let map = HashMap::new();
        let log = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path.as_ref().join("kvs.db"))?;
        let mut store = KvStore { store: map, log };
        store.load()?;
        Ok(store)
    }

    /// Save an operation into log file.
    fn log(&mut self, op: Operation) -> Result<()> {
        // Use CBOR as log format because it saves more spaces, and I can learn a
        // new data format, and it may be used in the network transfer.
        // Except this, I think JSON is the other data format I'll choose, as it's
        // human readable, extensible, and (maybe) converts faster than CBOR. More
        // importantly, it can be easily dealed with Linux command line tools.

        // Change to JSON format because serde_cbor doesn't have a byte_offset()
        // method for StreamDeserializer.
        serde_json::to_writer(&mut self.log, &op).map_err(KvsError::InvalidFile)
    }

    ///  Reads the entire log, one command at a time, recording the affected key and
    ///  file offset of the command to an in-memory key -> log pointer map
    fn load(&mut self) -> Result<()> {
        self.log.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(&self.log).into_iter::<Operation>();
        let mut pos = 0;
        while let Some(op) = stream.next() {
            let new_pos = stream.byte_offset();
            match op? {
                Operation::Set { key, .. } => self.store.insert(key, (pos, new_pos - pos)),
                Operation::Rm { key } => self.store.remove(&key),
                Operation::Get { .. } => None,
            };
            pos = new_pos;
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
    /// let mut store = KvStore::open("./").unwrap();
    /// store.set("key".to_owned(), "value1".to_owned());
    /// assert_eq!(Some("value1".to_owned()), store.get("key".to_owned()).unwrap());
    /// store.set("key".to_owned(), "value2".to_owned());
    /// assert_eq!(Some("value2".to_owned()), store.get("key".to_owned()).unwrap());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let old_len = self.log.stream_len()?;
        self.log(Operation::Set {
            key: key.clone(),
            value: value.clone(),
        })?;
        let new_len = self.log.stream_len()?;
        self.store
            .insert(key, (old_len as usize, (new_len - old_len) as usize));
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
    /// let mut store = KvStore::open("./").unwrap();
    /// store.set("key".to_owned(), "value".to_owned());
    /// assert_eq!(Some("value".to_owned()), store.get("key".to_owned()).unwrap());
    /// assert_eq!(None, store.get("kkkk".to_owned()).unwrap());
    /// ```
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some((index, len)) = self.store.get(&key) {
            self.log.seek(SeekFrom::Start(*index as u64))?;
            let mut value = vec![0; *len];
            self.log.read_exact(&mut value)?;
            if let Operation::Set { key: _, value } = serde_json::from_slice(&value)? {
                return Ok(Some(value));
            }
        }
        return Ok(None);
    }

    /// Remove a key's value
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key).ok_or_else(|| {
            println!("Key not found");
            KvsError::InvalidCommand {
                command: "Key not found".to_owned(),
            }
        })?;
        self.log(Operation::Rm { key })
    }
}
