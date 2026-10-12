use super::{call_io, core_io};
use crate::shell::Shell;
use rand::SeedableRng;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub keyset: PathBuf,
    #[arg(long)]
    pub signed_attachment: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    call_io::require_new(&args.out)?;
    let i = args.input.read()?;
    let attachment = super::signed_call_io::attachment(&args.signed_attachment, &i.limits)?;
    let imported = mori_ledger::signed::snapshot_signed_proving_keyset(
        &args.keyset,
        &args.input.action,
        &i.limits,
    )
    .map_err(core_io::ledger_operation)?;
    let derived =
        futures::executor::block_on(mori_ledger::signed::reopen_signed_local_proving_deployment(
            &i.source,
            &i.program,
            &i.deployment,
            &i.policy,
            &imported,
            &args.input.action,
            &args.input.params_dir,
            &i.limits,
        ))
        .map_err(core_io::ledger_operation)?;
    let statement = mori_ledger::signed::prepare_installed_call_signing_statement(
        &i.source,
        &i.program,
        &i.deployment,
        &i.policy,
        &i.predecessor,
        &i.parameters,
        &i.request,
        &args.input.action,
        &derived.public_deployment(),
        &i.limits,
    )
    .map_err(core_io::ledger_operation)?;
    let prepared = mori_ledger::signed::attach_signed_installed_call(statement, &attachment)
        .map_err(core_io::ledger_operation)?;
    let proven = futures::executor::block_on(mori_ledger::signed::prove_signed_installed_call(
        prepared,
        derived,
        rand::rngs::StdRng::from_entropy(),
    ))
    .map_err(core_io::ledger_operation)?;
    let output = mori_ledger::signed::export_signed_proven_call(&proven, &i.limits)
        .map_err(core_io::ledger_operation)?;
    call_io::publish(&args.out, &output)?;
    Ok(super::signed_call_io::status(
        shell,
        "Proven",
        &args.input.action,
        &args.out,
    ))
}
