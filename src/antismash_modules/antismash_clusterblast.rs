use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clusterblast {
    pub schema_version: i64,
    pub record_id: String,
    pub general: General,
    pub subcluster: Subcluster,
    pub knowncluster: Knowncluster,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct General {
    pub record_id: String,
    pub schema_version: i64,
    pub results: Vec<GeneralResult>,
    pub proteins: Vec<GeneralProtein>,
    pub search_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralResult {
    pub region_number: i64,
    pub total_hits: i64,
    pub ranking: Vec<Vec<Ranking>>,
    pub prefix: String,
}

/// this one is a bit odd as there are two dicts instead of one....
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ranking {
    pub accession: Option<String>,
    pub cluster_label: Option<String>,
    #[serde(default)]
    pub proteins: Vec<String>,
    pub description: Option<String>,
    pub cluster_type: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub hits: Option<i64>,
    pub core_gene_hits: Option<i64>,
    pub blast_score: Option<f64>,
    pub synteny_score: Option<i64>,
    pub core_bonus: Option<i64>,
    #[serde(default)]
    pub pairings: Vec<(String, i64, GeneralPairing)>,
    pub similarity: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralPairing {
    pub name: String,
    pub genecluster: String,
    pub start: i64,
    pub end: i64,
    pub strand: String,
    pub annotation: String,
    pub perc_ident: i64,
    pub blastscore: f64,
    pub perc_coverage: f64,
    pub evalue: f64,
    pub locus_tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralProtein {
    pub name: String,
    pub locus_tag: String,
    pub location: String,
    pub strand: String,
    pub annotations: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subcluster {
    pub record_id: String,
    pub schema_version: i64,
    pub results: Vec<SubclusterResult>,
    pub proteins: Vec<GeneralProtein>,
    pub search_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubclusterResult {
    pub region_number: i64,
    pub total_hits: i64,
    pub ranking: Vec<Vec<Ranking>>,
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Knowncluster {
    pub record_id: String,
    pub schema_version: i64,
    pub results: Vec<GeneralResult>,
    pub proteins: Vec<GeneralProtein>,
    pub search_type: String,
    pub data_version: String,
    pub mibig_entries: std::collections::HashMap<String, MibigEntry>,
}

/// Todo: We should make a struct for these values
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MibigEntry {
    #[serde(flatten)]
    pub entries: std::collections::HashMap<
        String,
        Vec<(String, String, String, i64, String, f64, f64, f64, f64)>,
    >,
}
