use crate::utils::authorizer::authorizer;
use crate::utils::openadr_models::OpenADREvent;
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use log::debug;
use std::sync::Arc;

/// Handler for the /events endpoint
///
/// This function returns a dummy OpenADR event object for an event in the past to test basic event handling response
/// mimicking a GET Events call to the VTN server. If a new event has been generated using the generate_event handler, the new generated event
/// will also be returned here.
///
/// # Parameters
/// - `headers`: The headers of the request
/// - `shared_memory`: The shared memory state of the application
///
/// # Returns
/// - `Result<Json<OpenADREvent>, (StatusCode, String)>`: The OpenADR event object if the auth is successful, otherwise an error
pub async fn get_events(
    headers: HeaderMap,
    shared_memory: State<Arc<AppState>>,
) -> Result<Json<Vec<OpenADREvent>>, (StatusCode, String)> {
    // auth
    let valid = authorizer(&shared_memory.secrets, headers).await;
    if !valid {
        debug!("Invalid auth header");
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Get the event storage
    let storage = shared_memory.event_storage.read().await;
    let events = storage.clone();

    debug!("Returning dummy event: {:?}", events);

    // Here's where we'd manipulate the event object, but for now we'll just return it as is
    Ok(Json(events))
}
