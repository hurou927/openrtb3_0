
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Site {
  ext: Option<SiteExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SiteExt {}

