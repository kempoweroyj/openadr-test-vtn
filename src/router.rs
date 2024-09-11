use axum::{Router, routing::get};
use crate::handlers::ping;


/// Build the router for the application
///
/// # Returns
/// - `Router`: The router for the application
pub fn build_router() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
}