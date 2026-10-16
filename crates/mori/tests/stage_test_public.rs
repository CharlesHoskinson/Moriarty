//! Ordinary public staged-test consumers against blinded independent R3 oracle copies.
//! Current reports and canonical preimages were frozen after implementation existed.
//! Oracle independence rests on blinding, not preimplementation timing.
//! No evaluator/compiler/backend import; all runtime comparisons remain UNRUN.
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-stage-test-public-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/core-stage-tests");
        for file in fs::read_dir(fixtures).unwrap() {
            let file = file.unwrap();
            if !file.file_type().unwrap().is_file() {
                continue;
            }
            fs::copy(file.path(), path.join(file.file_name())).unwrap();
        }
        Self(path)
    }

    fn read(&self, name: &str) -> Value {
        serde_json::from_slice(&fs::read(self.0.join(name)).unwrap()).unwrap()
    }

    fn write(&self, name: &str, value: &Value) {
        fs::write(self.0.join(name), serde_json::to_vec_pretty(value).unwrap()).unwrap();
    }

    fn run(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .args(["--color", "never", "test"])
            .arg(&self.0)
            .output()
            .unwrap()
    }

    fn expected(&self) -> Value {
        let manifest = self.read("mori-tests.json");
        let cases: Vec<_> = manifest["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| {
                let mut stages = vec![json!({
                    "index":0, "passed":true,
                    "actual":self.read(case["expected"].as_str().unwrap())
                })];
                for (index, event) in case["events"].as_array().unwrap().iter().enumerate() {
                    stages.push(json!({
                        "index":index+1, "passed":true,
                        "actual":self.read(event["expected"].as_str().unwrap())
                    }));
                }
                json!({"name":case["name"],"passed":true,"stages":stages})
            })
            .collect();
        json!({"status":"passed","cases":cases})
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn report(output: &Output, exit: i32) -> Value {
    assert_eq!(
        output.status.code(),
        Some(exit),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("{error}: {}", String::from_utf8_lossy(&output.stderr)))
}

#[test]
fn full_reports_match_two_await_delivery_and_contrasting_captured_fee() {
    let project = Project::new();
    let expected = project.expected();
    let output = project.run();
    assert_eq!(report(&output, 0), expected);
    assert!(String::from_utf8_lossy(&output.stderr).contains("2 passed; 0 failed"));
    // Pin complete public serialization, including all statement IDs and labels.
    let mut bytes = serde_json::to_vec_pretty(&expected).unwrap();
    bytes.push(b'\n');
    assert_eq!(output.stdout, bytes);
}

#[test]
fn expected_report_transcription_mismatches_return_failure_and_retain_later_results() {
    // These change only expected observations, never source, scenarios, events
    // or actual continuations. Each models an ordinary wrong test expectation.
    for (file, pointer, wrong) in [
        ("two-deliveries-start.json", "/attempted", json!("4")),
        ("two-deliveries-start.json", "/committed", json!("4")),
        (
            "two-deliveries-start.json",
            "/continuation/boundary/state/phase/value",
            json!("2"),
        ),
        (
            "two-deliveries-start.json",
            "/segment/frame_post/captures",
            json!([{"name":"unused","value":{"type":"UInt","value":"1"}}]),
        ),
        (
            "two-deliveries-start.json",
            "/segment/execution/effects/0/amount",
            json!("6"),
        ),
        (
            "two-deliveries-start.json",
            "/residual/0/amount",
            json!("4"),
        ),
        ("two-deliveries-observed.json", "/segment", json!({})),
        (
            "quoted-fee-complete.json",
            "/segment/execution/steps/1/statement/amount",
            json!(3),
        ),
    ] {
        let project = Project::new();
        let mut expected_report = project.expected();
        let mut expected_stage = project.read(file);
        *expected_stage.pointer_mut(pointer).unwrap() = wrong;
        project.write(file, &expected_stage);
        let (case, stage) = match file {
            "two-deliveries-start.json" => (0, 0),
            "two-deliveries-observed.json" => (0, 1),
            "quoted-fee-complete.json" => (1, 1),
            _ => unreachable!(),
        };
        expected_report["status"] = json!("failed");
        expected_report["cases"][case]["passed"] = json!(false);
        expected_report["cases"][case]["stages"][stage]["passed"] = json!(false);
        let output = project.run();
        assert_eq!(report(&output, 1), expected_report, "{file} {pointer}");
        assert!(String::from_utf8_lossy(&output.stderr).contains("1 passed; 1 failed"));
    }
}
