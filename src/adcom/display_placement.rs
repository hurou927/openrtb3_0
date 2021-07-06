use serde::{Deserialize, Serialize};

use super::{
    amphtml_rendering_type, api_framework, click_type, creative_subtype, display_context_type,
    display_format::DisplayFormat, display_placement_type, event_spec::EventSpec,
    native_format::NativeFormat, placement_position, size_unit, topframe,
};
use crate::openrtb3::bool;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayPlacement {
    pos: Option<placement_position::PlacementPosition>,

    #[serde(default = "bool::Bool::default_false", rename = "instl")]
    is_interstial: bool::Bool,

    topframe: Option<topframe::Topframe>,
    ifrbust: Vec<String>,

    #[serde(default)]
    clktype: click_type::ClickType,

    ampren: Option<amphtml_rendering_type::AmpHtmlRenderingType>,
    ptype: Option<display_placement_type::DisplayPlacementType>,
    context: Option<display_context_type::DisplayContextType>,
    mime: Vec<String>,
    api: Vec<api_framework::ApiFramework>,
    ctype: Vec<creative_subtype::Display>,
    w: Option<i32>,
    h: Option<i32>,
    #[serde(default)]
    unit: size_unit::SizeUnit,
    #[serde(default = "bool::Bool::default_false")]
    support_privacy: bool::Bool,
    displayfmt: Vec<DisplayFormat>,
    nativefmt: Option<NativeFormat>,
    event: Vec<EventSpec>,
    ext: Option<DisplayPlacementExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayPlacementExt {}
