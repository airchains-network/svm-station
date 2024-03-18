mod lib_store;

use {
    std::{
        error::Error,
        thread::{sleep, spawn},
        time::Duration,
    },
    lib_store::{
        node_client::node::air_solana,
        batch_client::batch::batch_settle,
        batch_store::batch_store::RocksDBConnection,
    },
};

fn main() {
    let node_run_handle = spawn(|| {
        air_solana();
    });

    let batch_seq_handle = spawn(move || {
        sleep(Duration::from_secs(20));
        let txn_config = match RocksDBConnection::open_txn("test-ledger/sequencer/txn", "test-ledger/sequencer/batch") {
            Ok(connection) => connection,
            Err(err) => {
                println!("Error opening TXN RocksDBConnection: {}", err);
                return;
            }
        };

        println!("txn_config : {:?}", txn_config.db);

        let batch_config = match RocksDBConnection::open_batch("test-ledger/sequencer/batch") {
            Ok(connection) => connection,
            Err(err) => {
                println!("Error opening BATCH RocksDBConnection: {}", err);
                return;
            }
        };

        batch_config.check_and_create_keys().unwrap();

        batch_settle(&txn_config, &batch_config)
    });

    node_run_handle.join().expect("Error joining settle thread");
    batch_seq_handle.join().expect("Error joining submit thread");
}
