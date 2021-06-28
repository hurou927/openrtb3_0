use serde::{Deserialize, Serialize};

use super::currency::*;
use super::seatbid::*;

use super::non_bid_reason_code::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response {
    pub id: String,
    pub bidid: Option<String>,
    #[serde(rename(serialize = "nbr", deserialize = "nbr"))]
    pub non_bid_reason_code: Option<NonBidReasonCode>,
    #[serde(default)]
    pub cur: Currency,
    #[serde(rename(serialize = "cdata", deserialize = "cdata"))]
    pub cookie_data: Option<String>,
    pub seatbid: Vec<Seatbid>,
    pub ext: Option<ResponseExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponseExt {}
