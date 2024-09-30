use crate::AppState;
use axum::extract::multipart::Multipart;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use log::debug;
use std::sync::Arc;

/// Dummy handler for the auth flow
///
/// Generally speaking the auth flow for a VTN should follow the standard oauth2 flow. For simplicity in the test tool,
/// we're just going to be returning static fake token. The test tool should not be used to store
/// sensitive data.
///
/// # Parameters
/// - `state`: The shared memory state of the application
/// - `headers`: The headers of the request
/// - `body`: The body of the request, should be multipart form data with grant_type and scope fields
///
/// # Returns
/// - `Result<String, (StatusCode, String)>`: The token if the auth is successful, otherwise an error
pub async fn post_auth(
    state: State<Arc<AppState>>,
    headers: HeaderMap,
    mut body: Multipart,
) -> Result<String, (StatusCode, String)> {
    // Fetch secrets
    let secrets = &state.secrets;

    // Extract auth headers and validate them
    let valid_header = secrets
        .get("BASIC_AUTH_HEADER")
        .expect("BASIC_AUTH_HEADER not set!"); // Panic if secret not set
    let auth_header = headers.get("Authorization");
    match auth_header {
        Some(header) => {
            let header = header.to_str().unwrap();
            if header != valid_header {
                debug!("Invalid auth header: {}", header);
                return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
            }
        }
        None => {
            debug!("No auth header found");
            return Err((
                StatusCode::UNAUTHORIZED,
                "No authorization header".to_string(),
            ));
        }
    }

    debug!("Auth header header validated");

    // Extract the form data and "generate" a token
    // the form data contents should be grant_type, and scope. grant type should be client_credentials,
    // scope we don't really care about here for testing purposes
    let mut found_grant_type = false;
    let mut found_scope = false;
    while let Some(field) = body.next_field().await.unwrap() {
        if field.name().is_some_and(|name| name == "grant_type") {
            let value = field.text().await.unwrap();
            if value != "client_credentials" {
                debug!("Invalid grant_type: {}", value);
                return Err((StatusCode::BAD_REQUEST, "Invalid grant_type".to_string()));
            }
            debug!("Grant type found: {}", value);
            found_grant_type = true;
            continue;
        }
        if field.name().is_some_and(|name| name == "scope") {
            let value = field.text().await.unwrap();
            debug!("Scope found: {}", value);
            found_scope = true;
            continue;
        }

        // Auth should contain these 2 fields and only these 2 fields, if anything else is found, return an error
        debug!("Invalid field found: {:?}", field.name());
        return Err((StatusCode::BAD_REQUEST, "Invalid form data".to_string()));
    }

    if !found_grant_type || !found_scope {
        debug!("Missing grant_type or scope");
        return Err((
            StatusCode::BAD_REQUEST,
            "Missing grant_type or scope".to_string(),
        ));
    }

    let token = secrets.get("DUMMY_TOKEN").expect("DUMMY_TOKEN not set");
    Ok(token)
}
