use serde::{Deserialize, Serialize};

use crate::openrtb3::{bool, language};

use super::{
    category_taxonomy, content_context, data::Data, media_rating, producer::Producer,
    production_quality,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Content {
    id: Option<String>,
    episode: Option<i64>,
    title: Option<String>,
    series: Option<String>,
    season: Option<String>,
    artist: Option<String>,
    genre: Option<String>,
    album: Option<String>,

    #[serde(rename = "isrc")]
    international_standard_recording: Option<String>,
    url: Option<String>,
    cat: Vec<String>,
    cattax: Option<category_taxonomy::CategoryTaxonomy>,
    prodq: Option<production_quality::ProductionQuality>,
    context: Option<content_context::ContentContext>,
    rating: Option<String>,

    #[serde(rename = "urating")]
    user_rating: Option<String>,

    #[serde(rename = "mrating")]
    media_rating: Option<media_rating::MediaRating>,

    keywords: Option<String>,
    #[serde(rename = "live")]
    is_live: Option<bool::Bool>,

    #[serde(rename = "srcrel")]
    is_direct: Option<bool::Bool>,

    len: Option<i32>,
    lang: Option<language::Language>,
    embed: Option<bool::Bool>,
    producer: Option<Producer>,
    data: Vec<Data>,

    ext: Option<ContentExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContentExt {}
