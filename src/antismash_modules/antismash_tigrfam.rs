//! This module contains structures for representing TIGRFAM (The Institute for Genomic Research's database of protein families) data.
//!
//! The main structure is `Tigrfam`, which represents a collection of TIGRFAM hits for a given record.
//! Each hit is represented by the `Hit` structure, containing details about the protein match.
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tigrfam {
    pub hits: Vec<Hit>,
    #[serde(rename = "record id")]
    pub record_id: String,
    pub schema: i64,
    #[serde(rename = "max evalue")]
    pub max_evalue: f64,
    #[serde(rename = "min score")]
    pub min_score: f64,
    // Todo: ENUM
    pub database: String,
    // Todo: ENUM
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
