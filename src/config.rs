use serde::{Deserialize, Serialize};
mod unit_variant {
    use crate::serde_utils::named_unit_variant;
    named_unit_variant!(all);
    named_unit_variant!(store);
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub tezos_rpc_url: String,
    pub enable_sync: bool,
    pub enable_api: bool,
    pub sync_block_level_from: i32,
    pub blocks_to_check_sync: i32,
    pub data_store_mode: DataStoreMode,
    pub smart_contracts: SmartContracts,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: String::from("eventz.db"),
            tezos_rpc_url: String::from("https://mainnet.tezos.marigold.dev"),
            enable_sync: true,
            enable_api: true,
            sync_block_level_from: -1,
            blocks_to_check_sync: 10,
            data_store_mode: DataStoreMode::Prune(100),
            smart_contracts: SmartContracts::All,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SmartContracts {
    #[serde(with = "unit_variant::all")]
    All,
    Only(Vec<String>),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataStoreMode {
    #[serde(with = "unit_variant::store")]
    Store,
    Prune(u32),
}
