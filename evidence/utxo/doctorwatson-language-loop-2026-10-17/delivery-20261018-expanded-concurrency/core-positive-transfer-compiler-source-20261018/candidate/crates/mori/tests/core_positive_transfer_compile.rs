//! A new Core/1 source through the public compiler and its published bundle.
use mori_backend::Artifact;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core-positive-transfer")
}

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core-positive-transfer-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self(root);
        for name in ["print-run.mori", "print-run.target.json"] {
            fs::copy(fixtures().join(name), project.0.join(name)).unwrap();
        }
        project
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn print_run_reaches_public_compile_and_publishes_its_actual_bundle() {
    let project = Project::new();
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&project.0)
        .args([
            "--quiet",
            "--color",
            "never",
            "compile",
            "print-run.mori",
            "--target",
            "print-run.target.json",
            "--out",
            "compiled",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty(), "quiet compile has no status line");

    let out = project.0.join("compiled");
    let artifact: Artifact =
        serde_json::from_slice(&fs::read(out.join("artifact.json")).unwrap()).unwrap();
    let expected_target: Value = serde_json::from_slice(
        &fs::read(fixtures().join("print-run.target.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(serde_json::to_value(&artifact.target).unwrap(), expected_target);
    assert_eq!(artifact.format, "moriarty-zkir-artifact/3");
    assert_eq!(artifact.abi, "moriarty-public-abi/3");
    assert_eq!(artifact.relation, "moriarty-zkir-relation/3");
    assert_eq!(artifact.authorization_mode, "unsigned");
    assert_eq!(artifact.semantics, "moriarty-core/1");
    assert_eq!(artifact.circuits.len(), 1);
    let circuit = &artifact.circuits[0];
    assert_eq!(circuit.action, "pay");
    assert!(!circuit.zkir.is_empty());
    // Publication fidelity uses the actual compiler output, not an IR fixture.
    assert_eq!(
        fs::read(out.join("000-pay.zkir")).unwrap(),
        circuit.zkir.as_bytes()
    );
    let leaves = fs::read_dir(&out)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        leaves,
        BTreeSet::from([
            "000-pay.zkir".to_owned(),
            "artifact.json".to_owned(),
            "source-projection.json".to_owned(),
        ])
    );
    let projection: Value =
        serde_json::from_slice(&fs::read(out.join("source-projection.json")).unwrap()).unwrap();
    assert_eq!(
        projection["compilation"]["compiled_program_hash"],
        artifact.program_hash
    );
    assert_eq!(projection["compilation"]["projection"]["agreement"], "PrintRun");
    assert_eq!(
        projection["compilation"]["projection"]["evaluation_performed"],
        false
    );
}
