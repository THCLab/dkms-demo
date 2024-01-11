use std::fs;
use std::path::PathBuf;

use controller::{identifier_controller::IdentifierController, EndRole, IdentifierPrefix, Oobi};

use crate::{
    utils::{load, load_controller},
    CliError, OobiRoles,
};

pub async fn handle_resolve(alias: &str, path: PathBuf) -> Result<(), CliError> {
    let cont = load_controller(alias).unwrap();
    let file = fs::read_to_string(path).expect(&format!("Should have been able to read the file"));
    for oobi in serde_json::from_str::<Vec<Oobi>>(&file).unwrap() {
        cont.resolve_oobi(oobi).await.unwrap();
    }
    Ok(())
}

/// Returns witnesses' identifiers of alias
pub fn witnesses(identifier: &IdentifierController) -> Result<Vec<IdentifierPrefix>, CliError> {
    Ok(identifier
        .source
        .get_state(&identifier.id)
        .unwrap()
        .witness_config
        .witnesses
        .into_iter()
        .map(IdentifierPrefix::Basic)
        .collect())
}

/// Returns watchers' identifiers of alias
pub fn watcher(identifier: &IdentifierController) -> Result<Vec<IdentifierPrefix>, CliError> {
    let watchers = identifier.source.get_watchers(&identifier.id).unwrap();
    Ok(watchers)
}

/// Returns mesagebox' identifiers of alias
pub fn mesagkesto(identifeir: &IdentifierController) -> Result<Vec<IdentifierPrefix>, CliError> {
    let msgbox = identifeir
        .source
        .get_messagebox_end_role(&identifeir.id)
        .unwrap()
        .into_iter()
        .map(|b| b.eid)
        .collect();
    Ok(msgbox)
}

pub fn handle_oobi(alias: &str, oobi_command: &Option<OobiRoles>) -> Result<Vec<Oobi>, CliError> {
    let identifier = load(alias).unwrap();
    let filter_locations = |identifiers: Vec<IdentifierPrefix>| -> Result<Vec<Oobi>, CliError> {
        Ok(identifiers
            .into_iter()
            .flat_map(|id| identifier.source.get_loc_schemas(&id).unwrap())
            .map(Oobi::Location)
            .collect())
    };

    match oobi_command {
        Some(OobiRoles::Witness) => filter_locations(witnesses(&identifier)?),
        Some(OobiRoles::Watcher) => filter_locations(watcher(&identifier)?),
        Some(OobiRoles::Messagebox) => filter_locations(mesagkesto(&identifier)?),
        None => {
            let witnesses = witnesses(&identifier)?;
            let locations = filter_locations(witnesses.clone())?;
            let witnesses_oobi = witnesses.clone().into_iter().map(|cid| {
                Oobi::EndRole(EndRole {
                    eid: cid.clone(),
                    role: keri::oobi::Role::Witness,
                    cid: identifier.id.clone(),
                })
            });
            Ok(locations.into_iter().chain(witnesses_oobi).collect())
        }
    }
}
