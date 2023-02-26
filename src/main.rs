use dotenvy::dotenv;
use std::error::Error;
mod api;
mod db;
mod indexer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().ok();

    // Load Service to sync the events
    let tr = tokio::runtime::Runtime::new().unwrap();
    tr.spawn(async {
        indexer::sync::run().await.unwrap();
    });

    // Start the WEB API
    api::server::run().await;

    Ok(())
}
