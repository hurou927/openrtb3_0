use serde::{Deserialize, Serialize};

use crate::openrtb3::bool;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Regs {
    coppa: Option<bool::Bool>,
    gdpr: Option<bool::Bool>,
    ext: Option<RegsExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RegsExt {}
