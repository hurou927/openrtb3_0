
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Data {
  ext: Option<DataExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DataExt {}

