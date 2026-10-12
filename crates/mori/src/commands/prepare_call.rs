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
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
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
    let bundle = mori_ledger::prepare::prepare_call(
        &source,
        &program,
        &deployment,
        &predecessor,
        &parameters,
        &request,
    )
    .map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    publish_leaves(&args.out, &bundle.leaves, &bundle.marker)?;
    shell.status(
        "Prepared",
        format!(
            "local unsigned, unproven call {} into {}",
            core_io::review_identity(&args.action),
            args.out.display()
        ),
    );
    Ok(ExitCode::SUCCESS)
}
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
#[cfg(test)]
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
