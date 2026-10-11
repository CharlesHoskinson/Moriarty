use super::{core_io, proof_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: proof_io::Input,
    /// New proof bundle file; refuses to overwrite existing files
    #[arg(long)]
    pub out: PathBuf,
}
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    proof_io::require_new(&args.out)?;
    let (source, scenario, target) = args.input.read()?;
    let (bundle, receipt) = mori_backend::proving::prove(
        &source,
        &args.input.action.action,
        &scenario,
        &target,
        &args.input.options(),
    )
    .map_err(core_io::operation)?;
    let bytes = serde_json::to_vec_pretty(&bundle).map_err(core_io::operation)?;
    mori_backend::proving::ProofBundle::from_json(
        std::str::from_utf8(&bytes).map_err(core_io::operation)?,
    )
    .map_err(core_io::operation)?;
    proof_io::publish_new(&args.out, &bytes)?;
    core_io::print(&receipt)?;
    Ok(ExitCode::SUCCESS)
}
