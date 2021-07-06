use serde::{Deserialize, Serialize};

use super::category_taxonomy;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Publisher {
    id: Option<String>,
    name: Option<String>,
    domain: Option<String>,
    cat: Vec<String>,
    cattax: Option<category_taxonomy::CategoryTaxonomy>,
    ext: Option<PublisherExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PublisherExt {}
