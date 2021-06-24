use crate::models::serde_helper::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OpenRtbRequest {
    pub id: String,

    #[serde(default = "default_as_zero")]
    pub test: i64,

    pub imp: Option<Imp>,
    pub site: Option<Site>,
    pub app: Option<App>,
    pub device: Option<Device>,
    pub user: Option<User>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Imp {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Site {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct App {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Device {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {}
