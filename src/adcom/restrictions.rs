
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Restrictions {
  ext: Option<RestrictionsExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RestrictionsExt {}

