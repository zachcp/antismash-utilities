use antismash_rs::antismash_modules::antismash_cluster_compare::ClusterCompare;
use antismash_rs::antismash_modules::antismash_nrps_pks::NrpsPks;
use antismash_rs::AntismashJson;
use anyhow::{Error, Result};
use serde_json::{from_str, Value};
use std::fs;
use std::path::Path;

fn parse_json_file<T: serde::de::DeserializeOwned>(file_path: &Path) -> Result<T> {
    let json_content = fs::read_to_string(file_path)?;
    let parsed = serde_json::from_str(&json_content)?;
    Ok(parsed)
}

#[test]
fn test_parse_antismash_json() -> Result<()> {
    let json_path = Path::new("tests/data/genomic.json");
    let antismash_data: AntismashJson = parse_json_file(json_path).map_err(Error::from)?;
    assert!(
        !antismash_data.records.is_empty(),
        "Records should not be empty"
    );
    antismash_data.print_record_names();
    antismash_data.get_adenylation_domains();
    Ok(())
}

#[test]
fn test_parse_cluster_compare_json() -> Result<()> {
    // Read the JSON file
    let json_path = Path::new("tests/data/cluster_compare.json");
    let antismash_data: ClusterCompare = parse_json_file(json_path).map_err(Error::from)?;
    Ok(())
}

#[test]
fn test_parse_nrps_pks_json() -> Result<()> {
    // Read the JSON file
    let json_path = Path::new("tests/data/nrps_pks.json");
    let antismash_data: NrpsPks = parse_json_file(json_path).map_err(Error::from)?;
    Ok(())
}
