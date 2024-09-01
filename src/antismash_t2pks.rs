use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct T2pks {
    pub schema_version: i64,
    pub record_id: String,
    pub protocluster_predictions: ProtoclusterPredictions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtoclusterPredictions {
    #[serde(flatten)]
    pub predictions: std::collections::HashMap<String, ProtoclusterPrediction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtoclusterPrediction {
    pub starter_units: Vec<(String, f64, f64)>,
    pub elongations: Vec<Value>,
    pub product_classes: Vec<String>,
    pub mol_weights: MolWeights,
    pub cds_preds: CdsPreds,
    pub start: i64,
    pub end: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MolWeights {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CdsPreds {
    #[serde(flatten)]
    pub predictions: std::collections::HashMap<String, Vec<(String, Option<String>, f64, f64)>>,
}
