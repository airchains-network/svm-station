mod perform_module;
mod solana_transaction_verification;

use {
    perform_module::{
        perform_node_run,
        perform_batch_seq,
    },
    std::{
        error::Error,
        thread::{sleep, spawn},
        sync::{Arc, Mutex},
        time::Duration,
    },
};


fn main() {
    let node_run_handle = spawn(|| {
        perform_node_run();
    });

    let batch_seq_handle = spawn(move || {
        sleep(Duration::from_secs(5));
        perform_batch_seq();
    });

    node_run_handle.join().expect("Error joining settle thread");
    batch_seq_handle.join().expect("Error joining submit thread");
}