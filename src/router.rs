use crate::handlers::{auth, ping};
use axum::{routing::get, routing::post, Router};

/// Build the router for the application
///
/// # Returns
/// - `Router`: The router for the application
pub fn build_router() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route("/auth", post(auth::auth))
}
