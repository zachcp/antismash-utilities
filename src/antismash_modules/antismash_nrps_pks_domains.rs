//! This module provides structures for parsing and representing NRPS-PKS (Non-Ribosomal Peptide Synthetase and Polyketide Synthase) domain information.
//!
//! It includes data structures for overall NRPS-PKS domain results, individual CDS (Coding Sequence) results,
//! and specific HMM (Hidden Markov Model) hits for motifs and domains.
//!
//! The main structures are:
//! - `NrpsPksDomains`: Represents the overall NRPS-PKS domain analysis results.
//! - `CDSResult`: Contains domain and motif HMM hits for a specific CDS.
//! - `MotifHmm` and `DomainHmm`: Represent individual HMM hits for motifs and domains respectively.
//!
//! These structures are designed to be serializable and deserializable using Serde,
//! facilitating easy conversion between Rust data structures and JSON representations.

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
    pub domain_hmms: Vec<DomainHmm>,
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
