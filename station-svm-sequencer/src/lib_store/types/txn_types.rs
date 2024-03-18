use serde::{Deserialize, Serialize};

// Main data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchTransaction {
    pub(crate) signature: String,
    pub(crate) is_vote: bool,
    pub(crate) slot: u32,
    pub(crate) message_type: u8,
    pub(crate) message: Message,
    pub(crate) message_hash: Vec<u8>,
    pub(crate) meta: Meta,
    pub(crate) signatures: Vec<String>,
    pub(crate) write_version: u64,
    pub(crate) index: u64,
}

// Structures to nest data
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub(crate) Legacy: Legacy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Legacy {
    pub(crate) header: Header,
    pub(crate) account_keys: Vec<String>,
    pub(crate) recent_blockhash: String,
    pub(crate) instructions: Vec<Instruction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub(crate) num_required_signatures: u8,
    pub(crate) num_readonly_signed_accounts: u8,
    pub(crate) num_readonly_unsigned_accounts: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub(crate) program_id_index: u8,
    pub(crate) accounts: Vec<u8>,
    pub(crate) data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub(crate) error: Option<String>,
    pub(crate) fee: u64,
    pub(crate) pre_balances: Vec<u64>,
    pub(crate) post_balances: Vec<u64>,
    pub(crate) inner_instructions: Vec<()>,
    pub(crate) log_messages: Vec<String>,
    pub(crate) pre_token_balances: Vec<()>,
    pub(crate) post_token_balances: Vec<()>,
    pub(crate) rewards: Vec<()>,
}
