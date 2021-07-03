
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImageAssetFormat {
  ext: Option<ImageAssetFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImageAssetFormatExt {}

