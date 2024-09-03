//! This module contains structures for parsing and representing RREfinder results.
//!
//! RREfinder is a tool for identifying RiPP Recognition Elements (RREs) in protein sequences.
//! The main structure `Rrefinder` represents the overall results, while `Hit` represents
//! individual RRE hits within protein sequences.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rrefinder {
    pub schema_version: i64,
    pub bitscore_cutoff: f64,
    pub hits_by_protocluster: Option<HashMap<String, Vec<String>>>,
    pub hits_by_cds: HashMap<String, Vec<Hit>>,
    pub min_length: i64,
    pub record_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    // Location
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
