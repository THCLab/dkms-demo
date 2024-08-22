use std::path::PathBuf;

use base64::{prelude::BASE64_STANDARD, Engine};
use cesrox::primitives::codes::seed::SeedCode;
use clap::{Parser, Subcommand};
use config_file::ConfigFileError;
use init::handle_init;
use kel::{handle_get_alias_kel, handle_get_identifier_kel, handle_kel_query, handle_rotate};
use keri::KeriError;
use keri_controller::{IdentifierPrefix, LocationScheme};
use mesagkesto::MesagkestoError;
use resolve::handle_resolve;
use said::SaidError;
use seed::{convert_to_seed, generate_seed};
use sign::handle_sign;
use tel::{handle_issue, handle_query, handle_tel_oobi};
use thiserror::Error;
use utils::{handle_info, LoadingError};

use crate::said::handle_sad;

mod init;
mod kel;
mod keri;
mod mesagkesto;
mod resolve;
mod said;
mod sign;
mod tel;
mod utils;
mod temporary_id;
mod seed;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

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
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    Kel {
        #[command(subcommand)]
        command: KelCommands,
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
    Said {
        #[command(subcommand)]
        command: SaidCommands,
    },
    Info {
        #[arg(short, long)]
        alias: String,
    },
    Sign {
        #[arg(short, long)]
        alias: String,
        #[arg(short, long)]
        data: String,
    },
    /// Generate Seed string from provided code and secret key encoded in base64. 
    /// Code specify algorithm to use. Possible values are: 
    ///     `A` for Ed25519 private key, 
    ///     `J` for ECDSA secp256k1 private key, 
    ///     'K' for Ed448 private key. 
    /// If no arguments it generates Ed25519 secret key.
    #[clap(verbatim_doc_comment)]
    Seed {
        #[arg(short, long)]
        code: Option<String>,
        #[arg(short, long)]
        secret_key: Option<String>,
    }
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
    Query {
        #[arg(short, long)]
        alias: String,
        #[arg(short, long)]
        issuer_id: String,
        #[arg(short, long)]
        registry_id: String,
        #[arg(short, long)]
        said: String,
    },
    Oobi {
        #[arg(short, long)]
        alias: String,
    },
}

#[derive(Subcommand)]
pub enum KelCommands {
    Rotate {
        #[arg(short, long)]
        alias: String,
        #[arg(short = 'c', long)]
        rotation_config: Option<PathBuf>,
    },
    Get {
        #[clap(flatten)]
        group: KelGettingGroup, 
        /// Identifier OOBI
        #[clap(short, long)]
        oobi: Option<String>,
        /// Watcher OOBI
        #[clap(short, long)]
        watcher_oobi: Option<String>,
    },
    Query {
        #[arg(short, long)]
        alias: String,
        #[arg(short, long)]
        identifier: String,
    },
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct KelGettingGroup {
    #[clap(short, long)]
    alias: Option<String>,
    #[clap(short, long)]
    identifier: Option<String>,
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

#[derive(Subcommand)]
pub enum SaidCommands {
    SAD {
        #[arg(short, long)]
        file: PathBuf,
    },
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    ConfigUnparsable(#[from] ConfigFileError),
    #[error("Keys derivation error")]
    KeysDerivationError,
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error("Path error: {0}")]
    PathError(String),
    #[error("Missing digest field")]
    MissingDigest,
    #[error(transparent)]
    MesagkestoError(#[from] MesagkestoError),
    #[error("Said error: {0}")]
    SaidError(#[from] SaidError),
    #[error("Error: {0}")]
    NotReady(String),
    #[error("Unknown identifier: {0}")]
    UnknownIdentifier(String),
    #[error(transparent)]
    KeriError(#[from] KeriError),
    #[error(transparent)]
    LoadingError(#[from] LoadingError),
    #[error("Unparsable identifier: {0}")]
    UnparsableIdentifier(String),
    #[error("Wrong secret key provided")]
    SecretKeyError,
    #[error("Invalid base64: {0}")]
    B64Error(String),
    #[error("Invalid seed code: {0}")]
    SeedError(String),
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    match cli.command {
        Some(Commands::Init {
            alias,
            keys_file,
            config,
        }) => {
            handle_init(alias, keys_file, config).await?;
        }
        Some(Commands::Kel { command }) => match command {
            KelCommands::Query { alias, identifier } => {
                let identifier: IdentifierPrefix = identifier.parse().unwrap();
                match handle_kel_query(&alias, &identifier).await {
                    Ok(kel) => {
                        println!("{}", kel);
                    }
                    Err(_e) => println!("Kel not ready yet"),
                }
            }
            KelCommands::Rotate {
                alias,
                rotation_config,
            } => {
                handle_rotate(&alias, rotation_config).await.unwrap();
            }
            KelCommands::Get {group,oobi, watcher_oobi } => {
                match (group.alias, group.identifier) {
                    (None, Some(id_str)) => {
                        let id: IdentifierPrefix = id_str.parse().map_err(|e| CliError::UnparsableIdentifier(id_str.to_string()))?;

                        match (oobi, watcher_oobi) {
                            (None, None) => println!("Missing OOBI of identifier that need to be find and watcher OOBI"),
                            (None, Some(_)) => println!("Missing OOBI of identifier that need to be find"),
                            (Some(_), None) => println!("Missing watcher OOBI"),
                            (Some(oobi), Some(watcher_oobi)) => {
                                let watcher_oobi: LocationScheme = serde_json::from_str(&watcher_oobi).expect("Can't parse watcher OOBI");
                                let kel = handle_get_identifier_kel(&id, oobi, watcher_oobi).await?;
                                match kel {
                                    Some(kel) => println!("{}", kel),
                                    None => println!("\nNo kel of {} found", &id),
                                };
                            },
                        };
                    },
                    (Some(alias), None) => {
                        let kel = handle_get_alias_kel(&alias).await?;
                        match kel {
                            Some(kel) => println!("{}", kel),
                            None => println!("\nNo kel of {} locally", alias),
                        };
                    },
                    _ => unreachable!(),
                };
                
            }
        },
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
            TelCommands::Query {
                alias,
                issuer_id,
                registry_id,
                said,
            } => {
                handle_query(&alias, &said, &registry_id, &issuer_id).await?;
            }
            TelCommands::Oobi { alias } => {
                handle_tel_oobi(&alias)?;
            }
        },
        Some(Commands::Said { command }) => match command {
            SaidCommands::SAD { file } => {
                let sad = handle_sad(file).await?;
                println!("{}", sad);
            }
        },
        Some(Commands::Info { alias }) => {
            handle_info(&alias)?;
        }
        Some(Commands::Sign { alias, data }) => {
            println!("{}", handle_sign(alias, &data)?);
        }
        Some(Commands::Seed { code, secret_key }) => {
            // seed is in b64
            let seed = match (code, secret_key) {
                (None, None) => Ok(generate_seed()),
                (None, Some(_sk)) => Ok("Code needs to be provided".to_string()),
                (Some(_code), None) => Ok("Key needs to be provided".to_string()),
                (Some(code), Some(sk)) => {
                    let code = (&code).parse().map_err(|e| CliError::SeedError(code.to_string()));
                    let sk = BASE64_STANDARD.decode(&sk).map_err(|_| CliError::B64Error(sk.to_string()));
                    match (code, sk) {
                        (Ok(code), Ok(sk)) => {
                            convert_to_seed(code, sk)
                        },
                        (Ok(_), Err(e)) | (Err(e), Ok(_)) | (Err(e), Err(_)) => Err(e),
                    }
                },
            };
            match seed {
                Ok(seed) => println!("{}", seed),
                Err(e) => println!("{}", e.to_string()),
            }
        }
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
