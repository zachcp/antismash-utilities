//! This module defines structures for parsing and representing data related to lassopeptides.
//! It includes structures for motifs, protoclusters, comparison results, and database hits.
//! The main structure `Lassopeptides` encapsulates all the related information.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lassopeptides {
    pub record_id: String,
    pub schema_version: i64,
    pub motifs: Motifs,
    pub new_cds_features: Vec<Value>,
    pub protoclusters: Protoclusters,
    pub comparippson_results: ComparippsonResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motifs {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Protoclusters {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparippsonResults {
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
