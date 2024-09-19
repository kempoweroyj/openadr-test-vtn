use crate::utils::init_storage::dummy_event_to_storage;
use crate::utils::openadr_models::{OpenADREvent, Subscription};
use dashmap::DashMap;
use log::info;
use tokio::sync::RwLock;

mod handlers;
mod router;
mod utils;

pub struct AppState {
    /// Events storage array
    pub event_storage: RwLock<Vec<OpenADREvent>>,
    /// Subscriptions storage map. Key is Subscription id, content is the subscription object itself.
    pub subscriptions: DashMap<String, Subscription>,
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
    let event_storage = utils::init_storage::init_storage().await;
    dummy_event_to_storage(&event_storage).await;

    // Build the router
    let router = router::build_router(event_storage);

    let app_listener_bind = "127.0.0.1:8080";
    info!("Starting listener on: {}", app_listener_bind);
    // run the app
    let listener = tokio::net::TcpListener::bind(app_listener_bind)
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
