use {
    crate::lib_store::{
        store_data::RocksDBConnection,
        constant::RootTxn,
    },
    solana_client::{
        rpc_client::RpcClient,
    },
    serde::{Deserialize, Serialize},
    std::{
        error::Error,
        thread::sleep,
        time::Duration,
    },
    solana_transaction_status::EncodedTransactionWithStatusMeta,
};

pub fn txn_settle(rocksdb_client: &RocksDBConnection) {
    let node_client = RpcClient::new("http://localhost:8899");

    loop {
        let block_count = match rocksdb_client.db.get(b"block_count") {
            Ok(Some(block_count_data)) => {
                let block_count_str = std::str::from_utf8(&block_count_data).unwrap();
                Some(block_count_str.parse::<u64>().unwrap())
            }
            _ => {
                println!("Error getting block_count from store data");
                None
            }
        };

        let block_data = match node_client.get_block(block_count.unwrap()) {
            Ok(data) => data,
            Err(e) => {
                println!("Error getting block data: {}", e);
                let _ = sleep(Duration::from_secs(3));
                continue;
            }
        };

        for trans_data in block_data.transactions {
            transaction_store_data(
                rocksdb_client,
                trans_data,
                &block_data.block_time.unwrap(),
                &(block_data.block_height.unwrap() as i64),
            );
        };

        let new_block_count_str = (block_count.unwrap() + 1).to_string();
        rocksdb_client.db.put(b"block_count", new_block_count_str.as_bytes())
            .expect("Failed to update block_count in RocksDB");

        println!("Block {} settled successfully  ✅   ", new_block_count_str);
    }
}


fn transaction_store_data(
    rocksdb_client: &RocksDBConnection,
    trans_data: EncodedTransactionWithStatusMeta,
    timestmap: &i64,
    slot_count: &i64,
) {
    let txn_count = match rocksdb_client.db.get(b"transaction_count") {
        Ok(Some(txn_count_data)) => {
            let txn_count_str = std::str::from_utf8(&txn_count_data).unwrap();
            Some(txn_count_str.parse::<u64>().unwrap())
        }
        _ => {
            println!("Error getting block_count from store data");
            None
        }
    };

    let meta_result = serde_json::to_string(&trans_data.meta)
        .expect("Failed to serialize the struct to JSON");

    let transaction_result = serde_json::to_string(&trans_data.transaction)
        .expect("Failed to serialize the struct to JSON");

    let root_txn = RootTxn {
        block_time: *timestmap,
        meta_result: meta_result,
        slot_height: *slot_count,
        transaction_result: transaction_result,
    };

    let root_txn_str = serde_json::to_string(&root_txn)
        .expect("Failed to serialize the struct to JSON");

    let txn_rocksdb_key = format!("txn-{:?}", txn_count.unwrap());

    if let Err(e) = rocksdb_client.db.put(
        txn_rocksdb_key.as_bytes(),
        root_txn_str.as_bytes(),
    ) {
        println!("Error in slot : {} , err: {}", slot_count, e);
    } else {
        let new_txn_count_str = (txn_count.unwrap() + 1).to_string();
        rocksdb_client.db.put(b"transaction_count", new_txn_count_str.as_bytes())
            .expect("Failed to update transaction_count in RocksDB");

        // println!("Transaction {} settled successfully  ✅   ", new_txn_count_str);
    }
}


