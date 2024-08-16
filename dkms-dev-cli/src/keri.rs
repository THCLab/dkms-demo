use anyhow::Result;
use controller::{
    controller::Controller, BasicPrefix, IdentifierPrefix, LocationScheme, Oobi, SelfSigningPrefix,
};
use keri_controller::{
    self as controller,
    identifier::{mechanics::{query_mailbox, MechanicsError}, Identifier},
};
use keri_core::{
    actor::prelude::SelfAddressingIdentifier,
    keys::KeysError,
    prefix::IndexedSignature,
    query::mailbox::SignedMailboxQuery,
    signer::Signer,
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeriError {
    #[error(transparent)]
    ControllerError(#[from] keri_controller::error::ControllerError),
    #[error(transparent)]
    MechanicsError(#[from] MechanicsError),
    #[error(transparent)]
    KeriError(#[from] keri_core::error::Error),
    #[error(transparent)]
    SigningError(#[from] KeysError),
}

pub async fn add_watcher(
    id: &mut Identifier,
    km: Arc<Signer>,
    watcher_oobi: &LocationScheme,
) -> Result<(), KeriError> {
    id.resolve_oobi(&Oobi::Location(watcher_oobi.clone()))
        .await?;
    let rpy = id.add_watcher(watcher_oobi.eid.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(rpy.as_bytes())?,
    );
    id.finalize_add_watcher(rpy.as_bytes(), signature).await?;
    Ok(())
}

// pub async fn add_messagebox(
//     id: &mut Identifier,
//     km: Arc<Signer>,
//     messagebox_oobi: &LocationScheme,
// ) -> Result<(), KeriError> {
//     // id.source.resolve_loc_schema(messagebox_oobi).await?;
//     id.resolve_oobi(&Oobi::Location(messagebox_oobi.clone())).await?;
//     let rpy = id.add_messagebox(messagebox_oobi.eid.clone())?;
//     let signature = SelfSigningPrefix::new(
//         cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
//         km.sign(rpy.as_bytes())?,
//     );
//     id.finalize_event(rpy.as_bytes(), signature).await?;

//     Ok(())
// }

pub async fn setup_identifier(
    cont: Arc<Controller>,
    signer: Arc<Signer>,
    next_pk: BasicPrefix,
    witness: Vec<LocationScheme>,
    witness_threshold: u64,
    messagebox: Option<LocationScheme>,
    watcher: Vec<LocationScheme>,
) -> Result<Identifier, KeriError> {
    let pks = vec![BasicPrefix::Ed25519(signer.public_key())];
    let npks = vec![next_pk];
    let signing_inception = cont.incept(pks, npks, witness.clone(), witness_threshold).await?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signer.sign(signing_inception.as_bytes())?,
    );
    let mut signing_identifier = cont.finalize_incept(signing_inception.as_bytes(), &signature)?;

    // let mut signing_identifier = Identifier::new(signing_identifier.clone(), cont.clone(), None);

    signing_identifier.notify_witnesses().await?;

    for wit in witness {
        // signing_identifier.resolve_oobi(Oobi::Location(oobi));
        if let IdentifierPrefix::Basic(wit_id) = &wit.eid { 
            query_mailbox(&mut signing_identifier, signer.clone(), &wit_id).await?;
        };
        
        // Send witness oobi to watcher.
        signing_identifier
            .send_oobi_to_watcher(&signing_identifier.id(), &Oobi::Location(wit.clone()))
            .await?;

        match &wit.eid {
            IdentifierPrefix::Basic(bp) => {
                let _queries = query_mailbox(&mut signing_identifier, signer.clone(), bp).await?;
            }
            _ => todo!(),
        }
    }

    // if let Some(messagebox_oobi) = messagebox {
    //     add_messagebox(&mut signing_identifier, signer.clone(), &messagebox_oobi).await?;
    // };

    for watch in watcher {
        add_watcher(&mut signing_identifier, signer.clone(), &watch).await?;
    }

    Ok(signing_identifier)
}

pub async fn incept_registry(id: &mut Identifier, signer: Arc<Signer>) -> Result<(), KeriError> {
    // Init tel
    let (reg_id, ixn) = id.incept_registry()?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        signer.sign(&ixn)?,
    );
    id.finalize_anchor(&ixn, signature).await?;

    id.notify_witnesses().await?;

    let witness_id = id.find_state(&id.id())?.witness_config.witnesses[0].clone();
    let _queries = query_mailbox(id, signer.clone(), &witness_id).await?;
    id.notify_backers().await?;

    // id.registry_id = Some(reg_id);

    Ok(())
}

pub async fn query_mailbox(
    id: &mut Identifier,
    km: Arc<Signer>,
    witness_id: &BasicPrefix,
) -> Result<Vec<SignedMailboxQuery>, KeriError> {
    let mut out = vec![];
    for qry in id
        .query_mailbox(&id.id(), &[witness_id.clone()])?
        .into_iter()
    {
        let signature = SelfSigningPrefix::Ed25519Sha512(km.sign(&qry.encode()?)?);
        let signatures = vec![IndexedSignature::new_both_same(signature.clone(), 0)];
        let signed_qry = SignedMailboxQuery::new_trans(qry.clone(), id.id().clone(), signatures);
        id.finalize_query_mailbox(vec![(qry, signature)]).await?;
        out.push(signed_qry)
    }
    Ok(out)
}

// pub async fn _query_messagebox(id: &Identifier, km: Arc<CryptoBox>) -> Result<String, KeriError> {
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
    id: &Identifier,
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

    id.finalize_query_tel(qry, signature).await?;
    Ok(())
}

pub async fn query_kel(
    about_who: &IdentifierPrefix,
    id: &Identifier,
    km: Arc<Signer>,
) -> Result<(), KeriError> {
    for watcher in id.watchers()? {
        let qry = id.query_full_log(about_who, watcher)?;
        let signature = SelfSigningPrefix::Ed25519Sha512(km.sign(&qry.encode()?)?);
        id.finalize_query(vec![(qry, signature)]).await;
    }
    Ok(())
}

pub async fn rotate(
    id: &mut Identifier,
    current_signer: Arc<Signer>,
    new_next_keys: Vec<BasicPrefix>,
    new_next_threshold: u64,
    witness_to_add: Vec<LocationScheme>,
    witness_to_remove: Vec<BasicPrefix>,
    witness_threshold: u64,
) -> Result<(), KeriError> {
    let current_keys_prefix = vec![BasicPrefix::Ed25519NT(current_signer.public_key())];

    // // If new witness is added, send own kel to them
    // let own_kel = id.get_kel().unwrap();// source.storage.get_kel_messages(&id.id())?.unwrap();
    // for witness in &witness_to_add {
    //     let oobi = Oobi::Location(witness.clone());
    //     id.resolve_oobi(&oobi).await?;
    //     for msg in own_kel.iter() {
    //         id.source
    //             .send_message_to(
    //                 &witness.eid,
    //                 keri_core::oobi::Scheme::Http,
    //                 Message::Notice(msg.clone()),
    //             )
    //             .await?;
    //     }
    // }

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
    id.finalize_rotate(rotation.as_bytes(), signature).await?;

    id.notify_witnesses().await?;

    let witnesses = id.find_state(id.id())?.witness_config.witnesses; // state.witness_config.witnesses[0].clone();
    for witness in witnesses {
        let _queries = query_mailbox(id, current_signer.clone(), &witness)
            .await
            .unwrap();
    }

    Ok(())
}

pub async fn issue(
    identifier: &mut Identifier,
    cred_said: SelfAddressingIdentifier,
    km: Arc<Signer>,
) -> Result<(), KeriError> {
    let (vc_id, ixn) = identifier.issue(cred_said.clone())?;
    let signature = SelfSigningPrefix::new(
        cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
        km.sign(&ixn)?,
    );
    assert_eq!(vc_id.to_string(), cred_said.to_string());
    identifier.finalize_anchor(&ixn, signature).await?;

    identifier.notify_witnesses().await?;
    let witnesses = identifier
        .find_state(&identifier.id())?
        .witness_config
        .witnesses;

    let _qry = query_mailbox(identifier, km.clone(), &witnesses[0]).await?;

    identifier.notify_backers().await?;

    Ok(())
}
