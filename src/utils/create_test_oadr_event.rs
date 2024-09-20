use crate::utils::openadr_models::OpenADREvent;
use log::debug;
use serde::{Deserialize, Serialize};

/// Create a test OpenADR event
///
/// This function creates a test OpenADR event with the given parameters and returns it for use.
/// Only creating events with limit type IMPORT_CAPACITY_LIMIT and payload type KW is supported.
///
/// # Parameters
/// - `body`: The parameters to create the event
///
/// # Returns
/// - `OpenADREvent`: The created OpenADR event
pub async fn create_test_oadr_event(body: EventParameters) -> OpenADREvent {
    debug!("Creating test event with parameters: {:?}", body);

    // Grab time
    let now = chrono::Utc::now();
    let start_time = now + chrono::Duration::minutes(body.minutes_in_future as i64);
    let event_id = format!("test_event_{}", now.timestamp());

    OpenADREvent {
        id: Some(event_id),
        created_date_time: Some(now.to_rfc3339()),
        modification_date_time: Some(now.to_rfc3339()),
        object_type: Some(crate::utils::openadr_models::ObjectTypes::EVENT),
        program_id: "1".to_string(),
        event_name: Some(body.event_name),
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
    }
}

/// Parameters for creating a test event
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventParameters {
    /// Event_name - Different from event ID which is VTN generated
    pub event_name: String,
    /// The oadr resource name to generate the event for
    pub oadr_resource_name: String,
    /// The length of the event in minutes
    pub length: i32,
    /// The power limit during the event in kW
    pub limit_kw: i64,
    /// Event will trigger in this many minutes
    pub minutes_in_future: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::openadr_models::Values;

    #[tokio::test]
    async fn test_create_test_oadr_event() {
        let params = EventParameters {
            event_name: "test_event".to_string(),
            oadr_resource_name: "resource".to_string(),
            length: 60,
            limit_kw: 100,
            minutes_in_future: 5,
        };

        let event = create_test_oadr_event(params).await;

        assert_eq!(event.program_id, "1");
        assert_eq!(event.event_name, Some("test_event".to_string()));
        assert_eq!(
            event.targets.unwrap()[0].values[0],
            Values::String("resource".to_string())
        );
        assert_eq!(
            event.payload_descriptors.clone().unwrap()[0].payload_type,
            "IMPORT_CAPACITY_LIMIT"
        );
        assert_eq!(
            event.payload_descriptors.unwrap()[0].units,
            Some("KW".to_string())
        );
        assert_eq!(
            event.intervals[0].payloads[0].values[0],
            Values::Integer(100)
        );
    }
}
