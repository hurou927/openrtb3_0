use super::{api_framework::ApiFramework, creative_subtype};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Video {
    mime: Vec<String>,
    api: Vec<ApiFramework>,
    ctype: Option<creative_subtype::Video>,
    dur: Option<i32>,
    adm: Option<String>,
    curl: Option<String>,
    ext: Option<VideoExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoExt {}
