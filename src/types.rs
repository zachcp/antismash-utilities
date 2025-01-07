use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MoleculeType {
    DNA,
    RNA,
    Protein,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Topology {
    Circular,
    Linear,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeneFunction {
    Core,
    Additional,
    Transport,
    Regulatory,
    Resistance,
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DetectionTool {
    Antismash,
    Hmmer,
    Blast,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductClass {
    #[serde(rename = "NRPS")]
    Nrps,
    #[serde(rename = "T1PKS")]
    T1pks,
    #[serde(rename = "T2PKS")]
    T2pks,
    #[serde(rename = "T3PKS")]
    T3pks,
    Terpene,
    Lanthipeptide,
    #[serde(other)]
    Other,
}

// New type patterns for domain values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EValue(pub f64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitScore(pub f64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub start: i64,
    pub end: i64,
    pub strand: Strand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Strand {
    #[serde(rename = "+")]
    Forward,
    #[serde(rename = "-")]
    Reverse,
}
