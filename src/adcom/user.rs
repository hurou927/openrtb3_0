
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
  ext: Option<UserExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserExt {}

