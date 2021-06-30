use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TitleAsset {
    text: String,
    len: Option<u32>,
    ext: Option<TitleAssetExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TitleAssetExt {}
