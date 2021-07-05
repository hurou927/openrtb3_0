
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayPlacement {
  ext: Option<DisplayPlacementExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayPlacementExt {}

