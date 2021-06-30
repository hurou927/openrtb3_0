
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtendedIdentifiers {
  ext: Option<ExtendedIdentifiersExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtendedIdentifiersExt {}

