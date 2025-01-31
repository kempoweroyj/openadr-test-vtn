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
/// via POST Subscription endpoint and just checking for access/collision with the subscription ID.
///
/// This endpoint can be useful to mimic behavior from for VTNs where the initial subscription is generated
/// via the UI.
///
/// This endpoint will always generate a subscription with the id "test", pointing towards a callback URL
/// specified in the DEFAULT_CALLBACK_URL environment variable and will overwrite any existing subscription with the same ID.
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
    let auth_valid = crate::utils::authorizer::authorizer(&state.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let time_now = chrono::Utc::now();

    let callback_url = std::env::var("DEFAULT_CALLBACK_URL").expect("DEFAULT_CALLBACK_URL not set!");

    // Create a new subscription
    let subscription = Subscription {
        id: Some("test".to_string()),
        created_date_time: Some(time_now.to_rfc3339()),
        modification_date_time: Some(time_now.to_rfc3339()),
        object_type: Some(SUBSCRIPTION),
        client_name: "testing_oadr3_VEN".to_string(),
        program_id: "test_program".to_string(),
        object_operations: vec![ObjectOperation {
            object_type: vec![EVENT],
            operations: Operations {
                operations: vec![POST],
            },
            callback_url,
            bearer_token: "".to_string(),
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
