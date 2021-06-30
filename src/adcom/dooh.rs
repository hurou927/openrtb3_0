
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dooh {
  ext: Option<DoohExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DoohExt {}

