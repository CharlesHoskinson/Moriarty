//! Public processes exercise the source checker and actual staged evaluator.
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-core3-public-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn copy(&self, fixture: &str, name: &str) {
        fs::copy(fixtures().join(fixture).join(name), self.0.join(name)).unwrap();
    }
    fn write(&self, name: &str, value: &Value) {
        fs::write(self.0.join(name), serde_json::to_vec(value).unwrap()).unwrap();
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
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures")
}
fn report(output: &Output) -> Value {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.ends_with(b"\n"));
    serde_json::from_slice(&output.stdout).unwrap()
}
fn keys(value: &Value) -> BTreeSet<&str> {
    value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect()
}
fn labels(actual: &Value) {
    assert_eq!(actual["qualification"], "local-reference-stage");
    assert_eq!(actual["authentication"], "none");
    assert_eq!(actual["consumption"], "untracked");
}

#[test]
fn both_declared_duty_sources_reach_check_and_complete_source_inspect() {
    let p = Project::new();
    let inputs: Value = serde_json::from_str(include_str!(
        "../../fixtures/core3-positive-prefix/SOURCE-INPUTS.json"
    ))
    .unwrap();
    for (source, expressions, statements) in [
        (
            "P1-RECONSTRUCTED-INPUT.mori",
            json!([
                {"kind":"Literal","value":{"type":"UInt","value":"3"}}
            ]),
            json!([{"kind":"Discharge","name":"loan","amount":0}]),
        ),
        (
            "P2-RECONSTRUCTED-INPUT.mori",
            json!([
                {"kind":"Name","name":"go"},
                {"kind":"Literal","value":{"type":"UInt","value":"3"}}
            ]),
            json!([
                {"kind":"Await","label":"ready","binders":[{"name":"go","ty":"Bool"}],"guard":0},
                {"kind":"Discharge","name":"loan","amount":1}
            ]),
        ),
    ] {
        p.copy("core3-positive-prefix", source);
        let checked = report(&p.run(&["check", source, "--json"]));
        assert_eq!(checked["status"], "accepted");
        assert_eq!(checked["language"], "moriarty/3");
        assert_eq!(checked["semantics"], "moriarty-core/3");
        assert_eq!(checked["diagnostics"], json!([]));
        let actions = json!([{"name":"pay","inputs":[],"statements":statements}]);
        assert_eq!(checked["actions"], actions);
        labels(&checked);
        let inspected = report(&p.run(&["inspect", source]));
        assert_eq!(
            keys(&inspected),
            BTreeSet::from([
                "status",
                "language",
                "semantics",
                "agreement",
                "states",
                "assets",
                "duties",
                "actions",
                "expressions",
                "stage_plan",
                "enforcement_report",
                "qualification",
                "authentication",
                "consumption"
            ])
        );
        assert_eq!(inspected["status"], "accepted");
        assert_eq!(inspected["language"], "moriarty/3");
        assert_eq!(inspected["semantics"], "moriarty-core/3");
        assert_eq!(inspected["agreement"], "P");
        assert_eq!(inspected["states"], json!([]));
        assert_eq!(inspected["assets"], json!([inputs["asset"].clone()]));
        assert_eq!(inspected["duties"], json!([inputs["source_duty"].clone()]));
        assert_eq!(inspected["actions"], actions);
        assert_eq!(inspected["expressions"], expressions);
        assert_eq!(inspected["stage_plan"]["format"], "moriarty-stage-plan/1");
        assert_eq!(inspected["enforcement_report"], Value::Null);
        labels(&inspected);
    }
}

#[test]
fn empty_core3_action_preserves_complete_report_and_literal_public_bytes() {
    let p = Project::new();
    p.copy("core3-public-consumers", "empty.mori");
    p.copy("core3-public-consumers", "empty-scenario.json");
    let output = p.run(&[
        "simulate",
        "empty.mori",
        "--action",
        "run",
        "--scenario",
        "empty-scenario.json",
    ]);
    let expected =
        include_bytes!("../../fixtures/core3-public-consumers/empty-complete.expected.json")
            .as_slice();
    assert_eq!(
        report(&output),
        serde_json::from_slice::<Value>(expected).unwrap()
    );
    assert_eq!(output.stdout, expected);
}

// Only the original oracle's stated financial/typed-value/work fields are
// compared here. Unknown whole-wire/hash fields are not filled or inferred.
fn compare_operation(actual: &Value, expected: &Value) {
    labels(actual);
    assert_eq!(actual["status"], expected["semantic_outcome"]);
    assert_eq!(actual["attempted"], expected["logical"]["attempted"]);
    assert_eq!(actual["committed"], expected["logical"]["committed_after"]);
    assert_eq!(actual["residual"], expected["positive_residual"]);
    if expected["segment_exists"] == false {
        assert_eq!(actual["segment"], Value::Null);
        return;
    }
    let segment = &actual["segment"];
    assert_eq!(segment["action"], "pay");
    assert_eq!(segment["frame_pre"], expected["frame_pre"]);
    assert_eq!(segment["frame_post"], expected["frame_post"]);
    let execution = &segment["execution"];
    assert_eq!(execution["semantics"], "moriarty-core/3");
    assert_eq!(execution["action"], "pay");
    assert_eq!(execution["pre"], expected["segment_pre"]);
    assert_eq!(execution["post"], expected["segment_post"]);
    assert_eq!(execution["effects"], expected["segment_effects"]);
    let steps = execution["steps"].as_array().unwrap();
    let observations = expected["step_observations"].as_array().unwrap();
    assert_eq!(steps.len(), observations.len());
    for (step, observation) in steps.iter().zip(observations) {
        assert_eq!(step["index"], observation["action_statement_ordinal"]);
        assert_eq!(step["value"], observation["typed_result"]);
        assert_eq!(
            step["snapshot"],
            observation["complete_post_statement_snapshot"]
        );
        assert_eq!(step["effects"], observation["new_effect_delta"]);
    }
}

#[test]
fn public_start_and_resume_use_actual_continuations_and_frozen_financial_projections() {
    let p = Project::new();
    let oracle: Value = serde_json::from_str(include_str!(
        "../../fixtures/core3-positive-prefix/EXPECTED-FINANCIAL-AND-WORK.json"
    ))
    .unwrap();
    for case in oracle["cases"].as_array().unwrap() {
        let source = case["source_input"].as_str().unwrap();
        p.copy("core3-positive-prefix", source);
        p.write("scenario.json", &case["initial_scenario"]);
        let mut actual = report(&p.run(&[
            "simulate",
            source,
            "--action",
            "pay",
            "--scenario",
            "scenario.json",
        ]));
        let operations = case["operation_history"].as_array().unwrap();
        compare_operation(&actual, &operations[0]);
        for operation in &operations[1..] {
            let previous = actual["continuation"].clone();
            assert!(previous.is_object());
            p.write("actual-continuation.json", &previous);
            let mut event = operation["event"].clone();
            // The independent input freezes the semantic tuple; the public
            // decoder additionally requires its existing transport format tag.
            event["format"] = json!("moriarty-stage-event/1");
            p.write("event.json", &event);
            actual = report(&p.run(&[
                "simulate",
                source,
                "--resume",
                "actual-continuation.json",
                "--event",
                "event.json",
            ]));
            compare_operation(&actual, operation);
            if operation["advanced"] == false {
                assert_eq!(actual["advanced"], false);
                assert_eq!(actual["continuation"], previous);
            }
        }
    }
}
