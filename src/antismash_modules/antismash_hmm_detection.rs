//! This module contains structures for representing HMM (Hidden Markov Model) detection results
//! in the context of biosynthetic gene cluster analysis. It includes definitions for various
//! components of the detection process, such as rule results, protoclusters, and domain definitions.
//!
//! The main structure is `HmmDetection`, which encapsulates the overall detection results.
//! Other important structures include `RuleResults`, `CdsByProtocluster`, and `DefinitionDomains`.
//!
//! This module is designed to work with Serde for serialization and deserialization of JSON data.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HmmDetection {
    pub record_id: String,
    pub schema_version: i64,
    pub enabled_types: Vec<String>,
    pub rule_results: RuleResults,
    // Todo: ENUM
    pub strictness: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleResults {
    pub schema_version: i64,
    // Todo: ENUM
    pub tool: String,
    pub cds_by_protocluster: Vec<(CdsByProtocluster, Vec<CdsByProtocluster2>)>,
    pub outside_protoclusters: Vec<Value>,
    pub multipliers: Multipliers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CdsByProtocluster {
    pub location: String,
    #[serde(rename = "type")]
    // Todo: ENUM
    pub type_field: String,
    pub qualifiers: Qualifiers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CdsByProtocluster2 {
    pub cds_name: String,
    pub domains: Vec<(String, f64, f64, i64, String)>,
    pub definition_domains: DefinitionDomains,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Qualifiers {
    #[serde(rename = "aStool")]
    // Todo: ENUM
    pub a_stool: Vec<String>,
    pub category: Vec<String>,
    // // Todo: bool?
    pub contig_edge: Vec<String>,
    pub core_location: Vec<String>,
    pub cutoff: Vec<String>,
    // Todo: ENUM
    pub contig_edge: Vec<String>,
    // Todo: ENUM
    pub detection_rule: Vec<String>,
    pub neighbourhood: Vec<String>,
    // Todo: ENUM
    pub product: Vec<String>,
    // Todo: int?
    pub protocluster_number: Vec<String>,
    // Todo: ENUM
    pub tool: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefinitionDomains {
    #[serde(rename = "NI-siderophore")]
    #[serde(default)]
    pub ni_siderophore: Vec<String>,
    #[serde(rename = "NRPS-like")]
    #[serde(default)]
    pub nrps_like: Vec<String>,
    #[serde(default)]
    pub other: Vec<String>,
    #[serde(rename = "NRPS")]
    #[serde(default)]
    pub nrps: Vec<String>,
    #[serde(rename = "T1PKS")]
    #[serde(default)]
    pub t1pks: Vec<String>,
    #[serde(default)]
    pub butyrolactone: Vec<String>,
    #[serde(default)]
    pub blactam: Vec<String>,
    #[serde(default)]
    pub terpene: Vec<String>,
    #[serde(default)]
    pub ranthipeptide: Vec<String>,
    #[serde(rename = "T3PKS")]
    #[serde(default)]
    pub t3pks: Vec<String>,
    #[serde(rename = "redox-cofactor")]
    #[serde(default)]
    pub redox_cofactor: Vec<String>,
    #[serde(rename = "lanthipeptide-class-iii")]
    #[serde(default)]
    pub lanthipeptide_class_iii: Vec<String>,
    #[serde(rename = "T2PKS")]
    #[serde(default)]
    pub t2pks: Vec<String>,
    #[serde(rename = "RiPP-like")]
    #[serde(default)]
    pub ri_pp_like: Vec<String>,
    #[serde(rename = "lanthipeptide-class-i")]
    #[serde(default)]
    pub lanthipeptide_class_i: Vec<String>,
    #[serde(default)]
    pub nucleoside: Vec<String>,
    #[serde(default)]
    pub melanin: Vec<String>,
    #[serde(rename = "aminopolycarboxylic-acid")]
    #[serde(default)]
    pub aminopolycarboxylic_acid: Vec<String>,
    #[serde(rename = "PKS-like")]
    #[serde(default)]
    pub pks_like: Vec<String>,
    #[serde(rename = "LAP")]
    #[serde(default)]
    pub lap: Vec<String>,
    #[serde(default)]
    pub ectoine: Vec<String>,
}

// note: can handle conversion better here....
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Multipliers {
    pub cutoff: f64,
    pub neighbourhood: f64,
}
