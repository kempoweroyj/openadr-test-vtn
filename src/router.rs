use crate::handlers::clear_events_list::post_clear_events;
use crate::handlers::{auth, events, generate_polled_event, ping};
use crate::AppState;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;

/// Build the router for the application
///
/// # Returns
/// - `Router`: The router for the application
pub fn build_router(shared_memory: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route("/auth", post(auth::auth))
        .route("/events", get(events::get_events))
        .route(
            "/generate_event",
            post(generate_polled_event::generate_polled_event),
        )
        .route("/clear_events", post(post_clear_events))
        .with_state(shared_memory)
}
