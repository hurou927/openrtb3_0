use serde::{Deserialize, Serialize};

use super::link_asset::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Banner {
    img: String,
    link: Option<LinkAsset>,
    ext: Option<BannerExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BannerExt {}
