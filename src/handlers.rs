use std::{fs::File, io::Write, path::PathBuf};

use config_file::FromConfigFile;
use controller::{CesrPrimitive, LocationScheme, SeedPrefix};
use ed25519_dalek::SigningKey;
use serde::{de, Deserialize};

use crate::{api, CliError};

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
    let id = api::incept(
        keys.current,
        controller::BasicPrefix::Ed25519NT(npk),
        Some(witness_oobi),
        alias.clone(),
        None,
        None,
    )
    .await
    .unwrap();

    // Save next keys seed
    let store_path = PathBuf::from(".");
    let mut nsk_path = store_path.clone();
    nsk_path.push(&alias);
    nsk_path.push("next_priv_key");
    let mut file = File::create(nsk_path).unwrap();
    file.write_all(keys.next.to_str().as_bytes()).unwrap();

    print!("Identifier for alias {} initialized: {}", alias, id);
    Ok(())
}
