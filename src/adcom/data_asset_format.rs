use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DataAssetFormat {
    ext: Option<DataAssetFormatExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DataAssetFormatExt {}
