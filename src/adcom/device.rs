use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Device {
    ext: Option<DeviceExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeviceExt {}
