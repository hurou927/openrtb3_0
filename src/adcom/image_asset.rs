use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImageAsset {
    url: String,
    w: Option<u32>,
    h: Option<u32>,

    #[serde(rename = "type")]
    native_image_asset_type: Option<u32>, // todo: enum
    ext: Option<ImageAssetExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImageAssetExt {}
