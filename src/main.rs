mod cli;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use eyre::{eyre, Result};
use serde_json::from_str;
use solana_sdk::{signature::Keypair, signer::keypair::read_keypair_file};

fn load_keypair(src: &str) -> Result<Keypair> {
  let keypair_from_file = shellexpand::full(&src)
    .map_err(|e| eyre!(e))
    .and_then(|path| -> Result<_> { Ok(PathBuf::from(&*path).canonicalize()?) })
    .and_then(|path| read_keypair_file(&path).map_err(|_| eyre!("Cannot read keypair")));

  match keypair_from_file {
    Ok(keypair) => Ok(keypair),
    Err(_) => Ok(Keypair::from_bytes(&bs58::decode(src).into_vec()?)?),
  }
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  if cli.json.is_none() && cli.base58.is_none() {
    return Err(eyre::eyre!("You must provide either a json or base58 private key"));
  }

  if let Some(json) = cli.json {
    if let Ok(wallet) = load_keypair(&json) {
      println!("{}", wallet.to_base58_string());
      return Ok(());
    }
    let array: Vec<_> = from_str(&json)?;
    println!("{}", bs58::encode(&array).into_string());
    return Ok(());
  }

  if let Some(b68) = cli.base58 {
    println!("{:?}", bs58::decode(&b68).into_vec()?);
    return Ok(());
  }

  Ok(())
}
