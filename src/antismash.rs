use crate::antismash_modules::antismash_active_site_finder::ActiveSiteFinder;
use crate::antismash_modules::antismash_cluster_compare::ClusterCompare;
use crate::antismash_modules::antismash_cluster_hmmer::ClusterHmm;
use crate::antismash_modules::antismash_clusterblast::Clusterblast;
use crate::antismash_modules::antismash_genefunctions::GeneFunction;
use crate::antismash_modules::antismash_hmm_detection::HmmDetection;
use crate::antismash_modules::antismash_lanthipeptides::Lanthipeptides;
use crate::antismash_modules::antismash_lassopeptides::Lassopeptides;
use crate::antismash_modules::antismash_nrps_pks::NrpsPks;
use crate::antismash_modules::antismash_nrps_pks_domains::NrpsPksDomains;
use crate::antismash_modules::antismash_pfam2go::Pfam2go;
use crate::antismash_modules::antismash_rrefinder::Rrefinder;
use crate::antismash_modules::antismash_sactipeptides::Sactipeptides;
use crate::antismash_modules::antismash_t2pks::T2pks;
use crate::antismash_modules::antismash_tfbs_finder::TfbsFinder;
use crate::antismash_modules::antismash_thiopeptides::Thiopeptides;
use crate::antismash_modules::antismash_tigrfam::Tigrfam;
use crate::antismash_modules::antismash_tta::Tta;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntismashJson {
    pub version: String,
    pub input_file: String,
    pub records: Vec<Record>,
    // TODO: Replace with an enum
    pub timings: HashMap<String, HashMap<String, f64>>,
    pub taxon: String,
    pub schema: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub seq: Seq,
    pub features: Vec<Feature>,
    pub name: String,
    pub description: String,
    pub dbxrefs: Vec<String>,
    pub annotations: Annotations,
    pub letter_annotations: LetterAnnotations,
    pub areas: Vec<Area>,
    pub gc_content: f64,
    pub modules: Modules,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Seq {
    pub data: String,
    // TODO: Replace with enum
    pub alphabet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub location: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub qualifiers: Qualifiers,
}

/// Feature Qualifiers
/// Todo: revist this. I stuck abunch of JSON::Values here to parse but its not ideas.
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Qualifiers {
    #[serde(default)]
    // Todo: ENUM / Special Parser here
    pub culture_collection: Vec<String>,
    #[serde(default)]
    // Todo: Special Parser here
    pub db_xref: Vec<String>,
    #[serde(default)]
    // Todo: ENUM
    pub mol_type: Vec<String>,
    #[serde(default)]
    pub organism: Vec<String>,
    #[serde(default)]
    pub strain: Vec<String>,
    #[serde(default)]
    pub type_material: Vec<String>,
    #[serde(default)]
    pub locus_tag: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub codon_start: Vec<i32>,
    #[serde(default)]
    pub inference: Vec<String>,
    #[serde(default)]
    pub note: Vec<String>,
    #[serde(default)]
    // Todo: cooerce to ENUM
    pub product: Vec<String>,
    #[serde(default)]
    pub protein_id: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub transl_table: Vec<String>,
    #[serde(default)]
    pub translation: Vec<String>,
    #[serde(rename = "aStool")]
    #[serde(default)]
    // Todo: Make Enum
    pub a_stool: Vec<String>,
    #[serde(default)]
    // Todo: cooerce to ENUM
    pub category: Vec<String>,
    #[serde(default)]
    // Todo: make bool
    pub contig_edge: Vec<String>,
    #[serde(default)]
    // Todo: cooerce to Location
    pub core_location: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub cutoff: Vec<i32>,
    #[serde(default)]
    // Todo: cooerce to ENUM
    pub detection_rule: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub neighbourhood: Vec<i32>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub protocluster_number: Vec<i32>,
    #[serde(default)]
    // Todo: cooerce to Enum
    pub tool: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub candidate_cluster_number: Vec<i32>,
    #[serde(default)]
    // Todo: cooerce to ENUM
    pub detection_rules: Vec<String>,
    #[serde(default)]
    // Todo: cooerce to Int // ENUM
    pub kind: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub protoclusters: Vec<i32>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub candidate_cluster_numbers: Vec<i32>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub region_number: Vec<i32>,
    #[serde(default)]
    // Todo: cooerce to ENUM
    pub rules: Vec<String>,
    #[serde(default)]
    pub subregion_numbers: Vec<String>,
    // Todo: cooerce to ENUM
    #[serde(rename = "aSDomain")]
    #[serde(default)]
    // Todo: cooerce to ENUM
    pub a_sdomain: Vec<String>,
    #[serde(rename = "aSTool")]
    #[serde(default)]
    // Todo: ENUM
    pub a_stool2: Vec<String>,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub detection: Vec<String>,
    #[serde(default)]
    pub domain_id: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub evalue: Vec<f64>,
    #[serde(default)]
    pub identifier: Vec<String>,
    #[serde(default)]
    pub label: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub protein_end: Vec<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub protein_start: Vec<i64>,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub score: Vec<f64>,
    #[serde(default)]
    // Todo: cooerce to ENUM
    pub database: Vec<String>,
    #[serde(rename = "NRPS_PKS")]
    #[serde(default)]
    // Todo: Review
    pub nrps_pks: Vec<String>,
    #[serde(default)]
    pub gene_functions: Vec<String>,
    #[serde(default)]
    // Todo: Review
    pub gene_kind: Vec<String>,
    #[serde(default)]
    // Todo: Review
    pub sec_met_domain: Vec<String>,
    #[serde(default)]
    // Todo: Review
    pub domain_subtypes: Vec<String>,
    #[serde(default)]
    pub pseudo: Vec<String>,
    #[serde(rename = "EC_number")]
    #[serde(default)]
    pub ec_number: Vec<String>,
    #[serde(default)]
    pub gene: Vec<String>,
    pub complete: Option<bool>,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub locus_tags: Vec<String>,
    // Todo: Enum
    pub starter_module: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_field: Vec<String>,
    pub final_module: Option<String>,
    #[serde(default)]
    pub anticodon: Vec<String>,
    pub incomplete: Option<String>,
    #[serde(default)]
    // Todo: Enum
    pub bound_moiety: Vec<String>,
    #[serde(default)]
    // Todo: Enum
    pub regulatory_class: Vec<String>,
    #[serde(rename = "ncRNA_class")]
    #[serde(default)]
    // Todo: Enum
    pub nc_rna_class: Vec<String>,
    #[serde(default)]
    // Todo: Enum
    pub rpt_family: Vec<String>,
    #[serde(default)]
    // Todo: Enum
    pub rpt_type: Vec<String>,
    #[serde(default)]
    pub rpt_unit_range: Vec<String>,
    #[serde(default)]
    pub rpt_unit_seq: Vec<String>,
    pub gene_synonym: Option<Vec<String>>,
    #[serde(default)]
    pub plasmid: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotations {
    // Todo: Enum
    pub molecule_type: String,
    // Todo: Enum
    pub topology: String,
    // Todo: Enum
    pub data_file_division: String,
    // Todo: Date
    pub date: String,
    pub accessions: Vec<String>,
    pub sequence_version: i64,
    pub keywords: Vec<String>,
    pub source: String,
    pub organism: String,
    pub taxonomy: Vec<String>,
    pub references: Vec<Reference>,
    pub comment: String,
    pub structured_comment: StructuredComment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub location: Option<Vec<String>>,
    pub authors: String,
    pub consrtm: String,
    pub title: String,
    pub journal: String,
    pub medline_id: String,
    pub pubmed_id: String,
    pub comment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuredComment {
    #[serde(rename = "Genome-Assembly-Data")]
    pub genome_assembly_data: GenomeAssemblyData,
    #[serde(rename = "Genome-Annotation-Data")]
    pub genome_annotation_data: GenomeAnnotationData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenomeAssemblyData {
    #[serde(rename = "Assembly Method")]
    // Todo: Enum
    pub assembly_method: String,
    #[serde(rename = "Genome Representation")]
    // Todo: Enum
    pub genome_representation: String,
    #[serde(rename = "Expected Final Version")]
    pub expected_final_version: String,
    #[serde(rename = "Genome Coverage")]
    pub genome_coverage: String,
    #[serde(rename = "Sequencing Technology")]
    // Todo: Enum
    pub sequencing_technology: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenomeAnnotationData {
    #[serde(rename = "Annotation Provider")]
    pub annotation_provider: String,
    #[serde(rename = "Annotation Date")]
    pub annotation_date: String,
    #[serde(rename = "Annotation Pipeline")]
    pub annotation_pipeline: String,
    #[serde(rename = "Annotation Method")]
    pub annotation_method: String,
    #[serde(rename = "Annotation Software revision")]
    pub annotation_software_revision: String,
    #[serde(rename = "Features Annotated")]
    pub features_annotated: String,
    #[serde(rename = "Genes (total)")]
    pub genes_total: String,
    #[serde(rename = "CDS (total)")]
    pub cds_total: String,
    #[serde(rename = "Genes (coding)")]
    pub genes_coding: String,
    #[serde(rename = "CDS (coding)")]
    pub cds_coding: String,
    #[serde(rename = "Genes (RNA)")]
    pub genes_rna: String,
    #[serde(rename = "rRNAs")]
    pub r_rnas: String,
    #[serde(rename = "complete rRNAs")]
    pub complete_r_rnas: String,
    #[serde(rename = "tRNAs")]
    pub t_rnas: String,
    #[serde(rename = "ncRNAs")]
    pub nc_rnas: String,
    #[serde(rename = "Pseudo Genes (total)")]
    pub pseudo_genes_total: String,
    #[serde(rename = "Pseudo Genes (ambiguous residues)")]
    pub pseudo_genes_ambiguous_residues: String,
    #[serde(rename = "Pseudo Genes (frameshifted)")]
    pub pseudo_genes_frameshifted: String,
    #[serde(rename = "Pseudo Genes (incomplete)")]
    pub pseudo_genes_incomplete: String,
    #[serde(rename = "Pseudo Genes (internal stop)")]
    pub pseudo_genes_internal_stop: String,
    #[serde(rename = "Pseudo Genes (multiple problems)")]
    pub pseudo_genes_multiple_problems: String,
    #[serde(rename = "CRISPR Arrays")]
    pub crispr_arrays: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetterAnnotations {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Area {
    pub start: i64,
    pub end: i64,
    // Todo: Enum
    pub products: Vec<String>,
    pub protoclusters: Option<HashMap<String, Protocluster>>,
    pub candidates: Vec<Candidate>,
    pub subregions: Vec<String>, // <--- Note this example does not have a subregion.
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Protocluster {
    pub category: String,
    pub start: i64,
    pub end: i64,
    pub core_start: i64,
    pub core_end: i64,
    pub product: String,
    //// Todo: Enum
    pub tool: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub start: i64,
    pub end: i64,
    pub kind: String,
    pub protoclusters: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Modules {
    #[serde(rename = "antismash.detection.hmm_detection")]
    pub antismash_detection_hmm_detection: HmmDetection,
    #[serde(rename = "antismash.detection.cluster_hmmer")]
    pub antismash_detection_cluster_hmmer: ClusterHmm,
    #[serde(rename = "antismash.detection.genefunctions")]
    pub antismash_detection_genefunctions: GeneFunction,
    #[serde(rename = "antismash.detection.nrps_pks_domains")]
    pub antismash_detection_nrps_pks_domains: NrpsPksDomains,
    #[serde(rename = "antismash.detection.tigrfam")]
    pub antismash_detection_tigrfam: Tigrfam,
    #[serde(rename = "antismash.modules.active_site_finder")]
    pub antismash_modules_active_site_finder: ActiveSiteFinder,
    #[serde(rename = "antismash.modules.cluster_compare")]
    pub antismash_modules_cluster_compare: ClusterCompare,
    #[serde(rename = "antismash.modules.clusterblast")]
    pub antismash_modules_clusterblast: Clusterblast,
    #[serde(rename = "antismash.modules.lanthipeptides")]
    pub antismash_modules_lanthipeptides: Lanthipeptides,
    #[serde(rename = "antismash.modules.lassopeptides")]
    pub antismash_modules_lassopeptides: Lassopeptides,
    #[serde(rename = "antismash.modules.nrps_pks")]
    pub antismash_modules_nrps_pks: NrpsPks,
    #[serde(rename = "antismash.modules.pfam2go")]
    pub antismash_modules_pfam2go: Pfam2go,
    #[serde(rename = "antismash.modules.rrefinder")]
    pub antismash_modules_rrefinder: Rrefinder,
    #[serde(rename = "antismash.modules.sactipeptides")]
    pub antismash_modules_sactipeptides: Sactipeptides,
    #[serde(rename = "antismash.modules.t2pks")]
    pub antismash_modules_t2pks: T2pks,
    #[serde(rename = "antismash.modules.tfbs_finder")]
    pub antismash_modules_tfbs_finder: TfbsFinder,
    #[serde(rename = "antismash.modules.thiopeptides")]
    pub antismash_modules_thiopeptides: Thiopeptides,
    #[serde(rename = "antismash.modules.tta")]
    pub antismash_modules_tta: Tta,
}
