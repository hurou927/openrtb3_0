use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoAsset {
    adm: Option<String>,
    curl: Option<String>,
    ext: Option<VideoAssetExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VideoAssetExt {}
