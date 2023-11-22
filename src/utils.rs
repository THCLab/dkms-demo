use std::{
    fs::{self},
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use controller::{
    config::ControllerConfig, identifier_controller::IdentifierController, Controller,
    IdentifierPrefix,
};

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
