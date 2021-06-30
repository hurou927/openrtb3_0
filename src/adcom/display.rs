use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Display {
    mime: Option<String>,
    api: Vec<i32>,
}
