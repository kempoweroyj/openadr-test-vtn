use crate::handlers::auth::post_auth;
use crate::handlers::clear_events_list::post_clear_events;
use crate::handlers::events::get_events;
use crate::handlers::generate_polled_event::post_generate_polled_event;
use crate::handlers::ping::get_ping;
use crate::handlers::subscription::{
    delete_subscription, get_subscription, get_subscriptions, post_subscription,
};
use crate::handlers::trigger_subscription_event::post_trigger_subscription_event;
use crate::AppState;
use axum::routing::delete;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;

/// Build the router for the application
///
/// # Returns
/// - `Router`: The router for the application
pub fn build_router(shared_memory: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(get_ping))
        .route("/auth", post(post_auth))
        .route("/events", get(get_events))
        .route("/trigger/event", post(post_generate_polled_event))
        .route("/clear_events", post(post_clear_events))
        .route("/subscription", post(post_subscription))
        .route("/subscription/:id", get(get_subscription))
        .route("/subscription", get(get_subscriptions))
        .route("/subscription/:id", delete(delete_subscription))
        .route(
            "/trigger/subscription/:id",
            post(post_trigger_subscription_event),
        )
        .with_state(shared_memory)
}
