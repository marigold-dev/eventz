use {
    config::Config,
    std::{
        collections::HashSet,
        error::Error,
        sync::{Arc, Mutex},
    },
    tokio::sync::broadcast,
};

mod api;
mod app_state;
mod config;
mod db;
mod indexer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Arc::new(confy::load_path::<Config>("config.yml")?);
    dbg!(&config);

    let _cache = Mutex::new(HashSet::<String>::new());
    let (tx, _rx) = broadcast::channel::<String>(100);
    let app_state = Arc::new(app_state::AppState { _cache, tx });

    // Load Service to sync the events
    let tr = tokio::runtime::Runtime::new().unwrap();
    let indexer_app_state = app_state.clone();
    let indexer_config = config.clone();
    tr.spawn(async {
        indexer::sync::run(indexer_app_state, indexer_config)
            .await
            .unwrap();
    });

    // Start the WEB API
    api::server::run(app_state.clone(), config.clone()).await;

    Ok(())
}
