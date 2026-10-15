//! Public Core test reports with independently frozen normal financial results.
//! No evaluator/compiler imports or runtime evidence directory dependencies.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

// Independent expectation freeze: 3911ef8990bbcdf067f641bcae032f48828b2035ddae55f8687969ecc0cb2395.
// Each case transfers 25 from alice100 to bob0. Sixteen observer balances42
// and incoming_duty11 survive. Repeated cases start from independent inputs.
const SOURCE: &str = r#"language "moriarty/1";
agreement NormalReportTransfer {
  asset TEST = { network: "local", issuer: "independent-report-oracle", reference: "TEST", scale: 0 };
  action pay(amount: UInt) {
    requires amount > 0;
    transfer(TEST, "alice", "bob", amount);
  }
}
"#;
static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-core-test-report-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn write_json(&self, name: &str, value: &Value) {
        fs::write(self.0.join(name), serde_json::to_vec(value).unwrap()).unwrap();
    }

    fn normal(count: usize, mixed: bool) -> Self {
        let project = Self::new();
        fs::write(project.0.join("source.mori"), SOURCE).unwrap();
        let mut scenario = snapshot("100", "0");
        scenario["inputs"] = json!({"amount":{"type":"UInt","value":"25"}});
        project.write_json("scenario.json", &scenario);
        let execution = execution();
        let cases = (0..count)
            .map(|index| {
                let mut expected = json!({
                    "status":"accepted",
                    "post":execution["post"],
                    "effects":execution["effects"]
                });
                if mixed {
                    match index {
                        0 => expected["post"]["balances"][0]["amount"] = json!("74"),
                        127 => expected["effects"][0]["amount"] = json!("24"),
                        255 => {
                            expected = json!({"status":"rejected","code":"mori::core::requires"})
                        }
                        _ => {}
                    }
                }
                json!({
                    "name":format!("normal-transfer-{index:03}"),
                    "source":"source.mori",
                    "scenario":"scenario.json",
                    "action":"pay",
                    "expected":expected
                })
            })
            .collect::<Vec<_>>();
        project.write_json(
            "mori-tests.json",
            &json!({"profile":"moriarty-tests/1","cases":cases}),
        );
        project
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn asset() -> Value {
    json!({"network":"local","issuer":"independent-report-oracle","reference":"TEST","scale":0})
}

fn snapshot(alice: &str, bob: &str) -> Value {
    let mut balances = vec![
        json!({"account":"alice","asset":asset(),"amount":alice}),
        json!({"account":"bob","asset":asset(),"amount":bob}),
    ];
    balances.extend((0..16).map(
        |index| json!({"account":format!("observer-{index:02}"),"asset":asset(),"amount":"42"}),
    ));
    json!({
        "state":{},
        "balances":balances,
        "liabilities":[{"name":"incoming_duty","asset":asset(),"debtor":"observer-00","creditor":"bob","amount":"11"}]
    })
}

// Full literal execution derived independently before the product is run.
// Source expression order gives predicate index2 and transfer amount index3.
fn execution() -> Value {
    let pre = snapshot("100", "0");
    let post = snapshot("75", "25");
    let effects =
        json!([{"kind":"Transfer","asset":asset(),"from":"alice","to":"bob","amount":"25"}]);
    json!({
        "semantics":"moriarty-core/1",
        "action":"pay",
        "pre":pre,
        "post":post,
        "steps":[
            {"index":0,"statement":{"kind":"Requires","predicate":2},"value":{"type":"Bool","value":true},"snapshot":pre,"effects":[]},
            {"index":1,"statement":{"kind":"Transfer","asset":"TEST","from":"alice","to":"bob","amount":3},"value":{"type":"UInt","value":"25"},"snapshot":post,"effects":effects}
        ],
        "effects":effects
    })
}

fn expected_report(count: usize, mixed: bool) -> Value {
    let actual = json!({"Ok":execution()});
    let cases = (0..count)
        .map(|index| {
            json!({
                "name":format!("normal-transfer-{index:03}"),
                "passed":!(mixed && [0,127,255].contains(&index)),
                "actual":actual
            })
        })
        .collect::<Vec<_>>();
    json!({"status":if mixed {"failed"} else {"passed"},"cases":cases})
}

fn run(project: &Project) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&project.0)
        .output()
        .unwrap()
}

fn check_output(output: &Output, exit: i32, passed: usize, failed: usize) -> Value {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(exit), "{stderr}");
    assert!(
        stderr.contains(&format!("{passed} passed; {failed} failed")),
        "{stderr}"
    );
    assert!(output.stdout.ends_with(b"\n"));
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!("expected one complete JSON report: {error}; stderr={stderr}")
    })
}

#[test]
fn small_report_preserves_legacy_pretty_bytes() {
    let expected = expected_report(1, false);
    let mut bytes = serde_json::to_vec_pretty(&expected).unwrap();
    bytes.push(b'\n');
    // Pin to the independent small stdout oracle, including its final newline.
    assert_eq!(bytes.len(), 28_093);
    assert_eq!(
        Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        "b57043f1d8fd55e80cfde729ef8e85f78225ff29129bc8da6696c7f1163e5118"
    );
    let output = run(&Project::normal(1, false));
    assert_eq!(check_output(&output, 0, 1, 0), expected);
    assert_eq!(output.stdout, bytes);
}

#[test]
fn large_report_retains_all_256_full_executions() {
    let output = run(&Project::normal(256, false));
    let report = check_output(&output, 0, 256, 0);
    assert!(output.stdout.len() > 1_048_576);
    assert_eq!(report, expected_report(256, false));
}

#[test]
fn mixed_report_retains_results_after_all_three_expectation_mismatches() {
    let output = run(&Project::normal(256, true));
    let report = check_output(&output, 1, 253, 3);
    assert!(output.stdout.len() > 1_048_576);
    assert_eq!(report, expected_report(256, true));
}

#[test]
fn fee_pool_positive_aggregate_retains_frozen_financial_outcomes() {
    let example = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/core/fee-pool")
        .canonicalize()
        .unwrap();
    let frozen: Value =
        serde_json::from_slice(&fs::read(example.join("cases/expected.json")).unwrap()).unwrap();
    let mut cases = frozen["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["expected"]["status"] == "accepted")
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(cases.len(), 87);
    for case in &mut cases {
        for field in ["source", "scenario"] {
            case[field] = json!(example.join(case[field].as_str().unwrap()));
        }
    }
    let project = Project::new();
    project.write_json(
        "mori-tests.json",
        &json!({"profile":"moriarty-tests/1","cases":cases}),
    );
    let output = run(&project);
    let report = check_output(&output, 0, 87, 0);
    assert!(output.stdout.len() > 1_048_576);
    assert_eq!(report["status"], "passed");
    let actual = report["cases"].as_array().unwrap();
    assert_eq!(actual.len(), cases.len());
    for (report, case) in actual.iter().zip(&cases) {
        assert_eq!(report["name"], case["name"]);
        assert_eq!(report["passed"], true);
        let execution = &report["actual"]["Ok"];
        assert_eq!(execution["semantics"], "moriarty-core/1");
        assert_eq!(execution["action"], case["action"]);
        assert_eq!(
            execution["post"], case["expected"]["post"],
            "{}",
            case["name"]
        );
        assert_eq!(
            execution["effects"], case["expected"]["effects"],
            "{}",
            case["name"]
        );
        assert!(execution["pre"].is_object());
        assert!(!execution["steps"].as_array().unwrap().is_empty());
    }
}

const HOSTILE: &str = "x\n}\"\\\r\u{1b}[31m\u{7f}\u{85}\u{202e}\u{1f9ea}";

fn rename(project: &Project, index: usize, name: Value) {
    let path = project.0.join("mori-tests.json");
    let mut manifest: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    manifest["cases"][index]["name"] = name;
    project.write_json("mori-tests.json", &manifest);
}

#[test]
fn large_report_escapes_hostile_case_name_once() {
    let project = Project::normal(256, false);
    rename(&project, 128, json!(HOSTILE));
    let output = run(&project);
    let report = check_output(&output, 0, 256, 0);
    assert!(output.stdout.len() > 1_048_576);
    assert!(
        output
            .stdout
            .iter()
            .all(|&b| b == b'\n' || (b.is_ascii() && !b.is_ascii_control()))
    );
    let mut expected = expected_report(256, false);
    expected["cases"][128]["name"] = json!(HOSTILE);
    assert_eq!(report, expected);
}

#[test]
fn unexportable_final_case_publishes_no_partial_report() {
    // 350,000 raw UTF-8 bytes keep the manifest under 1 MiB; each U+0085
    // exports as six ASCII bytes, so only the final case exceeds its cap.
    let project = Project::normal(128, false);
    rename(&project, 127, json!("\u{85}".repeat(175_000)));
    let output = run(&project);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("128 passed; 0 failed"), "{stderr}");
    assert!(stderr.contains("mori::core::json"), "{stderr}");
    assert!(!stderr.contains('\u{85}'));
    assert!(output.stdout.is_empty());
}
