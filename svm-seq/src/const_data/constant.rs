use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct Meta {
    pub(crate) err: Option<serde_json::Value>,
    pub(crate) status: Status,
    pub(crate) fee: u64,
    pub(crate) preBalances: Vec<u64>,
    pub(crate) postBalances: Vec<u64>,
    pub(crate) innerInstructions: Vec<serde_json::Value>,
    pub(crate) logMessages: Vec<String>,
    pub(crate) preTokenBalances: Vec<serde_json::Value>,
    pub(crate) postTokenBalances: Vec<serde_json::Value>,
    pub(crate) rewards: Vec<serde_json::Value>,
    pub(crate) loadedAddresses: LoadedAddresses,

}

#[derive(Debug, serde::Deserialize)]
pub struct LoadedAddresses {
    pub(crate) readonly: Vec<serde_json::Value>,
    pub(crate) writable: Vec<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Status {
    pub(crate) Ok: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub(crate) message: Message,
    pub(crate) signatures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub(crate) accountKeys: Vec<String>,
    pub(crate) header: Header,
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) recentBlockhash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub(crate) numReadonlySignedAccounts: u64,
    pub(crate) numReadonlyUnsignedAccounts: u64,
    pub(crate) numRequiredSignatures: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub(crate) accounts: Vec<u64>,
    pub(crate) data: String,
    pub(crate) programIdIndex: u64,
    pub(crate) stackHeight: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootTxn {
    pub(crate) block_time: i64,
    pub(crate) meta_result: String,
    pub(crate) slot_height: i64,
    pub(crate) transaction_result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchStruct {
    pub(crate) from_txn: Vec<String>,
    pub(crate) to_txn: Vec<String>,
    pub(crate) amounts_txn: Vec<String>,
    pub(crate) transaction_hash_txn: Vec<String>,
    pub(crate) sender_balances_txn: Vec<String>,
    pub(crate) receiver_balances_txn: Vec<String>,
    pub(crate) messages_txn: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub(crate) signatures: Vec<String>,
    pub(crate) message: NewMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMessage {
    pub(crate) accountKeys: Vec<NewAccountKey>,
    pub(crate) recentBlockhash: String,
    pub(crate) instructions: Vec<NewInstruction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccountKey {
    pub(crate) pubkey: String,
    pub(crate) writable: bool,
    pub(crate) signer: bool,
    pub(crate) source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewInstruction {
    pub(crate) programId: String,
    pub(crate) accounts: Vec<String>,
    pub(crate) data: String,
    pub(crate) stackHeight: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct NewMeta {
    pub(crate) err: Option<serde_json::Value>,
    pub(crate) status: Status,
    pub(crate) fee: u64,
    pub(crate) preBalances: Vec<u64>,
    pub(crate) postBalances: Vec<u64>,
    pub(crate) innerInstructions: Vec<serde_json::Value>,
    pub(crate) logMessages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewStatus {
    pub(crate) Ok: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBlockInfo {
    pub(crate) blockTime: u64,
}

#[derive(Debug, Deserialize)]
pub struct NewRoot {
    pub(crate) slot: u64,
    pub(crate) transaction: NewTransaction,
    pub(crate) meta: NewMeta,
    pub(crate) blockTime: Option<u64>,
}
