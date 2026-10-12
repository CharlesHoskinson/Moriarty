use super::{call_io, core_io};
use crate::shell::Shell;
use std::{io::Write, path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub proof: PathBuf,
}
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let i = args.input.read()?;
    // This reader never accesses PK/prover-cache leaves, including when the
    // producing keyset marker contains their optional descriptions.
    let imported =
        mori_ledger::call_artifacts::snapshot_public_keyset(&args.public_keyset, &i.limits)
            .map_err(core_io::ledger_operation)?;
    let public = futures::executor::block_on(
        mori_ledger::local_deployment::rederive_local_public_deployment(
            &i.source,
            &i.program,
            &i.deployment,
            &imported,
            &args.input.params_dir,
            &i.limits,
        ),
    )
    .map_err(core_io::ledger_operation)?;
    let expected = mori_ledger::prepare::prepare_installed_native_call(
        &i.source,
        &i.program,
        &i.deployment,
        &i.predecessor,
        &i.parameters,
        &i.request,
        &public,
        &i.limits,
    )
    .map_err(core_io::ledger_operation)?;
    let candidate = mori_ledger::call_artifacts::snapshot_proven_call(&args.proof, &i.limits)
        .map_err(core_io::ledger_operation)?;
    candidate
        .admit_context(&expected.proof_context())
        .map_err(core_io::ledger_operation)?;
    candidate
        .admit_inspection(&expected)
        .map_err(core_io::ledger_operation)?;
    let verified = mori_ledger::proving::verify_installed_call(&expected, candidate.intent())
        .map_err(core_io::ledger_operation)?;
    candidate
        .admit_readback(&expected, &verified, &i.limits)
        .map_err(core_io::ledger_operation)?;
    let bytes = mori_ledger::call_artifacts::verification_bytes(&verified, &i.limits)
        .map_err(core_io::ledger_operation)?;
    std::io::stdout()
        .lock()
        .write_all(&bytes)
        .map_err(core_io::operation)?;
    Ok(ExitCode::SUCCESS)
}
