
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Publisher {
  ext: Option<PublisherExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PublisherExt {}

