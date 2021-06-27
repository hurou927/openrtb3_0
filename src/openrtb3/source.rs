use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Source {
    pub tid: Option<String>,

    #[serde(rename(serialize = "ts", deserialize = "ts"))]
    pub supply_chain_start_timestamp_ms: Option<i64>,
    #[serde(rename(serialize = "ds", deserialize = "ds"))]
    pub digital_signature: Option<String>,
    pub dsmap: Option<String>,
    pub cert: Option<String>,
    pub digest: Option<String>,
    pub pchain: Option<String>,
    pub ext: Option<SourceExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SourceExt {}
