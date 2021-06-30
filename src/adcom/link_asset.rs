use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LinkAsset {
    url: String,

    #[serde(rename = "urlfb")]
    fallback_url: Option<String>,

    #[serde(rename = "trkr")]
    third_party_tracker_url: Vec<String>,

    ext: Option<LinkAssetExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LinkAssetExt {}
