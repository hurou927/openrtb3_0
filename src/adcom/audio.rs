use serde::{Deserialize, Serialize};

use super::api_framework::ApiFramework;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Audio {
    mime: Vec<String>,
    api: Vec<ApiFramework>,
    ctype: Option<super::creative_subtype::Video>,
    dur: Option<i32>,
    adm: Option<String>,
    curl: Option<String>,
    ext: Option<AudioExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AudioExt {}
