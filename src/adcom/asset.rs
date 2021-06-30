use crate::openrtb3::bool::*;
use serde::{Deserialize, Serialize};

use super::{image_asset::ImageAsset, link_asset::LinkAsset, title_asset::TitleAsset};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Asset {
    id: Option<i32>,
    #[serde(default = "Bool::default_false")]
    req: Bool,
    title: Option<TitleAsset>,
    image: Option<ImageAsset>,
    video: Option<VideoAsset>,
    data: Option<DataAsset>,
    link: Option<LinkAsset>,
    ext: Option<AssetExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetExt {}
