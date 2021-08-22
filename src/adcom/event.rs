use super::api_framework::ApiFramework;
use super::event_tracking_method::EventTrackingMethod;
use super::event_type::EventType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: EventType,
    method: EventTrackingMethod,
    api: Vec<ApiFramework>,
    url: String,
    cdata: HashMap<String, String>,
    ext: Option<EventExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventExt {}
