use serde::{Deserialize, Serialize};

use super::{
    click_type, companion, companion_type, creative_subtype, delivery_method, linearity_mode,
    placement_position, placement_subtypes_video, playback_cessation_mode, playback_method,
    size_unit, start_delay_mode,
};
use crate::openrtb3::bool;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoPlacement {
    ptype: Option<placement_subtypes_video::PlacementSubTypesVideo>,
    pos: Option<placement_position::PlacementPosition>,
    delay: Option<start_delay_mode::StartDelayMode>,
    skip: Option<bool::Bool>,

    #[serde(default)]
    skipmin: u32,
    #[serde(default)]
    skipafter: u32,
    playmethod: Option<playback_method::PlaybackMethod>,
    playend: Option<playback_cessation_mode::PlayabckCessationMode>,
    clktype: Option<click_type::ClickType>,
    mime: Vec<String>,
    api: Vec<String>,
    ctype: Vec<creative_subtype::Video>,
    w: Option<i32>,
    h: Option<i32>,

    #[serde(default)]
    unit: size_unit::SizeUnit,

    mindur: i32,
    maxdur: i32,

    #[serde(default)]
    maxext: i32,
    minbitr: Option<i32>,
    maxbitr: Option<i32>,
    delivery: Vec<delivery_method::DeliveryMethod>,
    maxseq: Option<i32>,
    linear: Option<linearity_mode::LinearityMode>,

    #[serde(default = "bool::Bool::default_true")]
    boxing: bool::Bool,
    comp: Vec<companion::Companion>,
    comptype: Vec<companion_type::CompanionType>,
    ext: Option<VideoPlacementExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoPlacementExt {}
