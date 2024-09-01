//! Module for handling TTA codon information in genomic data.
//!
//! This module provides structures for representing TTA codon occurrences
//! and associated genomic information such as GC content and thresholds.
//! It's designed to work with serialization and deserialization using Serde.

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tta {
    #[serde(rename = "TTA codons")]
    pub tta_codons: Vec<Codon>,
    pub schema_version: i64,
    pub record_id: String,
    pub gc_content: f64,
    pub threshold: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Codon {
    pub start: i64,
    pub strand: i64,
}
