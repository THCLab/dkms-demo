use std::sync::Arc;

use keri::actor::prelude::SelfAddressingIdentifier;
use serde_json::Value;

use crate::{
    keri::issue,
    utils::{load_id, load_signer},
    CliError,
};

pub async fn handle_issue(alias: &str, data: &str) -> Result<(), CliError> {
    let id = load_id(alias).unwrap();
    let root: Value = serde_json::from_str(data).unwrap();
    let digest: &str = root
        .get("d")
        .and_then(|v| v.as_str())
        .ok_or(CliError::MissingDigest)?;
    let said: SelfAddressingIdentifier = digest.parse().unwrap();

    let signer = Arc::new(load_signer(alias).unwrap());
    issue(id, said, signer).await.unwrap();

    Ok(())
}
