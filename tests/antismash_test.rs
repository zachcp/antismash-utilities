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
    assert!(antismash_data.records.len() > 0, "There should be records");
    antismash_data.print_record_names();
    antismash_data.get_adenylation_domains();
    Ok(())
}

#[test]
fn test_parse_cluster_compare_json() -> Result<()> {
    let json_path = Path::new("tests/data/cluster_compare.json");
    let cluster_data: ClusterCompare = parse_json_file(json_path).map_err(Error::from)?;
    assert_eq!(cluster_data.record_id, "CP027858.1");
    assert_eq!(cluster_data.schema_version, 1);
    assert_eq!(cluster_data.db_results.mibi_g.name, "MIBiG");
    // assert_eq!(
    //     cluster_data
    //         .db_results
    //         .mibi_g
    //         .by_region
    //         .regions
    //         .get("RegionToRegion_RiQ")
    //         .unwrap()
    //         .get("RegionToRegion_RiQRiQ"),
    //     0.739258902308216
    // );
    Ok(())
}

#[test]
fn test_parse_nrps_pks_json() -> Result<()> {
    let json_path = Path::new("tests/data/nrps_pks.json");
    let nrps_pks: NrpsPks = parse_json_file(json_path).map_err(Error::from)?;
    assert_eq!(nrps_pks.schema_version, 3);
    assert_eq!(nrps_pks.record_id, "CP027858.1");
    // assert_eq!(nrps_pks.domain_predictions.get("CP027858.1").unwrap(), 1);
    // assert_eq!(nrps_pks.consensus, "CP027858.1");
    Ok(())
}
