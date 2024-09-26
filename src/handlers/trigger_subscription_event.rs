use crate::utils::create_test_oadr_event::{create_test_oadr_event, EventParameters};
use crate::utils::openadr_models::ObjectTypes::EVENT;
use crate::utils::openadr_models::OpenADREvent;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use log::{debug, info};
use reqwest::StatusCode;
use std::sync::Arc;

/// Create an active event and send it to the VEN according to the specified subscription
///
/// Takes event parameters in body to create a new active event and sends it to the VEN according to the subscription
/// which is specified in the path parameter. Subscription info is used to create the request and send it to the VEN.
///
/// # Parameters
/// - `header_map`: The headers of the request
/// - `body`: The event parameters to create the event
/// - `subscription_id`: The ID of the subscription to send the event to
/// - `state`: The shared memory state of the application
///
/// # Returns
/// - `Result<StatusCode, (StatusCode, String)>`: The status code of the request, or an error if the request failed
pub async fn post_trigger_subscription_event(
    header_map: HeaderMap,
    subscription_id: Path<String>,
    state: State<Arc<AppState>>,
    body: Json<EventParameters>,
) -> Result<StatusCode, (StatusCode, String)> {
    debug!("Triggering subscription event with parameters: {:?}", body);
    // Auth
    let auth_valid = crate::utils::authorizer::authorizer(&state.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Check that subscription exists
    let subscription_storage = state.subscriptions.clone();
    let subscription = subscription_storage.get(&subscription_id.0);
    let subscription_object_operations = match subscription {
        Some(subscription) => subscription.clone().object_operations,
        None => {
            debug!("Subscription not found");
            return Err((StatusCode::NOT_FOUND, "Subscription not found".to_string()));
        }
    };

    // Validate parameters
    if body.length < 1 || body.limit_kw < 1 || body.minutes_in_future < 1 {
        return Err((StatusCode::BAD_REQUEST, "Invalid parameters".to_string()));
    }

    // Create a new event based on the event parameters
    let oadr_event: OpenADREvent = create_test_oadr_event(body.0).await;

    // Run through the object operations, and if they have an event type as an operation, send a request according to parameters
    for object_operation in subscription_object_operations {
        // Check if the object operations has events as an operation
        if object_operation.object_type.contains(&EVENT) {
            // Send the event to the VEN using the object operation parameters
            info!(
                "Sending event to VEN with parameters: {:?}",
                object_operation
            );
            let _request = reqwest::Client::new()
                .post(&object_operation.callback_url)
                .bearer_auth(&object_operation.bearer_token)
                .json(&oadr_event)
                .send()
                .await;
        }
    }
    Ok(StatusCode::OK)
}
