use super::adcom_audio::*;
use super::adcom_audit::*;
use super::adcom_creative_attribute::*;
use super::adcom_display::*;
use super::adcom_media_rating::*;
use super::adcom_video::*;
use super::bool::*;
use super::language::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MediaAd {
    pub id: String,

    #[serde(rename(serialize = "adomain", deserialize = "adomain"))]
    pub advertiser_domain: String,

    pub bundle: Option<String>,
    pub iurl: Option<String>,
    #[serde(rename(serialize = "cat", deserialize = "cat"))]
    pub category: Vec<String>,

    #[serde(rename(serialize = "cattax", deserialize = "cattax"))]
    pub category_taxonomy: Vec<String>,
    pub lang: Option<Language>,
    pub attr: Vec<CreativeAttribute>,
    pub is_secure: Option<Bool>,

    #[serde(rename(serialize = "mrating", deserialize = "mrating"))]
    pub media_rating: Option<MediaRating>,

    #[serde(rename(serialize = "init", deserialize = "init"))]
    pub ad_instanciation_timestamp_ms: Option<i64>,

    #[serde(rename(serialize = "lastmod", deserialize = "lastmod"))]
    pub last_modification_timestamp_ms: Option<i64>,

    pub display: Option<Display>,
    pub video: Option<Video>,
    pub audio: Option<Audio>,
    pub audit: Option<Audit>,
    pub ext: Option<MediaAdExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MediaAdExt {}
