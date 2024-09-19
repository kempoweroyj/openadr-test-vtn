use crate::handlers::auth::post_auth;
use crate::handlers::clear_events_list::post_clear_events;
use crate::handlers::events::get_events;
use crate::handlers::generate_initial_subscription::post_generate_initial_subscription;
use crate::handlers::generate_polled_event::post_generate_polled_event;
use crate::handlers::ping::get_ping;
use crate::handlers::subscription::{delete_subscription, get_subscription, get_subscriptions, post_subscription, put_subscription};
use crate::handlers::trigger_subscription_event::post_trigger_subscription_event;
use crate::AppState;
use axum::routing::{delete, put};
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
        .route("/admin/trigger/event", post(post_generate_polled_event))
        .route("/admin/trigger/clear_events", post(post_clear_events))
        .route("/subscription", post(post_subscription))
        .route("/subscription/:id", get(get_subscription))
        .route("/subscription", get(get_subscriptions))
        .route("/subscription/:id", delete(delete_subscription))
        .route("/subscription/:id", put(put_subscription))
        .route(
            "/admin/trigger/subscription/:id",
            post(post_trigger_subscription_event),
        )
        .route("/admin/trigger/initial_subscription", post(post_generate_initial_subscription))
        .with_state(shared_memory)
}
