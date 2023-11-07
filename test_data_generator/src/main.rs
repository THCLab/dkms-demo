use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use acdc::attributes::InlineAttributes;
use anyhow::Result;
use controller::{
    config::ControllerConfig, identifier_controller::IdentifierController, BasicPrefix, Controller,
    CryptoBox, IdentifierPrefix, KeyManager, LocationScheme, PublicKey, SelfSigningPrefix,
};
use keri::{
    actor::prelude::SelfAddressingIdentifier, prefix::IndexedSignature,
    query::query_event::SignedKelQuery,
};
use tempfile::Builder;

fn main() {}

async fn setup_identifier(
    out_dir: PathBuf,
    cont: Arc<Controller>,
    km: Arc<CryptoBox>,
    witness: LocationScheme,
    messagebox: Option<LocationScheme>,
    watcher: Option<LocationScheme>,
) -> Result<IdentifierController> {
    fs::create_dir_all(&out_dir).unwrap();
    let pks = vec![BasicPrefix::Ed25519(km.public_key())];
    let npks = vec![BasicPrefix::Ed25519(km.next_public_key())];
    let signing_inception = cont.incept(pks, npks, vec![witness.clone()], 1).await?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(&signing_inception.as_bytes())?,
    );
    let signing_identifier = cont
        .finalize_inception(signing_inception.as_bytes(), &signature)
        .await?;

    let mut id = IdentifierController::new(signing_identifier.clone(), cont.clone(), None);

    id.notify_witnesses().await?;

    let witness_id = match &witness.eid {
        controller::IdentifierPrefix::Basic(bp) => bp.clone(),
        _ => todo!(),
    };

    let queries = query_mailbox(&id, km.clone(), &witness_id).await?;
    let mut path = out_dir.clone();
    path.push("mailbox_qry_0");
    let mut file = File::create(path).unwrap();

    for qry in queries {
        file.write_all(&qry.to_cesr()?)?;
    }

    // Init tel
    let (reg_id, ixn) = id.incept_registry()?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(&ixn)?,
    );
    id.finalize_event(&ixn, signature).await?;

    id.notify_witnesses().await?;

    let queries = query_mailbox(&id, km.clone(), &witness_id).await?;
    let mut path = out_dir.clone();
    path.push("mailbox_qry_1");
    let mut file = File::create(path).unwrap();

    for qry in queries {
        file.write_all(&qry.to_cesr()?)?;
    }

    id.registry_id = Some(reg_id);

    if let Some(messagebox_oobi) = messagebox {
        cont.resolve_loc_schema(&messagebox_oobi).await?;
        let rpy = id.add_messagebox(messagebox_oobi.eid)?;
        let signature = SelfSigningPrefix::new(
            cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
            km.sign(rpy.as_bytes())?,
        );
        id.finalize_event(&rpy.as_bytes(), signature).await?;
        // TODO how to print reply
    };

    if let Some(watcher_oobi) = watcher {
        cont.resolve_loc_schema(&watcher_oobi).await?;
        let rpy = id.add_watcher(watcher_oobi.eid)?;
        let signature = SelfSigningPrefix::new(
            cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
            km.sign(rpy.as_bytes())?,
        );
        id.finalize_event(&rpy.as_bytes(), signature).await?;
        // TODO how to print reply
    };
    let mut path = out_dir.clone();
    path.push("kel");
    let mut file = File::create(path)?;
    file.write_all(id.get_kel()?.as_bytes())?;

    Ok(id)
}

async fn query_mailbox(
    id: &IdentifierController,
    km: Arc<CryptoBox>,
    witness_id: &BasicPrefix,
) -> Result<Vec<SignedKelQuery>> {
    let mut out = vec![];
    for (i, qry) in id
        .query_mailbox(&id.id, &[witness_id.clone()])
        .unwrap()
        .into_iter()
        .enumerate()
    {
        let signature = SelfSigningPrefix::Ed25519Sha512(km.sign(&qry.encode().unwrap()).unwrap());
        let signatures = vec![IndexedSignature::new_both_same(signature.clone(), 0)];
        let signed_qry = SignedKelQuery::new_trans(qry.clone(), id.id.clone(), signatures);
        println!(
            "\nSigned mailbox query: {}",
            String::from_utf8(signed_qry.to_cesr()?)?
        );
        id.finalize_query(vec![(qry, signature)]).await.unwrap();
        out.push(signed_qry)
    }
    Ok(out)
}

#[tokio::test]
pub async fn test_generating() -> Result<()> {
    // Create temporary db file.
    let signing_id_path = Builder::new()
        .prefix("test-db")
        .tempdir()
        .unwrap()
        .path()
        .to_path_buf();

    // Create temporary db file.
    let verifying_id_path = Builder::new()
        .prefix("test-db")
        .tempdir()
        .unwrap()
        .path()
        .to_path_buf();

    let signing_controller = Arc::new(Controller::new(ControllerConfig {
        db_path: signing_id_path,
        ..Default::default()
    })?);
    let verifying_controller = Arc::new(Controller::new(ControllerConfig {
        db_path: verifying_id_path,
        ..Default::default()
    })?);
    let witness: LocationScheme = serde_json::from_str(r#"{"eid":"BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC","scheme":"http","url":"http://witness1.sandbox.argo.colossi.network/"}"#).unwrap();
    let witness_id: BasicPrefix = "BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC".parse()?;

    let messagebox_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BFY1nGjV9oApBzo5Oq5JqjwQsZEQqsCCftzo3WJjMMX-","scheme":"http","url":"http://messagebox.sandbox.argo.colossi.network/"}"#).unwrap();
    let messagebox_id = "BFY1nGjV9oApBzo5Oq5JqjwQsZEQqsCCftzo3WJjMMX-";

    let watcher_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BF2t2NPc1bwptY1hYV0YCib1JjQ11k9jtuaZemecPF5b","scheme":"http","url":"http://watcher.sandbox.argo.colossi.network/"}"#).unwrap();

    let signing_key_manager = Arc::new(CryptoBox::new().unwrap());
    let dir_path_str = "./generated/identifier1";
    let out_path = PathBuf::from(dir_path_str);
    let signing_identifier = setup_identifier(
        out_path,
        signing_controller.clone(),
        signing_key_manager.clone(),
        witness.clone(),
        Some(messagebox_oobi),
        None,
    )
    .await?;

    let verifying_key_manager = Arc::new(CryptoBox::new().unwrap());
    let out_path = PathBuf::from("./generated/identifier2");
    let verifying_identifier = setup_identifier(
        out_path,
        verifying_controller,
        verifying_key_manager.clone(),
        witness,
        None,
        Some(watcher_oobi),
    )
    .await?;

    // Issuing ACDC
    let attr: InlineAttributes = r#"{"number":"123456789"}"#.parse()?;
    let registry_id = signing_identifier.registry_id.clone().unwrap().to_string();
    let acdc = acdc::Attestation::new_public_untargeted(
        &signing_identifier.id.to_string(),
        registry_id,
        "schema".to_string(),
        attr,
    );

    let path = "./generated/acdc";
    let mut file = File::create(path)?;
    file.write_all(&said::version::Encode::encode(&acdc)?)?;

    let cred_said: SelfAddressingIdentifier = acdc.clone().digest.unwrap().to_string().parse().unwrap();

    let (vc_id, ixn) = signing_identifier.issue(cred_said.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signing_key_manager.sign(&ixn)?,
    );
    signing_identifier.finalize_event(&ixn, signature).await?;

    let said = match vc_id {
        IdentifierPrefix::SelfAddressing(said) => said,
        _ => {
            unreachable!()
        }
    };
    signing_identifier.notify_witnesses().await?;

    let qry = query_mailbox(
        &signing_identifier,
        signing_key_manager.clone(),
        &witness_id,
    )
    .await?;

    // Save tel to file
    let tel = signing_controller.tel.get_tel(&said)?;
    let encoded = tel
        .iter()
        .map(|tel| tel.serialize().unwrap())
        .flatten()
        .collect::<Vec<_>>();
    let path = "./generated/tel";
    let mut file = File::create(path)?;
    file.write_all(&encoded)?;

    let exn = messagebox::forward_message(verifying_identifier.id.to_string(), String::from_utf8(said::version::Encode::encode(&acdc)?).unwrap());
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signing_key_manager.sign(&exn.to_string().as_bytes())?,
    );
    let signed_exn = signing_identifier.sign_to_cesr(&exn.to_string(), signature, 0)?;

    println!("\nExchange: {}", signed_exn);

    fs::create_dir_all("./generated/messagebox").unwrap();
    let path = "./generated/messagebox/exn";
    let mut file = File::create(path)?;
    file.write_all(&signed_exn.as_bytes())?;

    let qry = messagebox::query_by_sn(verifying_identifier.id.to_string(), 0);
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signing_key_manager.sign(&qry.to_string().as_bytes())?,
    );
    let signed_qry = signing_identifier.sign_to_cesr(&qry.to_string(), signature, 0)?;
    
    println!("\nQuery: {}", signed_qry);

    let path = "./generated/messagebox/qry";
    let mut file = File::create(path)?;
    file.write_all(&signed_qry.as_bytes())?;


    Ok(())
}
