use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TfbsFinder {
    pub schema_version: i64,
    pub pvalue: f64,
    pub start_overlap: i64,
    pub record_id: String,
    pub hits_by_region: HitsByRegion,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HitsByRegion {
    #[serde(flatten)]
    pub hits: std::collections::HashMap<String, Vec<Hit>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub name: String,
    pub start: i64,
    pub description: String,
    pub consensus: String,
    pub confidence: String,
    pub strand: i64,
    pub score: f64,
    pub max_score: f64,
}
