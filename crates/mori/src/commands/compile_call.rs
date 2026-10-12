use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub deployment: PathBuf,
    /// New output directory, containing tagged IR and static manifest
    #[arg(long)]
    pub out: PathBuf,
}
pub(crate) fn artifact_bytes(path: &Path) -> super::Result<Vec<u8>> {
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut bytes = vec![];
    file.take((mori_ledger::codec::OUTPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if bytes.len() > mori_ledger::codec::OUTPUT_BYTES {
        return Err(core_io::operation("relation artifact exceeds output bound"));
    }
    Ok(bytes)
}
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let (source, p) = core_io::program(&args.file)?;
    let deployment =
        mori_ledger::profile::Deployment::from_json(&core_io::read_source(&args.deployment)?)
            .map_err(core_io::operation)?;
    let artifact = mori_ledger::relation::compile(&source, &p, &deployment, &args.action)
        .map_err(core_io::operation)?;
    let ir = artifact.ir_bytes().map_err(core_io::operation)?;
    let manifest = artifact.manifest_bytes().map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    publish(&args.out, &ir, &manifest)?;
    shell.status(
        "Compiled",
        format!(
            "ordinary native call relation {} into {}",
            core_io::review_identity(&args.action),
            core_io::review_identity(&args.out.to_string_lossy())
        ),
    );
    Ok(ExitCode::SUCCESS)
}
fn publish(out: &Path, ir: &[u8], manifest: &[u8]) -> super::Result<()> {
    proof_io::publish_new(&out.join("relation.zkir.bin"), ir)?;
    // Completion marker follows the complete source-static IR leaf.
    proof_io::publish_new(&out.join("manifest.json"), manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collision_does_not_publish_completed_manifest() {
        let out = std::env::temp_dir().join(format!(
            "mori-call-artifact-collision-{}",
            std::process::id()
        ));
        std::fs::create_dir(&out).unwrap();
        std::fs::write(out.join("relation.zkir.bin"), b"keep").unwrap();
        assert!(publish(&out, b"new", b"marker").is_err());
        assert_eq!(
            std::fs::read(out.join("relation.zkir.bin")).unwrap(),
            b"keep"
        );
        assert!(!out.join("manifest.json").exists());
        std::fs::remove_dir_all(out).unwrap();
    }
}
