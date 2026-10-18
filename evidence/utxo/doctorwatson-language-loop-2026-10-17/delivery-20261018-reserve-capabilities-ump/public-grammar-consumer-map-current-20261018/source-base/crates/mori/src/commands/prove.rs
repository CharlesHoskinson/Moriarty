use super::proof_io;
#[cfg(feature = "native")]
use super::core_io;
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
#[cfg(feature = "native")]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    proof_io::require_new(&args.out)?;
    let (source, scenario, target, signed) = args.input.read()?;
    let action = &args.input.action.action;
    let options = args.input.options();
    let (bundle, receipt) = match &signed {
        Some(s) => mori_backend::proving::prove_signed(
            &source,
            action,
            &scenario,
            &target,
            &s.authority,
            s.at,
            &s.intent,
            &options,
        ),
        None => mori_backend::proving::prove(&source, action, &scenario, &target, &options),
    }
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

#[cfg(not(feature = "native"))]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let _ = (
        &args.input.action,
        &args.input.target,
        &args.input.params_dir,
        &args.input.max_k,
        &args.input.signature,
        &args.input.authority,
        &args.input.at,
        &args.out,
    );
    super::native_unavailable()
}
