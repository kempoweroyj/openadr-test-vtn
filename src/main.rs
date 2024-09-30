use crate::utils::init_storage::dummy_event_to_storage;
use crate::utils::openadr_models::{OpenADREvent, Subscription};
use dashmap::DashMap;
use shuttle_runtime::SecretStore;
use tokio::sync::RwLock;

mod handlers;
mod router;
mod utils;

/// Struct to maintain the shared state of the application
pub struct AppState {
    /// Events storage array
    pub event_storage: RwLock<Vec<OpenADREvent>>,
    /// Subscriptions storage map. Key is Subscription id, content is the subscription object itself.
    pub subscriptions: DashMap<String, Subscription>,
    /// Secrets store - Used to access the application secrets defined in Secrets.toml at runtime
    pub secrets: SecretStore,
}

/// Main function to start the application
#[shuttle_runtime::main]
async fn axum(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // initialize the event storage array. This is a simple dummy in memory solution that will clear on restart for simplicity.
    // In production use, consider using a database or other persistent storage solution.
    //
    // The event storage is an array of OpenADR events which gets held in a rwlock and Arc shared state for the handlers to access
    let event_storage = utils::init_storage::init_storage(secrets.clone()).await;
    dummy_event_to_storage(&event_storage).await;

    // Manually set environment variables from secrets.toml
    // This is a workaround for dotenvy/cargo config not working with shuttle runtime
    std::env::set_var("DEFAULT_CALLBACK_URL", secrets.get("DEFAULT_CALLBACK_URL").expect("DEFAULT_CALLBACK_URL not set in secrets.toml"));
    std::env::set_var("RUST_LOG", secrets.get("RUST_LOG").expect("RUST_LOG not set in secrets.toml"));

    // Build the router
    let router = router::build_router(event_storage);

    Ok(router.into())
}
