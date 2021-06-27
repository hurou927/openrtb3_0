use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Metric {
    #[serde(rename = "type")]
    pub metric_type: String,
    pub value: String,
    pub vendor: Option<String>,
    pub ext: Option<MetricExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MetricExt {}
