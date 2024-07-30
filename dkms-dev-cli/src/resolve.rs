use std::fs;
use std::path::PathBuf;

use keri_controller::{
    identifier::Identifier, EndRole, IdentifierPrefix, Oobi,
};

use crate::{keri::KeriError, utils::load, CliError, OobiRoles};

pub async fn handle_resolve(alias: &str, path: PathBuf) -> Result<(), CliError> {
    let id_cont = load(alias)?;
    let file = fs::read_to_string(path).expect("Should have been able to read the file");
    for oobi in serde_json::from_str::<Vec<Oobi>>(&file).unwrap() {
        let _ = id_cont
            .resolve_oobi(&oobi)
            .await
            .map_err(KeriError::MechanicsError);
        id_cont
            .send_oobi_to_watcher(&id_cont.id(), &oobi)
            .await
            .map_err(KeriError::ControllerError)?;
    }
    Ok(())
}

/// Returns witnesses' identifiers of alias
pub fn witnesses(identifier: &Identifier) -> Result<Vec<IdentifierPrefix>, CliError> {
    Ok(identifier
        .find_state(&identifier.id())
        .map_err(KeriError::MechanicsError)?
        .witness_config
        .witnesses
        .into_iter()
        .map(IdentifierPrefix::Basic)
        .collect())
}

/// Returns watchers' identifiers of alias
pub fn watcher(identifier: &Identifier) -> Result<Vec<IdentifierPrefix>, CliError> {
    let watchers = identifier.watchers()
        .map_err(KeriError::ControllerError)?;
    Ok(watchers)
}

/// Returns mesagebox' identifiers of alias
pub fn mesagkesto(identifeir: &Identifier) -> Result<Vec<IdentifierPrefix>, CliError> {
    let msgbox = identifeir
        // .source
        .get_end_role(&identifeir.id(), keri_core::oobi::Role::Messagebox)
        .map_err(KeriError::ControllerError)?
        .into_iter()
        .map(|b| b.eid)
        .collect();
    Ok(msgbox)
}

pub fn handle_oobi(alias: &str, oobi_command: &Option<OobiRoles>) -> Result<Vec<Oobi>, CliError> {
    let identifier = load(alias)?;
    let filter_locations = |identifiers: Vec<IdentifierPrefix>| -> Result<Vec<Oobi>, CliError> {
        Ok(identifiers
            .into_iter()
            .flat_map(|id| identifier.get_location(&id).unwrap())
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
                    role: keri_core::oobi::Role::Witness,
                    cid: identifier.id().clone(),
                })
            });
            Ok(locations.into_iter().chain(witnesses_oobi).collect())
        }
    }
}
