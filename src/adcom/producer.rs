
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Producer {
  ext: Option<ProducerExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProducerExt {}

