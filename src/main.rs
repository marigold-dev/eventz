use {
    dotenvy::dotenv,
    std::{
        collections::HashSet,
        error::Error,
        sync::{Arc, Mutex},
    },
    tokio::sync::broadcast,
};
mod api;
mod db;
mod indexer;

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    _cache: Mutex<HashSet<String>>,
    // Channel used to send messages to all connected clients.
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().ok();

    let _cache = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel::<String>(100);
    let app_state = Arc::new(AppState { _cache, tx });

    // Load Service to sync the events
    let tr = tokio::runtime::Runtime::new().unwrap();
    let indexer_app_state = app_state.clone();
    tr.spawn(async {
        indexer::sync::run(indexer_app_state).await.unwrap();
    });

    // Start the WEB API
    api::server::run(app_state.clone()).await;

    Ok(())
}
