
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DistributionChannel {
  ext: Option<DistributionChannelExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DistributionChannelExt {}

