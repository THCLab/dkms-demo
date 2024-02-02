use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use config_file::FromConfigFile;
use ed25519_dalek::SigningKey;
use figment::{
    providers::{Format, Yaml},
    Figment,
};
use keri_controller::{
    config::ControllerConfig, identifier_controller::IdentifierController, BasicPrefix,
    CesrPrimitive, Controller, LocationScheme, SeedPrefix,
};
use keri_core::signer::Signer;
use serde::{de, Deserialize, Serialize};

use crate::{keri::setup_identifier, CliError};

#[derive(Deserialize, Serialize, Debug)]
struct KelConfig {
    pub witness: Option<LocationScheme>,
    pub watcher: Option<LocationScheme>,
}

impl Default for KelConfig {
    fn default() -> Self {
        let witness_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC","scheme":"http","url":"http://localhost:3232/"}"#).unwrap();

        Self {
            witness: Some(witness_oobi),
            watcher: None,
        }
    }
}

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
        .map_err(|_e| de::Error::unknown_variant(s, &[]))
}

pub async fn handle_init(
    alias: String,
    keys_file: Option<PathBuf>,
    config_file: Option<PathBuf>,
) -> Result<(), CliError> {
    // Compute kel database path
    let mut store_path = home::home_dir().unwrap();
    store_path.push(".keri-cli");
    store_path.push(&alias);
    fs::create_dir_all(&store_path).unwrap();
    let mut db_path = store_path.clone();
    db_path.push("db");

    let keys = match keys_file {
        Some(file_path) => KeysConfig::from_config_file(file_path)?,
        None => KeysConfig::default(),
    };

    let kel_config = match config_file {
        Some(cfgs) => Figment::new()
            .merge(Yaml::file(cfgs.clone()))
            .extract()
            .unwrap_or_else(|_| panic!("Can't read file from path: {:?}", cfgs.to_str())),
        None => KelConfig::default(),
    };

    let (npk, _nsk) = keys
        .next
        .derive_key_pair()
        .map_err(|_e| CliError::KeysDerivationError)?;

    let id = incept(
        db_path,
        keys.current.clone(),
        keri_controller::BasicPrefix::Ed25519NT(npk),
        kel_config.witness,
        None,
        kel_config.watcher,
    )
    .await
    .unwrap();

    // Save next keys seed
    let mut nsk_path = store_path.clone();
    nsk_path.push("next_priv_key");
    let mut file = File::create(nsk_path).unwrap();
    file.write_all(keys.next.to_str().as_bytes()).unwrap();

    print!("\nIdentifier for alias {} initialized: {}", alias, id.id);

    // Save identifier
    let mut id_path = store_path.clone();
    id_path.push("id");
    let mut file = File::create(id_path)?;
    file.write_all(id.id.to_string().as_bytes())?;

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
    messagebox: Option<LocationScheme>,
    watcher: Option<LocationScheme>,
) -> anyhow::Result<IdentifierController> {
    let cont = Arc::new(Controller::new(ControllerConfig {
        db_path,
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
