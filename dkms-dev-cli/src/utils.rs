use std::{fs, path::PathBuf, sync::Arc};

use anyhow::Result;
use keri_controller::{
    config::ControllerConfig, identifier_controller::IdentifierController, Controller,
    IdentifierPrefix, SeedPrefix,
};
use keri_core::signer::Signer;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadingError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error("Path error: {0}")]
    PathError(String),
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("Controller error: {0}")]
    ControllerError(keri_controller::error::ControllerError),
    #[error("Signer error: {0}")]
    SignerError(String),
}

pub fn load(alias: &str) -> Result<IdentifierController, LoadingError> {
    let mut store_path = load_homedir()?;
    store_path.push(".keri-cli");
    store_path.push(alias);
    let mut id_path = store_path.clone();
    id_path.push("id");
    let mut registry_path = store_path.clone();
    registry_path.push("reg_id");

    let identifier: IdentifierPrefix = fs::read_to_string(id_path.clone())
        .map_err(|_e| {
            LoadingError::PathError("Should have been able to read the file".to_string())
        })?
        .parse()
        .map_err(|_e| {
            LoadingError::ParsingError(format!(
                "Can't parse identifier from file: {}",
                id_path.to_str().unwrap()
            ))
        })?;
    let registry_id = match fs::read_to_string(registry_path.clone()) {
        Ok(reg) => reg.parse().ok(),
        Err(_) => None,
    };

    let cont = Arc::new(load_controller(alias)?);
    Ok(IdentifierController::new(identifier, cont, registry_id))
}

pub fn load_identifier(alias: &str) -> Result<IdentifierPrefix, LoadingError> {
    let mut store_path = load_homedir()?;
    store_path.push(".keri-cli");
    store_path.push(alias);
    let mut id_path = store_path.clone();
    id_path.push("id");

    let identifier: IdentifierPrefix = fs::read_to_string(id_path.clone())
        .map_err(|_e| {
            LoadingError::PathError("Should have been able to read the file".to_string())
        })?
        .trim()
        .parse()
        .map_err(|_e| {
            LoadingError::ParsingError(format!(
                "Can't parse identifier from file: {}",
                id_path.to_str().unwrap()
            ))
        })?;
    Ok(identifier)
}

pub fn load_controller(alias: &str) -> Result<Controller, LoadingError> {
    let mut db_path = load_homedir()?;
    db_path.push(".keri-cli");
    db_path.push(alias);
    db_path.push("db");

    let cont = Controller::new(ControllerConfig {
        db_path,
        ..ControllerConfig::default()
    })
    .map_err(LoadingError::ControllerError)?;
    Ok(cont)
}

pub fn load_signer(alias: &str) -> Result<Signer, LoadingError> {
    let mut path = load_homedir()?;
    path.push(".keri-cli");
    path.push(alias);
    path.push("priv_key");
    let sk_str = fs::read_to_string(path)?;
    let seed: SeedPrefix = sk_str
        .parse()
        .map_err(|_e| LoadingError::SignerError("Seed parsing error".to_string()))?;
    let signer =
        Signer::new_with_seed(&seed).map_err(|e| LoadingError::SignerError(e.to_string()))?;

    Ok(signer)
}

pub fn load_next_signer(alias: &str) -> Result<Signer, LoadingError> {
    let mut path = load_homedir()?;
    path.push(".keri-cli");
    path.push(alias);
    path.push("next_priv_key");
    let sk_str = fs::read_to_string(path)?;
    let seed: SeedPrefix = sk_str
        .parse()
        .map_err(|_e| LoadingError::SignerError("Seed parsing error".to_string()))?;
    let signer =
        Signer::new_with_seed(&seed).map_err(|e| LoadingError::SignerError(e.to_string()))?;

    Ok(signer)
}

pub fn handle_info(alias: &str) -> Result<(), LoadingError> {
    let cont = load(alias)?;
    let info = if let Some(reg) = cont.registry_id {
        json!({"id": cont.id, "registry": reg})
    } else {
        json!({"id": cont.id})
    };
    println!("{}", serde_json::to_string(&info).unwrap());

    Ok(())
}

pub fn load_homedir() -> Result<PathBuf, LoadingError> {
    home::home_dir().ok_or(LoadingError::PathError("Can't load home dir".to_string()))
}
