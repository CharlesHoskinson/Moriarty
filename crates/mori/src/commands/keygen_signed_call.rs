use super::{call_io, core_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub signer_policy: PathBuf,
    #[arg(long)]
    pub seed_deployment: PathBuf,
    #[arg(long)]
    pub seed_state: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    call_io::require_new(&args.out)?;
    let limits = call_io::limits(&args.work_limits)?;
    mori_ledger::signed::validate_work_limits(&limits).map_err(core_io::ledger_operation)?;
    let (source, program) = call_io::program(&args.file, limits.max_input_bytes)?;
    let descriptor = mori_ledger::profile::Deployment::from_json(&call_io::text(
        &args.seed_deployment,
        limits.max_input_bytes,
    )?)
    .map_err(core_io::ledger_operation)?;
    let seed_bytes = call_io::binary(
        &args.seed_state,
        limits
            .max_input_bytes
            .min(mori_ledger::codec::INPUT_BYTES as u64),
    )?;
    let parameters = call_io::binary(
        &args.parameters,
        limits
            .max_input_bytes
            .min(mori_ledger::codec::INPUT_BYTES as u64),
    )?;
    // Complete source/ABI/frame/state/duty/UTXO/parameters/supply/keyless-map
    // admission precedes every model/SRS/keygen call. There is no request here.
    let policy = mori_ledger::signed::FixedSignerPolicy::from_json(&call_io::text(
        &args.signer_policy,
        limits
            .max_input_bytes
            .min(mori_ledger::signed::SIGNED_JSON_BYTES as u64),
    )?)
    .map_err(core_io::ledger_operation)?;
    let seed = mori_ledger::signed::admit_signed_local_seed(
        &source,
        &program,
        &descriptor,
        &seed_bytes,
        &parameters,
        &policy,
    )
    .map_err(core_io::ledger_operation)?;
    let derived = futures::executor::block_on(mori_ledger::signed::derive_signed_local_deployment(
        &seed,
        &args.action,
        &args.params_dir,
        &limits,
    ))
    .map_err(core_io::ledger_operation)?;
    let snapshot =
        mori_ledger::signed::build_signed_local_installed_snapshot(seed, &derived, &limits)
            .map_err(core_io::ledger_operation)?;
    let output = mori_ledger::signed::export_signed_local_deployment(&snapshot, &derived, &limits)
        .map_err(core_io::ledger_operation)?;
    call_io::publish(&args.out, &output)?;
    Ok(super::signed_call_io::status(
        shell,
        "Derived",
        &args.action,
        &args.out,
    ))
}
