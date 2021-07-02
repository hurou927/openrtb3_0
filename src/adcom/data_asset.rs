use super::native_data_asset_type::NativeDataAssetType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DataAsset {
    value: String,
    len: Option<u32>,

    #[serde(rename = "type")]
    data_type: Option<NativeDataAssetType>,
    ext: Option<DataAssetExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DataAssetExt {}
