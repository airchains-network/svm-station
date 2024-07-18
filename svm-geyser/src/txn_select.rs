use solana_sdk::instruction::CompiledInstruction;
use solana_sdk::message::v0::{LoadedAddresses, MessageAddressTableLookup};
use solana_sdk::message::{v0, Message, MessageHeader, SanitizedMessage};
use solana_sdk::reward_type::RewardType;
use solana_transaction_status::{InnerInstructions, Reward, TransactionTokenBalance};
use {
    crate::rocksdb_client::RocksDBConnection,
    serde_derive::{Deserialize, Serialize},
    solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoV2,
    solana_sdk::transaction::TransactionError,
    solana_transaction_status::TransactionStatusMeta,
};

const MAX_TRANSACTION_STATUS_LEN: usize = 256;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RocksDBCompiledInstruction {
    pub program_id_index: i16,
    pub accounts: Vec<i16>,
    pub data: String,
}

impl From<&CompiledInstruction> for RocksDBCompiledInstruction {
    fn from(instruction: &CompiledInstruction) -> Self {
        Self {
            program_id_index: instruction.program_id_index as i16,
            accounts: instruction
                .accounts
                .iter()
                .map(|account_idx| *account_idx as i16)
                .collect(),
            data: bs58::encode(instruction.data.clone()).into_string(),
            // data: instruction.data.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RocksDBInnerInstructions {
    pub index: i16,
    pub instructions: Vec<RocksDBCompiledInstruction>,
}

impl From<&InnerInstructions> for RocksDBInnerInstructions {
    fn from(instructions: &InnerInstructions) -> Self {
        Self {
            index: instructions.index as i16,
            instructions: instructions
                .instructions
                .iter()
                .map(|instruction| RocksDBCompiledInstruction::from(&instruction.instruction))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RocksDBTransactionTokenBalance {
    pub account_index: i16,
    pub mint: String,
    pub ui_token_amount: Option<f64>,
    pub owner: String,
}

impl From<&TransactionTokenBalance> for RocksDBTransactionTokenBalance {
    fn from(token_balance: &TransactionTokenBalance) -> Self {
        Self {
            account_index: token_balance.account_index as i16,
            mint: token_balance.mint.clone(),
            ui_token_amount: token_balance.ui_token_amount.ui_amount,
            owner: token_balance.owner.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RocksDBRewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

impl From<&RewardType> for RocksDBRewardType {
    fn from(reward_type: &RewardType) -> Self {
        match reward_type {
            RewardType::Fee => Self::Fee,
            RewardType::Rent => Self::Rent,
            RewardType::Staking => Self::Staking,
            RewardType::Voting => Self::Voting,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RocksDBReward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: i64,
    pub reward_type: Option<RocksDBRewardType>,
    pub commission: Option<i16>,
}

fn get_reward_type(reward: &Option<RewardType>) -> Option<RocksDBRewardType> {
    reward.as_ref().map(RocksDBRewardType::from)
}

impl From<&Reward> for RocksDBReward {
    fn from(reward: &Reward) -> Self {
        Self {
            pubkey: reward.pubkey.clone(),
            lamports: reward.lamports,
            post_balance: reward.post_balance as i64,
            reward_type: get_reward_type(&reward.reward_type),
            commission: reward
                .commission
                .as_ref()
                .map(|commission| *commission as i16),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RocksDBTransactionErrorCode {
    AccountInUse,
    AccountLoadedTwice,
    AccountNotFound,
    ProgramAccountNotFound,
    InsufficientFundsForFee,
    InvalidAccountForFee,
    AlreadyProcessed,
    BlockhashNotFound,
    InstructionError,
    CallChainTooDeep,
    MissingSignatureForFee,
    InvalidAccountIndex,
    SignatureFailure,
    InvalidProgramForExecution,
    SanitizeFailure,
    ClusterMaintenance,
    AccountBorrowOutstanding,
    WouldExceedMaxAccountCostLimit,
    WouldExceedMaxBlockCostLimit,
    UnsupportedVersion,
    InvalidWritableAccount,
    WouldExceedMaxAccountDataCostLimit,
    TooManyAccountLocks,
    AddressLookupTableNotFound,
    InvalidAddressLookupTableOwner,
    InvalidAddressLookupTableData,
    InvalidAddressLookupTableIndex,
    InvalidRentPayingAccount,
    WouldExceedMaxVoteCostLimit,
    WouldExceedAccountDataBlockLimit,
    WouldExceedAccountDataTotalLimit,
    DuplicateInstruction,
    InsufficientFundsForRent,
    MaxLoadedAccountsDataSizeExceeded,
    InvalidLoadedAccountsDataSizeLimit,
    ResanitizationNeeded,
    UnbalancedTransaction,
    ProgramExecutionTemporarilyRestricted,
}

impl From<&TransactionError> for RocksDBTransactionErrorCode {
    fn from(err: &TransactionError) -> Self {
        match err {
            TransactionError::AccountInUse => Self::AccountInUse,
            TransactionError::AccountLoadedTwice => Self::AccountLoadedTwice,
            TransactionError::AccountNotFound => Self::AccountNotFound,
            TransactionError::ProgramAccountNotFound => Self::ProgramAccountNotFound,
            TransactionError::InsufficientFundsForFee => Self::InsufficientFundsForFee,
            TransactionError::InvalidAccountForFee => Self::InvalidAccountForFee,
            TransactionError::AlreadyProcessed => Self::AlreadyProcessed,
            TransactionError::BlockhashNotFound => Self::BlockhashNotFound,
            TransactionError::InstructionError(_idx, _error) => Self::InstructionError,
            TransactionError::CallChainTooDeep => Self::CallChainTooDeep,
            TransactionError::MissingSignatureForFee => Self::MissingSignatureForFee,
            TransactionError::InvalidAccountIndex => Self::InvalidAccountIndex,
            TransactionError::SignatureFailure => Self::SignatureFailure,
            TransactionError::InvalidProgramForExecution => Self::InvalidProgramForExecution,
            TransactionError::SanitizeFailure => Self::SanitizeFailure,
            TransactionError::ClusterMaintenance => Self::ClusterMaintenance,
            TransactionError::AccountBorrowOutstanding => Self::AccountBorrowOutstanding,
            TransactionError::WouldExceedMaxAccountCostLimit => {
                Self::WouldExceedMaxAccountCostLimit
            }
            TransactionError::WouldExceedMaxBlockCostLimit => Self::WouldExceedMaxBlockCostLimit,
            TransactionError::UnsupportedVersion => Self::UnsupportedVersion,
            TransactionError::InvalidWritableAccount => Self::InvalidWritableAccount,
            TransactionError::WouldExceedAccountDataBlockLimit => {
                Self::WouldExceedAccountDataBlockLimit
            }
            TransactionError::WouldExceedAccountDataTotalLimit => {
                Self::WouldExceedAccountDataTotalLimit
            }
            TransactionError::TooManyAccountLocks => Self::TooManyAccountLocks,
            TransactionError::AddressLookupTableNotFound => Self::AddressLookupTableNotFound,
            TransactionError::InvalidAddressLookupTableOwner => {
                Self::InvalidAddressLookupTableOwner
            }
            TransactionError::InvalidAddressLookupTableData => Self::InvalidAddressLookupTableData,
            TransactionError::InvalidAddressLookupTableIndex => {
                Self::InvalidAddressLookupTableIndex
            }
            TransactionError::InvalidRentPayingAccount => Self::InvalidRentPayingAccount,
            TransactionError::WouldExceedMaxVoteCostLimit => Self::WouldExceedMaxVoteCostLimit,
            TransactionError::DuplicateInstruction(_) => Self::DuplicateInstruction,
            TransactionError::InsufficientFundsForRent { account_index: _ } => {
                Self::InsufficientFundsForRent
            }
            TransactionError::MaxLoadedAccountsDataSizeExceeded => {
                Self::MaxLoadedAccountsDataSizeExceeded
            }
            TransactionError::InvalidLoadedAccountsDataSizeLimit => {
                Self::InvalidLoadedAccountsDataSizeLimit
            }
            TransactionError::ResanitizationNeeded => Self::ResanitizationNeeded,
            TransactionError::UnbalancedTransaction => Self::UnbalancedTransaction,
            TransactionError::ProgramExecutionTemporarilyRestricted { account_index: _ } => {
                Self::ProgramExecutionTemporarilyRestricted
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RocksDBTransactionError {
    error_code: RocksDBTransactionErrorCode,
    error_detail: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RocksDBTransactionStatusMeta {
    pub error: Option<RocksDBTransactionError>,
    pub fee: i64,
    pub pre_balances: Vec<i64>,
    pub post_balances: Vec<i64>,
    pub inner_instructions: Option<Vec<RocksDBInnerInstructions>>,
    pub log_messages: Option<Vec<String>>,
    pub pre_token_balances: Option<Vec<RocksDBTransactionTokenBalance>>,
    pub post_token_balances: Option<Vec<RocksDBTransactionTokenBalance>>,
    pub rewards: Option<Vec<RocksDBReward>>,
}

fn get_transaction_error(result: &Result<(), TransactionError>) -> Option<RocksDBTransactionError> {
    if result.is_ok() {
        return None;
    }

    let error = result.as_ref().err().unwrap();
    Some(RocksDBTransactionError {
        error_code: RocksDBTransactionErrorCode::from(error),
        error_detail: {
            if let TransactionError::InstructionError(idx, instruction_error) = error {
                let mut error_detail = format!(
                    "InstructionError: idx ({}), error: ({})",
                    idx, instruction_error
                );
                if error_detail.len() > MAX_TRANSACTION_STATUS_LEN {
                    error_detail = error_detail
                        .to_string()
                        .split_off(MAX_TRANSACTION_STATUS_LEN);
                }
                Some(error_detail)
            } else {
                None
            }
        },
    })
}

impl From<&TransactionStatusMeta> for RocksDBTransactionStatusMeta {
    fn from(meta: &TransactionStatusMeta) -> Self {
        Self {
            error: get_transaction_error(&meta.status),
            fee: meta.fee as i64,
            pre_balances: meta
                .pre_balances
                .iter()
                .map(|balance| *balance as i64)
                .collect(),
            post_balances: meta
                .post_balances
                .iter()
                .map(|balance| *balance as i64)
                .collect(),
            inner_instructions: meta.inner_instructions.as_ref().map(|instructions| {
                instructions
                    .iter()
                    .map(RocksDBInnerInstructions::from)
                    .collect()
            }),
            log_messages: meta.log_messages.clone(),
            pre_token_balances: meta.pre_token_balances.as_ref().map(|balances| {
                balances
                    .iter()
                    .map(RocksDBTransactionTokenBalance::from)
                    .collect()
            }),
            post_token_balances: meta.post_token_balances.as_ref().map(|balances| {
                balances
                    .iter()
                    .map(RocksDBTransactionTokenBalance::from)
                    .collect()
            }),
            rewards: meta
                .rewards
                .as_ref()
                .map(|rewards| rewards.iter().map(RocksDBReward::from).collect()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RocksDBTransactionMessageHeader {
    pub num_required_signatures: i16,
    pub num_readonly_signed_accounts: i16,
    pub num_readonly_unsigned_accounts: i16,
}

impl From<&MessageHeader> for RocksDBTransactionMessageHeader {
    fn from(header: &MessageHeader) -> Self {
        Self {
            num_required_signatures: header.num_required_signatures as i16,
            num_readonly_signed_accounts: header.num_readonly_signed_accounts as i16,
            num_readonly_unsigned_accounts: header.num_readonly_unsigned_accounts as i16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RocksDBLegacy {
    pub header: RocksDBTransactionMessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<RocksDBCompiledInstruction>,
}

impl From<&Message> for RocksDBLegacy {
    fn from(message: &Message) -> Self {
        Self {
            header: RocksDBTransactionMessageHeader::from(&message.header),
            account_keys: message
                .account_keys
                .iter()
                .map(|key| key.to_string())
                .collect(),
            recent_blockhash: message.recent_blockhash.to_string(),
            instructions: message
                .instructions
                .iter()
                .map(RocksDBCompiledInstruction::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RocksDBTransactionMessageAddressTableLookup {
    pub account_key: String,
    pub writable_indexes: Vec<i16>,
    pub readonly_indexes: Vec<i16>,
}

impl From<&MessageAddressTableLookup> for RocksDBTransactionMessageAddressTableLookup {
    fn from(address_table_lookup: &MessageAddressTableLookup) -> Self {
        Self {
            account_key: address_table_lookup.account_key.to_string(),
            writable_indexes: address_table_lookup
                .writable_indexes
                .iter()
                .map(|idx| *idx as i16)
                .collect(),
            readonly_indexes: address_table_lookup
                .readonly_indexes
                .iter()
                .map(|idx| *idx as i16)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RocksDBV0 {
    pub header: RocksDBTransactionMessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<RocksDBCompiledInstruction>,
    pub address_table_lookups: Vec<RocksDBTransactionMessageAddressTableLookup>,
}

impl From<&v0::Message> for RocksDBV0 {
    fn from(message: &v0::Message) -> Self {
        Self {
            header: RocksDBTransactionMessageHeader::from(&message.header),
            account_keys: message
                .account_keys
                .iter()
                .map(|key| key.to_string())
                .collect(),
            recent_blockhash: message.recent_blockhash.to_string(),
            instructions: message
                .instructions
                .iter()
                .map(RocksDBCompiledInstruction::from)
                .collect(),
            address_table_lookups: message
                .address_table_lookups
                .iter()
                .map(RocksDBTransactionMessageAddressTableLookup::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RocksDBLoadedAddresses {
    pub writable: Vec<Vec<u8>>,
    pub readonly: Vec<Vec<u8>>,
}

impl From<&LoadedAddresses> for RocksDBLoadedAddresses {
    fn from(loaded_addresses: &LoadedAddresses) -> Self {
        Self {
            writable: loaded_addresses
                .writable
                .iter()
                .map(|pubkey| pubkey.as_ref().to_vec())
                .collect(),
            readonly: loaded_addresses
                .readonly
                .iter()
                .map(|pubkey| pubkey.as_ref().to_vec())
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RocksDBLoadedMessageV0 {
    pub message: RocksDBV0,
    pub loaded_addresses: RocksDBLoadedAddresses,
}

impl From<&v0::LoadedMessage<'_>> for RocksDBLoadedMessageV0 {
    fn from(message: &v0::LoadedMessage) -> Self {
        Self {
            message: RocksDBV0::from(&message.message as &v0::Message),
            loaded_addresses: RocksDBLoadedAddresses::from(
                &message.loaded_addresses as &LoadedAddresses,
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum RocksDBSanitizedMessage {
    Legacy(RocksDBLegacy),
    V0(RocksDBLoadedMessageV0),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxnStoreStruct {
    signature: String,
    is_vote: bool,
    slot: i64,
    message_type: i64,
    message: Option<RocksDBSanitizedMessage>,
    message_hash: Vec<u8>,
    meta: RocksDBTransactionStatusMeta,
    signatures: Vec<String>,
    write_version: i64,
    index: i64,
}

impl TxnStoreStruct {
    pub fn new() -> Self {
        Self {
            signature: "".to_string(),
            is_vote: false,
            slot: 0,
            message_type: 0,
            message: None,
            message_hash: vec![],
            meta: RocksDBTransactionStatusMeta {
                error: None,
                fee: 0,
                pre_balances: vec![],
                post_balances: vec![],
                inner_instructions: None,
                log_messages: None,
                pre_token_balances: None,
                post_token_balances: None,
                rewards: None,
            },
            signatures: vec![],
            write_version: 0,
            index: 0,
        }
    }

    pub fn from(
        signature: String,
        is_vote: bool,
        slot: i64,
        message_type: i64,
        message: Option<RocksDBSanitizedMessage>,
        message_hash: Vec<u8>,
        meta: RocksDBTransactionStatusMeta,
        signatures: Vec<String>,
        write_version: i64,
        index: i64,
    ) -> Self {
        Self {
            signature,
            is_vote,
            slot,
            message_type,
            message,
            message_hash,
            meta,
            signatures,
            write_version,
            index,
        }
    }

    // pub fn tx_store(
    //     rocksdb_connection: &RocksDBConnection,
    //     txn_count: Option<u64>,
    //     txn_data: &ReplicaTransactionInfoV2,
    //     slot: u64,
    // ) -> Self {
    //     let txn_data_struct = Self::from(
    //         txn_data.signature.to_string(),
    //         txn_data.is_vote,
    //         slot as i64,
    //         match txn_data.transaction.message() {
    //             SanitizedMessage::Legacy(_) => 0,
    //             SanitizedMessage::V0(_) => 1,
    //         },
    //         match txn_data.transaction.message() {
    //             SanitizedMessage::Legacy(legacy_message) => Some(RocksDBSanitizedMessage::Legacy(
    //                 RocksDBLegacy::from(legacy_message.message.as_ref()),
    //             )),
    //             SanitizedMessage::V0(v0_message) => Some(RocksDBSanitizedMessage::V0(
    //                 RocksDBLoadedMessageV0::from(v0_message),
    //             )),
    //         },
    //         txn_data.transaction.message_hash().as_ref().to_vec(),
    //         RocksDBTransactionStatusMeta::from(txn_data.transaction_status_meta),
    //         txn_data
    //             .transaction
    //             .signatures()
    //             .iter()
    //             .map(|signature| signature.to_string())
    //             .collect(),
    //         1,
    //         txn_data.index as i64,
    //     );
    //
    //     let root_txn_str = serde_json::to_string(&txn_data_struct)
    //         .expect("Failed to serialize the struct to JSON");
    //
    //     match rocksdb_connection.save_transaction(txn_count.unwrap(), root_txn_str.as_bytes(), txn_data_struct.slot as u64) {
    //         Ok(_) => {
    //             // println!("Transaction saved: {:?}", txn_data_struct.signature);
    //         }
    //         Err(e) => {
    //             println!("Error saving transaction to RocksDB: {}", e);
    //         }
    //     };
    //
    //     txn_data_struct
    // }

    pub fn tx_batch_store(
        rocksdb_connection: &RocksDBConnection,
        txn_data: &ReplicaTransactionInfoV2,
        slot: u64,
    ) -> Option<Self> {
        let txn_data_struct = Self::from(
            txn_data.signature.to_string(),
            txn_data.is_vote,
            slot as i64,
            match txn_data.transaction.message() {
                SanitizedMessage::Legacy(_) => 0,
                SanitizedMessage::V0(_) => 1,
            },
            match txn_data.transaction.message() {
                SanitizedMessage::Legacy(legacy_message) => Some(RocksDBSanitizedMessage::Legacy(
                    RocksDBLegacy::from(legacy_message.message.as_ref()),
                )),
                SanitizedMessage::V0(v0_message) => Some(RocksDBSanitizedMessage::V0(
                    RocksDBLoadedMessageV0::from(v0_message),
                )),
            },
            txn_data.transaction.message_hash().as_ref().to_vec(),
            RocksDBTransactionStatusMeta::from(txn_data.transaction_status_meta),
            txn_data
                .transaction
                .signatures()
                .iter()
                .map(|signature| signature.to_string())
                .collect(),
            1,
            txn_data.index as i64,
        );

        // println!("{}",txn_data_struct.signature);

        let root_txn_str = serde_json::to_string(&txn_data_struct)
            .expect("Failed to serialize the struct to JSON");

        match rocksdb_connection.save_transaction_batch(root_txn_str, txn_data_struct.slot as u64) {
            Ok(_) => {
                println!("Transaction saved: {:?}", txn_data_struct.signature);
            }
            Err(e) => {
                println!("Error saving transaction to RocksDB: {}", e);
            }
        };

        Some(txn_data_struct)
    }
}
