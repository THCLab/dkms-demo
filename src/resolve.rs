use std::fs;
use std::path::PathBuf;

use controller::{identifier_controller::IdentifierController, EndRole, IdentifierPrefix, Oobi};

use crate::{
    utils::{load, load_controller},
    CliError, OobiCommands,
};

pub async fn handle_resolve(alias: &str, path: PathBuf) -> Result<(), CliError> {
    let cont = load_controller(alias).unwrap();
    let file = fs::read_to_string(path).expect("Should have been able to read the file");
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

pub fn handle_oobi(
    alias: &str,
    oobi_command: &Option<OobiCommands>,
) -> Result<Vec<Oobi>, CliError> {
    let identifier = load(alias).unwrap();
    let filter_locations = |identifiers: Vec<IdentifierPrefix>| -> Result<Vec<Oobi>, CliError> {
        Ok(identifiers
            .into_iter()
            .flat_map(|id| identifier.source.get_loc_schemas(&id).unwrap())
            .map(Oobi::Location)
            .collect())
    };

    match oobi_command {
        Some(OobiCommands::Witness) => filter_locations(witnesses(&identifier)?),
        Some(OobiCommands::Watcher) => filter_locations(watcher(&identifier)?),
        Some(OobiCommands::Messagebox) => filter_locations(mesagkesto(&identifier)?),
        None => {
            let witnesses = witnesses(&identifier)?;
            let witnesses_oobi = witnesses
                .clone()
                .iter()
                .map(|cid| {
                    Oobi::EndRole(EndRole {
                        cid: cid.clone(),
                        role: keri::oobi::Role::Witness,
                        eid: identifier.id.clone(),
                    })
                })
                .chain(filter_locations(witnesses)?.into_iter())
                .collect();
            Ok(witnesses_oobi)
        }
    }
}
