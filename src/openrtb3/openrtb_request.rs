use serde::{Deserialize, Serialize};

use super::adcom_context::*;
use super::auction_type::*;
use super::bool::*;
use super::currency::*;
use super::item::*;
use super::source::*;
use super::wseat_type::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request {
    pub id: String,

    #[serde(default = "Bool::default_false")]
    #[serde(rename(serialize = "test", deserialize = "test"))]
    pub is_test: Bool,

    pub tmax: Option<i32>,

    #[serde(default)]
    #[serde(rename(serialize = "at", deserialize = "at"))]
    pub auction_type: AuctionType,

    #[serde(default = "Currency::default_currencies")]
    pub cur: Vec<Currency>,
    pub seat: Vec<String>,

    #[serde(default)]
    #[serde(rename(serialize = "wseat", deserialize = "wseat"))]
    pub wseat_type: WSeatType,

    pub cdata: String,
    pub source: Option<Source>,
    pub item: Vec<Item>,
    pub context: Option<Context>, // Layer-4
    pub ext: Option<RequestExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestExt {}
