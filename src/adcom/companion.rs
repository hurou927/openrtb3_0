use serde::{Deserialize, Serialize};

use super::display_placement;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Companion {
    id: String,

    display: Option<display_placement::DisplayPlacement>,
    ext: Option<CompanionExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompanionExt {}
