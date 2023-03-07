use serde::{Deserialize, Serialize};
mod unit_variant {
    use crate::serde_utils::named_unit_variant;
    named_unit_variant!(all);
    named_unit_variant!(use_constants);
    named_unit_variant!(store);
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub tezos_rpc_url: String,
    pub enable_sync: bool,
    pub enable_api: bool,
    pub data_store_mode: DataStoreMode,
    pub sync_block_level: i32,
    pub pooling_interval: PollingInterval,
    pub smart_contracts: SmartContracts,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: String::from("eventz.db"),
            tezos_rpc_url: String::from("https://mainnet.tezos.marigold.dev"),
            enable_sync: true,
            enable_api: true,
            data_store_mode: DataStoreMode::Store,
            sync_block_level: -1,
            pooling_interval: PollingInterval::UseConstants,
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
pub enum PollingInterval {
    #[serde(with = "unit_variant::use_constants")]
    UseConstants,
    Fixed(u64),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataStoreMode {
    #[serde(with = "unit_variant::store")]
    Store,
    Prune(usize),
}
