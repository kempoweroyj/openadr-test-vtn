use crate::utils::openadr_models::ObjectTypes::{EVENT, SUBSCRIPTION};
use crate::utils::openadr_models::Operation::POST;
use crate::utils::openadr_models::{ObjectOperation, Operations, Subscription};
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use log::info;
use std::sync::Arc;

/// This handler is to generate an initial subscription object in the VTN. This is intended to mimic behavior
/// from the UI of the VTN to generate an initial subscription. This can also be completely skipped by generating the subscription
/// via PUT Subscription endpoint and just checking for access/collision with the subscription ID.
///
/// This endpoint can be useful to mimic behavior from E.On Switch VTN where the initial subscription is generated
/// via the UI.
///
/// This endpoint will always generate a subscription with the id "test", pointing towards Kempower OpenADR3 API, and will overwrite any existing subscription with the same ID.
///
/// # Parameters
/// - `state`: The shared memory state of the application
/// - `header_map`: The headers of the request
///
/// # Returns
/// - `Result<StatusCode, (StatusCode, String)>`: The status code of the request, or an error if the request failed
pub async fn post_generate_initial_subscription(
    state: State<Arc<AppState>>,
    header_map: HeaderMap,
) -> Result<StatusCode, (StatusCode, String)> {
    // Auth
    let auth_valid = crate::utils::authorizer::authorizer(header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let time_now = chrono::Utc::now();

    // Create a new subscription
    let subscription = Subscription {
        id: Some("test".to_string()),
        created_date_time: Some(time_now.to_rfc3339()),
        modification_date_time: Some(time_now.to_rfc3339()),
        object_type: Some(SUBSCRIPTION),
        client_name: "ce_oadr3_api".to_string(),
        program_id: "test_program".to_string(),
        object_operations: vec![ObjectOperation {
            object_type: vec![EVENT],
            operations: Operations {
                operations: vec![POST],
            },
            callback_url: "https://dev.kempower.io/api/openadr3/event".to_string(),
            bearer_token: "".to_string()
        }],
        targets: None,
    };

    // Store the subscription
    let _storage = state
        .subscriptions
        .insert(subscription.clone().id.unwrap(), subscription.clone());

    info!("Initial subscription created: {:?}", subscription);

    Ok(StatusCode::CREATED)
}