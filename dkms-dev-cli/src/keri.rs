use anyhow::Result;
use controller::{
    identifier_controller::IdentifierController, BasicPrefix, Controller, IdentifierPrefix,
    LocationScheme, Oobi, SelfSigningPrefix,
};
use keri_controller as controller;
use keri_core::{
    actor::prelude::SelfAddressingIdentifier, event_message::signed_event_message::Message,
    keys::KeysError, prefix::IndexedSignature, query::query_event::SignedKelQuery, signer::Signer,
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeriError {
    #[error(transparent)]
    ControllerError(#[from] keri_controller::error::ControllerError),
    #[error(transparent)]
    KeriError(#[from] keri_core::error::Error),
    #[error(transparent)]
    SigningError(#[from] KeysError),
}

pub async fn add_watcher(
    id: &mut IdentifierController,
    km: Arc<Signer>,
    watcher_oobi: &LocationScheme,
) -> Result<(), KeriError> {
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
    id: &mut IdentifierController,
    km: Arc<Signer>,
    messagebox_oobi: &LocationScheme,
) -> Result<(), KeriError> {
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
    witness: Vec<LocationScheme>,
    messagebox: Option<LocationScheme>,
    watcher: Vec<LocationScheme>,
) -> Result<IdentifierController, KeriError> {
    let pks = vec![BasicPrefix::Ed25519(signer.public_key())];
    let npks = vec![next_pk];
    let signing_inception = cont.incept(pks, npks, witness.clone(), 1).await?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signer.sign(signing_inception.as_bytes())?,
    );
    let signing_identifier = cont
        .finalize_inception(signing_inception.as_bytes(), &signature)
        .await?;

    let mut id = IdentifierController::new(signing_identifier.clone(), cont.clone(), None);

    id.notify_witnesses().await?;

    for wit in witness {
        // Send witness oobi to watcher.
        id.source
            .send_oobi_to_watcher(&id.id, &Oobi::Location(wit.clone()))
            .await?;

        match &wit.eid {
            IdentifierPrefix::Basic(bp) => {
                let _queries = query_mailbox(&id, signer.clone(), bp).await?;
            }
            _ => todo!(),
        }
    }

    if let Some(messagebox_oobi) = messagebox {
        add_messagebox(&mut id, signer.clone(), &messagebox_oobi).await?;
    };

    for watch in watcher {
        add_watcher(&mut id, signer.clone(), &watch).await?;
    }

    Ok(id)
}

pub async fn incept_registry(
    id: &mut IdentifierController,
    signer: Arc<Signer>,
) -> Result<(), KeriError> {
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
) -> Result<Vec<SignedKelQuery>, KeriError> {
    let mut out = vec![];
    for qry in id.query_mailbox(&id.id, &[witness_id.clone()])?.into_iter() {
        let signature = SelfSigningPrefix::Ed25519Sha512(km.sign(&qry.encode()?)?);
        let signatures = vec![IndexedSignature::new_both_same(signature.clone(), 0)];
        let signed_qry = SignedKelQuery::new_trans(qry.clone(), id.id.clone(), signatures);
        // println!(
        //     "\nSigned mailbox query: {}",
        //     String::from_utf8(signed_qry.to_cesr()?)?
        // );
        id.finalize_query(vec![(qry, signature)]).await?;
        out.push(signed_qry)
    }
    Ok(out)
}

// pub async fn _query_messagebox(id: &IdentifierController, km: Arc<CryptoBox>) -> Result<String, KeriError> {
//     let qry = messagebox::query_by_sn(id.id.to_string(), 0);
//     let signature = SelfSigningPrefix::new(
//         cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
//         km.sign(qry.to_string().as_bytes())?,
//     );

//     let signed_qry = id.sign_to_cesr(&qry.to_string(), signature, 0)?;
//     let messagebox = id.source.get_messagebox_location(&id.id)?;

//     let client = reqwest::Client::new();
//     let res = client
//         .post(messagebox.first().unwrap().url.clone())
//         .body(signed_qry)
//         .send()
//         .await.unwrap()
//         .text()
//         .await.unwrap();

//     Ok(res)
// }

pub async fn query_tel(
    acdc_d: &SelfAddressingIdentifier,
    registry_id: SelfAddressingIdentifier,
    issuer_id: &IdentifierPrefix,
    id: &IdentifierController,
    km: Arc<Signer>,
) -> Result<(), KeriError> {
    let qry = id.query_tel(
        IdentifierPrefix::SelfAddressing(registry_id),
        IdentifierPrefix::SelfAddressing(acdc_d.clone()),
    )?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(qry.encode()?)?,
    );

    id.finalize_tel_query(issuer_id, qry, signature).await?;
    Ok(())
}

pub async fn query_kel(
    about_who: &IdentifierPrefix,
    id: &IdentifierController,
    km: Arc<Signer>,
) -> Result<(), KeriError> {
    for qry in id.query_own_watchers(about_who)? {
        let signature = SelfSigningPrefix::Ed25519Sha512(km.sign(&qry.encode()?)?);
        id.finalize_query(vec![(qry, signature)]).await?;
    }
    Ok(())
}

pub async fn rotate(
    id: &mut IdentifierController,
    current_signer: Arc<Signer>,
    new_next_keys: Vec<BasicPrefix>,
    new_next_threshold: u64,
    witness_to_add: Vec<LocationScheme>,
    witness_to_remove: Vec<BasicPrefix>,
    witness_threshold: u64,
) -> Result<(), KeriError> {
    let current_keys_prefix = vec![BasicPrefix::Ed25519NT(current_signer.public_key())];

    // If new witness is added, send own kel to them
    let own_kel = id.source.storage.get_kel_messages(&id.id)?.unwrap();
    for witness in &witness_to_add {
        id.source.resolve_loc_schema(witness).await?;
        for msg in own_kel.iter() {
            id.source
                .send_message_to(
                    &witness.eid,
                    keri_core::oobi::Scheme::Http,
                    Message::Notice(msg.clone()),
                )
                .await?;
        }
    }

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

    let witnesses = id.state.witness_config.witnesses[0].clone();

    let _queries = query_mailbox(id, current_signer.clone(), &witnesses)
        .await
        .unwrap();

    Ok(())
}

pub async fn issue(
    identifier: &mut IdentifierController,
    cred_said: SelfAddressingIdentifier,
    km: Arc<Signer>,
) -> Result<(), KeriError> {
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
        .get_state(&identifier.id)?
        .witness_config
        .witnesses;

    let _qry = query_mailbox(identifier, km.clone(), &witnesses[0]).await?;

    identifier.notify_backers().await?;

    Ok(())
}
