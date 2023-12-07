use std::{fs::File, io::Write, sync::Arc};

use keri::actor::prelude::SelfAddressingIdentifier;
use serde_json::Value;

use crate::{
    keri::issue,
    utils::{load, load_signer},
    CliError,
};

pub async fn handle_tel_incept(alias: &str) -> Result<(), CliError> {
    let mut id = load(alias).unwrap();
    let signer = Arc::new(load_signer(alias).unwrap());
    crate::keri::incept_registry(&mut id, signer).await.unwrap();

    // Save registry identifier
    let mut store_path = home::home_dir().unwrap();
    store_path.push(".keri-cli");
    store_path.push(&alias);

    let mut reg_path = store_path.clone();
    reg_path.push("reg_id");
    let mut file = File::create(reg_path)?;
    file.write_all(id.registry_id.as_ref().unwrap().to_string().as_bytes())?;

    Ok(())
}

pub async fn handle_issue(alias: &str, data: &str) -> Result<(), CliError> {
    let id = load(alias).unwrap();
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
