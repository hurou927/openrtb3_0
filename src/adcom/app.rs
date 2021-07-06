use serde::{Deserialize, Serialize};

use super::category_taxonomy;
use super::{content, publisher};
use crate::openrtb3::bool;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct App {
    id: Option<String>,
    name: Option<String>,
    #[serde(rename = "pub")]
    publisher: Option<publisher::Publisher>,
    content: Option<content::Content>,

    domain: Option<String>,
    cat: Vec<String>,
    sectcat: Vec<String>,
    pagecat: Vec<String>,
    cattax: Option<category_taxonomy::CategoryTaxonomy>,
    privpolicy: Option<bool::Bool>,
    keywords: Option<String>,
    bundle: Option<String>,
    storeid: Option<String>,

    #[serde(rename = "ver")]
    app_version: Option<String>,
    #[serde(default = "bool::Bool::default_false")]
    is_paid: bool::Bool,
    ext: Option<AppExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AppExt {}
