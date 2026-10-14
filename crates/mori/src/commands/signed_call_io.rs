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
    #[arg(long)]
    pub derive_supplied_frame: bool,
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
    fn validate_frame_mode(&self, version: SignedVersion) -> super::Result<()> {
        if self.derive_supplied_frame && version != SignedVersion::ConsumingV2 {
            return Err(core_io::operation(
                "--derive-supplied-frame requires consuming signed version2",
            ));
        }
        Ok(())
    }
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
        self.validate_frame_mode(deployment.version().map_err(core_io::ledger_operation)?)?;
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
        SignedStaticBindings::from_source(
            &source,
            &program,
            &deployment.bindings,
            &policy,
            deployment.version().map_err(core_io::ledger_operation)?,
        )
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
pub fn descriptor(
    input: &Input,
    i: &Inputs,
    public: &SignedPublicLocalDeployment,
) -> super::Result<SignedInstalledDeployment> {
    if input.derive_supplied_frame {
        derive_signed_supplied_frame(
            &i.source,
            &i.program,
            &i.deployment,
            &i.policy,
            &i.predecessor,
            &i.parameters,
            public,
            &i.limits,
        )
        .map_err(core_io::ledger_operation)
    } else {
        Ok(i.deployment.clone())
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    #[test]
    fn nonce_v2_cli_frame_option_is_explicit_and_v2_only() {
        for command in [
            "prepare-signed-call",
            "prove-signed-call",
            "verify-signed-call",
        ] {
            let mut arguments = vec![
                "mori",
                command,
                "source.mori",
                "--signer-policy",
                "policy.json",
                "--deployment",
                "deployment.json",
                "--predecessor",
                "state.bin",
                "--parameters",
                "ledger-parameters.bin",
                "--request",
                "request.json",
                "--action",
                "first",
                "--params-dir",
                "unused",
                "--work-limits",
                "limits.json",
            ];
            match command {
                "prepare-signed-call" => {
                    arguments.extend(["--public-keyset", "public", "--out", "output"])
                }
                "prove-signed-call" => arguments.extend([
                    "--keyset",
                    "keys",
                    "--signed-attachment",
                    "attachment.json",
                    "--out",
                    "output",
                ]),
                _ => arguments.extend([
                    "--public-keyset",
                    "public",
                    "--proof",
                    "proof",
                    "--signed-attachment",
                    "attachment.json",
                ]),
            }
            for flag in [false, true] {
                let mut args = arguments.clone();
                if flag {
                    args.push("--derive-supplied-frame");
                }
                let cli = crate::cli::Cli::try_parse_from(args).unwrap();
                let input = match cli.command {
                    crate::cli::Command::PrepareSignedCall(a) => a.input,
                    crate::cli::Command::ProveSignedCall(a) => a.input,
                    crate::cli::Command::VerifySignedCall(a) => a.input,
                    _ => panic!(),
                };
                assert_eq!(input.derive_supplied_frame, flag);
                input
                    .validate_frame_mode(SignedVersion::ConsumingV2)
                    .unwrap();
                assert_eq!(
                    input.validate_frame_mode(SignedVersion::LegacyV1).is_err(),
                    flag
                );
            }
        }
    }
}
