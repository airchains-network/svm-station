use std::fs::create_dir;
use std::io::ErrorKind::AlreadyExists;
use std::path::Path;
use {
    crate::lib_store::{
        batch_store::batch_store::RocksDBConnection,
        types::{batch_type::BatchStruct, txn_types::BatchTransaction},
    },
    std::{
        error::Error,
        thread::sleep,
        time::Duration,
        io::Write,
        fs::File,
    },
    serde_json::from_str,
};

pub fn batch_settle(txn_rocksdb_client: &RocksDBConnection, batch_rocksdb_client: &RocksDBConnection) {
    loop {
        if let Err(err) = process_batch(txn_rocksdb_client, batch_rocksdb_client) {
            println!("Error: {}", err);
            // Retry after a delay
            sleep(Duration::from_secs(5));
            continue;
        }
    }
}

fn process_batch(txn_rocksdb_client: &RocksDBConnection, batch_rocksdb_client: &RocksDBConnection) -> Result<(), Box<dyn Error>> {
    let mut batch = BatchStruct {
        slot: Vec::new(),
        tx_signature: Vec::new(),
        fee: Vec::new(),
        pre_balance: Vec::new(),
        post_balance: Vec::new(),
        account_keys: Vec::new(),
        amount: Vec::new(),
        instructions: Vec::new(),
        recent_block_hash: Vec::new(),
    };

    let batch_start_index = match batch_rocksdb_client.get_batch_start_index() {
        Ok(Some(batch_start_index)) => Some(batch_start_index),
        _ => {
            return Err("Error getting batch_start_index from store data".into());
        }
    };

    let batch_count = match batch_rocksdb_client.get_batch_count() {
        Ok(Some(batch_count)) => Some(batch_count),
        _ => {
            return Err("Error getting batch_count from store data".into());
        }
    };


    // let batch_start_index = match rocksdb_client.db.get(b"batch_start_index") {
    //     Ok(Some(batch_start_index_data)) => Some(
    //         std::str::from_utf8(&batch_start_index_data)
    //             .unwrap()
    //             .parse::<u64>()
    //             .unwrap(),
    //     ),
    //     _ => {
    //         println!("Error getting batch_start_index from store data");
    //         return Err("Error getting batch_start_index from store data".into());
    //     }
    // };
    //
    // let batch_count = match rocksdb_client.db.get(b"batch_count") {
    //     Ok(Some(batch_count_data)) => Some(
    //         std::str::from_utf8(&batch_count_data)
    //             .unwrap()
    //             .parse::<u64>()
    //             .unwrap(),
    //     ),
    //     _ => {
    //         println!("Error getting batch_count from store data");
    //         return Err("Error getting batch_count from store data".into());
    //     }
    // };

    let mut current_batch_start_index = batch_start_index.unwrap();
    let mut current_batch_tx_count = 0;

    while current_batch_tx_count < 25 {
        let txn_rocksdb_key = format!("txn-{:?}", current_batch_start_index);
        println!("txn_rocksdb_key : {:?}", txn_rocksdb_key);

        let txn_data = match txn_rocksdb_client.get_transaction(current_batch_start_index) {
            Ok(Some(txn_data)) => txn_data,
            Ok(None) => {
                println!("Unexpected: Key exists but no value found");
                sleep(Duration::from_secs(2));
                current_batch_start_index += 1;
                continue;
            }
            Err(err) => {
                println!("Error getting transaction data: {}", err);
                break;
            }
        };

        // let txn_data = match txn_rocksdb_client.db.get(txn_rocksdb_key.as_bytes()) {
        //     Ok(Some(data)) => data,
        //     Ok(None) => {
        //         println!("Unexpected: Key exists but no value found");
        //         sleep(Duration::from_secs(2));
        //         current_batch_start_index += 1;
        //         continue;
        //     }
        //     Err(err) => {
        //         println!("Error getting transaction data: {}", err);
        //         break;
        //     }
        // };

        let txn_data_str = std::str::from_utf8(&txn_data).unwrap();

        let txn_json: BatchTransaction = from_str(txn_data_str).unwrap();

        let differences = calculate_differences(&txn_json.meta.pre_balances, &txn_json.meta.post_balances, &txn_json.meta.fee);

        batch.slot.push(txn_json.slot as u64);
        batch.tx_signature.push(txn_json.signature);
        batch.fee.push(txn_json.meta.fee.to_string());
        batch.pre_balance.push(txn_json.meta.pre_balances);
        batch.post_balance.push(txn_json.meta.post_balances);
        batch.account_keys.push(txn_json.message.Legacy.account_keys);
        batch.amount.push(differences);
        batch.instructions.push(txn_json.message.Legacy.instructions);
        batch.recent_block_hash.push(txn_json.message.Legacy.recent_blockhash);

        current_batch_tx_count += 1;
        current_batch_start_index += 1;
    }

    let batch_json =
        serde_json::to_string(&batch).expect("Failed to serialize the struct to JSON");


    let file_path = Path::new(&"test-ledger/test");
    match create_dir(file_path) {
        Ok(()) => (),
        Err(e) => {
            println!("Error creating dir: {:?}", e);
            if e.kind() != AlreadyExists {
                println!("Error creating dirs: {:?}", e);
                return Err(Box::new(e));
            }
        }
    }
    let file_path = file_path.join(format!("batch-{:?}.json", batch_count.unwrap()));
    let mut file = File::create(file_path).unwrap();
    file.write_all(batch_json.as_bytes()).unwrap();


    // println!("batch_json : {:?}", batch_json);

    let batch_rocksdb_put = match batch_rocksdb_client.save_batch(batch_count.unwrap(), batch_json.as_bytes()) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to update batch in RocksDB: {}", e))
    };

    let batch_count_put_value = (batch_count.unwrap() + 1).to_string();
    let batch_count_put = match batch_rocksdb_client.save_batch_count(batch_count_put_value.as_bytes()) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to update batch_count in RocksDB: {}", e))
    };

    let batch_start_put_value = (batch_start_index.unwrap() + 25).to_string();
    let batch_start_put = match batch_rocksdb_client.save_batch_start_index(batch_start_put_value.as_bytes()) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to update batch_start_index in RocksDB: {}", e))
    };


    // let batch_rocksdb_key = format!("batch-{:?}", batch_count);
    // let batch_rocksdb_put = match rocksdb_client.db.put(batch_rocksdb_key.as_bytes(), batch_json.as_bytes()) {
    //     Ok(()) => Ok(()),
    //     Err(e) => Err(format!("Failed to update batch in RocksDB: {}", e))
    // };

    // let batch_count_put_value = (batch_count.unwrap() + 1).to_string();
    // let batch_count_put = match rocksdb_client.db.put("batch_count", batch_count_put_value.as_bytes()) {
    //     Ok(()) => Ok(()),
    //     Err(e) => Err(format!("Failed to update batch_count in RocksDB: {}", e))
    // };
    //
    // let batch_start_put_value = (batch_start_index.unwrap() + 26).to_string();
    // let batch_start_put = match rocksdb_client.db.put("batch_start_index", batch_start_put_value.as_bytes()) {
    //     Ok(()) => Ok(()),
    //     Err(e) => Err(format!("Failed to update batch_start_index in RocksDB: {}", e))
    // };

    println!("Batch {} settled successfully  ✅   ", batch_count.unwrap());

    Ok(())
}

fn calculate_differences(pre_balances: &Vec<u64>, post_balances: &Vec<u64>, fee: &u64) -> Vec<u64> {
    let mut differences = Vec::new();

    for i in 0..pre_balances.len() {
        if i == 0 {
            differences.push(pre_balances[i] - post_balances[i] - fee);
        } else {
            differences.push(pre_balances[i] - post_balances[i]);
        }
    }

    differences
}