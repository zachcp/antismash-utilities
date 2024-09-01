use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NrpsPksDomains {
    pub cds_results: HashMap<String, CDSResult>,
    pub schema_version: i64,
    pub record_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDSResult {
    pub domain_hmms: Vec<Value>,
    pub motif_hmms: Vec<MotifHmm>,
    pub modules: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotifHmm {
    pub hit_id: String,
    pub query_start: i64,
    pub query_end: i64,
    pub evalue: f64,
    pub bitscore: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainHmm {
    pub hit_id: String,
    pub query_start: i64,
    pub query_end: i64,
    pub evalue: f64,
    pub bitscore: f64,
}
