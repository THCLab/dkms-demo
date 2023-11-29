use std::{fs, path::PathBuf, sync::Arc};

use anyhow::Result;
use controller::{
    identifier_controller::IdentifierController, BasicPrefix, Controller, CryptoBox,
    IdentifierPrefix, KeyManager, LocationScheme, Oobi, SelfSigningPrefix,
};
use keri::{
    actor::prelude::SelfAddressingIdentifier, prefix::IndexedSignature,
    query::query_event::SignedKelQuery, signer::Signer,
};

pub async fn add_watcher(
    id: &IdentifierController,
    km: Arc<Signer>,
    watcher_oobi: &LocationScheme,
) -> Result<()> {
    id.source.resolve_loc_schema(&watcher_oobi).await?;
    let rpy = id.add_watcher(watcher_oobi.eid.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(rpy.as_bytes())?,
    );
    id.finalize_event(&rpy.as_bytes(), signature).await?;
    Ok(())
}

pub async fn add_messagebox(
    id: &IdentifierController,
    km: Arc<Signer>,
    messagebox_oobi: &LocationScheme,
) -> Result<()> {
    id.source.resolve_loc_schema(&messagebox_oobi).await?;
    let rpy = id.add_messagebox(messagebox_oobi.eid.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(rpy.as_bytes())?,
    );
    id.finalize_event(&rpy.as_bytes(), signature).await?;

    Ok(())
}

pub async fn setup_identifier(
    cont: Arc<Controller>,
    signer: Arc<Signer>,
    next_pk: BasicPrefix,
    witness: LocationScheme,
    messagebox: Option<LocationScheme>,
    watcher: Option<LocationScheme>,
) -> Result<IdentifierController> {
    let pks = vec![BasicPrefix::Ed25519(signer.public_key())];
    let npks = vec![next_pk];
    let signing_inception = cont.incept(pks, npks, vec![witness.clone()], 1).await?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signer.sign(&signing_inception.as_bytes())?,
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

    let _queries = query_mailbox(&id, signer.clone(), &witness_id).await?;

    // Init tel
    let (reg_id, ixn) = id.incept_registry()?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signer.sign(&ixn)?,
    );
    id.finalize_event(&ixn, signature).await?;

    id.notify_witnesses().await?;

    let _queries = query_mailbox(&id, signer.clone(), &witness_id).await?;
    id.notify_backers().await?;

    id.registry_id = Some(reg_id);

    if let Some(messagebox_oobi) = messagebox {
        add_messagebox(&id, signer.clone(), &messagebox_oobi).await?;
    };

    if let Some(watcher_oobi) = watcher {
        add_watcher(&id, signer, &watcher_oobi).await?;
    };

    Ok(id)
}

pub async fn query_mailbox(
    id: &IdentifierController,
    km: Arc<Signer>,
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
        // println!(
        //     "\nSigned mailbox query: {}",
        //     String::from_utf8(signed_qry.to_cesr()?)?
        // );
        id.finalize_query(vec![(qry, signature)]).await.unwrap();
        out.push(signed_qry)
    }
    Ok(out)
}

pub async fn query_messagebox(id: &IdentifierController, km: Arc<CryptoBox>) -> Result<String> {
    let qry = messagebox::query_by_sn(id.id.to_string(), 0);
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(&qry.to_string().as_bytes())?,
    );

    let signed_qry = id.sign_to_cesr(&qry.to_string(), signature, 0)?;
    let messagebox = id.source.get_messagebox_location(&id.id).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post(messagebox.get(0).unwrap().url.clone())
        .body(signed_qry)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn query_tel(
    acdc_d: SelfAddressingIdentifier,
    registry_id: SelfAddressingIdentifier,
    id: &IdentifierController,
    km: Arc<CryptoBox>,
    signer_oobi: Oobi,
) -> Result<()> {
    let qry = id.query_tel(
        IdentifierPrefix::SelfAddressing(registry_id),
        IdentifierPrefix::SelfAddressing(acdc_d),
    )?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(&qry.encode().unwrap())?,
    );

    id.source.resolve_oobi(signer_oobi).await?;
    id.finalize_tel_query(&id.id, qry, signature).await?;
    Ok(())
}

pub async fn issue(
    identifier: IdentifierController,
    cred_said: SelfAddressingIdentifier,
    km: Arc<Signer>,
) -> Result<()> {
    let (vc_id, ixn) = identifier.issue(cred_said.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(&ixn)?,
    );
    assert_eq!(vc_id.to_string(), cred_said.to_string());
    identifier.finalize_event(&ixn, signature).await?;

    let said = match vc_id {
        IdentifierPrefix::SelfAddressing(said) => said,
        _ => {
            unreachable!()
        }
    };
    identifier.notify_witnesses().await?;
    let witnesses = identifier
        .source
        .get_state(&identifier.id)
        .unwrap()
        .witness_config
        .witnesses;

    let qry = query_mailbox(&identifier, km.clone(), &witnesses[0]).await?;

    identifier.notify_backers().await?;

    // println!("\nkel: {:?}", identifier.get_kel());

    // // Save tel to file
    // let tel = identifier.source.tel.get_tel(&said)?;
    // let encoded = tel
    //     .iter()
    //     .map(|tel| tel.serialize().unwrap())
    //     .flatten()
    //     .collect::<Vec<_>>();

    Ok(())
}
