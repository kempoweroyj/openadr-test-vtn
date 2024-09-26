use crate::utils::openadr_models::Subscription;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use log::debug;
use reqwest::StatusCode;
use std::sync::Arc;

/// Create/Update a subscription. If the subscription already exists, it will be updated.
///
/// Subscriptions can be used by the VEN receive event/program notifications from the VTN server.
/// For the purposes of the testing tool, the VTN has to support creating and modifying of the subscription
/// so that the tool maintains a valid access token and callback URL.
///
/// # Parameters
/// - `header_map`: The headers of the request
/// - `state`: The shared memory state of the application
/// - `subscription`: The subscription object to create/update
///
/// # Returns
/// - `Result<StatusCode, (StatusCode, String)>`: The status code of the request, or an error if the request failed
pub async fn post_subscription(
    header_map: HeaderMap,
    state: State<Arc<AppState>>,
    subscription: Json<Subscription>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Auth
    let auth_valid = crate::utils::authorizer::authorizer(&state.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Marshall the subscription
    let subscription = subscription.0;

    // Validate that an ID exists. Unlike the actual standard, in reality this is actually required every time
    if subscription.id.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Subscription ID is required".to_string(),
        ));
    }

    // Store the subscription
    //
    // If this was a real VTN, there should definitely be some more validation here to ensure there's
    // no multiple customers subscriptions etc, but for the purposes of the testing tool, we'll just ignore any
    // existing data and overwrite it
    let _storage = state
        .subscriptions
        .insert(subscription.clone().id.unwrap(), subscription.clone());

    debug!("Subscription created/updated: {:?}", subscription);

    Ok(StatusCode::OK)
}

/// Get all subscriptions
///
/// This function returns all the subscriptions that have been created in the VTN.
/// In a real VTN, this would be used to list all the subscriptions for the VEN/client, but we have no granular
/// access control in this tool so it just returns all the subscriptions.
///
/// # Parameters
/// - `header_map`: The headers of the request
/// - `state`: The shared memory state of the application
///
/// # Returns
/// - `Result<Vec<Subscription>, (StatusCode, String)>`: The list of subscriptions, or an error if the request failed
pub async fn get_subscriptions(
    header_map: HeaderMap,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Subscription>>, (StatusCode, String)> {
    // Auth
    let auth_valid = crate::utils::authorizer::authorizer(&state.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Get the subscriptions
    let subscriptions = state.subscriptions.clone();

    // return all the subscriptions
    let mut subscriptions_array: Vec<Subscription> = vec![];
    for (_key, subscription) in subscriptions {
        subscriptions_array.push(subscription);
    }

    debug!("Returning subscriptions: {:?}", subscriptions_array);
    Ok(Json(subscriptions_array))
}

/// Get a specific subscription
///
/// This function returns a specific subscription that has been created in the VTN.
///
/// # Parameters
/// - `header_map`: The headers of the request
/// - `state`: The shared memory state of the application
/// - `subscription_id`: The ID of the subscription to get as a path parameter
pub async fn get_subscription(
    header_map: HeaderMap,
    state: State<Arc<AppState>>,
    subscription_id: Path<String>,
) -> Result<Json<Subscription>, (StatusCode, String)> {
    // Auth
    let auth_valid = crate::utils::authorizer::authorizer(&state.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    debug!("Getting subscription: {:?}", subscription_id.0);
    // Get the subscription
    let subscription_storage = state.subscriptions.clone();
    let subscription = subscription_storage.get(&subscription_id.0);

    match subscription {
        Some(subscription) => Ok(Json(subscription.clone())),
        None => Err((StatusCode::NOT_FOUND, "Subscription not found".to_string())),
    }
}

/// Delete a specific subscription
///
/// This function deletes a specific subscription that has been created in the VTN.
/// Normally there should be access control to ensure that the user has the correct permissions to delete the subscription,
/// but for the purposes of the testing tool, there's no granular access control so anyone can delete any subscription.
///
/// # Parameters
/// - `header_map`: The headers of the request
/// - `state`: The shared memory state of the application
/// - `subscription_id`: The ID of the subscription to delete as a path parameter
///
/// # Returns
/// - `Result<StatusCode, (StatusCode, String)>`: The status code of the request, or an error if the request failed
pub async fn delete_subscription(
    header_map: HeaderMap,
    state: State<Arc<AppState>>,
    subscription_id: Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Auth
    let auth_valid = crate::utils::authorizer::authorizer(&state.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    debug!("Deleting subscription: {:?}", subscription_id.0);

    // Ensure the subscription exists
    if !state.subscriptions.contains_key(&subscription_id.0) {
        return Err((StatusCode::NOT_FOUND, "Subscription not found".to_string()));
    }

    // Delete the subscription
    let _deleted = state.subscriptions.remove(&subscription_id.0);

    Ok(StatusCode::OK)
}

/// Update an existing subscription
///
/// # Parameters
/// - `header_map`: The headers of the request
/// - `state`: The shared memory state of the application
/// - `subscription_id`: The ID of the subscription to update as a path parameter
/// - `subscription`: The subscription object to update
///
/// # Returns
/// - `Result<StatusCode, (StatusCode, String)>`: The status code of the request, or an error if the request failed
pub async fn put_subscription(
    header_map: HeaderMap,
    state: State<Arc<AppState>>,
    subscription_id: Path<String>,
    subscription: Json<Subscription>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Auth
    let auth_valid = crate::utils::authorizer::authorizer(&state.secrets, header_map).await;
    if !auth_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Marshall the subscription
    let subscription = subscription.0;

    // Check that the subscription ID matches the path parameter, and it exists in storage
    if subscription.id.is_none() || subscription.id.clone().unwrap() != subscription_id.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Subscription ID mismatch".to_string(),
        ));
    }
    // Check that the subscription exists
    if !state.subscriptions.contains_key(&subscription_id.0) {
        return Err((StatusCode::NOT_FOUND, "Subscription not found".to_string()));
    }

    // Store the subscription
    //
    // If this was a real VTN, there should definitely be some more validation here to ensure there's
    // no multiple customers subscriptions etc, but for the purposes of the testing tool, we'll just ignore any
    // existing data and overwrite it
    let _storage = state
        .subscriptions
        .insert(subscription.clone().id.unwrap(), subscription.clone());

    debug!("Subscription created/updated: {:?}", subscription);

    Ok(StatusCode::OK)
}
