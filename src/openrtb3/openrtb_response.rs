use serde::{Deserialize, Serialize};

use super::currency::*;
use super::seatbid::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response {
    pub id: String,
    pub bidid: Option<String>,
    pub nbr: Option<i32>,
    #[serde(default)]
    pub cur: Currency,
    pub cdata: Option<String>,
    pub seatbid: Vec<Seatbid>,
    pub ext: Option<ResponseExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponseExt {}
