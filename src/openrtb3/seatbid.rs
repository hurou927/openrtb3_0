use serde::{Deserialize, Serialize};

use super::bid::*;
use super::package::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Seatbid {
    pub seat: Option<String>,
    pub package: Package,
    pub bid: Vec<Bid>,
    pub ext: Option<SeatbidExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SeatbidExt {}
