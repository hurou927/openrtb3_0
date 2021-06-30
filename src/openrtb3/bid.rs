use serde::{Deserialize, Serialize};

use crate::adcom::media_ad::*;
use super::bid_macro::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Bid {
    pub id: Option<String>,
    pub item: String, // id of the item object
    pub price: f64,
    pub deal: Option<String>,
    pub cid: Option<String>,
    pub tactic: Option<String>,
    pub purl: Option<String>,
    pub burl: Option<String>,
    pub lurl: Option<String>,
    pub exp: Option<i32>,
    pub mid: Option<String>,

    #[serde(rename = "macro")]
    pub bid_macro: Vec<BidMacro>,
    pub media: Vec<MediaAd>, // Layer-4
    pub ext: Vec<BidExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BidExt {}
