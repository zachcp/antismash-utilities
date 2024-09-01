use antismash_rs::antismash_modules::antismash_cluster_compare::ClusterCompare;
use antismash_rs::antismash_modules::antismash_nrps_pks::NrpsPks;

use antismash_rs::AntismashJson;
use serde_json::{from_str, Value};
use std::fs;
use std::path::Path; // Replace 'your_crate_name' with your actual crate name

#[test]
fn test_parse_antismash_json() {
    // Read the JSON file
    let json_path = Path::new("tests/data/genomic.json");
    let json_content = fs::read_to_string(json_path).expect("Failed to read the genomic.json file");

    let simple_content: Value = from_str(&json_content).expect("Failed to parse JSON");
    if let Value::Object(map) = simple_content {
        for key in map.keys() {
            println!("Key: {}", key);
        }
    } else {
        println!("The JSON root is not an object");
    }

    // Parse the JSON content
    let result: Result<AntismashJson, serde_json::Error> = serde_json::from_str(&json_content);

    // Assert that the parsing was successful
    assert!(
        result.is_ok(),
        "Failed to parse the JSON: {:?}",
        result.err()
    );

    let antismash_data = result.unwrap();

    // Add more specific assertions here to validate the parsed data
    // For example:
    assert!(
        !antismash_data.records.is_empty(),
        "Records should not be empty"
    );
}

#[test]
fn test_parse_cluster_compare_json() {
    // Read the JSON file
    let json_path = Path::new("tests/data/cluster_compare.json");
    let json_content = fs::read_to_string(json_path).expect("Failed to read the genomic.json file");

    // Parse the JSON content
    let result: Result<ClusterCompare, serde_json::Error> = serde_json::from_str(&json_content);

    // Assert that the parsing was successful
    assert!(
        result.is_ok(),
        "Failed to parse the JSON: {:?}",
        result.err()
    );

    let antismash_data = result.unwrap();
}
#[test]
fn test_parse_nrps_pks_json() {
    // Read the JSON file
    let json_path = Path::new("tests/data/nrps_pks.json");
    let json_content = fs::read_to_string(json_path).expect("Failed to read the genomic.json file");

    // Parse the JSON content
    let result: Result<NrpsPks, serde_json::Error> = serde_json::from_str(&json_content);

    // Assert that the parsing was successful
    assert!(
        result.is_ok(),
        "Failed to parse the JSON: {:?}",
        result.err()
    );

    let antismash_data = result.unwrap();
}
