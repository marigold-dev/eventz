use {
    std::{collections::HashSet, sync::Mutex},
    tokio::sync::broadcast,
};

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    pub _cache: Mutex<HashSet<String>>,
    // Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<String>,
}