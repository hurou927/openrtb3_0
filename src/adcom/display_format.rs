use serde::{Deserialize, Serialize};

use super::expandable_direction;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayFormat {
    w: Option<i32>,
    h: Option<i32>,
    wratio: Option<i32>,
    hratio: Option<i32>,
    expdir: Vec<expandable_direction::ExpandableDirection>,
    ext: Option<DisplayFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DisplayFormatExt {}
