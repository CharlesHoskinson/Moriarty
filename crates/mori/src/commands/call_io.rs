//! Explicit installed-call inputs and create-new publication. No proving
//! provider, parameter fallback or alternate TransitionClaim consumer exists.
use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use mori_ledger::{
    call_artifacts::{self, ArtifactLeaves},
    profile::{CallRequest, InstalledDeployment, NativeWorkLimits},
};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};
#[derive(Debug, clap::Args)]
pub struct Input {
    pub file: PathBuf,
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
    pub deployment: InstalledDeployment,
    pub predecessor: Vec<u8>,
    pub parameters: Vec<u8>,
    pub request: CallRequest,
    pub limits: NativeWorkLimits,
}
pub fn limits(path: &Path) -> super::Result<NativeWorkLimits> {
    let text = text(path, mori_ledger::codec::INPUT_BYTES as u64)?;
    let value = NativeWorkLimits::from_json(&text).map_err(core_io::operation)?;
    if text.len() as u64 > value.max_input_bytes {
        return Err(core_io::operation(
            "work limits input exceeds explicit input bound",
        ));
    }
    Ok(value)
}
pub fn binary(path: &Path, cap: u64) -> super::Result<Vec<u8>> {
    call_artifacts::read_bounded_file(path, cap).map_err(core_io::operation)
}
pub fn text(path: &Path, cap: u64) -> super::Result<String> {
    String::from_utf8(binary(
        path,
        cap.min(mori_ledger::codec::INPUT_BYTES as u64),
    )?)
    .map_err(core_io::operation)
}
pub fn program(path: &Path, cap: u64) -> super::Result<(String, mori_core::Program)> {
    let source = text(path, cap)?;
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    Ok((source, program))
}
impl Input {
    pub fn read(&self) -> super::Result<Inputs> {
        let limits = limits(&self.work_limits)?;
        let (source, program) = program(&self.file, limits.max_input_bytes)?;
        let deployment =
            InstalledDeployment::from_json(&text(&self.deployment, limits.max_input_bytes)?)
                .map_err(core_io::operation)?;
        let request = CallRequest::from_json(&text(&self.request, limits.max_input_bytes)?)
            .map_err(core_io::operation)?;
        if self.action != request.action {
            return Err(core_io::operation("selected action differs from request"));
        }
        let action = program
            .actions
            .iter()
            .find(|a| a.name == self.action)
            .ok_or_else(|| core_io::operation("unknown source action"))?;
        if request.inputs.len() != action.inputs.len()
            || action
                .inputs
                .iter()
                .any(|i| request.inputs.get(&i.name).is_none_or(|v| v.ty() != i.ty))
        {
            return Err(core_io::operation(
                "request input names or types differ from source",
            ));
        }
        let predecessor = binary(&self.predecessor, limits.max_input_bytes)?;
        let parameters = binary(&self.parameters, limits.max_input_bytes)?;
        Ok(Inputs {
            source,
            program,
            deployment,
            predecessor,
            parameters,
            request,
            limits,
        })
    }
}
pub fn require_new(out: &Path) -> super::Result<()> {
    proof_io::require_new(out)
}
pub fn publish(out: &Path, artifact: &ArtifactLeaves) -> super::Result<()> {
    std::fs::create_dir(out).map_err(|source| CliError::Write {
        path: out.into(),
        source,
    })?;
    publish_leaves(out, artifact)
}
fn publish_leaves(out: &Path, artifact: &ArtifactLeaves) -> super::Result<()> {
    for (name, bytes) in &artifact.leaves {
        proof_io::publish_new(&out.join(name), bytes)?;
    }
    proof_io::publish_new(&out.join(artifact.marker_name), &artifact.marker)
}
pub fn status(shell: &mut Shell, label: &str, action: &str, out: &Path) -> ExitCode {
    shell.status(
        label,
        format!(
            "local unsigned source-derived installed call {} into {}",
            core_io::review_identity(action),
            core_io::review_identity(&out.to_string_lossy())
        ),
    );
    ExitCode::SUCCESS
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn existing_output_directory_is_preserved_without_marker() {
        let out =
            std::env::temp_dir().join(format!("mori-installed-publish-{}", std::process::id()));
        std::fs::create_dir(&out).unwrap();
        std::fs::write(out.join("sentinel"), b"keep").unwrap();
        // Pure publication bytes, no native result/capability construction.
        let artifact = ArtifactLeaves {
            leaves: Default::default(),
            marker_name: "public-keyset.json",
            marker: b"publication-only".to_vec(),
        };
        assert!(publish(&out, &artifact).is_err());
        assert_eq!(std::fs::read(out.join("sentinel")).unwrap(), b"keep");
        assert!(!out.join("public-keyset.json").exists());
        std::fs::remove_dir_all(out).unwrap();
    }
    #[test]
    fn partial_leaf_collision_preserves_files_and_never_writes_own_marker() {
        for (case, leaf) in ["a.bin", "b.bin", "public-keyset.json"]
            .into_iter()
            .enumerate()
        {
            let out = std::env::temp_dir().join(format!(
                "mori-installed-leaf-collision-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir(&out).unwrap();
            std::fs::write(out.join(leaf), b"keep").unwrap();
            let artifact = ArtifactLeaves {
                leaves: [
                    ("a.bin".into(), b"first".to_vec()),
                    ("b.bin".into(), b"second".to_vec()),
                ]
                .into_iter()
                .collect(),
                marker_name: "public-keyset.json",
                marker: b"publication-only".to_vec(),
            };
            assert!(publish_leaves(&out, &artifact).is_err());
            assert_eq!(std::fs::read(out.join(leaf)).unwrap(), b"keep");
            if leaf != "public-keyset.json" {
                assert!(!out.join("public-keyset.json").exists());
            }
            std::fs::remove_dir_all(out).unwrap();
        }
    }
}
