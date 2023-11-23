use std::path::PathBuf;

use config_file::FromConfigFile;
use controller::{Oobi, LocationScheme, BasicPrefix, IdentifierPrefix};

use crate::{
    utils::{load_controller, load, load_identifier},
    CliError,
};

pub async fn handle_resolve(alias: &str, path: PathBuf) -> Result<(), CliError> {
    let cont = load_controller(alias).unwrap();
    for oobi in Vec::<Oobi>::from_config_file(path)? {
        cont.resolve_oobi(oobi).await.unwrap();
    }
    Ok(())
}

/// Returns witnesses' identifiers of alias
pub fn witnesses(alias: &str) -> Result<Vec<IdentifierPrefix>, CliError> {
    let id = load(alias).unwrap();
    Ok(id
        .source
        .get_state(&id.id)
        .unwrap()
        .witness_config
        .witnesses
        .into_iter()
        .map(IdentifierPrefix::Basic)
        .collect()
    )
}

/// Returns watchers' identifiers of alias
pub fn watcher(alias: &str) -> Result<Vec<IdentifierPrefix>, CliError> {
    let id = load(alias).unwrap();
    let watchers = id.source.get_watchers(&id.id).unwrap();
    Ok(watchers)
}

/// Returns mesagebox' identifiers of alias
pub fn mesagkesto(alias: &str) -> Result<Vec<IdentifierPrefix>, CliError> {
    let id = load(alias).unwrap();
    let msgbox = id.source.get_messagebox_end_role(&id.id).unwrap()
        .into_iter()
        .map(|b| b.eid)
        .collect();
    Ok(msgbox)
}

pub fn handle_oobi(alias: &str, ids: &[IdentifierPrefix]) -> Result<Vec<LocationScheme>, CliError> {
    let id = load(alias).unwrap();
    Ok(ids
        .into_iter()
        .flat_map(|identifier| {
            id.source
                .get_loc_schemas(&identifier)
                .unwrap()
        })
        .collect())
}
