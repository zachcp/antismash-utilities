//! This module contains structures and functions for parsing and representing
//! antiSMASH cluster comparison results.
//!
//! It includes data structures for representing various aspects of cluster comparisons,
//! such as region details, hits, and reference information. The module also provides
//! custom deserialization logic to handle different shapes of comparison data.
//!

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterCompare {
    pub record_id: String,
    pub schema_version: i64,
    pub db_results: DbResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbResults {
    #[serde(rename = "MIBiG")]
    pub mibi_g: MibiG,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MibiG {
    pub name: String,
    pub by_region: ByRegion,
    pub url_format: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByRegion {
    #[serde(flatten)]
    pub regions: std::collections::HashMap<String, std::collections::HashMap<String, Region>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub scores_by_region: ScoresByRegion,
    pub reference_regions: ReferenceRegions,
    #[serde(deserialize_with = "deserialize_details")]
    pub details: ComparisonDetails,
    pub hits: RegionHits,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonDetails {
    #[serde(rename = "region_to_region")]
    RegionToRegion { details: Vec<Detail> },
    #[serde(rename = "proto_to_region")]
    ProtoToRegion {
        details: HashMap<String, HashMap<String, Detail>>,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoresByRegion {
    #[serde(flatten)]
    pub scores: std::collections::HashMap<String, f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceRegions {
    #[serde(flatten)]
    pub reference: std::collections::HashMap<String, Reference>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub accession: String,
    pub cdses: Cdses,
    pub cds_mapping: CdsMapping,
    pub start: i64,
    pub end: i64,
    pub products: Vec<String>,
    pub organism: String,
    pub description: String,
    pub protoclusters: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cdses {
    #[serde(flatten)]
    pub cdses: std::collections::HashMap<String, CDS>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDS {
    pub function: String,
    pub components: Components,
    pub location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Components {
    pub modules: Vec<Value>,
    pub secmet: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CdsMapping {
    #[serde(flatten)]
    pub mapping: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionDetails {
    pub kind: String,
    pub details: ComparisonDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detail {
    pub identity: f64,
    pub order: f64,
    pub component: Option<f64>,
    pub ref_id: String,
    pub final_score: f64,
    pub mode: String,
    pub hits: HashMap<String, Hit>,
}

// /// Todo: the String here is of the form "<BGC>: Region". We should  probably extract this.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionHits {
    #[serde(flatten)]
    pub hits: std::collections::HashMap<String, std::collections::HashMap<String, Hit>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub reference_record: String,
    pub reference_id: String,
    pub query_cds_name: String,
    pub percent_identity: f64,
    pub blast_score: f64,
    pub percent_coverage: f64,
    pub evalue: f64,
}

fn deserialize_details<'de, D>(deserializer: D) -> Result<ComparisonDetails, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    if let Some(obj) = value.as_object() {
        if let Some(type_field) = obj.get("type") {
            // If the "type" field is present, parse it manually
            match type_field.as_str() {
                Some("region_to_region") => {
                    let details: Vec<Detail> =
                        serde_json::from_value(obj["details"].clone()).map_err(D::Error::custom)?;
                    Ok(ComparisonDetails::RegionToRegion { details })
                }
                Some("proto_to_region") => {
                    let details: HashMap<String, HashMap<String, Detail>> =
                        serde_json::from_value(obj["details"].clone()).map_err(D::Error::custom)?;
                    Ok(ComparisonDetails::ProtoToRegion { details })
                }
                _ => Err(D::Error::custom("Invalid type for ComparisonDetails")),
            }
        } else {
            // If no "type" field, try to determine the type based on the structure
            if obj.get("details").and_then(|d| d.as_array()).is_some() {
                let details: Vec<Detail> =
                    serde_json::from_value(obj["details"].clone()).map_err(D::Error::custom)?;
                Ok(ComparisonDetails::RegionToRegion { details })
            } else {
                let details: HashMap<String, HashMap<String, Detail>> =
                    serde_json::from_value(obj["details"].clone()).map_err(D::Error::custom)?;
                Ok(ComparisonDetails::ProtoToRegion { details })
            }
        }
    } else {
        Err(D::Error::custom("Invalid details format"))
    }
}
