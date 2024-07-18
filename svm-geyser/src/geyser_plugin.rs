use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use {
    crate::{
        rocksdb_client::RocksDBConnection,
        txn_settle::SvmTxnSelector,
        txn_select::TxnStoreStruct,
    },
    log::*,
    serde_derive::{Deserialize, Serialize},
    serde_json,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaTransactionInfoVersions, Result,
    },
    std::{fs::File, io::Read},
};
use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaEntryInfoVersions;

#[derive(Default)]
pub struct GeyserPluginRocksDB {
    client: Option<RocksDBConnection>,
    transaction_selector: Option<SvmTxnSelector>,
    batch_starting_slot: Option<u64>,
}

impl std::fmt::Debug for GeyserPluginRocksDB {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeyserPluginRocksdbConfig {
    pub host: Option<String>,
}

static mut TX_COUNT_RECORD: u64 = 0;

impl GeyserPlugin for GeyserPluginRocksDB {
    fn name(&self) -> &'static str {
        "GeyserPluginRocksDB"
    }

    fn on_load(&mut self, config_file: &str, is_bool: bool) -> Result<()> {
        solana_logger::setup_with_default("info");
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );
        let mut file = File::open(config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let result: serde_json::Value = serde_json::from_str(&contents).unwrap();
        self.transaction_selector = Some(Self::create_transaction_selector_from_config(&result));

        // Extract the path as a string without extra quotes
        let path = result["path"].as_str().ok_or("Path not found or is not a string").unwrap();
        let seq_path = Path::new(path).join("rocksdb_seq");

        let config = RocksDBConnection::open(&seq_path, false).unwrap();
        config.check_and_create_keys().unwrap();

        self.client = Some(config);

        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction_info: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> Result<()> {
        match &self.client {
            None => {
                return Err(GeyserPluginError::SlotStatusUpdateError {
                    msg: "Failed to persist the transaction info to the Rocksdb. Rocksdb client not found.".to_string()
                });
            }
            Some(client) => match transaction_info {
                ReplicaTransactionInfoVersions::V0_0_2(transaction_info) => unsafe {
                    if let Some(transaction_selector) = &self.transaction_selector {
                        if !transaction_selector.is_transaction_selected(
                            transaction_info.is_vote,
                            Box::new(transaction_info.transaction.message().account_keys().iter()),
                        ) {
                            return Ok(());
                        }
                    } else {
                        return Ok(());
                    }

                    let txn_count = client.get_transaction_count().unwrap();

                    let _ = TxnStoreStruct::tx_batch_store(client, transaction_info, slot);
                }
                _ => {
                    return Err(GeyserPluginError::SlotStatusUpdateError {
                        msg: "Failed to persist the transaction info to the RocksDB database. Unsupported format.".to_string()
                    });
                }
            },
        }

        Ok(())
    }

    fn transaction_notifications_enabled(&self) -> bool {
        self.transaction_selector
            .as_ref()
            .map_or_else(|| false, |selector| selector.is_enabled())
    }
}

impl GeyserPluginRocksDB {
    fn create_transaction_selector_from_config(config: &serde_json::Value) -> SvmTxnSelector {
        let transaction_selector = &config["transaction_selector"];

        if transaction_selector.is_null() {
            SvmTxnSelector::default()
        } else {
            let accounts = &transaction_selector["mentions"];
            let accounts: Vec<String> = if accounts.is_array() {
                accounts
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|val| val.as_str().unwrap().to_string())
                    .collect()
            } else {
                Vec::default()
            };
            SvmTxnSelector::new(&accounts)
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the GeyserPluginPostgres pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = GeyserPluginRocksDB::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}

#[cfg(test)]
pub(crate) mod tests {
    use {super::*, serde_json};

    #[test]
    fn test_accounts_selector_from_config() {
        let config = "{\"transaction_selector\" : { \
           \"mentions\" : [\"9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin\"] \
        }}";

        let config: serde_json::Value = serde_json::from_str(config).unwrap();
        GeyserPluginRocksDB::create_transaction_selector_from_config(&config);
    }
}
