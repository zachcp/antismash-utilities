use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterHmm {
    pub hits: Vec<Hit>,
    #[serde(rename = "record id")]
    pub record_id: String,
    pub schema: i64,
    #[serde(rename = "max evalue")]
    pub max_evalue: f64,
    #[serde(rename = "min score")]
    pub min_score: f64,
    pub database: String,
    pub tool: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub location: String,
    pub label: String,
    pub locus_tag: String,
    pub domain: String,
    pub evalue: f64,
    pub score: f64,
    pub identifier: String,
    pub description: String,
    pub protein_start: i64,
    pub protein_end: i64,
    pub translation: String,
}
