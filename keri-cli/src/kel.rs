use std::sync::Arc;

use keri_controller::IdentifierPrefix;

use crate::{keri::query_kel, utils::{load, load_signer}, CliError};

pub async fn handle_kel_query(alias: &str, about_who: &IdentifierPrefix) -> Result<(), CliError> {
    let id = load(alias).unwrap();
    let signer = Arc::new(load_signer(alias).unwrap());

	query_kel(about_who, &id, signer).await.unwrap();
	let kel = id.source.storage.get_kel(&about_who).unwrap().map(|v| String::from_utf8(v).unwrap());
	dbg!(kel);

    Ok(())
}