mod local_cluster {
    pub mod air_solana;
}


mod batch_client {
    pub mod txn_settle {
        pub mod txn_settle;
    }

    pub mod batch_settle {
        pub mod batch_settle;
    }

    pub mod da_settle {
        pub mod da_start;
        pub mod da_exec;
    }
}

mod store {
    pub mod store_data;
}

mod const_data {
    pub(crate) mod constant;
}
// pub(crate) mod constant;


use {
    std::{
        error::Error,
        thread::{sleep, spawn},
        sync::{Arc, Mutex},
        time::Duration,
    },
    local_cluster::{
        air_solana::air_solana,
    },
    batch_client::{
        txn_settle::txn_settle::txn_settle,
        batch_settle::batch_settle,
        da_settle::{
            da_start::da_start,
            da_exec::da_exec,
        },
    },
    // txn_settle::txn_settle,
    store::store_data::{RocksDBConnection, check_and_create_keys},
    const_data::constant::{
        Meta,
        Transaction,
        RootTxn,
        BatchStruct,
        NewRoot,
    },
    rayon::join,
};

pub fn perform_node_run() {
    air_solana()
}


pub fn perform_txn_settle(rocksdb_connection: &RocksDBConnection) {
    // txn_settle(&rocksdb_connection);
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
    println!("Performing batch settle");
    batch_settle::batch_settle(&rocksdb_connection);
}

pub fn perform_da_settle() {
    da_start();
}

pub fn perform_da_exec(data_value: &str) -> Result<(bool, String), String> {
    let da_exec_data = match da_exec(data_value) {
        Ok(da_exec_data) => da_exec_data,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(da_exec_data)
}

pub fn perform_batch_seq() {
    let store_perform = match perform_store_data() {
        Ok(store) => store,
        Err(err) => {
            eprintln!("Error creating store: {}", err);
            return;
        }
    };

    perform_check_and_create_keys(&store_perform).unwrap();

    let settle_handle = spawn(|| {
        perform_da_settle();
    });

    let submit_handle = spawn(move || {
        sleep(Duration::from_secs(15));
        perform_store_txn(&store_perform);
    });

    settle_handle.join().expect("Error joining settle thread");
    submit_handle.join().expect("Error joining submit thread");
}

pub fn perform_store_txn(rocksdb_connection: &RocksDBConnection) {
    join(
        || perform_txn_settle(&rocksdb_connection),
        || perform_batch_settle(&rocksdb_connection),
    );
}