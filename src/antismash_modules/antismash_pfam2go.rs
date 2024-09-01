//! This module provides structures for handling Pfam2GO (Protein Family to Gene Ontology) data.
//!
//! The main structure is `Pfam2go`, which contains a collection of Pfam entries
//! mapped to their corresponding data. This module is designed to work with
//! serialized data, utilizing the `serde` framework for easy JSON handling.
//!
//! # Structures
//!
//! - `Pfam2go`: The main structure representing the entire Pfam2GO dataset.
//! - `Pfam`: Represents individual Pfam entries and their associated data.
//!
//! These structures are designed to be easily serializable and deserializable,
//! making them suitable for working with JSON data from various sources.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pfam2go {
    pub pfams: HashMap<String, Pfam>,
    pub record_id: String,
    pub schema_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pfam {
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, String>,
}
