use serde::{Deserialize, Serialize};

use super::{
    amphtml_rendering_type, api_framework, click_type, creative_subtype, display_context_type,
    display_format::DisplayFormat, display_placement_type, event_spec::EventSpec,
    native_format::NativeFormat, placement::PlacementExt, size_unit, topframe,
};
use crate::openrtb3::bool;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayPlacement {
    pos: i32,

    #[serde(rename = "instl")]
    #[serde(default = "bool::Bool::default_false")]
    is_interstitial: bool::Bool,

    topframe: topframe::Topframe,
    ifrbust: Vec<String>,

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
    unit: Option<size_unit::SizeUnit>,

    #[serde(default = "bool::Bool::default_false")]
    support_privacy_notice: bool::Bool,

    displayfmt: Vec<DisplayFormat>,
    nativefmt: Option<NativeFormat>,
    event: Vec<EventSpec>,
    ext: Option<PlacementExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayPlacementExt {}
