use crate::{
    utils::{load, load_signer},
    CliError,
};

pub fn handle_sign(alias: String, data: &str) -> Result<String, CliError> {
    let cont = load(&alias).unwrap();
    let sk = load_signer(&alias).unwrap();

    let signature = keri_controller::SelfSigningPrefix::Ed25519Sha512(sk.sign(data).unwrap());
    Ok(cont.sign_to_cesr(data, signature, 0).unwrap())
}
