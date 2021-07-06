use serde::{Deserialize, Serialize};

use crate::openrtb3::bool;

use super::category_taxonomy;
use super::{content, publisher};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Site {
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
    page: Option<String>,
    #[serde(rename = "ser")]
    referrer_url: Option<String>,
    search: Option<String>,
    mobile: Option<bool::Bool>,
    amp: Option<bool::Bool>,
    ext: Option<SiteExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SiteExt {}
