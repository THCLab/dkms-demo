use thiserror::Error;

use crate::{
    utils::{load, load_identifier, load_signer},
    CliError,
};

#[derive(Error, Debug)]
pub enum MesagkestoError {
    #[error("Signing error")]
    SigningError,
}

/// Generates exchange message with provided data. Message is signed and ready
/// to be sent to mesagkesto.
pub fn handle_exchange(alias: &str, data: &str, receiver_alias: &str) -> Result<String, CliError> {
    let receiver = load_identifier(receiver_alias).unwrap();
    let exn = messagebox::forward_message(receiver.to_string(), data.to_string());

    let signer_id = load(alias).unwrap();
    let signer = load_signer(alias).unwrap();

    let signature = keri_controller::SelfSigningPrefix::Ed25519Sha512(
        signer
            .sign(exn.to_string().as_bytes())
            .map_err(|_e| MesagkestoError::SigningError)?,
    );
    Ok(signer_id
        .sign_to_cesr(&exn.to_string(), signature, 0)
        .map_err(|_e| MesagkestoError::SigningError)?)
}

/// Generates query message of identifier's mesagkesto. Message is signed and
/// ready to be sent to mesagkesto.
pub fn handle_pull(alias: &str) -> Result<String, CliError> {
    let signer_id = load(alias).unwrap();
    let signer = load_signer(alias).unwrap();

    let qry = messagebox::query_by_sn(signer_id.id.to_string(), 0);

    let signature = keri_controller::SelfSigningPrefix::Ed25519Sha512(
        signer
            .sign(qry.to_string().as_bytes())
            .map_err(|_e| MesagkestoError::SigningError)?,
    );
    Ok(signer_id
        .sign_to_cesr(&qry.to_string(), signature, 0)
        .map_err(|_e| MesagkestoError::SigningError)?)
}
