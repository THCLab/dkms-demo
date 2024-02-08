use anyhow::Result;
use controller::{
    identifier_controller::IdentifierController, BasicPrefix, Controller, CryptoBox,
    IdentifierPrefix, KeyManager, LocationScheme, Oobi, SelfSigningPrefix,
};
use keri_controller as controller;
use keri_core::{
    actor::prelude::SelfAddressingIdentifier, prefix::IndexedSignature,
    query::query_event::SignedKelQuery, signer::Signer,
};
use std::sync::Arc;

pub async fn add_watcher(
    id: &IdentifierController,
    km: Arc<Signer>,
    watcher_oobi: &LocationScheme,
) -> Result<()> {
    id.source.resolve_loc_schema(watcher_oobi).await?;
    let rpy = id.add_watcher(watcher_oobi.eid.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(rpy.as_bytes())?,
    );
    id.finalize_event(rpy.as_bytes(), signature).await?;
    Ok(())
}

pub async fn add_messagebox(
    id: &IdentifierController,
    km: Arc<Signer>,
    messagebox_oobi: &LocationScheme,
) -> Result<()> {
    id.source.resolve_loc_schema(messagebox_oobi).await?;
    let rpy = id.add_messagebox(messagebox_oobi.eid.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(rpy.as_bytes())?,
    );
    id.finalize_event(rpy.as_bytes(), signature).await?;

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
        signer.sign(signing_inception.as_bytes())?,
    );
    let signing_identifier = cont
        .finalize_inception(signing_inception.as_bytes(), &signature)
        .await?;

    let id = IdentifierController::new(signing_identifier.clone(), cont.clone(), None);

    id.notify_witnesses().await?;

    let witness_id = match &witness.eid {
        controller::IdentifierPrefix::Basic(bp) => bp.clone(),
        _ => todo!(),
    };

    let _queries = query_mailbox(&id, signer.clone(), &witness_id).await?;

    if let Some(messagebox_oobi) = messagebox {
        add_messagebox(&id, signer.clone(), &messagebox_oobi).await?;
    };

    if let Some(watcher_oobi) = watcher {
        add_watcher(&id, signer, &watcher_oobi).await?;
    };

    // Send witness oobi to watcher.
    id.source
        .send_oobi_to_watcher(&id.id, &Oobi::Location(witness))
        .await
        .unwrap();

    Ok(id)
}

pub async fn incept_registry(id: &mut IdentifierController, signer: Arc<Signer>) -> Result<()> {
    // Init tel
    let (reg_id, ixn) = id.incept_registry()?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signer.sign(&ixn)?,
    );
    id.finalize_event(&ixn, signature).await?;

    id.notify_witnesses().await?;

    let witness_id = id.source.get_state(&id.id)?.witness_config.witnesses[0].clone();
    let _queries = query_mailbox(id, signer.clone(), &witness_id).await?;
    id.notify_backers().await?;

    id.registry_id = Some(reg_id);

    Ok(())
}

pub async fn query_mailbox(
    id: &IdentifierController,
    km: Arc<Signer>,
    witness_id: &BasicPrefix,
) -> Result<Vec<SignedKelQuery>> {
    let mut out = vec![];
    for qry in id
        .query_mailbox(&id.id, &[witness_id.clone()])
        .unwrap()
        .into_iter()
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

pub async fn _query_messagebox(id: &IdentifierController, km: Arc<CryptoBox>) -> Result<String> {
    let qry = messagebox::query_by_sn(id.id.to_string(), 0);
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(qry.to_string().as_bytes())?,
    );

    let signed_qry = id.sign_to_cesr(&qry.to_string(), signature, 0)?;
    let messagebox = id.source.get_messagebox_location(&id.id).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post(messagebox.first().unwrap().url.clone())
        .body(signed_qry)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn query_tel(
    acdc_d: &SelfAddressingIdentifier,
    registry_id: SelfAddressingIdentifier,
    issuer_id: &IdentifierPrefix,
    id: &IdentifierController,
    km: Arc<Signer>,
) -> Result<()> {
    let qry = id.query_tel(
        IdentifierPrefix::SelfAddressing(registry_id),
        IdentifierPrefix::SelfAddressing(acdc_d.clone()),
    )?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(qry.encode().unwrap())?,
    );

    id.finalize_tel_query(issuer_id, qry, signature)
        .await
        .unwrap();
    Ok(())
}

pub async fn query_kel(
    about_who: &IdentifierPrefix,
    id: &IdentifierController,
    km: Arc<Signer>,
) -> Result<()> {
    for qry in id.query_own_watchers(about_who)? {
        let signature = SelfSigningPrefix::Ed25519Sha512(km.sign(&qry.encode()?)?);
        id.finalize_query(vec![(qry, signature)]).await?;
    }
    Ok(())
}

pub async fn rotate(
    id: &IdentifierController,
    current_signer: Arc<Signer>,
    new_next_keys: Vec<BasicPrefix>,
    new_next_threshold: u64,
    witness_to_add: Vec<LocationScheme>,
    witness_to_remove: Vec<BasicPrefix>,
    witness_threshold: u64,
) -> Result<()> {
    let current_keys_prefix = vec![BasicPrefix::Ed25519NT(current_signer.public_key())];
    let rotation = id
        .rotate(
            current_keys_prefix,
            new_next_keys,
            new_next_threshold,
            witness_to_add,
            witness_to_remove,
            witness_threshold,
        )
        .await?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        current_signer.sign(rotation.as_bytes())?,
    );
    id.finalize_event(rotation.as_bytes(), signature).await?;

    id.notify_witnesses().await?;

    let state = id.source.get_state(&id.id).unwrap();
    // TODO
    let witnesses = state.witness_config.witnesses[0].clone();

    let _queries = query_mailbox(&id, current_signer.clone(), &witnesses).await?;

    Ok(())
}

pub async fn issue(
    identifier: &IdentifierController,
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

    identifier.notify_witnesses().await?;
    let witnesses = identifier
        .source
        .get_state(&identifier.id)
        .unwrap()
        .witness_config
        .witnesses;

    let _qry = query_mailbox(identifier, km.clone(), &witnesses[0]).await?;

    identifier.notify_backers().await?;

    Ok(())
}
