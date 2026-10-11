use super::{core_io, proof_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: proof_io::Input,
    /// Native proof bundle to verify without modifying it
    #[arg(long)]
    pub proof: PathBuf,
}
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    // Mode and argument group are settled before the candidate is read.
    let (source, scenario, target, signed) = args.input.read()?;
    let bundle = mori_backend::proving::ProofBundle::from_json(&core_io::read_source(&args.proof)?)
        .map_err(core_io::operation)?;
    let action = &args.input.action.action;
    let options = args.input.options();
    let receipt = match &signed {
        Some(s) => mori_backend::proving::verify_signed(
            &source,
            action,
            &scenario,
            &target,
            &s.authority,
            s.at,
            &s.intent,
            &bundle,
            &options,
        ),
        None => {
            mori_backend::proving::verify(&source, action, &scenario, &target, &bundle, &options)
        }
    }
    .map_err(core_io::operation)?;
    core_io::print(&receipt)?;
    Ok(ExitCode::SUCCESS)
}
