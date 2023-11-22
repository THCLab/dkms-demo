use std::path::PathBuf;

use config_file::FromConfigFile;
use controller::{Oobi, Controller, config::ControllerConfig};

use crate::CliError;

pub async fn handle_resolve(alias: &str, path: PathBuf) -> Result<(), CliError> {
	let mut db_path = PathBuf::from(".");
	db_path.push(alias);
	db_path.push("db");
	let cont = Controller::new(ControllerConfig { db_path, .. ControllerConfig::default()}).unwrap();
    for oobi in Vec::<Oobi>::from_config_file(path)? {
		cont.resolve_oobi(oobi).await.unwrap();
	};
	Ok(())
}