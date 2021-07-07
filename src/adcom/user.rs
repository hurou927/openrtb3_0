use serde::{Deserialize, Serialize};

use super::{data, extended_identifiers::ExtendedIdentifiers, geo::Geo};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    id: Option<String>,
    buyeruid: Option<String>,
    yob: Option<u32>,
    gender: Option<String>,
    keywords: Option<String>,
    consent: Option<String>,
    geo: Option<Geo>,
    data: Option<data::Data>,
    eids: Option<ExtendedIdentifiers>,
    ext: Option<UserExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserExt {}
