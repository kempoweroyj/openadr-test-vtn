use crate::utils::init_event_storage::dummy_event_to_storage;
use crate::utils::openadr_models::OpenADREvent;
use log::info;
use tokio::sync::RwLock;

mod handlers;
mod router;
mod utils;

pub struct AppState {
    pub event_storage: RwLock<Vec<OpenADREvent>>,
}

#[tokio::main]
async fn main() {
    // loading env file
    dotenvy::dotenv().expect("Failed to load .env file");

    // Starting logger
    env_logger::builder().format_timestamp_millis().init();
    info!("Logger started");

    // initialize the event storage array. This is very much a dummy in memory solution that will clear on restart
    // If we ever take this tool to production use, this should be replaced with a proper db solution, probably dynamo
    // This is much quicker to deal with for proof of concept purposes.
    //
    // The event storage is a array of OpenADR events which gets held in a rwlock and Arc shared state for the handlers to access
    let event_storage = utils::init_event_storage::init_event_storage().await;
    dummy_event_to_storage(&event_storage).await;

    // Build the router
    let router = router::build_router(event_storage);

    // run the app
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
