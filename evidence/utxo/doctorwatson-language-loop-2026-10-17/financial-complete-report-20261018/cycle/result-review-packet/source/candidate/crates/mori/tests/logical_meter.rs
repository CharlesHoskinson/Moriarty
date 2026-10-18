//! Actual mori simulate command; no native, trace generation or network execution.
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
const SOURCE: &str = r#"language "moriarty/1"; agreement Counter {
state count: UInt = 0;
action run(amount: UInt) {
let total = count + amount;
requires total <= 100;
next count = total;
ensures count == total;
emit "count"(count);
} }"#;
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-logical-meter-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        let f = Self(path);
        f.write("source.mori", SOURCE);
        f.write("scenario.json", r#"{"state":{},"inputs":{"amount":{"type":"UInt","value":"7"}},"balances":[],"liabilities":[]}"#);
        f
    }
    fn write(&self, name: &str, bytes: &str) {
        std::fs::write(self.0.join(name), bytes).unwrap();
    }
    fn command(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_mori"));
        cmd.args(["--color", "never", "simulate"])
            .arg(self.0.join("source.mori"))
            .args(["--action", "run", "--scenario"])
            .arg(self.0.join("scenario.json"));
        cmd
    }
    fn run(&self, cap: Option<&str>) -> Output {
        let mut cmd = self.command();
        if let Some(cap) = cap {
            cmd.arg(format!("--logical-cap={cap}"));
        }
        cmd.output().unwrap()
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn decoded(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap()
}
fn cost(value: &Value, cap: &str, used: &str) {
    assert_eq!(value["profile"], "moriarty-reference-logical-cost/1");
    assert_eq!(value["logical"], json!({"cap":cap,"used":used}));
    for field in [
        "dust",
        "service",
        "external",
        "native",
        "sponsor",
        "signed_budget",
    ] {
        assert!(value.get(field).is_none());
    }
}
#[test]
fn existing_simulate_bytes_and_execution_stay_unchanged_with_optional_cap() {
    let f = Files::new();
    let ordinary = f.run(None);
    assert!(ordinary.status.success());
    assert!(ordinary.stderr.is_empty());
    let expected = decoded(&ordinary);
    assert_eq!(expected["status"], "accepted");
    assert_eq!(expected["qualification"], "local-reference-execution");
    assert_eq!(expected.as_object().unwrap().len(), 3);
    let program = mori_core::parse(SOURCE).unwrap();
    let scenario =
        serde_json::from_str(&std::fs::read_to_string(f.0.join("scenario.json")).unwrap()).unwrap();
    let execution = mori_core::simulate(&program, "run", &scenario).unwrap();
    let legacy = json!({"status":"accepted","qualification":"local-reference-execution","execution":execution});
    assert_eq!(
        ordinary.stdout,
        format!("{}\n", mori_core::json::export(&legacy).unwrap()).into_bytes()
    );
    for cap in ["16", "340282366920938463463374607431768211455"] {
        let output = f.run(Some(cap));
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        let metered = decoded(&output);
        cost(&metered, cap, "16");
        assert_eq!(metered["execution"], expected["execution"]);
        assert_eq!(metered["qualification"], "local-reference-execution");
    }
    assert_eq!(f.run(None).stdout, ordinary.stdout);
}
#[test]
fn zero_and_late_refusals_report_consumed_work_without_partial_execution() {
    let f = Files::new();
    for (cap, step) in [("0", 0), ("15", 4)] {
        let output = f.run(Some(cap));
        assert!(!output.status.success());
        assert!(output.stderr.is_empty());
        let rejected = decoded(&output);
        cost(&rejected, cap, cap);
        assert_eq!(rejected["status"], "rejected");
        assert_eq!(rejected["error"]["code"], "mori::core::meter");
        assert_eq!(rejected["error"]["step"], step);
        for field in ["execution", "post", "effects"] {
            assert!(rejected.get(field).is_none());
        }
    }
    assert!(f.run(Some("16")).status.success());
}
#[test]
fn input_refusal_reports_zero_and_legacy_rejection_bytes_are_preserved() {
    let f = Files::new();
    f.write("scenario.json", r#"{"state":{},"inputs":{"amount":{"type":"Bool","value":true}},"balances":[],"liabilities":[]}"#);
    let ordinary = f.run(None);
    assert!(!ordinary.status.success());
    let expected = decoded(&ordinary);
    assert_eq!(expected.as_object().unwrap().len(), 2);
    assert_eq!(
        ordinary.stdout,
        format!("{}\n", mori_core::json::export(&expected).unwrap()).into_bytes()
    );
    let metered = f.run(Some("0"));
    assert!(!metered.status.success());
    let rejected = decoded(&metered);
    cost(&rejected, "0", "0");
    assert_eq!(rejected["error"], expected["error"]);
}
#[test]
fn canonical_uint_caps_and_trace_conflict_refuse_before_any_file_read() {
    let f = Files::new();
    std::fs::remove_file(f.0.join("source.mori")).unwrap();
    for cap in [
        "",
        "-1",
        "+1",
        "00",
        "01",
        "1_0",
        "1.0",
        " 1",
        "1 ",
        "１２",
        "340282366920938463463374607431768211456",
    ] {
        let output = f.run(Some(cap));
        assert!(!output.status.success(), "cap {cap:?}");
        assert!(output.stdout.is_empty());
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains("--logical-cap"), "{error}");
        assert!(!error.contains("No such file"), "{error}");
    }
    let output = f
        .command()
        .args(["--logical-cap", "16", "--trace", "--target"])
        .arg(f.0.join("missing-target.json"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("--logical-cap") && error.contains("--trace"),
        "{error}"
    );
    assert!(!error.contains("No such file"), "{error}");
}
