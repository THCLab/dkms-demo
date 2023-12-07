use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use config_file::ConfigFileError;
use init::handle_init;
use mesagkesto::MesagkestoError;
use resolve::handle_resolve;
use tel::handle_issue;
use thiserror::Error;

mod init;
mod keri;
mod mesagkesto;
mod resolve;
mod tel;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Init new signer
    Init {
        #[arg(short, long)]
        alias: String,
        #[arg(short, long)]
        keys_file: Option<PathBuf>,
    },
    Tel {
        #[command(subcommand)]
        command: TelCommands,
    },
    Mesagkesto {
        #[command(subcommand)]
        command: MesagkestoCommands,
    },
    /// Returns saved oobis of provided `alias`
    Oobi {
        #[command(subcommand)]
        command: OobiCommands,
    },
}

#[derive(Subcommand)]
pub enum MesagkestoCommands {
    Exchange {
        #[arg(short, long)]
        alias: String,
        #[arg(short, long)]
        content: String,
        #[arg(short, long)]
        receiver: String,
    },
    Query {
        #[arg(short, long)]
        alias: String,
    },
}

#[derive(Subcommand)]
pub enum TelCommands {
    Incept {
        #[arg(short, long)]
        alias: String,
    },
    Issue {
        #[arg(short, long)]
        alias: String,
        #[arg(short, long)]
        credential_json: String,
    },
}

#[derive(Subcommand)]
pub enum OobiCommands {
    Get {
        #[arg(short, long)]
        alias: String,
        #[command(subcommand)]
        role: Option<OobiRoles>,
    },
    // Resolves provided oobi and saves it
    Resolve {
        #[arg(short, long)]
        alias: String,
        #[arg(short, long)]
        file: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum OobiRoles {
    Witness,
    Watcher,
    Messagebox,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    SeedsUnparsable(#[from] ConfigFileError),
    #[error("Keys derivation error")]
    KeysDerivationError,
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error("Missing digest field")]
    MissingDigest,
    #[error(transparent)]
    MesagkestoError(#[from] MesagkestoError),
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Init { alias, keys_file }) => {
            handle_init(alias, keys_file).await?;
        }
        Some(Commands::Mesagkesto { command }) => match command {
            MesagkestoCommands::Exchange {
                content,
                receiver,
                alias,
            } => {
                println!(
                    "{}",
                    mesagkesto::handle_exchange(&alias, &content, &receiver)?
                );
            }
            MesagkestoCommands::Query { alias } => {
                let qry = mesagkesto::handle_pull(&alias)?;
                println!("{}", qry);
            }
        },
        Some(Commands::Oobi { command }) => match command {
            OobiCommands::Get { role, alias } => {
                let lcs = resolve::handle_oobi(&alias, &role)?;
                println!("{}", serde_json::to_string(&lcs).unwrap());
            }
            OobiCommands::Resolve { alias, file } => handle_resolve(&alias, file).await?,
        },
        Some(Commands::Tel { command }) => match command {
            TelCommands::Incept { alias } => {
                tel::handle_tel_incept(&alias).await?;
            }
            TelCommands::Issue {
                alias,
                credential_json,
            } => {
                handle_issue(&alias, &credential_json).await?;
            }
        },
        None => {}
    }
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use std::{path::PathBuf, sync::Arc};

//     use acdc::attributes::InlineAttributes;
//     use anyhow::Result;
//     use controller::{
//         config::ControllerConfig, BasicPrefix, Controller, CryptoBox, EndRole, IdentifierPrefix,
//         KeyManager, LocationScheme, SelfSigningPrefix,
//     };
//     use keri::actor::prelude::SelfAddressingIdentifier;
//     use tempfile::Builder;

//     use crate::keri::{query_mailbox, setup_identifier};

//     #[tokio::test]
//     pub async fn test_generating() -> Result<()> {
//         // Create temporary db file.
//         let signing_id_path = Builder::new()
//             .prefix("test-db")
//             .tempdir()
//             .unwrap()
//             .path()
//             .to_path_buf();

//         // Create temporary db file.
//         let verifying_id_path = Builder::new()
//             .prefix("test-db")
//             .tempdir()
//             .unwrap()
//             .path()
//             .to_path_buf();

//         let signing_controller = Arc::new(Controller::new(ControllerConfig {
//             db_path: signing_id_path,
//             ..Default::default()
//         })?);
//         let verifying_controller = Arc::new(Controller::new(ControllerConfig {
//             db_path: verifying_id_path,
//             ..Default::default()
//         })?);
//         let witness_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC","scheme":"http","url":"http://witness1.sandbox.argo.colossi.network/"}"#).unwrap();
//         let witness_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC","scheme":"http","url":"http://localhost:3232/"}"#).unwrap();
//         let witness_id: BasicPrefix = "BJq7UABlttINuWJh1Xl2lkqZG4NTdUdqnbFJDa6ZyxCC".parse()?;

//         let messagebox_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BFY1nGjV9oApBzo5Oq5JqjwQsZEQqsCCftzo3WJjMMX-","scheme":"http","url":"http://messagebox.sandbox.argo.colossi.network/"}"#).unwrap();
//         let messagebox_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BFY1nGjV9oApBzo5Oq5JqjwQsZEQqsCCftzo3WJjMMX-","scheme":"http","url":"http://localhost:8080/"}"#).unwrap();
//         let messagebox_id = "BFY1nGjV9oApBzo5Oq5JqjwQsZEQqsCCftzo3WJjMMX-";

//         let watcher_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BF2t2NPc1bwptY1hYV0YCib1JjQ11k9jtuaZemecPF5b","scheme":"http","url":"http://watcher.sandbox.argo.colossi.network/"}"#).unwrap();
//         let watcher_oobi: LocationScheme = serde_json::from_str(r#"{"eid":"BF2t2NPc1bwptY1hYV0YCib1JjQ11k9jtuaZemecPF5b","scheme":"http","url":"http://localhost:3235/"}"#).unwrap();

//         let signing_key_manager = Arc::new(CryptoBox::new().unwrap());
//         let dir_path_str = "./generated/identifier1";
//         let out_path = PathBuf::from(dir_path_str);
//         let signing_identifier = setup_identifier(
//             signing_controller.clone(),
//             signing_key_manager.clone(),
//             witness_oobi.clone(),
//             Some(messagebox_oobi),
//             None,
//         )
//         .await?;

//         let verifying_key_manager = Arc::new(CryptoBox::new().unwrap());
//         let out_path2 = PathBuf::from("./generated/identifier2");
//         let verifying_identifier = setup_identifier(
//             verifying_controller,
//             verifying_key_manager.clone(),
//             witness_oobi.clone(),
//             None,
//             Some(watcher_oobi),
//         )
//         .await?;

//         // Issuing ACDC
//         let attr: InlineAttributes = r#"{"number":"123456789"}"#.parse()?;
//         let registry_id = signing_identifier.registry_id.clone().unwrap().to_string();
//         let acdc = acdc::Attestation::new_public_untargeted(
//             &signing_identifier.id.to_string(),
//             registry_id,
//             "schema".to_string(),
//             attr,
//         );

//         // let path = "./generated/acdc";
//         // let mut file = File::create(path)?;
//         // file.write_all(&said::version::Encode::encode(&acdc)?)?;

//         let cred_said: SelfAddressingIdentifier =
//             acdc.clone().digest.unwrap().to_string().parse().unwrap();

//         let (vc_id, ixn) = signing_identifier.issue(cred_said.clone())?;
//         let signature = SelfSigningPrefix::new(
//             cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
//             signing_key_manager.sign(&ixn)?,
//         );
//         assert_eq!(vc_id.to_string(), cred_said.to_string());
//         signing_identifier.finalize_event(&ixn, signature).await?;

//         let said = match vc_id {
//             IdentifierPrefix::SelfAddressing(said) => said,
//             _ => {
//                 unreachable!()
//             }
//         };
//         signing_identifier.notify_witnesses().await?;

//         let qry = query_mailbox(
//             &signing_identifier,
//             signing_key_manager.clone(),
//             &witness_id,
//         )
//         .await?;

//         let mut path = out_path;
//         // path.push("kel");
//         // let mut file = File::create(path)?;
//         // file.write_all(signing_identifier.get_kel()?.as_bytes())?;
//         signing_identifier.notify_backers().await?;

//         println!("\nkel: {:?}", signing_identifier.get_kel());

//         // Save tel to file
//         let tel = signing_controller.tel.get_tel(&said)?;
//         let encoded = tel
//             .iter()
//             .map(|tel| tel.serialize().unwrap())
//             .flatten()
//             .collect::<Vec<_>>();
//         // let path = "./generated/tel";
//         // let mut file = File::create(path)?;
//         // file.write_all(&encoded)?;

//         // fs::create_dir_all("./generated/messagebox").unwrap();
//         // Signer's oobi
//         let end_role_oobi = EndRole {
//             eid: IdentifierPrefix::Basic(witness_id.clone()),
//             cid: signing_identifier.id.clone(),
//             role: keri::oobi::Role::Witness,
//         };
//         let oobi0 = serde_json::to_string(&witness_oobi).unwrap();
//         let oobi1 = serde_json::to_string(&end_role_oobi).unwrap();
//         // let path = "./generated/identifier1/oobi0";
//         // let mut file = File::create(path)?;
//         // file.write_all(&oobi0.as_bytes())?;

//         // let path = "./generated/identifier1/oobi1";
//         // let mut file = File::create(path)?;
//         // file.write_all(&oobi1.as_bytes())?;

//         let exn = messagebox::forward_message(
//             verifying_identifier.id.to_string(),
//             String::from_utf8(said::version::Encode::encode(&acdc)?).unwrap(),
//         );
//         let signature = SelfSigningPrefix::new(
//             cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
//             signing_key_manager.sign(&exn.to_string().as_bytes())?,
//         );

//         let signed_exn = signing_identifier.sign_to_cesr(&exn.to_string(), signature, 0)?;

//         println!("\nExchange: {}", signed_exn);

//         // let path = "./generated/messagebox/exn";
//         // let mut file = File::create(path)?;
//         // file.write_all(&signed_exn.as_bytes())?;

//         // Verifier's oobi
//         let end_role_oobi = EndRole {
//             eid: IdentifierPrefix::Basic(witness_id),
//             cid: verifying_identifier.id.clone(),
//             role: keri::oobi::Role::Witness,
//         };
//         let oobi00 = serde_json::to_string(&witness_oobi).unwrap();
//         let oobi11 = serde_json::to_string(&end_role_oobi).unwrap();
//         // let path = "./generated/identifier2/oobi0";
//         // let mut file = File::create(path)?;
//         // file.write_all(&oobi00.as_bytes())?;

//         // let path = "./generated/identifier2/oobi1";
//         // let mut file = File::create(path)?;
//         // file.write_all(&oobi11.as_bytes())?;

//         let qry = messagebox::query_by_sn(verifying_identifier.id.to_string(), 0);
//         let signature = SelfSigningPrefix::new(
//             cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
//             verifying_key_manager.sign(&qry.to_string().as_bytes())?,
//         );
//         let signed_qry = verifying_identifier.sign_to_cesr(&qry.to_string(), signature, 0)?;

//         println!("\nQuery: {}", signed_qry);

//         // let path = "./generated/messagebox/qry";
//         // let mut file = File::create(path)?;
//         // file.write_all(&signed_qry.as_bytes())?;

//         let acdc_d = acdc.digest.clone().unwrap().to_string().parse().unwrap();
//         let acdc_sai: SelfAddressingIdentifier = acdc.digest.unwrap().to_string().parse().unwrap();
//         let acdc_ri: IdentifierPrefix = acdc.registry_identifier.parse().unwrap();
//         let qry = verifying_identifier.query_tel(acdc_ri, acdc_d)?;
//         let signature = SelfSigningPrefix::new(
//             cesrox::primitives::codes::self_signing::SelfSigning::Ed25519Sha512,
//             verifying_key_manager.sign(&qry.encode().unwrap())?,
//         );
//         let signed_qry = verifying_identifier.sign_to_cesr(
//             &String::from_utf8(qry.encode().unwrap()).unwrap(),
//             signature.clone(),
//             0,
//         )?;
//         let path = "./generated/messagebox/tel_qry";
//         // let mut file = File::create(path)?;
//         // file.write_all(&signed_qry.as_bytes())?;

//         // verifying_identifier.source.resolve_oobi(serde_json::from_str(&oobi0).unwrap()).await?;
//         verifying_identifier
//             .source
//             .resolve_oobi(serde_json::from_str(&oobi1).unwrap())
//             .await?;
//         verifying_identifier
//             .finalize_tel_query(&signing_identifier.id, qry, signature)
//             .await?;

//         let tel = verifying_identifier.source.tel.get_tel(&cred_said);
//         let state = verifying_identifier.source.tel.get_vc_state(&cred_said);
//         println!("state: {:?}", state);

//         Ok(())
//     }
// }
