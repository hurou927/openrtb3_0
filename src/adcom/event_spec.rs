use serde::{Deserialize, Serialize};

use super::{api_framework, event_tracking_method, event_type, list_type};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventSpec {
    #[serde(rename = "type")]
    event_type: event_type::EventType,

    method: Vec<event_tracking_method::EventTrackingMethod>,
    api: Vec<api_framework::ApiFramework>,

    #[serde(rename = "jstrk")]
    js_tracker_domains: Vec<String>,

    #[serde(rename = "wjs")]
    #[serde(default = "list_type::ListType::default_allowed")]
    js_trakcer_domains_type: list_type::ListType,

    #[serde(rename = "pxtrk")]
    pixel_tracker_domains: Vec<String>,

    #[serde(rename = "wpx")]
    #[serde(default = "list_type::ListType::default_allowed")]
    pixel_trakcer_domains_type: list_type::ListType,

    ext: Option<EventSpecExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EventSpecExt {}
