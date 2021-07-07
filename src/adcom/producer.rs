use serde::{Deserialize, Serialize};

use super::category_taxonomy;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Producer {
    id: Option<String>,
    name: Option<String>,
    domain: Option<String>,
    cat: Option<String>,
    cattax: Option<category_taxonomy::CategoryTaxonomy>,
    ext: Option<ProducerExt>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProducerExt {}
