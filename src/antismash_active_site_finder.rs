use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveSiteFinder {
    #[serde(rename = "schema version")]
    pub schema_version: i64,
    #[serde(rename = "record id")]
    pub record_id: String,
    pub pairings: Vec<(String, Vec<String>)>,
}
