use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thiopeptides {
    pub record_id: String,
    pub schema_version: i64,
    #[serde(rename = "protoclusters with motifs")]
    pub protoclusters_with_motifs: Vec<Value>,
    pub motifs: Vec<Value>,
    pub cds_features: CdsFeatures,
    pub comparippson: Comparippson,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CdsFeatures {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comparippson {
    pub db_results: Vec<DbResult>,
    pub aliases: Aliases,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbResult {
    pub database: Database,
    pub hits: Hits,
    pub aliases: Aliases,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub version: String,
    pub url: String,
    pub id_format: String,
    pub description_format: String,
    pub fields: Vec<String>,
    pub dir_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hits {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aliases {}
