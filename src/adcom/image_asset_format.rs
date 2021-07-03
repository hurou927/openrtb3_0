use serde::{Deserialize, Serialize};

use super::native_image_asset_type;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImageAssetFormat {
    #[serde(rename = "type")]
    asset_type: Option<native_image_asset_type::NativeImageAssetType>,
    mime: String,
    w: Option<i32>,
    h: Option<i32>,
    wmin: Option<i32>,
    hmin: Option<i32>,
    wratio: Option<i32>,
    hratio: Option<i32>,

    ext: Option<ImageAssetFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImageAssetFormatExt {}
