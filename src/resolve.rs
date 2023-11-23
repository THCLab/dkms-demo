use std::path::PathBuf;

use config_file::FromConfigFile;
use controller::Oobi;

use crate::{
    utils::{load_controller, load},
    CliError,
};

pub async fn handle_resolve(alias: &str, path: PathBuf) -> Result<(), CliError> {
    let cont = load_controller(alias).unwrap();
    for oobi in Vec::<Oobi>::from_config_file(path)? {
        cont.resolve_oobi(oobi).await.unwrap();
    }
    Ok(())
}

/// Returns urls of witness of alias
pub async fn handle_witness(alias: &str) -> Result<Vec<url::Url>, CliError> {
    let id = load(alias).unwrap();
    let witnesses = id
        .source
        .get_state(&id.id)
        .unwrap()
        .witness_config
        .witnesses;
    Ok(witnesses
        .into_iter()
        .flat_map(|wit| {
            id.source
                .get_loc_schemas(&controller::IdentifierPrefix::Basic(wit))
                .unwrap()
        })
        .map(|loc| loc.url)
        .collect())
}
