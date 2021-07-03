use crate::openrtb3::{bool::Bool, language::Language};

use super::{
    audio_placement::AudioPlacement, display_placement::DisplayPlacement,
    server_side_ad_insertion_type::ServerSideAdInsertionType, video_placement::VideoPlacement,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Placement {
    tagid: String,

    #[serde(rename = "ssai")]
    #[serde(default)]
    server_side_ad_insertion: ServerSideAdInsertionType,
    sdk: Option<String>,
    sdkver: Option<String>,

    #[serde(default = "Bool::default_false")]
    reward: Bool,
    wlang: Vec<Language>,
    secure: Option<Bool>,

    #[serde(rename = "admx")]
    support_adm: Option<Bool>,

    #[serde(rename = "curlx")]
    support_curl: Option<Bool>,
    display: Option<DisplayPlacement>,
    video: Option<VideoPlacement>,
    audio: Option<AudioPlacement>,
    ext: Option<PlacementExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PlacementExt {}
