use std::{fs::{self, File}, io::Write, sync::Arc};

use ed25519_dalek::SigningKey;
use keri_controller::{BasicPrefix, CesrPrimitive, IdentifierPrefix, SeedPrefix};

use crate::{keri::{query_kel, rotate}, utils::{load, load_next_signer, load_signer}, CliError};

pub async fn handle_kel_query(alias: &str, about_who: &IdentifierPrefix) -> Result<(), CliError> {
    let id = load(alias).unwrap();
    let signer = Arc::new(load_signer(alias).unwrap());

	query_kel(about_who, &id, signer).await.unwrap();
	let kel = id.source.storage.get_kel(&about_who).unwrap().map(|v| String::from_utf8(v).unwrap());
	dbg!(kel);

    Ok(())
}

pub async fn handle_rotate(alias: &str) -> Result<(), CliError> {
    let id = load(alias).unwrap();
    // Load next keys as current
    let current_signer = Arc::new(load_next_signer(alias).unwrap());

	// Generate new next keys
	let new_next = SigningKey::generate(&mut rand::rngs::OsRng);
    let new_seed = SeedPrefix::RandomSeed256Ed25519(new_next.as_bytes().to_vec());
	let (npk, _nsk) = new_seed.derive_key_pair().unwrap();
	let next_bp = BasicPrefix::Ed25519NT(npk) ;
	
    // Rotate keys
	rotate(&id, current_signer, vec![next_bp]).await.unwrap();
	   
    print!("\nKeys rotated for alias {} ({})", alias, id.id);
	
    // Save new settings in file 
	let mut store_path = home::home_dir().unwrap();
    store_path.push(".keri-cli");
    store_path.push(&alias);

	let mut nsk_path = store_path.clone();
    nsk_path.push("next_priv_key");
   
    let mut priv_key_path = store_path.clone();
    priv_key_path.push("priv_key");

    fs::copy(&nsk_path, priv_key_path).unwrap();

    // Save new next key
    let mut file = File::create(nsk_path).unwrap();
    file.write_all(new_seed.to_str().as_bytes()).unwrap();

    Ok(())
}

pub async fn handle_get_kel(alias: &str, about_who: &IdentifierPrefix) -> Result<Option<String>, CliError> {
    let id = load(alias).unwrap();

	Ok(id.source.storage.get_kel(&about_who).unwrap().map(|v| String::from_utf8(v).unwrap()))
}