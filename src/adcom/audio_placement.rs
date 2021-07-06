use serde::{Deserialize, Serialize};

use crate::openrtb3::bool;

use super::{
    api_framework, companion::Companion, companion_type, creative_subtype, delivery_method,
    feed_type, start_delay_mode, volume_normalization_mode,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AudioPlacement {
    delay: Option<start_delay_mode::StartDelayMode>,
    skippable: Option<bool::Bool>,
    #[serde(default)]
    skipmin: i32,

    #[serde(default)]
    skipafter: i32,

    feed: Option<feed_type::FeedType>,
    nvol: Option<volume_normalization_mode::VolumeNormalizationMode>,
    mime: Vec<String>,
    api: Vec<api_framework::ApiFramework>,
    ctype: Vec<creative_subtype::Video>,
    mindur: Option<i32>,
    maxdur: Option<i32>,
    maxext: Option<i32>,
    minbitr: Option<i32>,
    maxbitr: Option<i32>,
    delivery: Option<delivery_method::DeliveryMethod>,
    maxseq: Option<i32>,
    comp: Option<Companion>,
    comptype: Option<companion_type::CompanionType>,
    ext: Option<AudioPlacementExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AudioPlacementExt {}
