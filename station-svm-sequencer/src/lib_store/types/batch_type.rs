use {
    crate::lib_store::types::txn_types::Instruction,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchStruct {
    pub(crate) slot: Vec<u64>,
    pub(crate) tx_signature: Vec<String>,
    pub(crate) fee: Vec<String>,
    pub(crate) pre_balance: Vec<Vec<u64>>,
    pub(crate) post_balance: Vec<Vec<u64>>,
    pub(crate) account_keys: Vec<Vec<String>>,
    pub(crate) amount: Vec<Vec<u64>>,
    pub(crate) instructions: Vec<Vec<Instruction>>,
    pub(crate) recent_block_hash: Vec<String>,
}