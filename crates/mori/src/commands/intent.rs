use std::process::ExitCode;

use clap::ValueEnum;

use crate::cli::{ActionInput, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Signature scheme of the owner key
    #[arg(long)]
    pub scheme: Scheme,

    /// Owner public key, hex encoded
    #[arg(long, value_name = "HEX")]
    pub public_key: String,

    /// How the signing bytes are framed
    #[arg(long)]
    pub framing: Framing,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Scheme {
    #[value(name = "schnorr_bip340")]
    SchnorrBip340,
    #[value(name = "ecdsa_secp256k1_sha256")]
    EcdsaSecp256k1Sha256,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Framing {
    Raw,
    MidnightSignData,
}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori intent")
}
