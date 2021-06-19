use crate::models::serde_helper::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenRtbRequest {
    pub id: String,

    #[serde(default = "default_as_false")]
    #[serde(deserialize_with = "bool_from_int")]
    pub test: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Imp {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {}
