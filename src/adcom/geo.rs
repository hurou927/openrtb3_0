
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Geo {
  ext: Option<GeoExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GeoExt {}

