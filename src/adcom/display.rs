use serde::{Deserialize, Serialize};

use super::api_framework::*;
use super::banner::*;
use super::event::*;
use super::native::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Display {
    mime: Option<String>,
    api: Vec<ApiFramework>,
    ctype: Option<super::creative_subtype::Display>,
    w: Option<u32>,
    h: Option<u32>,
    wratio: Option<u32>,
    hratio: Option<u32>,

    #[serde(rename = "priv")]
    privacy_url: Option<String>,
    adm: Option<String>,
    curl: Option<String>,
    banner: Option<Banner>,
    native: Option<Native>,
    event: Vec<Event>,
    ext: Option<DisplayExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayExt {}
