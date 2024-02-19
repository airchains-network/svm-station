use {
    crate::perform_module::{
        batch_client::da_settle::da_exec::da_exec,
        store::store_data::RocksDBConnection,
        const_data::constant::{Meta, Transaction, RootTxn, BatchStruct},
    },
    std::{thread::sleep, time::Duration},
};

pub fn batch_settle(rocksdb_client: &RocksDBConnection) {
    loop {
        if let Err(err) = process_batch(rocksdb_client) {
            println!("Error: {}", err);
            // Retry after a delay
            sleep(Duration::from_secs(10));
        }
    }
}

fn process_batch(rocksdb_client: &RocksDBConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut batch = BatchStruct {
        from_txn: Vec::new(),
        to_txn: Vec::new(),
        amounts_txn: Vec::new(),
        transaction_hash_txn: Vec::new(),
        sender_balances_txn: Vec::new(),
        receiver_balances_txn: Vec::new(),
        messages_txn: Vec::new(),
    };

    let batch_start_index = match rocksdb_client.db.get(b"batch_start_index") {
        Ok(Some(batch_start_index_data)) => Some(
            std::str::from_utf8(&batch_start_index_data)
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        ),
        _ => {
            println!("Error getting batch_start_index from store data");
            return Err("Error getting batch_start_index from store data".into());
        }
    };

    let batch_count = match rocksdb_client.db.get(b"batch_count") {
        Ok(Some(batch_count_data)) => Some(
            std::str::from_utf8(&batch_count_data)
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        ),
        _ => {
            println!("Error getting batch_count from store data");
            return Err("Error getting batch_count from store data".into());
        }
    };

    for tx_count in (batch_start_index.unwrap() as i64)..(25 * batch_count.unwrap() as i64) {
        let txn_rocksdb_key = format!("txn-{:?}", tx_count);
        let txn_data = match rocksdb_client.db.get(txn_rocksdb_key.as_bytes()) {
            Ok(Some(txn_data)) => txn_data,
            _ => {
                println!("Error getting txn data from store data");
                continue;
            }
        };

        let txn_data_str = std::str::from_utf8(&txn_data).unwrap();

        let txn_json: RootTxn = serde_json::from_str(txn_data_str).unwrap();

        let meta_result: Meta = serde_json::from_str(&txn_json.meta_result).unwrap();
        let transaction_result: Transaction = serde_json::from_str(&txn_json.transaction_result).unwrap();

        batch.from_txn.push(transaction_result.message.accountKeys[0].to_string());
        batch.to_txn.push(transaction_result.message.accountKeys[1].to_string());
        batch.amounts_txn.push("100".to_string());
        batch.transaction_hash_txn.push(transaction_result.signatures[0].to_string());
        batch.sender_balances_txn.push(meta_result.postBalances[0].to_string());
        batch.receiver_balances_txn.push(meta_result.postBalances[1].to_string());
        batch.messages_txn.push(transaction_result.message.instructions[0].data.to_string());
    }

    let batch_json =
        serde_json::to_string(&batch).expect("Failed to serialize the struct to JSON");

    let da_value = match da_exec(batch_json.as_str()) {
        Ok(da_exec_data) => da_exec_data,
        Err(err) => {
            println!("Error: {}", err);
            return Err("Error executing da_exec".into());
        }
    };

    println!("da_value: {:?}", da_value.1);

    let batch_rocksdb_key = format!("batch-{:?}", batch_count);
    rocksdb_client
        .db
        .put(batch_rocksdb_key.as_bytes(), batch_json.as_bytes())
        .map_err(|e| format!("Failed to update transaction_count in RocksDB: {}", e))?;

    println!("Batch {} settled successfully  ✅   ", batch_count.unwrap());

    let batch_count_put = (batch_count.unwrap() + 1).to_string();
    rocksdb_client
        .db
        .put(b"batch_count", batch_count_put.as_bytes())
        .map_err(|e| format!("Failed to update batch_count in RocksDB: {}", e))?;

    let batch_start_put = (batch_start_index.unwrap() + 26).to_string();
    rocksdb_client
        .db
        .put(b"batch_start_index", batch_start_put.as_bytes())
        .map_err(|e| format!("Failed to update batch_start_index in RocksDB: {}", e))?;

    // batch_start_index = batch_start_index + 26;
    // batch_count = batch_count + 1;

    Ok(())
}
