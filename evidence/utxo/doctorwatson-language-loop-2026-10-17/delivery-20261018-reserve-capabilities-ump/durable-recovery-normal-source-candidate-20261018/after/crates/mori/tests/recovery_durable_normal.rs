//! Complete positive local recovery observations through fresh public processes.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const IDENTITIES: [(&str, &str); 4] = [
    ("{{OWNER_BASE_PROGRAM_ID}}", "base_program"),
    ("{{OWNER_BASE_PLAN_ID}}", "base_plan"),
    ("{{OWNER_RECOVERY_MODULE_ID}}", "recovery_module"),
    ("{{OWNER_RECOVERY_PLAN_ID}}", "recovery_plan"),
];

fn fixtures(case: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/recovery-durable-normal")
        .join(case)
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

/// Fill identity slots only. All economic, source, plan, control, history and
/// accounting fields remain literal expectations. This checks identity
/// consistency; canonical identity conformance requires its own frozen oracle.
fn expected(case: &str, name: &str, bindings: &Value) -> Value {
    fn fill(value: &mut Value, bindings: &Value) {
        match value {
            Value::String(text) => {
                if let Some((_, key)) = IDENTITIES.iter().find(|(slot, _)| text.as_str() == *slot) {
                    *value = bindings[*key].clone();
                }
            }
            Value::Array(values) => values.iter_mut().for_each(|v| fill(v, bindings)),
            Value::Object(values) => values.values_mut().for_each(|v| fill(v, bindings)),
            _ => {}
        }
    }
    let mut value = read_json(&fixtures(case).join(name));
    fill(&mut value, bindings);
    value
}

struct Project {
    root: PathBuf,
    operation: usize,
}

impl Project {
    fn new(case: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-recovery-durable-normal-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("scratch")).unwrap();
        fs::create_dir(root.join("observations")).unwrap();
        for name in [
            "base.mori",
            "recovery.mori",
            "start.scenario.json",
            "normal-false.event.json",
            "recover.event.json",
        ] {
            fs::copy(fixtures(case).join(name), root.join(name)).unwrap();
        }
        Self { root, operation: 0 }
    }

    /// Each call launches and waits for a separate public executable. Expected
    /// checkpoints are never written into the operation's input directory.
    fn invoke(&mut self, command: &str, args: &[&str]) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args(["--color", "never", command, "base.mori"])
            .args(["--recovery-module", "recovery.mori"])
            .args(args)
            .output()
            .unwrap();
        let stem = self.root.join("observations").join(self.operation.to_string());
        self.operation += 1;
        fs::write(stem.with_extension("stdout"), &output.stdout).unwrap();
        fs::write(stem.with_extension("stderr"), &output.stderr).unwrap();
        fs::write(stem.with_extension("status"), output.status.to_string()).unwrap();
        assert_eq!(
            output.status.code(), Some(0),
            "observations retained at {}: {}", self.root.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty(), "{}", self.root.display());
        assert!(output.stdout.is_ascii(), "{}", self.root.display());
        assert_eq!(output.stdout.last(), Some(&b'\n'));
        serde_json::from_slice(&output.stdout).unwrap()
    }

    fn checkpoint(&self, name: &str) -> Value {
        read_json(&self.root.join("scratch").join(name))
    }

    fn reopen(&mut self, name: &str) -> Value {
        let relative = format!("scratch/{name}");
        let path = self.root.join(&relative);
        let before = fs::read(&path).unwrap();
        let report = self.invoke("simulate", &["--reopen", &relative]);
        assert_eq!(fs::read(&path).unwrap(), before, "reopen is read-only");
        report
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

fn journey(case: &str, action: &str, observe_false: bool) {
    let mut project = Project::new(case);
    let inspected = project.invoke("inspect", &[]);
    let start = project.invoke("simulate", &[
        "--action", action, "--scenario", "start.scenario.json",
        "--logical-cap", "13", "--checkpoint-out", "scratch/pending.json",
    ]);
    let bindings = start["checkpoint"]["bindings"].clone();
    for (slot, key) in IDENTITIES {
        let identity = bindings[key].as_str().expect("paired identity string");
        assert!(!identity.is_empty());
        assert_ne!(identity, slot, "documentary marker is not a concrete identity");
    }
    assert_eq!(inspected, expected(case, "program.inspect.example.json", &bindings));
    assert_eq!(start, expected(case, "start.report.example.json", &bindings));
    let pending_bytes = fs::read(project.root.join("scratch/pending.json")).unwrap();
    assert_eq!(project.checkpoint("pending.json"), expected(
        case, "pending.start.checkpoint.example.json", &bindings,
    ));
    assert_eq!(project.checkpoint("pending.json"), start["checkpoint"]);
    assert_eq!(project.reopen("pending.json"), expected(
        case, "reopen.report.example.json", &bindings,
    ));

    // Each direct/observed journey has a separate start and separate files.
    // No returned pending checkpoint is consumed along two recovery paths.
    let (input, report_fixture, checkpoint_fixture, reopened_fixture, attempted) = if observe_false {
        let observed = project.invoke("simulate", &[
            "--resume", "scratch/pending.json", "--event", "normal-false.event.json",
            "--checkpoint-out", "scratch/observed.json",
        ]);
        assert_eq!(observed, expected(case, "resume-normal-false.report.example.json", &bindings));
        assert_eq!(project.checkpoint("observed.json"), expected(
            case, "pending.after-normal-false.checkpoint.example.json", &bindings,
        ));
        assert_eq!(project.checkpoint("observed.json"), observed["checkpoint"]);
        (
            "scratch/observed.json", "recover-after-normal-false.report.example.json",
            "closed.after-normal-false.checkpoint.example.json",
            "reopen-closed-after-normal-false.report.example.json", "15",
        )
    } else {
        (
            "scratch/pending.json", "recover-direct.report.example.json",
            "closed.direct.checkpoint.example.json", "reopen-closed.report.example.json", "13",
        )
    };
    let closed = project.invoke("simulate", &[
        "--recover", input, "--recovery-event", "recover.event.json",
        "--checkpoint-out", "scratch/closed.json",
    ]);
    assert_eq!(closed, expected(case, report_fixture, &bindings));
    assert_eq!(project.checkpoint("closed.json"), expected(case, checkpoint_fixture, &bindings));
    assert_eq!(project.checkpoint("closed.json"), closed["checkpoint"]);
    assert_eq!(project.reopen("closed.json"), expected(case, reopened_fixture, &bindings));
    assert_eq!(fs::read(project.root.join("scratch/pending.json")).unwrap(), pending_bytes);
    assert_eq!(closed["checkpoint"]["accounting"]["committed"], "13");
    assert_eq!(closed["checkpoint"]["accounting"]["attempted_total"], attempted);
}

#[test]
fn controlled_refund_preserves_prefix_fees_history_and_unrelated_duty_after_reopen() {
    journey("custody-refund", "fund", false);
    journey("custody-refund", "fund", true);
}

#[test]
fn forward_compensation_keeps_supplier_payment_and_residual_eight_after_reopen() {
    journey("forward-compensation", "purchase", false);
    journey("forward-compensation", "purchase", true);
}
