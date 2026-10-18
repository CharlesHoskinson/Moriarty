#[cfg(feature = "native")]
use super::{core_io, proof_io};
#[cfg(feature = "native")]
use crate::error::CliError;
use crate::shell::Shell;
#[cfg(feature = "native")]
use std::{io::Read, path::Path};
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    /// Source-static native relation directory produced by compile-call
    #[arg(long)]
    pub relation: Option<PathBuf>,
    #[arg(long)]
    pub out: PathBuf,
}
#[cfg(feature = "native")]
fn binary(path: &Path) -> super::Result<Vec<u8>> {
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut bytes = Vec::new();
    file.take((mori_ledger::codec::INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if bytes.len() > mori_ledger::codec::INPUT_BYTES {
        return Err(core_io::operation("native public input exceeds 1 MiB"));
    }
    Ok(bytes)
}
#[cfg(feature = "native")]
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let (source, program) = core_io::program(&args.file)?;
    let deployment =
        mori_ledger::profile::Deployment::from_json(&core_io::read_source(&args.deployment)?)
            .map_err(core_io::operation)?;
    let request =
        mori_ledger::profile::CallRequest::from_json(&core_io::read_source(&args.request)?)
            .map_err(core_io::operation)?;
    if request.action != args.action {
        return Err(core_io::operation(
            "selected action differs from public request",
        ));
    }
    let predecessor = binary(&args.predecessor)?;
    let parameters = binary(&args.parameters)?;
    let bundle = if let Some(path) = &args.relation {
        let ir = super::compile_call::artifact_bytes(&path.join("relation.zkir.bin"))?;
        let manifest = super::compile_call::artifact_bytes(&path.join("manifest.json"))?;
        let artifact = mori_ledger::relation::admit(
            &source,
            &program,
            &deployment,
            &request.action,
            &ir,
            &manifest,
        )
        .map_err(core_io::operation)?;
        mori_ledger::prepare::prepare_native_call_checked(
            &source,
            &program,
            &deployment,
            &predecessor,
            &parameters,
            &request,
            &artifact,
        )
        .map(mori_ledger::prepare::PreparedNativeCall::into_bundle)
    } else {
        mori_ledger::prepare::prepare_call(
            &source,
            &program,
            &deployment,
            &predecessor,
            &parameters,
            &request,
        )
    }
    .map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    publish_leaves(&args.out, &bundle.leaves, &bundle.marker)?;
    shell.status(
        "Prepared",
        format!(
            "{} call {} into {}",
            if args.relation.is_some() {
                "local relation checked, unsigned, unproven"
            } else {
                "local unsigned, unproven"
            },
            core_io::review_identity(&args.action),
            core_io::review_identity(&args.out.to_string_lossy())
        ),
    );
    Ok(ExitCode::SUCCESS)
}
#[cfg(feature = "native")]
fn publish_leaves(
    out: &Path,
    leaves: &std::collections::BTreeMap<String, Vec<u8>>,
    marker: &[u8],
) -> super::Result<()> {
    for (name, bytes) in leaves {
        proof_io::publish_new(&out.join(name), bytes)?;
    }
    proof_io::publish_new(&out.join("preparation.json"), marker)
}
#[cfg(all(test, feature = "native"))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    #[test]
    fn publication_leaf_collisions_preserve_files_links_and_directories() {
        // Exercise the same publication function consumed after actual CLI preparation.
        for leaf in ["abi.json", "offer.bin", "preparation.json"] {
            for kind in 0..4 {
                let root = std::env::temp_dir().join(format!(
                    "mori-prepare-publish-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                std::fs::create_dir(&root).unwrap();
                let path = root.join(leaf);
                let sentinel = root.join("unrelated");
                std::fs::write(&sentinel, b"preserve unrelated").unwrap();
                match kind {
                    0 => std::fs::write(&path, b"existing public sentinel").unwrap(),
                    1 => std::fs::create_dir(&path).unwrap(),
                    2 => {
                        #[cfg(unix)]
                        std::os::unix::fs::symlink(&sentinel, &path).unwrap();
                    }
                    _ => {
                        #[cfg(unix)]
                        std::os::unix::fs::symlink(root.join("absent"), &path).unwrap();
                    }
                }
                let leaves = [
                    ("abi.json".into(), b"first".to_vec()),
                    ("offer.bin".into(), b"middle".to_vec()),
                ]
                .into_iter()
                .collect();
                assert!(publish_leaves(&root, &leaves, b"complete marker").is_err());
                assert_eq!(std::fs::read(&sentinel).unwrap(), b"preserve unrelated");
                if leaf != "preparation.json" {
                    assert!(!root.join("preparation.json").exists());
                }
                if kind == 0 {
                    assert_eq!(std::fs::read(&path).unwrap(), b"existing public sentinel");
                } else if kind >= 2 {
                    assert!(
                        std::fs::symlink_metadata(&path)
                            .unwrap()
                            .file_type()
                            .is_symlink()
                    );
                } else {
                    assert!(path.is_dir());
                }
                std::fs::remove_dir_all(root).unwrap();
            }
        }
    }
}

#[cfg(not(feature = "native"))]
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let _ = (
        &args.file,
        &args.action,
        &args.deployment,
        &args.predecessor,
        &args.parameters,
        &args.request,
        &args.relation,
        &args.out,
    );
    super::native_unavailable()
}
