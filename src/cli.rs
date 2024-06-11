use clap::Parser;

#[derive(Clone, Parser)]
pub struct Cli {
  /// json format private key
  #[arg(short, long)]
  pub json: Option<String>,
  /// base58 format private key
  #[arg(short, long)]
  pub base58: Option<String>,
}
