use serde::{Deserialize, Serialize};

use super::asset_format::AssetFormat;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NativeFormat {
    asset: Vec<AssetFormat>,
    ext: Option<NativeFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NativeFormatExt {}
