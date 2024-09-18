use serde::{Deserialize, Serialize};

/// OpenADR event object
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "Event")]
#[serde(rename_all = "camelCase")]
pub struct OpenADREvent {
    /// VTN provisioned ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// VTN Provisioned on creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    /// VTN Provisioned on modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_date_time: Option<String>,
    /// Used as discriminator, eg. notification object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectTypes>,
    /// Program ID
    #[serde(rename = "programID")]
    pub program_id: String,
    /// Event name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// Priority of the event - lower number is higher priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// Targets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<ValuesMap>>,
    /// Report descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_descriptors: Option<Vec<ReportDescriptor>>,
    /// Payload descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_descriptors: Option<Vec<EventPayloadDescriptor>>,
    /// Interval Period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_period: Option<IntervalPeriod>,
    /// Intervals
    pub intervals: Vec<Interval>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventPayloadDescriptor {
    /// Object type discriminator
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<PayloadDescriptorType>, //always event here
    /// payload type - Example: PRICE
    #[serde(rename = "payloadType")]
    pub payload_type: String,
    /// Units - Units of measure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    /// Currency - Currency of the payload - Example: USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PayloadDescriptorType {
    #[serde(rename = "EVENT_PAYLOAD_DESCRIPTOR")]
    EVENT,
    #[serde(rename = "REPORT_PAYLOAD_DESCRIPTOR")]
    REPORT,
}

/// An object that may be used to request a report from a VEN
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportDescriptor {
    /// Payload type - Example: USAGE
    #[serde(rename = "payloadType")]
    pub payload_type: String,
    /// Reading type - Example: DIRECT_READ
    #[serde(rename = "readingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reading_type: Option<String>,
    /// Unit of measure - Example: kWh
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    /// Targets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<ValuesMap>>,
    /// Aggregate - True if the report should data from all targeted results, false if the report should be generated for each target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<bool>,
    /// Start interval - the interval on which to generate a report, -1 to generate a report at the end of the last interval
    #[serde(rename = "startInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_interval: Option<i64>,
    /// num intervals - the number of intervals to generate a report for, -1 to generate a report for all intervals
    #[serde(rename = "numIntervals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_intervals: Option<i64>,
    /// Historical - True indicates report on intervals preceding startInterval.
    /// False indicates report on intervals following startInterval (e.g. forecast).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
    /// Frequency - the number of intervals that elapse between the reports, -1 indicates the same as numIntervals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// Repeat - the number of times to repeat the report, -1 indicates repeat indefinitely
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval {
    /// Id of the interval
    pub id: i64,
    /// Interval period
    #[serde(rename = "intervalPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_period: Option<IntervalPeriod>,
    /// Interval period payloads
    pub payloads: Vec<ValuesMap>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntervalPeriod {
    /// Start time of the interval in iso8601 format
    pub start: String,
    /// Duration of the interval in iso8601 format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Randomize start time range, can be added as a duration to the start time
    #[serde(rename = "randomizeStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_start: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValuesMap {
    #[serde(rename = "type")]
    pub kind: String,
    pub values: Vec<Values>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Values {
    String(String),
    Integer(i64),
    Boolean(bool),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ObjectTypes {
    PROGRAM,
    EVENT,
    REPORT,
    SUBSCRIPTION,
    VEN,
    RESOURCE,
}
impl ObjectTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectTypes::PROGRAM => "PROGRAM",
            ObjectTypes::EVENT => "EVENT",
            ObjectTypes::REPORT => "REPORT",
            ObjectTypes::SUBSCRIPTION => "SUBSCRIPTION",
            ObjectTypes::VEN => "VEN",
            ObjectTypes::RESOURCE => "RESOURCE",
        }
    }
}

/// OpenADR 3.0 Subscription Model
///
/// This module contains the Subscription model for OpenADR 3.0. This model is used to represent the Subscription object in the OpenADR 3.0 specification.
/// Will be used to test subscription handling for the VEN.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<String>,
    pub created_date_time: Option<String>,
    pub modification_date_time: Option<String>,
    pub object_type: Option<ObjectTypes>,
    pub client_name: String,
    #[serde(rename = "programID")]
    pub program_id: String,
    pub object_operations: Vec<ObjectOperation>,
    pub targets: Option<Vec<ValuesMap>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectOperation {
    pub object_type: Vec<ObjectTypes>,
    pub operations: Operations,
    pub callback_url: String,
    pub bearer_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Operations {
    pub operations: Vec<Operation>,
}

/// Possible subscription operation types
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Operation {
    GET,
    POST,
    PUT,
    DELETE,
}
