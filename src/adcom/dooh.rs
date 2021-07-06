use serde::{Deserialize, Serialize};

use super::{content, publisher};
use super::{dooh_placement_type, dooh_venue_type};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dooh {
    id: Option<String>,
    name: Option<String>,
    #[serde(rename = "pub")]
    publisher: Option<publisher::Publisher>,
    content: Option<content::Content>,

    venue: Option<dooh_venue_type::DoohVenueType>,
    fixed: Option<dooh_placement_type::DoohPlacementType>,
    etime: Option<i32>,
    dpi: Option<i32>,

    ext: Option<DoohExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DoohExt {}
