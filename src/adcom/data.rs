use serde::{Deserialize, Serialize};

use super::segment::Segment;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Data {
    id: Option<String>,
    name: Option<String>,
    segment: Vec<Segment>,
    ext: Option<DataExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DataExt {}
