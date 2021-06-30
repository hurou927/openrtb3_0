
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct App {
  ext: Option<AppExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AppExt {}

