use crate::openrtb3::bool;
use serde::{Deserialize, Serialize};

use super::{
    data_asset_format::DataAssetFormat, image_asset_format::ImageAssetFormat,
    title_asset::TitleAsset, title_asset_format::TitleAssetFormat, video_placement::VideoPlacement,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetFormat {
    id: i64,

    #[serde(default = "bool::Bool::default_false")]
    req: bool::Bool,

    title: Option<TitleAssetFormat>,
    img: Option<ImageAssetFormat>,
    video: Option<VideoPlacement>,
    data: Option<DataAssetFormat>,
    ext: Option<AssetFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetFormatExt {}
