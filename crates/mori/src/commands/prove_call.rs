use super::call_io;
#[cfg(feature = "native")]
use super::core_io;
use crate::shell::Shell;
#[cfg(feature = "native")]
use rand::SeedableRng;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: call_io::Input,
    #[arg(long)]
    pub keyset: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
#[cfg(feature = "native")]
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    call_io::require_new(&args.out)?;
    let i = args.input.read()?;
    let imported = mori_ledger::call_artifacts::snapshot_proving_keyset(
        &args.keyset,
        &args.input.action,
        &i.limits,
    )
    .map_err(core_io::ledger_operation)?;
    let derived = futures::executor::block_on(
        mori_ledger::local_deployment::reopen_local_proving_deployment(
            &i.source,
            &i.program,
            &i.deployment,
            &imported,
            &args.input.action,
            &args.input.params_dir,
            &i.limits,
        ),
    )
    .map_err(core_io::ledger_operation)?;
    let prepared = mori_ledger::prepare::prepare_installed_native_call(
        &i.source,
        &i.program,
        &i.deployment,
        &i.predecessor,
        &i.parameters,
        &i.request,
        &derived.public_deployment(),
        &i.limits,
    )
    .map_err(core_io::ledger_operation)?;
    let proven = futures::executor::block_on(mori_ledger::proving::prove_installed_call(
        prepared,
        derived,
        rand::rngs::StdRng::from_entropy(),
    ))
    .map_err(core_io::ledger_operation)?;
    let output = mori_ledger::call_artifacts::export_proven_call(&proven, &i.limits)
        .map_err(core_io::ledger_operation)?;
    call_io::publish(&args.out, &output)?;
    Ok(call_io::status(
        shell,
        "Proven",
        &args.input.action,
        &args.out,
    ))
}

#[cfg(not(feature = "native"))]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let _ = (
        &args.input.file,
        &args.input.deployment,
        &args.input.predecessor,
        &args.input.parameters,
        &args.input.request,
        &args.input.action,
        &args.input.params_dir,
        &args.input.work_limits,
        &args.keyset,
        &args.out,
    );
    super::native_unavailable()
}
