use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneFunction {
    pub schema_version: i64,
    pub record_id: String,
    pub tools: Vec<Tool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    pub schema_version: i64,
    pub record_id: String,
    pub tool: String,
    pub best_hits: HashMap<String, Hit>,
    pub mapping: Mapping,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub hit_id: String,
    pub query_start: i64,
    pub query_end: i64,
    pub evalue: f64,
    pub bitscore: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mapping {
    #[serde(flatten)]
    pub mapping: std::collections::HashMap<String, String>,
}
