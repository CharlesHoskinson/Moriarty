use super::core_io;
use crate::{error::CliError, shell::Shell};
use std::{
    path::Path,
    process::ExitCode,
};

#[cfg(test)]
use std::path::PathBuf;

pub use crate::cli_args::compile::Args;

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let source = core_io::read_source(&args.file)?;
    // Selected HostOnly preparation returns before either native/backend path.
    if args.execution_profile.is_some() || args.source_profile.is_some() || args.entry {
        return super::compile_sherlock::run(&args, &source, |verb, message| shell.status(verb, message));
    }
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        let target: mori_backend::Target = core_io::read_json(&args.target)?;
        let artifact = mori_backend::stage::compile_staged(&source, &target, &args.out)
            .map_err(core_io::operation)?;
        let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
        std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
            path: args.out.clone(),
            source,
        })?;
        publish_staged_artifact(&artifact, &manifest, &args.out)?;
        shell.status(
            "Compiled",
            format!("{} circuits into {}", artifact.circuits.len(), args.out.display()),
        );
        return Ok(ExitCode::SUCCESS);
    }
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
    let artifact = reported.artifact;
    let projection = serde_json::to_string_pretty(&serde_json::json!({
        "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
        "compilation": reported.source_projection,
    }))
    .map_err(core_io::operation)?;
    // Compile and serialize fully before touching the destination.
    let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    super::proof_io::publish_new(&args.out.join("source-projection.json"), projection.as_bytes())?;
    publish_artifact(&artifact, &manifest, &args.out)?;
    shell.status(
        "Compiled",
        format!(
            "{} circuits into {}",
            artifact.circuits.len(),
            args.out.display()
        ),
    );
    Ok(ExitCode::SUCCESS)
}

// The caller has reserved a new directory after all input/compiler preflight.
fn publish_artifact(
    artifact: &mori_backend::Artifact,
    manifest: &str,
    out: &Path,
) -> super::Result<()> {
    for (index, circuit) in artifact.circuits.iter().enumerate() {
        let path = out.join(format!("{index:03}-{}.zkir", circuit.action));
        super::proof_io::publish_new(&path, circuit.zkir.as_bytes())?;
    }
    let path = out.join("artifact.json");
    // The manifest is the completion marker; publish it after every circuit.
    super::proof_io::publish_new(&path, manifest.as_bytes())?;
    Ok(())
}

// The staged compiler returns the real artifact; publication stays in the CLI.
fn publish_staged_artifact(
    artifact: &mori_backend::stage::StagedArtifact,
    manifest: &str,
    out: &Path,
) -> super::Result<()> {
    for (index, circuit) in artifact.circuits.iter().enumerate() {
        let path = out.join(format!("{index:03}-{}.s{}.zkir", circuit.action, circuit.segment));
        super::proof_io::publish_new(&path, circuit.zkir.as_bytes())?;
    }
    // As in publish_artifact, the manifest is the completion marker.
    super::proof_io::publish_new(&out.join("artifact.json"), manifest.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Reserved(PathBuf);
    impl Reserved {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "mori-compile-leaves-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Reserved {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn artifact() -> (mori_backend::Artifact, String) {
        let p=mori_core::parse("language \"moriarty/1\"; agreement Publication { state count: UInt = 0; action z_first() { next count = count + 1; } action a_second() { next count = count + 2; } }").unwrap();
        let a =
            mori_backend::compile(&p, &mori_backend::Target::ledger9("local", "leaves")).unwrap();
        let manifest = serde_json::to_string_pretty(&a).unwrap();
        (a, manifest)
    }
    #[test]
    fn raced_files_at_first_second_and_manifest_are_never_clobbered() {
        let (a, m) = artifact();
        for leaf in ["000-z_first.zkir", "001-a_second.zkir", "artifact.json"] {
            let d = Reserved::new();
            let p = d.0.join(leaf);
            std::fs::write(&p, b"third-party sentinel").unwrap();
            std::fs::write(d.0.join("unrelated"), b"user addition").unwrap();
            assert!(
                publish_artifact(&a, &m, &d.0).is_err(),
                "raced {leaf} must refuse"
            );
            assert_eq!(std::fs::read(&p).unwrap(), b"third-party sentinel");
            assert_eq!(
                std::fs::read(d.0.join("unrelated")).unwrap(),
                b"user addition"
            );
            if leaf != "artifact.json" {
                assert!(!d.0.join("artifact.json").exists());
            }
            let completed = match leaf {
                "000-z_first.zkir" => 0,
                "001-a_second.zkir" => 1,
                _ => 2,
            };
            assert_eq!(std::fs::read_dir(&d.0).unwrap().count(), completed + 2);
            for (i, c) in a.circuits.iter().take(completed).enumerate() {
                assert_eq!(
                    std::fs::read(d.0.join(format!("{i:03}-{}.zkir", c.action))).unwrap(),
                    c.zkir.as_bytes()
                );
            }
        }
    }
    #[cfg(unix)]
    #[test]
    fn raced_symlinks_dangling_symlinks_and_directories_are_preserved() {
        let (a, m) = artifact();
        for leaf in ["001-a_second.zkir", "artifact.json"] {
            for kind in ["symlink", "dangling", "directory", "directory-symlink"] {
                let d = Reserved::new();
                let external = Reserved::new();
                let target = external.0.join("target");
                let p = d.0.join(leaf);
                match kind {
                    "symlink" => {
                        std::fs::write(&target, b"external sentinel").unwrap();
                        std::os::unix::fs::symlink(&target, &p).unwrap();
                    }
                    "dangling" => std::os::unix::fs::symlink(&target, &p).unwrap(),
                    "directory" => std::fs::create_dir(&p).unwrap(),
                    _ => {
                        std::fs::create_dir(&target).unwrap();
                        std::os::unix::fs::symlink(&target, &p).unwrap();
                    }
                }
                assert!(publish_artifact(&a, &m, &d.0).is_err());
                if kind == "directory" {
                    assert!(p.is_dir());
                    assert_eq!(std::fs::read_dir(&p).unwrap().count(), 0);
                } else {
                    assert_eq!(std::fs::read_link(&p).unwrap(), target);
                }
                if kind == "symlink" {
                    assert_eq!(std::fs::read(&target).unwrap(), b"external sentinel");
                } else if kind == "dangling" {
                    assert!(!target.exists());
                }
                if leaf != "artifact.json" {
                    assert!(!d.0.join("artifact.json").exists());
                }
                assert_eq!(
                    std::fs::read(d.0.join("000-z_first.zkir")).unwrap(),
                    a.circuits[0].zkir.as_bytes()
                );
                assert_eq!(
                    std::fs::read_dir(&d.0).unwrap().count(),
                    if leaf == "artifact.json" { 3 } else { 2 }
                );
            }
        }
    }
    #[test]
    fn final_manifest_follows_all_complete_ordered_circuits() {
        let (a, m) = artifact();
        let d = Reserved::new();
        publish_artifact(&a, &m, &d.0).unwrap();
        assert_eq!(
            std::fs::read(d.0.join("artifact.json")).unwrap(),
            m.as_bytes()
        );
        assert_eq!(std::fs::read_dir(&d.0).unwrap().count(), 3);
        for (i, c) in a.circuits.iter().enumerate() {
            assert_eq!(
                std::fs::read(d.0.join(format!("{i:03}-{}.zkir", c.action))).unwrap(),
                c.zkir.as_bytes()
            );
        }
    }
}
