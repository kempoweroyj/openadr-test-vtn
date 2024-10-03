use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use std::sync::Arc;

/// Clear the list of events stored in the test VTN
///
/// Can be used to test the /events endpoint with an empty list of events and ensure poller handles empty
/// event lists gracefully
///
/// # Parameters
/// - `shared_memory`: The shared memory state of the application
/// - `header_map`: The headers of the request
///
/// # Returns
/// - `Result<StatusCode, (StatusCode, String)>`: The status code of the request, or an error if the request failed
pub async fn post_clear_events(
    shared_memory: State<Arc<AppState>>,
    header_map: HeaderMap,
) -> Result<StatusCode, (StatusCode, String)> {
    // auth
    let auth_valid = crate::utils::authorizer::authorizer(&shared_memory.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Clear the events list
    let mut storage = shared_memory.event_storage.write().await;
    storage.clear();

    Ok(StatusCode::OK)
}
