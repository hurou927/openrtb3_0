use crate::openrtb3::bool;
use serde::{Deserialize, Serialize};

use super::{
    data_asset_format::DataAssetFormat, image_asset::ImageAsset, title_asset::TitleAsset,
    video_placement::VideoPlacement,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetFormat {
    id: i64,

    #[serde(default = "bool::Bool::default_false")]
    req: bool::Bool,

    title: Option<TitleAsset>,
    img: Option<ImageAsset>,
    video: Option<VideoPlacement>,
    data: Option<DataAssetFormat>,
    ext: Option<AssetFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetFormatExt {}
