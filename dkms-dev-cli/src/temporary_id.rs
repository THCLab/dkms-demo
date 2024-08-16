use std::{fs, io};

use keri_controller::{identifier::Identifier, LocationScheme};

use crate::{init::{handle_new_id, KelConfig, KeysConfig}, utils, CliError};

pub(crate) async fn create_temporary_id(watcher_oobi: LocationScheme) -> Result<(Identifier, KeysConfig), CliError> {
    let mut store_path = utils::load_homedir()?;
    store_path.push(".dkms-dev-cli");
    store_path.push("default");
    fs::create_dir_all(&store_path)?;
    let mut db_path = store_path.clone();
    db_path.push("db");
    
    let keys = KeysConfig::default();
    
    let kel_conf = KelConfig { witness: None, watcher: Some(vec![watcher_oobi]), witness_threshold:0 };
    
    handle_new_id(&keys, kel_conf, &db_path).await.map(|id| (id, keys))
}

pub(crate) fn clear_temporary_id() -> Result<(), CliError> {
    let mut store_path = utils::load_homedir().unwrap();
    store_path.push(".dkms-dev-cli");
    store_path.push("default");
    Ok(fs::remove_dir_all(store_path)?)
}
