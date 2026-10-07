use std::path::PathBuf;
use std::process::ExitCode;

use crate::cli::{ActionInput, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Signature JSON produced by the external signer
    #[arg(long, value_name = "FILE")]
    pub signature: PathBuf,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori verify-intent")
}
