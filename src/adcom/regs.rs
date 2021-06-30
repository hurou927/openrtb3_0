
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Regs {
  ext: Option<RegsExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RegsExt {}

