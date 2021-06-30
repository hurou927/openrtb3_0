use super::{asset::Asset, link_asset::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Native {
    link: Option<LinkAsset>,
    asset: Vec<Asset>,
    ext: Option<NativeExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NativeExt {}
