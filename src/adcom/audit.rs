use serde::{Deserialize, Serialize};

use super::audio_status_code;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Audit {
    status: Option<audio_status_code::AudioStatusCode>,
    feed_back: Vec<String>,
    init: Option<i32>,
    lastmod: Option<i32>,
    corr: Option<Correction>,
    ext: Option<AuditExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AuditExt {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Correction {}
