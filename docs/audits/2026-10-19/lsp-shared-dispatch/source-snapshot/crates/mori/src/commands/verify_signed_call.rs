#[cfg(feature = "native")]
use super::core_io;
use crate::shell::Shell;
#[cfg(feature = "native")]
use std::io::Write;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub proof: PathBuf,
    #[arg(long)]
    pub signed_attachment: PathBuf,
}
#[cfg(feature = "native")]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let i = args.input.read()?;
    // This reader never accesses PK/prover-cache leaves, including when the
    // producing keyset marker contains their optional descriptions.
    let attachment = super::signed_call_io::attachment(&args.signed_attachment, &i.limits)?;
    let imported = mori_ledger::signed::snapshot_signed_public_keyset(
        &args.public_keyset,
        &i.limits,
        i.deployment.version().map_err(core_io::ledger_operation)?,
    )
    .map_err(core_io::ledger_operation)?;
    let public = futures::executor::block_on(
        mori_ledger::signed::rederive_signed_local_public_deployment(
            &i.source,
            &i.program,
            &i.deployment,
            &i.policy,
            &imported,
            &args.input.params_dir,
            &i.limits,
        ),
    )
    .map_err(core_io::ledger_operation)?;
    let deployment = super::signed_call_io::descriptor(&args.input, &i, &public)?;
    let statement = mori_ledger::signed::prepare_installed_call_signing_statement(
        &i.source,
        &i.program,
        &deployment,
        &i.policy,
        &i.predecessor,
        &i.parameters,
        &i.request,
        &args.input.action,
        &public,
        &i.limits,
    )
    .map_err(core_io::ledger_operation)?;
    let expected = mori_ledger::signed::attach_signed_installed_call(statement, &attachment)
        .map_err(core_io::ledger_operation)?;
    let candidate = mori_ledger::signed::snapshot_signed_proven_call(
        &args.proof,
        &i.limits,
        i.deployment.version().map_err(core_io::ledger_operation)?,
    )
    .map_err(core_io::ledger_operation)?;
    let verified = candidate
        .verify(&expected, &i.limits)
        .map_err(core_io::ledger_operation)?;
    let bytes = mori_ledger::call_artifacts::verification_bytes(&verified, &i.limits)
        .map_err(core_io::ledger_operation)?;
    std::io::stdout()
        .lock()
        .write_all(&bytes)
        .map_err(core_io::operation)?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(not(feature = "native"))]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let _ = (
        &args.input.file,
        &args.input.signer_policy,
        &args.input.deployment,
        &args.input.predecessor,
        &args.input.parameters,
        &args.input.request,
        &args.input.action,
        &args.input.params_dir,
        &args.input.work_limits,
        &args.input.derive_supplied_frame,
        &args.public_keyset,
        &args.proof,
        &args.signed_attachment,
    );
    super::native_unavailable()
}
