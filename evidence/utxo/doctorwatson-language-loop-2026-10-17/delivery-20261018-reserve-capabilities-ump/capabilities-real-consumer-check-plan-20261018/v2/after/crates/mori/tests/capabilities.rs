//! Public capability consumers only; no compiler, target, SDK or native operation.
use serde_json::Value;
use std::{
    collections::BTreeSet,
    ffi::OsString,
    fs,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "mori-capabilities-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        Self(dir)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .env("PATH", "")
            .stdin(Stdio::null())
            .args(["--color", "never"])
            .args(args)
            .output()
            .unwrap()
    }
    fn snapshot(&self) -> Vec<(OsString, Vec<u8>)> {
        let mut files: Vec<_> = fs::read_dir(&self.0)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                (entry.file_name(), fs::read(entry.path()).unwrap())
            })
            .collect();
        files.sort();
        files
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn json(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "exit {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn row<'a>(report: &'a Value, section: &str, id: &str) -> &'a Value {
    report[section]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == id)
        .unwrap()
}

#[test]
fn public_query_distinguishes_profiles_build_consumers_and_unchecked_targets() {
    let f = Files::new();
    fs::write(f.0.join("user-note.txt"), "user content\n").unwrap();
    let before = f.snapshot();
    let output = f.run(&["capabilities", "--json"]);
    let report = json(&output);
    assert_eq!(before, f.snapshot());
    assert!(output.stdout.ends_with(b"\n"));
    assert!(output.stdout.len() < 65_536);
    for args in [vec!["capabilities"], vec!["--quiet", "capabilities", "--json"]] {
        let next = f.run(&args);
        assert_eq!(json(&next), report);
        assert_eq!(next.stdout, output.stdout);
        assert_eq!(before, f.snapshot());
    }
    let fields: BTreeSet<_> = report.as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(fields, BTreeSet::from([
        "format", "producer", "catalogue", "profiles", "consumers", "targets",
        "otherExecutables", "limitations"
    ]));
    assert_eq!(report["format"], "moriarty-cli-capabilities/1");
    assert_eq!(report["producer"]["implementation"], "rust-workspace-cli");
    assert_eq!(report["producer"]["packageVersion"], env!("CARGO_PKG_VERSION"));
    assert!(report["producer"]["binarySourceRevision"].is_null());
    assert_eq!(report["catalogue"]["runtimeEvidence"], "not-included");
    for (name, enabled) in [
        ("native", cfg!(feature = "native")),
        ("source6-expansion", cfg!(feature = "source6-expansion")),
        ("core2-disjoint-draft", cfg!(feature = "core2-disjoint-draft")),
    ] {
        assert_eq!(report["producer"]["buildFeatures"][name], enabled);
    }
    for section in ["profiles", "consumers", "targets", "otherExecutables"] {
        let values = report[section].as_array().unwrap();
        let ids: BTreeSet<_> = values.iter().map(|value| value["id"].as_str().unwrap()).collect();
        assert_eq!(ids.len(), values.len());
    }
    assert_eq!(report["profiles"].as_array().unwrap().len(), 3);
    assert_eq!(row(&report, "profiles", "rust-beta-frontend")["sourceProfile"], "moriarty-beta/1");
    assert!(row(&report, "profiles", "rust-beta-frontend")["semanticsProfile"].is_null());
    let transfer = row(&report, "consumers", "beta-to-source6-transfer");
    assert_eq!(transfer["inputProfile"], "moriarty-beta/1");
    assert_eq!(transfer["outputProfile"], "Source/6");
    assert_eq!(transfer["availableInBuild"], cfg!(feature = "source6-expansion"));
    assert_eq!(row(&report, "consumers", "core2-disjoint-draft")["availableInBuild"], cfg!(feature = "core2-disjoint-draft"));
    assert_eq!(row(&report, "consumers", "core2-reference")["availableInBuild"], true);
    assert!(row(&report, "consumers", "core2-reference")["outputProfile"].is_null());
    for (id, artifact) in [("core1-compile", "moriarty-zkir-artifact/3"), ("core2-compile", "moriarty-zkir-artifact/4")] {
        let compiler = row(&report, "consumers", id);
        assert_eq!(compiler["availableInBuild"], cfg!(feature = "native"));
        assert_eq!(compiler["outputProfile"], artifact);
    }
    let targeted = row(&report, "consumers", "core2-target-inspect");
    assert_eq!(targeted["implementation"], "unsupported");
    assert_eq!(targeted["availableInBuild"], false);
    for id in ["rust-lsp", "rust-mcp"] {
        let editor = row(&report, "consumers", id);
        assert_eq!(editor["implementation"], "stub");
        assert_eq!(editor["availableInBuild"], false);
    }
    for target in report["targets"].as_array().unwrap() {
        assert!(target["configuredTarget"].is_null());
        assert_eq!(target["qualification"], "not-checked");
        for id in target["consumerIds"].as_array().unwrap() {
            row(&report, "consumers", id.as_str().unwrap());
        }
    }
    assert_eq!(row(&report, "otherExecutables", "node-beta-cli")["availability"], "not-inspected");
}

#[test]
fn reported_core_profiles_select_real_source_only_cli_consumers() {
    let f = Files::new();
    let report = json(&f.run(&["capabilities", "--json"]));
    for (id, language, semantics, body) in [
        ("rust-core1", "moriarty/1", "moriarty-core/1", "next count = count + 1;"),
        ("rust-core2", "moriarty/2", "moriarty-core/2", "await review(ok: Bool) when ok; next count = count + 1;"),
    ] {
        let profile = row(&report, "profiles", id);
        assert_eq!(profile["sourceProfile"], language);
        assert_eq!(profile["semanticsProfile"], semantics);
        let source = format!(
            "{}\nagreement CapabilityProbe {{ state count: UInt = 0; action run() {{ {body} }} }}\n",
            profile["sourceHeader"].as_str().unwrap()
        );
        fs::write(f.0.join("source.mori"), source).unwrap();
        let before = f.snapshot();
        let checked = json(&f.run(&["check", "source.mori", "--json"]));
        assert_eq!(checked["status"], "accepted");
        assert_eq!(checked["language"], language);
        let inspected = json(&f.run(&["inspect", "source.mori"]));
        assert_eq!(inspected["status"], "accepted");
        assert_eq!(inspected["semantics"], semantics);
        assert!(inspected["enforcement_report"].is_null());
        if language == "moriarty/2" {
            assert!(row(&report, "consumers", "core2-inspect")["outputProfile"].is_null());
            assert!(inspected.get("format").is_none());
            assert_eq!(inspected["stage_plan"]["format"], "moriarty-stage-plan/1");
        } else {
            assert_eq!(inspected["format"], "moriarty-enforcement-report/1");
        }
        assert_eq!(before, f.snapshot());
    }
}
