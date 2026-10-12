use super::{call_io, core_io};
use mori_ledger::{profile::NativeWorkLimits, signed::*};
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Input {
    pub file: PathBuf,
    #[arg(long)]
    pub signer_policy: PathBuf,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
}
pub struct Inputs {
    pub source: String,
    pub program: mori_core::Program,
    pub policy: FixedSignerPolicy,
    pub deployment: SignedInstalledDeployment,
    pub predecessor: Vec<u8>,
    pub parameters: Vec<u8>,
    pub request: SignedCallRequest,
    pub limits: NativeWorkLimits,
}
impl Input {
    pub fn read(&self) -> super::Result<Inputs> {
        let limits = call_io::limits(&self.work_limits)?;
        validate_work_limits(&limits).map_err(core_io::ledger_operation)?;
        let (source, program) = call_io::program(&self.file, limits.max_input_bytes)?;
        let policy = FixedSignerPolicy::from_json(&call_io::text(
            &self.signer_policy,
            limits.max_input_bytes.min(SIGNED_JSON_BYTES as u64),
        )?)
        .map_err(core_io::ledger_operation)?;
        let deployment = SignedInstalledDeployment::from_json(&call_io::text(
            &self.deployment,
            limits.max_input_bytes,
        )?)
        .map_err(core_io::ledger_operation)?;
        let request = SignedCallRequest::from_json(&call_io::text(
            &self.request,
            limits.max_input_bytes.min(SIGNED_JSON_BYTES as u64),
        )?)
        .map_err(core_io::ledger_operation)?;
        if request.action != self.action || deployment.signer_policy != policy {
            return Err(core_io::operation(
                "independent selected action or signer policy differs",
            ));
        }
        // Original-source whole-policy admission precedes imported key/SRS work.
        SignedStaticBindings::from_source(&source, &program, &deployment.bindings, &policy)
            .map_err(core_io::ledger_operation)?
            .admit_request(&request, &self.action)
            .map_err(core_io::ledger_operation)?;
        let predecessor = call_io::binary(&self.predecessor, limits.max_input_bytes)?;
        let parameters = call_io::binary(&self.parameters, limits.max_input_bytes)?;
        Ok(Inputs {
            source,
            program,
            policy,
            deployment,
            predecessor,
            parameters,
            request,
            limits,
        })
    }
}
pub fn attachment(
    path: &std::path::Path,
    l: &NativeWorkLimits,
) -> super::Result<SignedCallAttachment> {
    SignedCallAttachment::from_json(&call_io::text(
        path,
        l.max_input_bytes.min(SIGNED_JSON_BYTES as u64),
    )?)
    .map_err(core_io::ledger_operation)
}
pub fn status(
    shell: &mut crate::shell::Shell,
    label: &str,
    action: &str,
    out: &std::path::Path,
) -> ExitCode {
    shell.status(label,format!("local signed call with fixed compiled JubJub policy and supplied state (unqualified) {} into {}",core_io::review_identity(action),core_io::review_identity(&out.to_string_lossy())));
    ExitCode::SUCCESS
}
