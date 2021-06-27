use serde::{Deserialize, Serialize};

use super::domain_spec::*;
use super::openrtb_request::*;
use super::openrtb_response::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OpenRtb3_0 {
    pub openrtb: OpenRtb,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OpenRtb {
    pub ver: Option<String>,

    pub domainspec: DomainSpec,

    pub domainver: String,
    pub request: Option<Request>,
    pub response: Option<Response>,
}
