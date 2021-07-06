use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Context {
    ext: Option<ContextExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContextExt {}
