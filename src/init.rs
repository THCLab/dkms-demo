use std::{fs::File, io::Write, path::PathBuf, sync::Arc};

use config_file::FromConfigFile;
use controller::{
    config::ControllerConfig, identifier_controller::IdentifierController, BasicPrefix,
    CesrPrimitive, Controller, LocationScheme, SeedPrefix,
};
use ed25519_dalek::SigningKey;
use keri::signer::Signer;
use serde::{de, Deserialize};

use crate::{keri::setup_identifier, CliError};

#[derive(Deserialize)]
struct KeysConfig {
    #[serde(deserialize_with = "deserialize_key")]
    pub current: SeedPrefix,
    #[serde(deserialize_with = "deserialize_key")]
    pub next: SeedPrefix,
}

impl Default for KeysConfig {
    fn default() -> Self {
        let current = SigningKey::generate(&mut rand::rngs::OsRng);
        let next = SigningKey::generate(&mut rand::rngs::OsRng);
        Self {
            current: SeedPrefix::RandomSeed256Ed25519(current.as_bytes().to_vec()),
            next: SeedPrefix::RandomSeed256Ed25519(next.as_bytes().to_vec()),
        }
    }
}

fn deserialize_key<'de, D>(deserializer: D) -> Result<SeedPrefix, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    s.parse::<SeedPrefix>()
        .map_err(|e| de::Error::unknown_variant(s, &[]))
}

pub async fn handle_init(alias: String, keys_file: Option<PathBuf>) -> Result<(), CliError> {
    // Compute kel database path
    let mut store_path = PathBuf::from(".");
    store_path.push(&alias);
    let mut db_path = store_path.clone();
    db_path.push("db");

    let keys = match keys_file {
        Some(file_path) => {
            KeysConfig::from_config_file(file_path).map_err(|_e| CliError::SeedsUnparsable)?
        }
        None => KeysConfig::default(),
    };
    let (npk, _nsk) = keys
        .next
        .derive_key_pair()
        .map_err(|e| CliError::KeysDerivationError)?;

    let witness_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC","scheme":"http","url":"http://witness1.sandbox.argo.colossi.network/"}"#).unwrap();

    let id = incept(
        db_path,
        keys.current.clone(),
        controller::BasicPrefix::Ed25519NT(npk),
        Some(witness_oobi),
        alias.clone(),
        None,
        None,
    )
    .await
    .unwrap();

    // Save next keys seed
    let mut nsk_path = store_path.clone();
    nsk_path.push(&alias);
    nsk_path.push("next_priv_key");
    let mut file = File::create(nsk_path).unwrap();
    file.write_all(keys.next.to_str().as_bytes()).unwrap();

    print!("Identifier for alias {} initialized: {}", alias, id.id);

    // Save identifier
    let mut id_path = store_path.clone();
    id_path.push("id");
    let mut file = File::create(id_path)?;
    file.write_all(id.id.to_string().as_bytes())?;

    // Save registry identifier
    let mut reg_path = store_path.clone();
    reg_path.push("reg_id");
    let mut file = File::create(reg_path)?;
    file.write_all(id.registry_id.unwrap().to_string().as_bytes())?;

    // Save private key
    let mut priv_key_path = store_path.clone();
    priv_key_path.push("priv_key");
    let mut file = File::create(priv_key_path)?;
    file.write_all(keys.current.to_str().as_bytes())?;
    Ok(())
}

async fn incept(
    db_path: PathBuf,
    priv_key: SeedPrefix,
    next_key: BasicPrefix,
    witness: Option<LocationScheme>,
    alias: String,
    messagebox: Option<LocationScheme>,
    watcher: Option<LocationScheme>,
) -> anyhow::Result<IdentifierController> {
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

    Ok(id)
}
