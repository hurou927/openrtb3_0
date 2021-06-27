use serde::{Deserialize, Serialize};

use super::auction_type::*;
use super::currency::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Deal {
    pub id: String,
    #[serde(rename(serialize = "flr", deserialize = "flr"))]
    pub floor_price: Option<f64>,

    #[serde(default)]
    #[serde(rename(serialize = "flrcur", deserialize = "flrcur"))]
    pub floor_price_currency: Currency,

    #[serde(default)]
    #[serde(rename(serialize = "at", deserialize = "at"))]
    pub auction_type: Option<AuctionType>,

    #[serde(rename(serialize = "wseat", deserialize = "wseat"))]
    pub allowed_seats: Vec<String>,

    #[serde(rename(serialize = "wadomain", deserialize = "wadomain"))]
    pub allowed_advertiser_domain: Vec<String>,
    pub ext: Option<DealExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DealExt {}
