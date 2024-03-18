use std::fs::{create_dir, create_dir_all};
use std::io::ErrorKind::AlreadyExists;
use std::path::Path;
use {
    rocksdb::{DBWithThreadMode, MultiThreaded, Options, WriteBatch, WriteOptions},
    std::error::Error,
};

// * Set initial values
const TRANSACTION_COUNT_KEY: &str = "transaction_count";
const TRANSACTION_COUNT: &str = "0";
// const BLOCK_COUNT_KEY: &str = "block_count";
// const BLOCK_COUNT: &str = "1";
// const BATCH_COUNT_KEY: &str = "batch_count";
// const BATCH_COUNT: &str = "1";
// const BATCH_START_INDEX_KEY: &str = "batch_start_index";
// const BATCH_START_INDEX: &str = "0";


pub struct RocksDBConnection {
    pub db: DBWithThreadMode<MultiThreaded>,
}

impl RocksDBConnection {
    pub fn open(folder_path: &str) -> Result<Self, Box<dyn Error>> {
        let dir_path = Path::new(&folder_path);
        match create_dir_all(dir_path) {
            Ok(()) => (),
            Err(e) => {
                if e.kind() != AlreadyExists {
                    println!("Error creating dirs: {:?}", e);
                    return Err(Box::new(e));
                }
            }
        };

        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db_path = dir_path.join("txn");
        match DBWithThreadMode::open_default(&db_path) {
            Ok(db) => {
                Ok(Self { db })
            }
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn check_and_create_keys(&self) -> Result<(), Box<dyn Error>> {

        // * Check if keys exist
        let get_results = self.db.get(TRANSACTION_COUNT_KEY.as_bytes());
        match get_results {
            Ok(Some(_)) => {
                println!("Keys already exist");
                return Ok(());
            }
            _ => (),
        }

        // * Prepare writes for missing keys
        let mut write_batch = WriteBatch::default();
        let write_options = WriteOptions::default();
        write_batch.put(TRANSACTION_COUNT_KEY.as_bytes(), TRANSACTION_COUNT);

        self.db.write_opt(write_batch, &write_options)?;

        println!("Keys created: {:?}", TRANSACTION_COUNT_KEY);

        return Ok(());

        // // * Check if keys exist more efficiently
        // let keys_to_check = [
        //     BLOCK_COUNT_KEY,
        //     TRANSACTION_COUNT_KEY,
        //     BATCH_COUNT_KEY,
        //     BATCH_START_INDEX_KEY
        // ];
        //
        // let get_results = self.db.get(TRANSACTION_COUNT_KEY.as_bytes());
        //
        // let existing_keys: Vec<_> = get_results
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(i, result)| match result {
        //         Ok(Some(_)) => Some(TRANSACTION_COUNT_KEY), // Key exists
        //         _ => None, // Key doesn't exist or there was an error
        //     })
        //     .collect();
        //
        // // * Prepare writes for missing keys
        // let mut write_batch = WriteBatch::default();
        // let write_options = WriteOptions::default();
        //
        // for key in keys_to_check.iter() {
        //     if !existing_keys.contains(key) {
        //         match key {
        //             &BLOCK_COUNT_KEY => write_batch.put(key.as_bytes(), &BLOCK_COUNT),
        //             &TRANSACTION_COUNT_KEY => write_batch.put(key.as_bytes(), &TRANSACTION_COUNT),
        //             &BATCH_COUNT_KEY => write_batch.put(key.as_bytes(), &BATCH_COUNT),
        //             &BATCH_START_INDEX_KEY => write_batch.put(key.as_bytes(), &BATCH_START_INDEX),
        //             _ => unreachable!() // Other key values should not exist
        //         }
        //     }
        // }
        //
        // if write_batch.is_empty() {
        //     self.db.write_opt(write_batch, &write_options)?;
        //     println!("Keys already exist");
        //     return Ok(());
        // }
        //
        // self.db.write_opt(write_batch, &write_options)?;
        //
        // println!(
        //     "Keys created: {:?}",
        //     keys_to_check
        //         .iter()
        //         .filter(|key| !existing_keys.contains(*key))
        //         .collect::<Vec<_>>()
        // );
        //
        // return Ok(());
    }

    pub fn get_transaction_count(&self) -> Result<Option<u64>, Box<dyn Error>> {
        match self.db.get(TRANSACTION_COUNT_KEY.as_bytes()) {
            Ok(Some(txn_count_data)) => {
                match std::str::from_utf8(&txn_count_data) {
                    Ok(txn_count_str) => match txn_count_str.parse::<u64>() {
                        Ok(count) => Ok(Some(count)),
                        Err(parse_err) => {
                            println!("Error parsing transaction count: {}", parse_err);
                            Ok(None)
                        }
                    }
                    Err(utf8_err) => {
                        println!("Error decoding transaction count data: {}", utf8_err);
                        Ok(None)
                    }
                }
            }
            Err(db_err) => {
                println!("Error getting transaction count from RocksDB: {}", db_err);
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn save_transaction(&self, txn_count: u64, txn_data: &[u8], slot: u64) -> Result<(), Box<dyn Error>> {
        println!("slot : {:?}", slot);
        let txn_rocksdb_key = format!("txn-{:?}", txn_count);
        println!("txn_rocksdb_key : {:?}", txn_rocksdb_key);
        let mut write_batch = WriteBatch::default();
        let write_options = WriteOptions::default();
        write_batch.put(txn_rocksdb_key.as_bytes(), txn_data);
        let new_txn_count_str = (txn_count + 1).to_string();
        write_batch.put(TRANSACTION_COUNT_KEY.as_bytes(), new_txn_count_str.as_bytes());

        if txn_count == 25 as u64 {
            println!("txn_data : {:?}", txn_count);
            std::thread::sleep(std::time::Duration::from_secs(20));
        }

        match self.db.write_opt(write_batch, &write_options) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }
}
