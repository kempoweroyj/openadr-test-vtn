use crate::utils::authorizer::authorizer;
use crate::utils::openadr_models;
use crate::utils::openadr_models::{OpenADREvent, Values};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use log::debug;

/// Handler for the /events endpoint
///
/// This function returns a dummy OpenADR event object for an event in the past to test basic event handling response
/// mimicking a GET Events call to the VTN server. If a new event has been generated using the generate_event handler, the new generated event
/// will also be returned here.
///
/// # Parameters
/// - `headers`: The headers of the request
///
/// # Returns
/// - `Result<Json<OpenADREvent>, (StatusCode, String)>`: The OpenADR event object if the auth is successful, otherwise an error
pub async fn get_events(headers: HeaderMap) -> Result<Json<OpenADREvent>, (StatusCode, String)> {
    // auth
    let valid = authorizer(headers).await;
    if !valid {
        debug!("Invalid auth header");
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let dummy_oadr_json = r#"{
        "id": "LisaTest",
        "createdDateTime": "2024-03-06T10:55:26.543Z",
        "modificationDateTime": "2024-03-06T10:55:26.543Z",
        "objectType": "EVENT",
        "programID": "1",
        "eventName": "activationRequest",
        "targets": [
        {
            "type": "RESOURCE_NAME",
            "values": [
            "LISA"
            ]
        },
        {
            "type": "ORGANIZATION_ID",
            "values": [
            "81"
            ]
        }
        ],
        "reportDescriptors": [],
        "payloadDescriptors": [
        {
            "payloadType": "IMPORT_CAPACITY_LIMIT",
            "units": "KW",
            "objectType": "EVENT_PAYLOAD_DESCRIPTOR"
        }
        ],
        "intervalPeriod": {
            "start": "2024-09-04T10:30:30.000Z",
            "duration": "PT2M",
            "randomizeStart": "PT0S"
        },
        "intervals": [
        {
            "id": 0,
            "payloads": [
            {
                "type": "IMPORT_CAPACITY_LIMIT",
                "values": [
                30
                ]
            }
            ]
        }
        ]
    }"#;

    // Create dummy OpenADR event object for an event in the past
    let mut dummy_event: OpenADREvent = OpenADREvent {
        id: Some("LisaTest".to_string()),
        created_date_time: Some("2024-03-06T10:55:26.543Z".to_string()),
        modification_date_time: Some("2024-03-06T10:55:26.543Z".to_string()),
        object_type: Some(openadr_models::ObjectTypes::EVENT),
        program_id: "1".to_string(),
        event_name: Some("activationRequest".to_string()),
        priority: None,
        targets: Some(vec![
            openadr_models::ValuesMap {
                kind: "RESOURCE_NAME".to_string(),
                values: vec![Values::String("LISA".to_string())],
            },
            openadr_models::ValuesMap {
                kind: "ORGANIZATION_ID".to_string(),
                values: vec![Values::String("81".to_string())],
            },
        ]),
        report_descriptors: Some(vec![]),
        payload_descriptors: Some(vec![
            openadr_models::EventPayloadDescriptor {
                object_type: Some(openadr_models::PayloadDescriptorType::EVENT),
                payload_type: "IMPORT_CAPACITY_LIMIT".to_string(),
                units: Some("KW".to_string()),
                currency: None,
            },
        ]),
        interval_period: Some(openadr_models::IntervalPeriod {
            start: "2024-09-04T10:30:30.000Z".to_string(),
            duration: Some("PT2M".to_string()),
            randomize_start: Some("PT0S".to_string()),
        }),
        intervals: vec![
            openadr_models::Interval {
                id: 0,
                payloads: vec![
                    openadr_models::ValuesMap {
                        kind: "IMPORT_CAPACITY_LIMIT".to_string(),
                        values: vec![Values::Integer(30)],
                    },
                ],
                interval_period: None,
            },
        ],
    };

    debug!("Returning dummy event: {:?}", dummy_event);

    // Here's where we'd manipulate the event object, but for now we'll just return it as is
    Ok(Json(dummy_event))
}
