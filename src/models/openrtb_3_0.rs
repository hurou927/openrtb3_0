use crate::models::serde_helper::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum At {
    FirstPrice = 1,
    SecondPrice = 2,
}
impl Default for At {
    fn default() -> Self {
        At::SecondPrice
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Test {
    NonTest = 0,
    Test = 1,
}
impl Default for Test {
    fn default() -> Self {
        Test::NonTest
    }
}
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum Package {
    IndividualWinsAccepted = 0,
    PacakgeWinOrLossOnly = 1,
}
impl Default for Package {
    fn default() -> Self {
        Package::IndividualWinsAccepted
    }
}
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum WSeat {
    Block = 0,
    Allow = 1,
}
impl Default for WSeat {
    fn default() -> Self {
        WSeat::Allow
    }
}

fn def_flrcur() -> String {
    "USD".to_string()
}

fn def_curs() -> Vec<String> {
    ["USD".to_string()].to_vec()
}
fn def_domainspec() -> String {
    "adcom".to_string()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct OpenRtb3_0 {
    openrtb: OpenRtb,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct OpenRtb {
    ver: Option<String>,

    #[serde(default = "def_domainspec")]
    domainspec: String,

    domainver: String,
    request: Option<Request>,
    response: Option<Response>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Request {
    id: String,

    #[serde(default)]
    test: Test,

    tmax: Option<i32>,

    #[serde(default)]
    at: At,

    #[serde(default = "def_curs")]
    cur: Vec<String>,

    seat: Vec<String>,
    #[serde(default)]
    wseat: WSeat,
    cdata: String,
    source: Option<Source>,
    item: Vec<Item>,
    context: Option<Context>, // Layer-4
    ext: Option<RequestExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RequestExt {}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Context {}

fn def_qty() -> i32 {
    1
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Item {
    id: String,

    #[serde(default = "def_qty")]
    qty: i32,
    qtyflt: Option<f64>,
    seq: Option<i32>,
    flr: Option<f64>,

    #[serde(default = "def_flrcur")]
    flrcur: String,

    exp: Option<i32>,
    dt: Option<i32>,

    #[serde(default)]
    dlvy: i32,

    metric: Vec<Metric>,
    deal: Vec<Deal>,
    #[serde(default)]
    private: i32,
    spec: Spec, // Layer-4
    ext: Option<ItemExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Source {
    tid: Option<String>,
    ts: i64,
    ds: String,
    dsmap: Option<String>,
    cert: Option<String>,
    digest: Option<String>,
    pchain: Option<String>,
    ext: Option<SourceExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SourceExt {}

fn def_cur() -> String {
    "USD".to_string()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Response {
    id: String,
    bidid: Option<String>,
    nbr: Option<i32>,
    #[serde(default = "def_cur")]
    cur: String,
    cdata: Option<String>,
    seatbid: Vec<Seatbid>,
    ext: Option<ResponseExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Seatbid {
    seat: Option<String>,
    package: Package,
    bid: Vec<Bid>,
    ext: Option<SeatbidExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SeatbidExt {}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Bid {
    id: Option<String>,
    item: String, // id of the item object
    price: f64,
    deal: Option<String>,
    cid: Option<String>,
    tactic: Option<String>,
    purl: Option<String>,
    burl: Option<String>,
    lurl: Option<String>,
    exp: Option<i32>,
    mid: Option<String>,
    #[serde(rename = "macro")]
    bid_macro: Vec<BidMacro>,
    media: Vec<MediaAd>, // Layer-4
    ext: Vec<BidExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ResponseExt {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BidMacro {
    key: String,
    value: String,
    ext: Option<BidMacroExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BidMacroExt {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MediaAd {}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BidExt {}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Metric {
    #[serde(rename = "type")]
    metric_type: String,
    value: String,
    vendor: Option<String>,
    ext: Option<MetricExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MetricExt {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Deal {
    id: String,
    flr: Option<f64>,
    flrcur: Option<String>,
    at: Option<At>,
    wseat: Vec<WSeat>,
    wadomain: Vec<String>,
    ext: Option<DealExt>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DealExt {}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Spec {}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ItemExt {}
