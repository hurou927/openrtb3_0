use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TitleAssetFormat {
    len: u32,
    ext: Option<TitleAssetFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TitleAssetFormatExt {}
