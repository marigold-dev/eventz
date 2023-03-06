use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    #[serde(default)]
    pub data_store_mode: DataStoreMode,
    pub indirect: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataStoreMode {
    Store,
    Prune(usize),
}

impl Default for DataStoreMode {
    fn default() -> Self {
        DataStoreMode::Store
    }
}
