use std::fs::{create_dir, create_dir_all};
use std::io::ErrorKind::AlreadyExists;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use {
    rocksdb::{DBWithThreadMode, MultiThreaded, Options, WriteBatch, WriteOptions},
    std::error::Error,
};

// * Set initial values
const TRANSACTION_COUNT_KEY: &str = "transaction_count";
const TRANSACTION_COUNT: &str = "0";
const BLOCK_COUNT_KEY: &str = "block_count";
const BLOCK_COUNT: &str = "1";
const BATCH_COUNT_KEY: &str = "batch_count";
const BATCH_COUNT: &str = "1";
const BATCH_START_INDEX_KEY: &str = "batch_start_index";
const BATCH_START_INDEX: &str = "0";
const SECONDARY_DB_PATH: &str = "secondary";


pub struct RocksDBConnection {
    pub db: DBWithThreadMode<MultiThreaded>,
}

impl RocksDBConnection {
    pub fn open(folder_path: &Path, secondary: bool) -> Result<Self, Box<dyn Error>> {
        // let mut seq_path = folder_path.join("rocksdb_seq");
        match create_dir_all(folder_path) {
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

        if secondary {
            sleep(Duration::from_secs(5));
            let secondary_db_path = folder_path.join(SECONDARY_DB_PATH);
            match DBWithThreadMode::open_as_secondary(&opts, folder_path, &secondary_db_path) {
                Ok(db) => Ok(Self { db }),
                Err(e) => {
                    println!("{:?}", e);
                    Err(Box::new(e))
                }
            }
        } else {
            match DBWithThreadMode::open_default(&folder_path) {
                Ok(db) => Ok(Self { db }),
                Err(e) => Err(Box::new(e))
            }
        }
    }

    // pub fn open(folder_path: &Path, secondary: bool) -> Result<Self, Box<dyn Error>> {
    //     // let seq_path = folder_path.join("rocksdb_seq");
    //
    //     // Attempt to create the directory and handle any potential errors
    //     if let Err(e) = create_dir_all(&folder_path) {
    //         if e.kind() != AlreadyExists {
    //             println!("Error creating dirs: {:?}", e);
    //             return Err(Box::new(e));
    //         }
    //     }
    //
    //     // Set up RocksDB options
    //     let mut opts = Options::default();
    //     opts.create_if_missing(true);
    //
    //     // Open the database in secondary mode if specified, otherwise open as primary
    //     if secondary {
    //         sleep(Duration::from_secs(5));
    //         let secondary_db_path = folder_path.join(SECONDARY_DB_PATH);
    //         println!("secondary : {:?}", secondary_db_path);
    //         match DBWithThreadMode::open_as_secondary(&opts, folder_path, &secondary_db_path) {
    //             Ok(db) => Ok(Self { db }),
    //             Err(e) => {
    //                 println!("Error opening DB as secondary: {:?}", e);
    //                 Err(Box::new(e))
    //             }
    //         }
    //     } else {
    //         println!("primary : {:?}", folder_path);
    //         match DBWithThreadMode::open_default(&folder_path) {
    //             Ok(db) => Ok(Self { db }),
    //             Err(e) => {
    //                 println!("Error opening DB as primary: {:?}", e);
    //                 Err(Box::new(e))
    //             }
    //         }
    //     }
    // }

    pub fn check_and_create_keys(&self) -> Result<(), Box<dyn Error>> {
        // * Check if keys exist more efficiently
        let keys_to_check = [
            BLOCK_COUNT_KEY,
            TRANSACTION_COUNT_KEY,
            BATCH_COUNT_KEY,
            BATCH_START_INDEX_KEY,
        ];

        let mut existing_keys = Vec::new();

        for key in &keys_to_check {
            match self.db.get(key.as_bytes()) {
                Ok(Some(_)) => existing_keys.push(*key), // Key exists
                Ok(None) => (),                          // Key doesn't exist
                Err(_) => (),                            // There was an error
            }
        }

        // * Prepare writes for missing keys
        let mut write_batch = WriteBatch::default();
        let write_options = WriteOptions::default();

        for key in &keys_to_check {
            if !existing_keys.contains(key) {
                match *key {
                    BLOCK_COUNT_KEY => write_batch.put(key.as_bytes(), &BLOCK_COUNT),
                    TRANSACTION_COUNT_KEY => write_batch.put(key.as_bytes(), &TRANSACTION_COUNT),
                    BATCH_COUNT_KEY => write_batch.put(key.as_bytes(), &BATCH_COUNT),
                    BATCH_START_INDEX_KEY => write_batch.put(key.as_bytes(), &BATCH_START_INDEX),
                    _ => unreachable!(), // Other key values should not exist
                }
            }
        }

        if !write_batch.is_empty() {
            self.db.write_opt(write_batch, &write_options)?;
            println!(
                "Keys created: {:?}",
                keys_to_check
                    .iter()
                    .filter(|key| !existing_keys.contains(*key))
                    .collect::<Vec<_>>()
            );
        } else {
            println!("Keys already exist");
        }

        Ok(())
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
        let txn_rocksdb_key = format!("txn-{:?}", txn_count);
        let mut write_batch = WriteBatch::default();
        let write_options = WriteOptions::default();
        write_batch.put(txn_rocksdb_key.as_bytes(), txn_data);
        let new_txn_count_str = (txn_count + 1).to_string();
        write_batch.put(TRANSACTION_COUNT_KEY.as_bytes(), new_txn_count_str.as_bytes());

        match self.db.write_opt(write_batch, &write_options) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn save_transaction_batch(&self, txn_data: String, slot: u64) -> Result<(), Box<dyn Error>> {
        // Get the current batch start index
        let batch_start_index = match self.get_batch_start_index() {
            Ok(Some(batch_start_index)) => batch_start_index,
            _ => return Err("Error getting batch_start_index from store data".into()),
        };

        // Get the current batch count
        let batch_count = match self.get_batch_count() {
            Ok(Some(batch_count)) => batch_count,
            _ => return Err("Error getting batch_count from store data".into()),
        };

        // Get the current transaction count
        let transaction_count = match self.get_transaction_count() {
            Ok(Some(transaction_count)) => transaction_count,
            _ => return Err("Error getting transaction_count from store data".into()),
        };

        // Retrieve the existing batch data
        let batch_rocksdb_key = format!("batch-{:?}", batch_count);
        let existing_data = self.db.get(&batch_rocksdb_key)?;
        let mut data_array: Vec<String> = if let Some(value) = existing_data {
            serde_json::from_slice(&value)?
        } else {
            vec![]
        };

        // Prepare a write batch
        let mut write_batch = WriteBatch::default();
        let write_options = WriteOptions::default();

        // Check if the current batch has reached its limit
        if data_array.len() >= 25 {
            // Increment the batch count
            let new_batch_count = batch_count + 1;

            // Update the batch count in the database
            write_batch.put(BATCH_COUNT_KEY.as_bytes(), new_batch_count.to_string().as_bytes());

            // Update the batch key and reset data_array for the new batch
            let new_batch_rocksdb_key = format!("batch-{:?}", new_batch_count);
            data_array = vec![txn_data]; // Initialize the new batch with the current transaction

            // Serialize the updated array
            let updated_data = serde_json::to_vec(&data_array)?;

            // Store the updated array in RocksDB
            write_batch.put(new_batch_rocksdb_key.as_bytes(), &updated_data);
        } else {
            // Add the transaction to the current batch
            data_array.push(txn_data);

            // Serialize the updated array
            let updated_data = serde_json::to_vec(&data_array)?;

            // Store the updated array in RocksDB
            write_batch.put(batch_rocksdb_key.as_bytes(), &updated_data);
        }

        // Increment the transaction count
        let new_transaction_count = transaction_count + 1;
        write_batch.put(TRANSACTION_COUNT_KEY.as_bytes(), new_transaction_count.to_string().as_bytes());

        // Write the batch to RocksDB
        match self.db.write_opt(write_batch, &write_options) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn get_batch_data(&self, batch_number: u64) -> Result<Vec<String>, Box<dyn Error>> {
        let batch_rocksdb_key = format!("batch-{:?}", batch_number);
        match self.db.get(&batch_rocksdb_key) {
            Ok(Some(data)) => {
                println!("{:?}", data.len());
                let data_array: Vec<String> = serde_json::from_slice(&data)?;
                Ok(data_array)
            }
            Ok(None) => {
                println!("none", );
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Batch data not found",
                )))
            }
            Err(e) => {
                println!("{:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn get_pod_len(&self, batch_number: u64) -> Result<u64, Box<dyn Error>> {
        match self.get_batch_data(batch_number) {
            Ok(pod_data) => {
                let len_pod_data = pod_data.len() as u64;
                Ok(len_pod_data)
            }
            Err(e) => Err(e),
        }
    }

    // New method to get the latest batch number
    pub fn get_latest_batch_number(&self) -> Result<u64, Box<dyn Error>> {
        match self.db.get(BATCH_COUNT_KEY.as_bytes()) {
            Ok(Some(data)) => {
                let batch_count: u64 = String::from_utf8(data)?.parse()?;
                match self.get_pod_len(batch_count) {
                    Ok(pod_data_len) => {
                        if pod_data_len == 25 {
                            Ok(batch_count)
                        } else {
                            if batch_count > 0 {
                                Ok(batch_count - 1)
                            } else {
                                Ok(0)
                            }
                        }
                    }
                    Err(e) => Err(e)
                }
            }
            Ok(None) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Batch count not found",
            ))),
            Err(e) => Err(Box::new(e)),
        }
    }
}
