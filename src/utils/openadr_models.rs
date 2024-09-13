use serde::{Deserialize, Serialize};

/// OpenADR event object
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Event")]
#[serde(rename_all = "camelCase")]
pub struct OpenADREvent {
    /// VTN provisioned ID
    id: Option<String>,
    /// VTN Provisioned on creation
    created_date_time: Option<String>,
    /// VTN Provisioned on modification
    modification_date_time: Option<String>,
    /// Used as discriminator, eg. notification object
    object_type: Option<ObjectTypes>,
    /// Program ID
    program_id: String,
    /// Event name
    event_name: Option<String>,
    /// Priority of the event - lower number is higher priority
    priority: Option<i64>,
    /// Targets
    targets: Option<Vec<ValuesMap>>,
    /// Report descriptors
    report_descriptors: Option<Vec<ReportDescriptor>>,
    /// Payload descriptors
    payload_descriptors: Option<Vec<EventPayloadDescriptor>>,
    /// Interval Period
    interval_period: Option<IntervalPeriod>,
    /// Intervals
    intervals: Vec<Interval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventPayloadDescriptor {
    /// Object type discriminator
    #[serde(rename = "objectType")]
    object_type: Option<PayloadDescriptorType>, //always event here
    /// payload type - Example: PRICE
    #[serde(rename = "payloadType")]
    payload_type: String,
    /// Units - Units of measure
    units: Option<String>,
    /// Currency - Currency of the payload - Example: USD
    currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PayloadDescriptorType {
    EVENT,
    REPORT,
}
impl PayloadDescriptorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PayloadDescriptorType::EVENT => "EVENT_PAYLOAD_DESCRIPTOR",
            PayloadDescriptorType::REPORT => "REPORT_PAYLOAD_DESCRIPTOR",
        }
    }
}

/// An object that may be used to request a report from a VEN
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDescriptor {
    /// Payload type - Example: USAGE
    #[serde(rename = "payloadType")]
    payload_type: String,
    /// Reading type - Example: DIRECT_READ
    #[serde(rename = "readingType")]
    reading_type: Option<String>,
    /// Unit of measure - Example: kWh
    units: Option<String>,
    /// Targets
    targets: Option<Vec<ValuesMap>>,
    /// Aggregate - True if the report should data from all targeted results, false if the report should be generated for each target
    aggregate: Option<bool>,
    /// Start interval - the interval on which to generate a report, -1 to generate a report at the end of the last interval
    #[serde(rename = "startInterval")]
    start_interval: Option<i64>,
    /// num intervals - the number of intervals to generate a report for, -1 to generate a report for all intervals
    #[serde(rename = "numIntervals")]
    num_intervals: Option<i64>,
    /// Historical - True indicates report on intervals preceding startInterval.
    /// False indicates report on intervals following startInterval (e.g. forecast).
    historical: Option<bool>,
    /// Frequency - the number of intervals that elapse between the reports, -1 indicates the same as numIntervals
    frequency: Option<i64>,
    /// Repeat - the number of times to repeat the report, -1 indicates repeat indefinitely
    repeat: Option<i64>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Interval {
    /// Id of the interval
    id: i64,
    /// Interval period
    #[serde(rename = "intervalPeriod")]
    interval_period: Option<IntervalPeriod>,
    /// Interval period payloads
    payloads: Vec<ValuesMap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntervalPeriod {
    /// Start time of the interval in iso8601 format
    start: String,
    /// Duration of the interval in iso8601 format
    duration: Option<String>,
    /// Randomize start time range, can be added as a duration to the start time
    #[serde(rename = "randomizeStart")]
    randomize_start: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValuesMap {
    #[serde(rename = "type")]
    pub kind: String,
    values: Vec<Values>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Values {
    String(String),
    Integer(i64),
    Boolean(bool),
}

#[derive(Debug, Serialize, Deserialize)]
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