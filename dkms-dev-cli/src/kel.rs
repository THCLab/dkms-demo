use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use crate::{keri::KeriError, temporary_id::{clear_temporary_id, create_temporary_id}, utils::{self, load_homedir}};
use ed25519_dalek::SigningKey;
use figment::{
    providers::{Format, Yaml},
    Figment,
};
use keri_controller::{identifier::query::QueryResponse, BasicPrefix, CesrPrimitive, IdentifierPrefix, LocationScheme, Oobi, SeedPrefix, SelfSigningPrefix};
use keri_core::{actor::prelude::Message, signer::Signer};
use serde::{Deserialize, Serialize};

use crate::{
    keri::{query_kel, rotate},
    utils::{load, load_next_signer, load_signer},
    CliError,
};

#[derive(Debug, Deserialize, Serialize)]
struct RotationConfig {
    witness_to_add: Vec<LocationScheme>,
    witness_to_remove: Vec<BasicPrefix>,
    witness_threshold: u64,
    new_next_seed: Option<SeedPrefix>,
    new_next_threshold: u64,
}

impl Default for RotationConfig {
    fn default() -> Self {
        let current = SigningKey::generate(&mut rand::rngs::OsRng);
        Self {
            witness_to_add: Default::default(),
            witness_to_remove: Default::default(),
            witness_threshold: 1,
            new_next_seed: Some(SeedPrefix::RandomSeed256Ed25519(
                current.as_bytes().to_vec(),
            )),
            new_next_threshold: 1,
        }
    }
}

pub async fn handle_kel_query(
    alias: &str,
    about_who: &IdentifierPrefix,
) -> Result<String, CliError> {
    let id = load(alias)?;
    let signer = Arc::new(load_signer(alias)?);

    query_kel(about_who, &id, signer)
        .await
        .map_err(|e| CliError::NotReady(e.to_string()))?;
    let kel = id
        .get_kel(about_who)
        .ok_or(CliError::UnknownIdentifier(about_who.to_str()))?;
    let kel_str = kel
        .into_iter()
        .flat_map(|kel| Message::Notice(kel).to_cesr().unwrap());
    Ok(String::from_utf8(kel_str.collect()).unwrap())
}

pub async fn handle_rotate(alias: &str, config_path: Option<PathBuf>) -> Result<(), CliError> {
    let rotation_config = match config_path {
        Some(cfgs) => Figment::new()
            .merge(Yaml::file(cfgs.clone()))
            .extract()
            .unwrap_or_else(|_| panic!("Can't read file from path: {:?}", cfgs.to_str())),
        None => RotationConfig::default(),
    };

    let mut id = load(alias)?;
    // Load next keys as current
    let current_signer = Arc::new(load_next_signer(alias)?);

    let new_next_seed = rotation_config.new_next_seed.unwrap_or({
        let current = SigningKey::generate(&mut rand::rngs::OsRng);
        SeedPrefix::RandomSeed256Ed25519(current.as_bytes().to_vec())
    });

    let (npk, _nsk) = new_next_seed
        .derive_key_pair()
        .map_err(|_e| CliError::KeysDerivationError)?;
    let next_bp = BasicPrefix::Ed25519NT(npk);

    // Rotate keys
    rotate(
        &mut id,
        current_signer,
        vec![next_bp],
        rotation_config.new_next_threshold,
        rotation_config.witness_to_add,
        rotation_config.witness_to_remove,
        rotation_config.witness_threshold,
    )
    .await?;

    print!("\nKeys rotated for alias {} ({})", alias, id.id());

    // Save new settings in file
    let mut store_path = load_homedir()?;
    store_path.push(".dkms-dev-cli");
    store_path.push(alias);

    let mut nsk_path = store_path.clone();
    nsk_path.push("next_priv_key");

    let mut priv_key_path = store_path.clone();
    priv_key_path.push("priv_key");

    fs::copy(&nsk_path, priv_key_path)?;

    // Save new next key
    let mut file = File::create(nsk_path)?;
    file.write_all(new_next_seed.to_str().as_bytes())?;

    Ok(())
}

/// Returns KEL of identifier of provided alias that is stored locally.
pub async fn handle_get_alias_kel(
    alias: &str,
) -> Result<Option<String>, CliError> {
    let id = load(alias)?;

    let kel = id
        .get_own_kel()
        .ok_or(CliError::UnknownIdentifier(id.id().to_string()))?;
    let kel_str = kel
        .into_iter()
        .flat_map(|kel| Message::Notice(kel).to_cesr().unwrap());
    Ok(Some(String::from_utf8(kel_str.collect()).unwrap()))
}

/// Queries provided watcher about identifier's KEL, and returns it.
pub async fn handle_get_identifier_kel(
    identifier: &IdentifierPrefix,
    oobi: String,
    watcher_oobi: LocationScheme,
) -> Result<Option<String>, CliError> {

    let watcher_id = watcher_oobi.eid.clone();
    let (id, keys_conf) = create_temporary_id(watcher_oobi).await?;
    let signer =
        Arc::new(Signer::new_with_seed(&keys_conf.current).unwrap());

    for oobi in serde_json::from_str::<Vec<Oobi>>(&oobi).unwrap() {
        id
            .send_oobi_to_watcher(&id.id(), &oobi)
            .await
            .map_err(KeriError::ControllerError)?;
    }

    let qry = id.query_full_log(identifier, watcher_id.clone()).unwrap();
    let signature = SelfSigningPrefix::Ed25519Sha512(signer.sign(&qry.encode().unwrap()).unwrap());
    let (mut qry_reps, _errs) = id.finalize_query(vec![(qry, signature)]).await;
    while let QueryResponse::NoUpdates = qry_reps {
        let qry = id.query_full_log(identifier, watcher_id.clone()).unwrap();
        let signature = SelfSigningPrefix::Ed25519Sha512(signer.sign(&qry.encode().unwrap()).unwrap());
        let (qry_resp, _errs) = id.finalize_query(vec![(qry, signature)]).await;
        qry_reps = qry_resp;
    };

    let kel = id
        .get_kel(identifier)
        .ok_or(CliError::UnknownIdentifier(identifier.to_string()))?;
    let kel_str = kel
        .into_iter()
        .flat_map(|kel| Message::Notice(kel).to_cesr().unwrap());
    clear_temporary_id()?;
    Ok(Some(String::from_utf8(kel_str.collect()).unwrap()))
}

