
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Segment {
  ext: Option<SegmentExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SegmentExt {}

