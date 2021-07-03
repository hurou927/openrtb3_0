use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoPlacement {
    ext: Option<VideoPlacementExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoPlacementExt {}
