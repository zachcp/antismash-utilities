//! This module defines the structure for Sactipeptides and related components.
//!
//! Sactipeptides are a class of ribosomally synthesized and post-translationally modified peptides (RiPPs)
//! characterized by intramolecular thioether cross-links. This module provides data structures to represent
//! the analysis results of sactipeptide biosynthetic gene clusters.
//!
//! The main structure is `Sactipeptides`, which contains information about motifs, new CDS features,
//! protoclusters, and comparison results with databases.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sactipeptides {
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
