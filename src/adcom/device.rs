use serde::{Deserialize, Serialize};

use crate::openrtb3::{bool, language};

use super::{connection_type, device_type, geo::Geo, operating_system};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Device {
    device_type: Option<device_type::DeviceType>,
    ua: Option<String>,
    ifa: Option<String>,
    dnt: Option<bool::Bool>,
    lmt: Option<bool::Bool>,
    make: Option<String>,
    model: Option<String>,
    os: Option<operating_system::OperatingSystem>,
    osv: Option<String>,
    hwv: Option<String>,
    h: Option<i32>,
    w: Option<i32>,
    ppi: Option<i32>,
    pxratio: Option<f64>,
    js: Option<i32>,
    lang: Option<language::Language>,
    ip: Option<String>,
    ipv6: Option<String>,
    xff: Option<String>,
    iptr: Option<i32>,
    carrer: Option<String>,
    mccmnc: Option<String>,
    mccmncsim: Option<String>,
    contype: Option<connection_type::ConnectionType>,
    geofetch: Option<bool::Bool>,
    geo: Option<Geo>,
    ext: Option<DeviceExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeviceExt {}
