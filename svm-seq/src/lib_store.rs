#[path = "batch_client"]
mod batch_client {
    #[path = "txn_settle/txn_settle.rs"]
    pub mod txn_settle;

    #[path = "batch_settle/batch_settle.rs"]
    pub mod batch_settle;
}

#[path = "store/store_data.rs"]
mod store_data;

#[path = "db_client/connection.rs"]
mod connection;

#[path = "const_data/constant.rs"]
mod constant;

use {
    std::{
        error::Error,
        thread::sleep,
        time::Duration,
    },
    batch_client::{
        txn_settle::txn_settle,
        batch_settle::batch_settle,
    },
    // txn_settle::txn_settle,
    store_data::{RocksDBConnection, check_and_create_keys},
    connection::{PostgresConnection, create_table},
    constant::{
        Meta,
        Transaction,
        RootTxn,
        BatchStruct,
    },
};

pub async fn perform_postgres_conn() -> Result<PostgresConnection, Box<dyn Error>> {
    let db_url = "postgres://postgres:857634@localhost:5432/postgres";

    let postgres_connection = match PostgresConnection::connect(db_url).await {
        Ok(connection) => connection,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    Ok(postgres_connection)
}

pub async fn perform_create_table(postgres_connection: &PostgresConnection) {
    create_table(&postgres_connection).await;
}


pub fn perform_txn_settle(rocksdb_connection: &RocksDBConnection) {
    txn_settle(&rocksdb_connection);
}

pub fn perform_store_data() -> Result<RocksDBConnection, Box<dyn Error>> {
    let rocksdb_connection = match RocksDBConnection::open("test-ledger") {
        Ok(connection) => connection,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(rocksdb_connection)
}

pub fn perform_check_and_create_keys(rocksdb_connection: &RocksDBConnection) -> Result<(), Box<dyn std::error::Error>> {
    check_and_create_keys(&rocksdb_connection)
}

pub fn perform_batch_settle(rocksdb_connection: &RocksDBConnection) {
    sleep(Duration::from_secs(5));
    batch_settle(&rocksdb_connection);
}
