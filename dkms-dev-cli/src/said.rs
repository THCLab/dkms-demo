use std::{fs, path::PathBuf};

use indexmap::IndexMap;
use said::{
    derivation::{HashFunction, HashFunctionCode},
    sad::DerivationCode,
};
use thiserror::Error;

use crate::CliError;

#[derive(Error, Debug)]
pub enum SaidError {
    #[error("Missing `d` field in provided json")]
    MissingSaidField,
    #[error("Serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub async fn handle_sad(path: PathBuf) -> Result<String, CliError> {
    let file = fs::read_to_string(path).expect("Should have been able to read the file");
    let inserted = insert_said(&file)?;

    Ok(inserted)
}

fn insert_said(input: &str) -> Result<String, SaidError> {
    let mut map: IndexMap<String, serde_json::Value> = serde_json::from_str(input)?;
    if let Some(_dig) = map.get("d") {
        let code = HashFunctionCode::Blake3_256;
        map["d"] = serde_json::Value::String("#".repeat(code.full_size()));
        let said = HashFunction::from(code).derive(&serde_json::to_vec(&map)?);
        map["d"] = serde_json::Value::String(said.to_string());
        Ok(serde_json::to_string(&map)?)
    } else {
        Err(SaidError::MissingSaidField)
    }
}

#[test]
fn test_json_to_sad() {
    let data = r#"{"hello":"world","d":""}"#;
    let said_inserted = insert_said(data);

    let to_compute = format!(r#"{{"hello":"world","d":"{}"}}"#, "#".repeat(44));
    let expected_said =
        HashFunction::from(HashFunctionCode::Blake3_256).derive(to_compute.as_bytes());

    let json: serde_json::Value = serde_json::from_str(&said_inserted.unwrap()).unwrap();
    if let Some(serde_json::Value::String(dig)) = json.get("d") {
        assert_eq!(dig, &expected_said.to_string());
    };
}
