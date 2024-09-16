use crate::utils::openadr_models::OpenADREvent;
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Generate an event that can be polled from the get_events endpoint
///
/// Takes parameters to define the event length, the oadr resource to generate even for, limits, and how far away the event should be generated in minutes
/// The event will be generated and placed into the shared memory.
/// The use case for this is that automated tests will be able to generate an event with known parameters
/// which can then be polled and the flow of the event can be tested.
pub async fn generate_polled_event(
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

    // grab time information
    let now = chrono::Utc::now();
    let start_time = now + chrono::Duration::minutes(body.minutes_in_future as i64);

    let event_name = format!("test_event_{}", now.timestamp());

    // create the event
    let event = OpenADREvent {
        id: Some(event_name),
        created_date_time: Some(now.to_rfc3339()),
        modification_date_time: Some(now.to_rfc3339()),
        object_type: Some(crate::utils::openadr_models::ObjectTypes::EVENT),
        program_id: "1".to_string(),
        event_name: Some("test_event".to_string()),
        priority: None,
        targets: Some(vec![
            crate::utils::openadr_models::ValuesMap {
                kind: "RESOURCE_NAME".to_string(),
                values: vec![crate::utils::openadr_models::Values::String(
                    body.oadr_resource_name.clone(),
                )],
            },
            crate::utils::openadr_models::ValuesMap {
                kind: "ORGANIZATION_ID".to_string(),
                values: vec![crate::utils::openadr_models::Values::String(
                    "TestVTN".to_string(),
                )],
            },
        ]),
        report_descriptors: Some(vec![]),
        payload_descriptors: Some(vec![crate::utils::openadr_models::EventPayloadDescriptor {
            object_type: Some(crate::utils::openadr_models::PayloadDescriptorType::EVENT),
            payload_type: "IMPORT_CAPACITY_LIMIT".to_string(),
            units: Some("KW".to_string()),
            currency: None,
        }]),
        interval_period: Some(crate::utils::openadr_models::IntervalPeriod {
            start: start_time.to_rfc3339(),
            duration: Some(format!("PT{}M", body.length)),
            randomize_start: Some("PT0S".to_string()),
        }),
        intervals: vec![crate::utils::openadr_models::Interval {
            id: 0,
            interval_period: None,
            payloads: vec![crate::utils::openadr_models::ValuesMap {
                kind: "IMPORT_CAPACITY_LIMIT".to_string(),
                values: vec![crate::utils::openadr_models::Values::Integer(body.limit_kw)],
            }],
        }],
    };

    // add the event to the storage
    let mut storage = shared_mem.event_storage.write().await;
    storage.push(event.clone());

    // log the event
    log::info!("Generated event: {:?}", event);

    // return success
    Ok(StatusCode::CREATED)
}

/// Parameters for the generate_polled_event handler
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventParameters {
    /// The oadr resource name to generate the event for
    pub oadr_resource_name: String,
    /// The length of the event in minutes
    pub length: i32,
    /// The power limit during the event in kW
    pub limit_kw: i64,
    /// Event will trigger in this many minutes
    pub minutes_in_future: i32,
}
