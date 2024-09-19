use crate::utils::create_test_oadr_event::{create_test_oadr_event, EventParameters};
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use std::sync::Arc;

/// Generate an event that can be polled from the get_events endpoint
///
/// Takes parameters to define the event length, the oadr resource to generate event for, limits, and how far away the event should be generated in minutes
/// The event will be generated and placed into the shared memory.
/// The use case for this is that automated tests will be able to generate an event with known parameters
/// which can then be polled and the flow of the event can be tested.
pub async fn post_generate_polled_event(
    headers: HeaderMap,
    shared_mem: State<Arc<AppState>>,
    body: Json<EventParameters>,
) -> Result<StatusCode, (StatusCode, String)> {
    // auth TODO should be different auth method for these admin endpoints
    let auth_valid = crate::utils::authorizer::authorizer(headers).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // validate params
    if body.length < 1 || body.limit_kw < 1 || body.minutes_in_future < 1 {
        return Err((StatusCode::BAD_REQUEST, "Invalid parameters".to_string()));
    }

    // create the event
    let event = create_test_oadr_event(body.0).await;

    // add the event to the storage
    let mut storage = shared_mem.event_storage.write().await;
    storage.push(event.clone());

    // log the event
    log::info!("Generated event: {:?}", event);

    // return success
    Ok(StatusCode::CREATED)
}
