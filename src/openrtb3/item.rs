use serde::{Deserialize, Serialize};

use super::adcom_spec::*;
use super::currency::*;
use super::deal::*;
use super::delivery_method::*;
use super::metric::*;

fn default_qty() -> i32 {
    1
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    id: String,

    #[serde(default = "default_qty")]
    #[serde(rename(serialize = "qty", deserialize = "qty"))]
    pub quantity_of_billable_event_int: i32,
    #[serde(rename(serialize = "qtyflt", deserialize = "qtyflt"))]
    pub quantity_of_biaalbel_event_float: Option<f64>,

    // If multiple items are offered in the same bid request, the sequence number allows for the coordinated delivery.
    #[serde(rename(serialize = "seq", deserialize = "seq"))]
    pub sequence_number: Option<i32>,

    #[serde(rename(serialize = "flr", deserialize = "flr"))]
    pub floor_price: Option<f64>,

    #[serde(default)]
    #[serde(rename(serialize = "flrcur", deserialize = "flrcur"))]
    pub floor_price_currency: Currency,

    #[serde(rename(serialize = "exp", deserialize = "exp"))]
    pub fullfillment_delay_time_sec: Option<i64>,
    #[serde(rename(serialize = "dt", deserialize = "dt"))]
    pub fullfillment_timestamp_ms: Option<i64>,

    #[serde(default)]
    #[serde(rename(serialize = "dlvy", deserialize = "dlvy"))]
    pub delivery_method: DeliveryMethod,

    pub metric: Vec<Metric>,
    pub deal: Vec<Deal>,
    #[serde(default)]
    pub private: i32,
    pub spec: Spec, // Layer-4
    pub ext: Option<ItemExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemExt {}
