use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BidMacro {
    pub key: String,
    pub value: String,
    pub ext: Option<BidMacroExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BidMacroExt {}
