use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Segment {
    id: Option<String>,
    name: Option<String>,
    value: Option<String>,
    ext: Option<SegmentExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SegmentExt {}
