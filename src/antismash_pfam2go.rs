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
