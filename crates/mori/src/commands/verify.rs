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
    let bundle = mori_backend::proving::ProofBundle::from_json(&core_io::read_source(&args.proof)?)
        .map_err(core_io::operation)?;
    let (source, scenario, target) = args.input.read()?;
    let receipt = mori_backend::proving::verify(
        &source,
        &args.input.action.action,
        &scenario,
        &target,
        &bundle,
        &args.input.options(),
    )
    .map_err(core_io::operation)?;
    core_io::print(&receipt)?;
    Ok(ExitCode::SUCCESS)
}
