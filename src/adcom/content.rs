
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Content {
  ext: Option<ContentExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContentExt {}

