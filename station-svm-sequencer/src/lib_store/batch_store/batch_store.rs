use std::fs::create_dir_all;
use std::sync::{Arc, Mutex};
use {
    rocksdb::{DBWithThreadMode, MultiThreaded, Options, WriteBatch, WriteOptions},
    std::error::Error,
    std::path::Path,
    std::fs::create_dir,
    std::io::ErrorKind::AlreadyExists,
};

const BLOCK_COUNT_KEY: &str = "block_count";
const BLOCK_COUNT: &str = "1";
const BATCH_COUNT_KEY: &str = "batch_count";
const BATCH_COUNT: &str = "1";
const BATCH_START_INDEX_KEY: &str = "batch_start_index";
const BATCH_START_INDEX: &str = "0";

pub struct RocksDBConnection {
    pub db: DBWithThreadMode<MultiThreaded>,
}

impl RocksDBConnection {
    pub fn open_txn(txn_path: &str, batch_path: &str) -> Result<Self, Box<dyn Error>> {
        let tx_path = Path::new(&txn_path);

        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_max_background_jobs(2);
        opts.set_max_write_buffer_number(2);

        match DBWithThreadMode::open_for_read_only(&opts, &tx_path, true) {
            Ok(db) => Ok(Self { db }),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn get_transaction(&self, txn_count: u64) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let txn_rocksdb_key = format!("txn-{:?}", txn_count);
        match self.db.get(txn_rocksdb_key.as_bytes()) {
            Ok(Some(txn_data)) => Ok(Some(txn_data)),
            Ok(None) => Ok(None),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn open_batch(batch_path: &str) -> Result<Self, Box<dyn Error>> {
        let db_path = Path::new(&batch_path);

        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_max_background_jobs(2);
        opts.set_max_write_buffer_number(2);

        match DBWithThreadMode::open(&opts, &db_path) {
            Ok(db) => Ok(Self { db }),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn check_and_create_keys(&self) -> Result<(), Box<dyn Error>> {
        // * Check if keys exist more efficiently
        let keys_to_check = [
            BLOCK_COUNT_KEY,
            BATCH_COUNT_KEY,
            BATCH_START_INDEX_KEY
        ];

        let get_results: Vec<_> = keys_to_check
            .iter()
            .map(|key| self.db.get(key.as_bytes()))
            .collect();

        let existing_keys: Vec<_> = get_results
            .iter()
            .enumerate()
            .filter_map(|(i, result)| match result {
                Ok(Some(_)) => Some(keys_to_check[i]), // Key exists
                _ => None, // Key doesn't exist or there was an error
            })
            .collect();

        // * Prepare writes for missing keys
        let mut write_batch = WriteBatch::default();
        let write_options = WriteOptions::default();

        for key in keys_to_check.iter() {
            if !existing_keys.contains(key) {
                match key {
                    &BLOCK_COUNT_KEY => write_batch.put(key.as_bytes(), &BLOCK_COUNT),
                    &BATCH_COUNT_KEY => write_batch.put(key.as_bytes(), &BATCH_COUNT),
                    &BATCH_START_INDEX_KEY => write_batch.put(key.as_bytes(), &BATCH_START_INDEX),
                    _ => unreachable!() // Other key values should not exist
                }
            }
        }

        if write_batch.is_empty() {
            self.db.write_opt(write_batch, &write_options)?;
            println!("Keys already exist");
            return Ok(());
        }

        self.db.write_opt(write_batch, &write_options)?;

        println!(
            "Keys created: {:?}",
            keys_to_check
                .iter()
                .filter(|key| !existing_keys.contains(*key))
                .collect::<Vec<_>>()
        );

        return Ok(());
    }

    pub fn get_batch_start_index(&self) -> Result<Option<u64>, Box<dyn Error>> {
        match self.db.get(BATCH_START_INDEX_KEY.as_bytes()) {
            Ok(Some(batch_start_index_data)) => Ok(Some(
                std::str::from_utf8(&batch_start_index_data)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            )),
            Ok(None) => Ok(None),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn get_batch_count(&self) -> Result<Option<u64>, Box<dyn Error>> {
        match self.db.get(BATCH_COUNT_KEY.as_bytes()) {
            Ok(Some(batch_count_data)) => Ok(Some(
                std::str::from_utf8(&batch_count_data)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            )),
            Ok(None) => Ok(None),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn save_batch(&self, batch_count: u64, batch_data: &[u8]) -> Result<(), Box<dyn Error>> {
        let batch_rocksdb_key = format!("batch-{:?}", batch_count);
        self.db.put(batch_rocksdb_key.as_bytes(), batch_data)?;
        Ok(())
    }

    pub fn save_batch_count(&self, batch_count: &[u8]) -> Result<(), Box<dyn Error>> {
        self.db.put(BATCH_COUNT_KEY.as_bytes(), batch_count)?;
        Ok(())
    }

    pub fn save_batch_start_index(&self, batch_start_index: &[u8]) -> Result<(), Box<dyn Error>> {
        self.db.put(BATCH_START_INDEX_KEY.as_bytes(), batch_start_index)?;
        Ok(())
    }
}