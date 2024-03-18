use {
    crate::{
        seq_client::rocksdb_connection::RocksDBConnection,
        postgres_client::postgres_client_transaction::DbTransaction,
    },
    solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoV2,
    solana_sdk::{
        message::SanitizedMessage,
    },
};
use crate::postgres_client::postgres_client_transaction::{DbLoadedMessageV0, DbTransactionMessage, DbTransactionStatusMeta};
use crate::seq_client::rocksdb_connection::check_and_create_keys;

pub fn transaction_store_data(
    transaction_info: &ReplicaTransactionInfoV2,
    slot: u64,
) {
    let rocks_db_connection = RocksDBConnection::open("test-ledger").unwrap();

    check_and_create_keys(&rocks_db_connection).unwrap();

    let txn_count = match rocks_db_connection.db.get(b"transaction_count") {
        Ok(Some(txn_count_data)) => {
            let txn_count_str = std::str::from_utf8(&txn_count_data).unwrap();
            Some(txn_count_str.parse::<u64>().unwrap())
        }
        _ => {
            println!("Error getting block_count from store data");
            None
        }
    };

    let txn_data = DbTransaction {
        signature: transaction_info.signature.to_string(),
        is_vote: transaction_info.is_vote,
        slot: slot as i64,
        message_type: match transaction_info.transaction.message() {
            SanitizedMessage::Legacy(_) => 0,
            SanitizedMessage::V0(_) => 0,
        },
        legacy_message: match transaction_info.transaction.message() {
            SanitizedMessage::Legacy(legacy_message) => {
                Some(DbTransactionMessage::from(legacy_message.message.as_ref()))
            }
            _ => None,
        },
        v0_loaded_message: match transaction_info.transaction.message() {
            SanitizedMessage::V0(loaded_message) => Some(DbLoadedMessageV0::from(loaded_message)),
            _ => None,
        },
        message_hash: transaction_info
            .transaction
            .message_hash()
            .as_ref()
            .to_vec(),
        meta: DbTransactionStatusMeta::from(transaction_info.transaction_status_meta),
        signatures: transaction_info
            .transaction
            .signatures()
            .iter()
            .map(|signature| signature.to_string())
            .collect(),
        write_version: 1,
        index: transaction_info.index as i64,
    };

    let root_txn_str = serde_json::to_string(&txn_data)
        .expect("Failed to serialize the struct to JSON");

    let txn_rocksdb_key = format!("txn-{:?}", txn_count.unwrap());

    if let Err(e) = rocks_db_connection.db.put(
        txn_rocksdb_key.as_bytes(),
        root_txn_str.as_bytes(),
    ) {
        println!("Error in transaction: {}, err: {}", root_txn_str, e);
    } else {
        let new_txn_count_str = (txn_count.unwrap() + 1).to_string();
        rocks_db_connection.db.put(b"transaction_count", new_txn_count_str.as_bytes())
            .expect("Failed to update transaction_count in RocksDB");
    }
}
