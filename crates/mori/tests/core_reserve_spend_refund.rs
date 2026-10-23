//! Ordinary Core/1 financial prefixes from an independently frozen source case.
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/reserve-spend-refund")
}

fn json_file(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

// This selects observable fields; it never computes a financial transition.
// Complete statement/ExprId correspondence remains a separate oracle boundary.
fn financial_execution(execution: &Value) -> Value {
    let steps = execution["steps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|step| {
            json!({
                "index": step["index"],
                "value": step["value"],
                "snapshot": step["snapshot"],
                "effects": step["effects"],
            })
        })
        .collect::<Vec<_>>();
    json!({
        "semantics": execution["semantics"],
        "action": execution["action"],
        "pre": execution["pre"],
        "post": execution["post"],
        "steps": steps,
        "effects": execution["effects"],
    })
}

fn assert_financial_prefixes(execution: &Value) {
    let root = fixtures();
    assert_eq!(
        financial_execution(execution),
        json_file(&root.join("financial-execution-projection.expected.json")),
        "every typed result, changed and preserved row, and new effect delta"
    );
    let kinds = execution["steps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|step| step["statement"]["kind"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        Value::Array(kinds),
        json_file(&root.join("source-statement-kinds.expected.json")),
        "all source statement kinds, without inventing expression roots"
    );
    let mut prefix = Vec::new();
    let prefixes = execution["steps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|step| {
            prefix.extend(step["effects"].as_array().unwrap().iter().cloned());
            json!({
                "index": step["index"],
                "effects": prefix.clone(),
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(
        Value::Array(prefixes),
        json_file(&root.join("cumulative-prefix-effects.expected.json")),
        "all ordered effect prefixes against independent literals"
    );
}

#[test]
fn reserve_library_keeps_complete_financial_prefixes() {
    let root = fixtures();
    let source = fs::read_to_string(root.join("reserve-spend-refund.mori")).unwrap();
    let program = mori_core::parse(&source).expect("the ordinary source must parse");
    let scenario =
        serde_json::from_slice(&fs::read(root.join("opening.scenario.json")).unwrap()).unwrap();
    let execution = mori_core::simulate(&program, "run", &scenario)
        .expect("the positive reference call must complete");
    assert_financial_prefixes(&serde_json::to_value(execution).unwrap());
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
    temporary: bool,
}

impl Project {
    fn new() -> Self {
        let (root, temporary) = match std::env::var_os("MORI_CORE_REFUND_OBSERVATIONS") {
            Some(path) => {
                let path = PathBuf::from(path);
                assert!(path.is_absolute(), "observation path must be absolute");
                (path, false)
            }
            None => (
                std::env::temp_dir().join(format!(
                    "mori-reserve-refund-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                )),
                true,
            ),
        };
        fs::create_dir(&root).expect("use a fresh observation directory");
        for name in ["reserve-spend-refund.mori", "opening.scenario.json"] {
            fs::copy(fixtures().join(name), root.join(name)).unwrap();
        }
        Self { root, temporary }
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        if self.temporary && !std::thread::panicking() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

#[test]
fn reserve_public_cli_keeps_complete_financial_prefixes() {
    let project = Project::new();
    let args = [
        "--color",
        "never",
        "simulate",
        "reserve-spend-refund.mori",
        "--action",
        "run",
        "--scenario",
        "opening.scenario.json",
    ];
    let captured = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&project.root)
        .args(args)
        .output();
    if let Err(error) = &captured {
        fs::write(
            project.root.join("process.json"),
            serde_json::to_vec_pretty(&json!({
                "executable": env!("CARGO_BIN_EXE_mori"),
                "args": args,
                "launchError": error.to_string(),
            }))
            .unwrap(),
        )
        .unwrap();
    }
    let output = captured.expect("the reviewed public binary must launch");
    // Save complete actual process bytes and outcome before result assertions.
    fs::write(project.root.join("stdout.json"), &output.stdout).unwrap();
    fs::write(project.root.join("stderr.txt"), &output.stderr).unwrap();
    fs::write(
        project.root.join("process.json"),
        serde_json::to_vec_pretty(&json!({
            "executable": env!("CARGO_BIN_EXE_mori"),
            "args": args,
            "status": output.status.code(),
            "success": output.status.success(),
        }))
        .unwrap(),
    )
    .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let mut keys = report
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    keys.sort_unstable();
    assert_eq!(keys, ["execution", "qualification", "status"]);
    assert_eq!(report["status"], "accepted");
    assert_eq!(report["qualification"], "local-reference-execution");
    assert_financial_prefixes(&report["execution"]);
}
