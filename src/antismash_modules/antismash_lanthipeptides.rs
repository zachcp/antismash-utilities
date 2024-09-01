//! This module defines the structures for parsing and representing lanthipeptide data.
//!
//! It includes structures for motifs, protoclusters, and comparison results from various databases.
//! The main structure is `Lanthipeptides`, which contains all the relevant information about
//! lanthipeptides found in a genomic sequence.
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lanthipeptides {
    pub record_id: String,
    pub schema_version: i64,
    pub motifs: Motifs,
    pub new_cds_features: Vec<Value>,
    pub protoclusters: Protoclusters,
    pub comparippson: Comparippson,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motifs {
    #[serde(flatten)]
    pub motifs: std::collections::HashMap<String, Vec<Motif>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motif {
    pub peptide_class: String,
    pub peptide_subclass: String,
    pub monoisotopic_mass: f64,
    pub molecular_weight: f64,
    pub alternative_weights: Vec<f64>,
    pub tail: String,
    pub core: String,
    pub leader: String,
    pub location: String,
    pub score: f64,
    pub locus_tag: String,
    pub tool: String,
    pub detailed_info: DetailedInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetailedInfo {
    #[serde(rename = "RODEO_score")]
    pub rodeo_score: Vec<String>,
    pub number_of_bridges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Protoclusters {
    #[serde(flatten)]
    pub clusters: std::collections::HashMap<String, Vec<String>>,
}

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
pub struct Hits {
    #[serde(flatten)]
    pub hits: std::collections::HashMap<String, Vec<Hit>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub query_name: String,
    pub reference_fields: ReferenceFields,
    pub query: Query,
    pub reference: Reference,
    pub match_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceFields {
    pub accession: String,
    pub locus: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub compounds: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub sequence: String,
    pub start: i64,
    pub end: i64,
    pub full_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub sequence: String,
    pub start: i64,
    pub end: i64,
    pub full_length: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aliases {}
