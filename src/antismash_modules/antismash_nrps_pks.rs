//! This module defines structures for representing and parsing NRPS (Non-Ribosomal Peptide Synthetase)
//! and PKS (Polyketide Synthase) data from antiSMASH output.
//!
//! It includes structures for domain predictions, consensus information, and region predictions,
//! allowing for detailed analysis of secondary metabolite biosynthesis gene clusters.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NrpsPks {
    pub schema_version: i64,
    pub record_id: String,
    pub domain_predictions: HashMap<String, Option<DomainPrediction>>,
    pub consensus: Consensus,
    pub consensus_transat: ConsensusTransat,
    pub region_predictions: RegionPredictions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainPrediction {
    pub nrpys: Option<Nrpys>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nrpys {
    pub aa10: String,
    pub aa34: String,
    pub stachelhaus_matches: Vec<StachelhausMatch>,
    pub physiochemical_class: PhysiochemicalClass,
    pub large_cluster: LargeCluster,
    pub small_cluster: SmallCluster,
    pub single_amino: SingleAmino,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StachelhausMatch {
    pub substrates: Vec<Substrate>,
    pub signature: String,
    pub aa10_score: f64,
    pub aa34_score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Substrate {
    // Todo: ENUM
    pub long: String,
    // Todo: ENUM
    pub short: String,
    // Todo: ENUM
    pub norine: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysiochemicalClass {
    // Todo: ENUM
    pub name: String,
    pub score: f64,
    pub substrates: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LargeCluster {
    pub name: String,
    pub score: f64,
    pub substrates: Vec<Substrate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmallCluster {
    // Todo: ENUM
    pub name: String,
    pub score: f64,
    pub substrates: Vec<Substrate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleAmino {
    // Todo: ENUM
    pub name: String,
    pub score: f64,
    pub substrates: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consensus {
    #[serde(flatten)]
    pub predictions: std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsensusTransat {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionPredictions {
    #[serde(flatten)]
    pub predictions: std::collections::HashMap<String, Vec<RegionPrediction>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionPrediction {
    pub sc_number: i64,
    pub polymer: String,
    pub docking_used: bool,
    pub smiles: String,
    pub ordering: Vec<String>,
}
