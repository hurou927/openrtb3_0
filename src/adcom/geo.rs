use serde::{Deserialize, Serialize};

use super::{ip_location_service, location_type};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Geo {
    #[serde(rename = "type")]
    location_type: Option<location_type::LocationType>,
    lat: Option<f64>,
    lon: Option<f64>,
    accur: Option<i32>,
    lastfix: Option<i32>,
    ipserv: Option<ip_location_service::IpLocationService>,
    country: Option<String>,
    metro: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    utcoffset: Option<i32>,

    ext: Option<GeoExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GeoExt {}
