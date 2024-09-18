use crate::utils::openadr_models;
use crate::utils::openadr_models::{OpenADREvent, Subscription, Values};
use crate::AppState;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Initialize the event storage for the application
///
/// This function initializes the event storage for the application. This is a dummy in memory solution that will clear on restart.
/// If we ever take this tool to production use, this should be replaced with a proper db solution, probably dynamo.
/// This is much quicker to deal with for proof of concept purposes.
///
/// # Returns
/// - `Arc<AppState>`: The shared memory state of the application
pub async fn init_storage() -> Arc<AppState> {
    let event_storage: RwLock<Vec<OpenADREvent>> = RwLock::new(Vec::new());

    // Subscriptions use a map so that we can easily fetch/remove them by id
    let subscriptions: DashMap<String, Subscription> = DashMap::new();
    let shared_memory = AppState { event_storage, subscriptions };
    Arc::new(shared_memory)
}

/// Add a dummy event to the event storage
///
/// # Parameters
/// - `shared_memory`: The shared memory state of the application
pub async fn dummy_event_to_storage(shared_memory: &Arc<AppState>) {
    let dummy_event: OpenADREvent = OpenADREvent {
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
        payload_descriptors: Some(vec![openadr_models::EventPayloadDescriptor {
            object_type: Some(openadr_models::PayloadDescriptorType::EVENT),
            payload_type: "IMPORT_CAPACITY_LIMIT".to_string(),
            units: Some("KW".to_string()),
            currency: None,
        }]),
        interval_period: Some(openadr_models::IntervalPeriod {
            start: "2024-09-04T10:30:30.000Z".to_string(),
            duration: Some("PT2M".to_string()),
            randomize_start: Some("PT0S".to_string()),
        }),
        intervals: vec![openadr_models::Interval {
            id: 0,
            payloads: vec![openadr_models::ValuesMap {
                kind: "IMPORT_CAPACITY_LIMIT".to_string(),
                values: vec![Values::Integer(30)],
            }],
            interval_period: None,
        }],
    };

    let mut storage = shared_memory.event_storage.write().await;
    storage.push(dummy_event);
}
