use super::core_io;
use crate::{error::CliError, shell::Shell};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    /// Exact target profile JSON with network, anchor and compiler limits
    #[arg(long)]
    pub target: PathBuf,
    /// New output directory; refuses to overwrite existing artifacts
    #[arg(long)]
    pub out: PathBuf,
    /// Explicit Sherlock execution profile; omission preserves ordinary Core compilation
    #[arg(long)]
    pub execution_profile: Option<String>,
    /// Explicit source dialect on the selected Sherlock route; no autodetection
    #[arg(long, requires = "execution_profile")]
    pub source_profile: Option<String>,
    /// Select the draft program's sole closed entry; no external inputs
    #[arg(long, requires = "execution_profile", conflicts_with = "action")]
    pub entry: bool,
    /// Exact parameter-free source action for selected Sherlock preparation
    #[arg(long, requires = "execution_profile")]
    pub action: Option<String>,
    /// Exact configuration for the selected source profile; no numerical defaults
    #[arg(long, requires = "execution_profile")]
    pub execution_config: Option<PathBuf>,
    /// Explicit host observation after actual preparation/admission
    #[arg(long, requires = "execution_profile")]
    pub observe: bool,
}
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let source = core_io::read_source(&args.file)?;
    // Selected HostOnly preparation returns before either native/backend path.
    if args.execution_profile.is_some() || args.source_profile.is_some() || args.entry {
        return compile_sherlock(&args, &source, shell);
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

/// Selected SDK-free host artifact path. Observation is explicit and retains
/// its separate H1/command-closure dependency; flags confer no internal authority.
fn compile_sherlock(args: &Args, source: &str, shell: &mut Shell) -> super::Result<ExitCode> {
    use mori_core::sherlock_compile_artifact::{self as host, CompileFailure};
    use host::draft_contract::{select_source, SourceSelection};
    let execute = || -> Result<(SelectedSherlockArtifact, String), CompileFailure> {
        let profile = args.execution_profile.as_deref().ok_or_else(||
            CompileFailure::input("select", "missing --execution-profile"))?;
        if profile != host::EXECUTION_PROFILE {
            return Err(CompileFailure::input("select", format!("unsupported execution profile: {profile}")));
        }
        let selection = select_source(args.source_profile.as_deref(), args.action.as_deref(), args.entry)
            .map_err(|error| match error {
                host::draft_contract::SelectionError::MissingAction =>
                    CompileFailure::input("select", "selected Sherlock compilation requires --action"),
                other => CompileFailure {
                    stage: "select", actual_error: serde_json::json!({
                        "selection_error": format!("{other:?}"),
                        "source_profile": args.source_profile.as_deref(),
                        "entry": args.entry, "action": args.action.as_deref(),
                    }), evaluation_performed: false, events_forwarded: "0".into(), completed: false,
                },
            })?;
        if matches!(&selection, SourceSelection::DraftEntry) && args.observe {
            return Err(CompileFailure::input("select",
                "draft preparation is compile-only; --observe is unsupported"));
        }
        let config_path = args.execution_config.as_deref().ok_or_else(||
            CompileFailure::input("config", "selected Sherlock compilation requires --execution-config"))?;
        let config = core_io::read_source(config_path)
            .map_err(|error| CompileFailure::input("config", error.to_string()))?;
        // Target bytes are an explicit descriptor and digest, not native support.
        let target = core_io::read_source(&args.target)
            .map_err(|error| CompileFailure::input("config", error.to_string()))?;
        let artifact = match selection {
            SourceSelection::CoreAction(action) => SelectedSherlockArtifact::Core(
                host::compile_host(source, action, &config, &target, args.observe)?),
            SourceSelection::DraftEntry => SelectedSherlockArtifact::Draft(
                host::compile_draft_host(source, &config, &target)?),
        };
        let manifest = artifact.manifest()?;
        Ok((artifact, manifest))
    };
    let (artifact, manifest) = match execute() {
        Ok(result) => result,
        Err(failure) => {
            return Err(report_sherlock_failure(&failure));
        }
    };
    // Complete preparation/optional observation and serialization before reserving
    // NEWDIR. Existing publish_new never replaces a concurrently created leaf.
    std::fs::create_dir(&args.out).map_err(|source| {
        let actual = CliError::Write { path:args.out.clone(), source };
        report_sherlock_failure(&artifact.failure("publish", actual.to_string()))
    })?;
    super::proof_io::publish_new(&args.out.join("sherlock-artifact.json"), manifest.as_bytes())
        .map_err(|error| report_sherlock_failure(&artifact.failure("publish", error.to_string())))?;
    shell.status("Prepared", format!("Sherlock HostOnly artifact into {}", args.out.display()));
    Ok(ExitCode::SUCCESS)
}

// Serialize the selected artifact directly: no new envelope around artifact/1.
enum SelectedSherlockArtifact {
    Core(mori_core::sherlock_compile_artifact::CompileArtifact),
    Draft(mori_core::sherlock_compile_artifact::draft_report::DraftCompileArtifact),
}
impl SelectedSherlockArtifact {
    fn failure(&self, stage: &'static str, message: impl Into<String>)
        -> mori_core::sherlock_compile_artifact::CompileFailure
    {
        match self { Self::Core(value) => value.failure(stage, message),
            Self::Draft(value) => value.failure(stage, message) }
    }
    fn manifest(&self) -> Result<String, mori_core::sherlock_compile_artifact::CompileFailure> {
        match self {
            Self::Core(value) => serde_json::to_string_pretty(value)
                .map_err(|error| value.failure("observation", error.to_string())),
            Self::Draft(value) => serde_json::to_string_pretty(value)
                .map_err(|error| value.failure("serialization", error.to_string())),
        }
    }
}

fn report_sherlock_failure(
    failure: &mori_core::sherlock_compile_artifact::CompileFailure,
) -> mori_diagnostics::Report {
    match mori_core::json::export(failure) {
        Ok(diagnostic) => core_io::operation(diagnostic),
        Err(error) => core_io::operation(error),
    }
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
