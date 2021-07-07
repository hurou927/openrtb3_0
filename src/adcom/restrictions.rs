use serde::{Deserialize, Serialize};

use super::{category_taxonomy, creative_attribute};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Restrictions {
    bcat: Vec<String>,
    cattax: Option<category_taxonomy::CategoryTaxonomy>,
    badv: Vec<String>,
    bapp: Vec<String>,
    battr: Vec<creative_attribute::CreativeAttribute>,
    ext: Option<RestrictionsExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RestrictionsExt {}
