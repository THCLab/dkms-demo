use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use controller::{
    config::ControllerConfig, identifier_controller::IdentifierController, BasicPrefix,
    CesrPrimitive, Controller, CryptoBox, IdentifierPrefix, LocationScheme, SeedPrefix,
};
use keri::signer::Signer;

use crate::keri::setup_identifier;

fn load_id(alias: String) -> Result<IdentifierController> {
    let mut store_path = PathBuf::from(".");
    store_path.push(alias);
    let mut id_path = store_path.clone();
    id_path.push("id");
    let mut registry_path = store_path.clone();
    registry_path.push("reg_id");
    let mut db_path = store_path.clone();
    db_path.push("db");

    let identifier: IdentifierPrefix = fs::read_to_string(id_path)
        .expect("Should have been able to read the file")
        .parse()
        .unwrap();
    let registry_id: IdentifierPrefix = fs::read_to_string(registry_path)
        .expect("Should have been able to read the file")
        .parse()
        .unwrap();
    let cont = Arc::new(
        Controller::new(ControllerConfig {
            db_path: db_path.into(),
            ..ControllerConfig::default()
        })
        .unwrap(),
    );
    Ok(IdentifierController::new(
        identifier,
        cont,
        Some(registry_id),
    ))
}

pub async fn incept(
    priv_key: SeedPrefix,
    next_key: BasicPrefix,
    witness: Option<LocationScheme>,
    alias: String,
    messagebox: Option<LocationScheme>,
    watcher: Option<LocationScheme>,
) -> Result<IdentifierPrefix> {
    let mut store_path = PathBuf::from(".");
    store_path.push(alias);
    let mut db_path = store_path.clone();
    db_path.push("db");
    let cont = Arc::new(Controller::new(ControllerConfig {
        db_path: db_path.clone().into(),
        ..ControllerConfig::default()
    })?);
    let signer = Arc::new(Signer::new_with_seed(&priv_key)?);
    let id = setup_identifier(
        cont,
        signer,
        next_key,
        witness.unwrap(),
        messagebox,
        watcher,
    )
    .await?;

    let mut id_path = store_path.clone();
    id_path.push("id");
    let mut file = File::create(id_path)?;
    file.write_all(id.id.to_string().as_bytes())?;

    let mut reg_path = store_path.clone();
    reg_path.push("reg_id");
    let mut file = File::create(reg_path)?;
    file.write_all(id.registry_id.unwrap().to_string().as_bytes())?;

    let mut priv_key_path = store_path.clone();
    priv_key_path.push("priv_key");
    let mut file = File::create(priv_key_path)?;
    file.write_all(priv_key.to_str().as_bytes())?;

    Ok(id.id)
}

// #[tokio::test]
// async fn ttt() {
// 	let witness_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC","scheme":"http","url":"http://witness1.sandbox.argo.colossi.network/"}"#).unwrap();

// 	let id = incept(None, None, Some(witness_oobi), "alice".to_string(), None, None).await.unwrap();

// 	// let loaded = load_id("alice".to_string()).unwrap();
// }

#[tokio::test]
async fn tttt() {
    // let witness_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC","scheme":"http","url":"http://witness1.sandbox.argo.colossi.network/"}"#).unwrap();

    // let id = incept(None, None, Some(witness_oobi), "alice".to_string(), None, None).await.unwrap();

    let loaded = load_id("alice".to_string()).unwrap();
}
