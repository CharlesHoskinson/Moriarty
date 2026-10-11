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
    /// Expected account authority and domain binding JSON
    #[arg(long)]
    pub authority: PathBuf,
    /// Current logical time used to check the signed validity interval
    #[arg(long)]
    pub at: u128,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let (source, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let authority = super::core_io::read_json(&args.authority)?;
    let signed = super::core_io::read_json(&args.signature)?;
    match mori_core::intent::verify_and_simulate(
        &program,
        &mori_core::intent::hash_bytes(source.as_bytes()),
        &args.input.action,
        &scenario,
        &authority,
        &signed,
        mori_core::UInt(args.at),
    ) {
        Ok(execution) => {
            if args.output.review {
                println!(
                    "Signature and local authority constraints verified at logical time {}.",
                    args.at
                );
                println!(
                    "Complete local reference effects follow; ledger acceptance requires its own verification."
                );
            }
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "signature_valid":true, "qualification":"local-signature-and-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}
