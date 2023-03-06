use {
    crate::config::Config,
    diesel::{prelude::*, sqlite::SqliteConnection},
    std::sync::Arc,
};

pub fn establish_connection(config: Arc<Config>) -> SqliteConnection {
    SqliteConnection::establish(&config.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", config.database_url))
}
