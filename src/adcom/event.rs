use super::api_framework;
use super::api_framework::ApiFramework;
use super::event_tracking_method::EventTrackingMethod;
use super::event_type::EventType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: EventType,
    method: EventTrackingMethod,
    api: Vec<ApiFramework>,
    url: String,
    cdata: HashMap<String, String>,
    ext: Option<ExentExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventExt {}
