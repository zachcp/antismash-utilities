//! This module contains structures for representing active site finder results.
//!
//! The main structure is `ActiveSiteFinder`, which stores information about
//! active sites found in a genomic record, including schema version, record ID,
//! and pairings of active sites with their corresponding gene clusters.
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveSiteFinder {
    #[serde(rename = "schema version")]
    pub schema_version: i64,
    #[serde(rename = "record id")]
    pub record_id: String,
    pub pairings: Vec<(String, Vec<String>)>,
}
