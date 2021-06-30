
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtendedIdentifierUids {
  ext: Option<ExtendedIdentifierUidsExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtendedIdentifierUidsExt {}

