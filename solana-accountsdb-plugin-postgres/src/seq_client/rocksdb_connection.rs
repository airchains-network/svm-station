use {
    rocksdb::{DBWithThreadMode, SingleThreaded, Options, WriteBatch, WriteOptions}
};

pub(crate) struct RocksDBConnection {
    pub db: DBWithThreadMode<SingleThreaded>,
}

impl RocksDBConnection {
    pub fn open(folder_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if !std::path::Path::new(folder_path).exists() {
            std::fs::create_dir(folder_path)?;
        }

        let db_path = format!("{}/rocksdb_data", folder_path);

        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_max_background_jobs(2);

        let db = DBWithThreadMode::open(&opts, &db_path)?;

        Ok(Self { db })
    }
}

pub fn check_and_create_keys(db: &RocksDBConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Set initial values
    let block_count_key = "block_count";
    let transaction_count_key = "transaction_count";
    let batch_count_key = "batch_count";
    let batch_start_index_key = "batch_start_index";

    // Check if keys exist
    let block_count_exists = db.db.get(block_count_key.as_bytes())?.is_some();
    let transaction_count_exists = db.db.get(transaction_count_key.as_bytes())?.is_some();
    let batch_count_exists = db.db.get(batch_count_key.as_bytes())?.is_some();
    let batch_start_exists = db.db.get(batch_start_index_key.as_bytes())?.is_some();

    // If any key does not exist, create it
    let mut write_batch = WriteBatch::default();
    let write_options = WriteOptions::default();

    if !block_count_exists {
        write_batch.put(block_count_key.as_bytes(), b"1");
    }

    if !transaction_count_exists {
        write_batch.put(transaction_count_key.as_bytes(), b"0");
    }

    if !batch_count_exists {
        write_batch.put(batch_count_key.as_bytes(), b"1");
    }

    if !batch_start_exists {
        write_batch.put(batch_start_index_key.as_bytes(), b"0");
    }

    if !write_batch.is_empty() {
        db.db.write_opt(write_batch, &write_options)?;
        println!("Keys created successfully ✅   ");
    } else {
        println!("Keys already exist");
    }

    Ok(())
}