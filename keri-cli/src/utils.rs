use std::{
    fs::{self, File},
    sync::Arc,
};

use anyhow::Result;
use controller::{
    config::ControllerConfig, identifier_controller::IdentifierController, Controller,
    IdentifierPrefix, SeedPrefix,
};
use keri::signer::Signer;

pub fn load(alias: &str) -> Result<IdentifierController> {
    let mut store_path = home::home_dir().unwrap();
    store_path.push(".keri-cli");
    store_path.push(alias);
    let mut id_path = store_path.clone();
    id_path.push("id");
    let mut registry_path = store_path.clone();
    registry_path.push("reg_id");

    let identifier: IdentifierPrefix = fs::read_to_string(id_path)
        .expect("Should have been able to read the file")
        .parse()
        .unwrap();
    let registry_id: Option<IdentifierPrefix> = fs::read_to_string(registry_path)
        .expect("Should have been able to read the file")
        .parse()
        .ok();
    let cont = Arc::new(load_controller(&alias).unwrap());
    Ok(IdentifierController::new(identifier, cont, registry_id))
}

pub fn load_identifier(alias: &str) -> Result<IdentifierPrefix> {
    let mut store_path = home::home_dir().unwrap();
    store_path.push(".keri-cli");
    store_path.push(alias);
    let mut id_path = store_path.clone();
    id_path.push("id");

    let identifier: IdentifierPrefix = fs::read_to_string(id_path)
        .expect("Should have been able to read the file")
        .parse()
        .unwrap();
    Ok(identifier)
}

pub fn load_controller(alias: &str) -> Result<Controller> {
    let mut db_path = home::home_dir().unwrap();
    db_path.push(".keri-cli");
    db_path.push(alias);
    db_path.push("db");

    let cont = Controller::new(ControllerConfig {
        db_path,
        ..ControllerConfig::default()
    })
    .unwrap();
    Ok(cont)
}

pub fn load_signer(alias: &str) -> Result<Signer> {
    let mut path = home::home_dir().unwrap();
    path.push(".keri-cli");
    path.push(alias);
    path.push("priv_key");
    let sk_str = fs::read_to_string(path).expect("Should have been able to read the file");
    let seed: SeedPrefix = sk_str.parse().unwrap();
    let signer = Signer::new_with_seed(&seed).unwrap();

    Ok(signer)
}
