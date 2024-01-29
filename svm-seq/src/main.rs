mod lib_store;

use {
    lib_store::{
        perform_store_data,
        perform_check_and_create_keys,
        perform_postgres_conn,
        perform_create_table,
        perform_txn_settle,
        perform_batch_settle,
    },
    std::{
        thread,
        sync::{Arc, Mutex},
    },
};

fn main() {
    let store_perform = match perform_store_data() {
        Ok(store) => store,
        Err(err) => {
            eprintln!("Error creating store: {}", err);
            return;
        }
    };

    perform_check_and_create_keys(&store_perform).unwrap();

    rayon::join(
        || perform_txn_settle(&store_perform),
        || perform_batch_settle(&store_perform),
    );

    // perform_txn_settle(&store_perform);
    //
    // perform_batch_settle(&store_perform);
}