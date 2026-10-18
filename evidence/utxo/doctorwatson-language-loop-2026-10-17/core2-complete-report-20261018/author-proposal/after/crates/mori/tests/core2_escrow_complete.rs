//! Complete local Core/2 escrow observations through public consumers.
use mori_core::{
    Scenario, UInt,
    stage::{self, Continuation, PendingFrame, StageOutcome},
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core2-escrow-complete")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn assert_report(actual: &Value, name: &str, attempted: &str, committed: &str) {
    assert_eq!(
        actual,
        &json_fixture(&format!("{name}.report.expected.json")),
        "{name}: complete report"
    );
    assert_eq!(actual["attempted"], attempted, "{name}: attempted work");
    assert_eq!(actual["committed"], committed, "{name}: committed work");
}

fn assert_outcome(
    outcome: StageOutcome,
    name: &str,
    attempted: &str,
    committed: &str,
) -> Option<Continuation> {
    let continuation = match &outcome {
        StageOutcome::Waiting { continuation, .. } => Some(continuation.clone()),
        StageOutcome::Complete { .. } => None,
    };
    let actual = stage::outcome_report(Ok(outcome));
    assert_report(&actual, name, attempted, committed);
    continuation
}

#[test]
fn escrow_public_load_matches_complete_program_and_filled_plan() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        json_fixture("escrow.program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        json_fixture("escrow.plan.expected.json")
    );
}

#[test]
fn escrow_library_start_false_observation_and_delivery_match_complete_reports() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    let scenario: Scenario = serde_json::from_value(json_fixture("escrow.scenario.json")).unwrap();
    let false_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-false.event.json")).unwrap(),
    )
    .unwrap();
    let true_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-true.event.json")).unwrap(),
    )
    .unwrap();

    let started = stage::start(&program, "deliver", scenario, Some(UInt(11))).unwrap();
    let first = assert_outcome(started, "start", "5", "5").unwrap();
    let before_observation = first.clone();
    let observed = stage::resume(&program, first, false_event, Some("deliver"), None).unwrap();
    let pending = assert_outcome(observed, "delivery-false", "2", "5").unwrap();
    assert_eq!(pending, before_observation);

    let funded_boundary = pending.boundary.clone();
    let pending_frame = PendingFrame {
        site: pending.site,
        captures: pending.captures.clone(),
    };
    let completed = stage::resume(&program, pending, true_event, Some("deliver"), None).unwrap();
    let StageOutcome::Complete {
        segment, residual, ..
    } = &completed
    else {
        panic!("delivery must complete");
    };
    assert_eq!(segment.frame_pre, pending_frame);
    assert_eq!(segment.execution.pre, funded_boundary);
    assert!(residual.is_empty());
    assert_eq!(segment.execution.post.liabilities[0].name, "release");
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(0));
    assert!(assert_outcome(completed, "delivery-true", "6", "11").is_none());
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core2-escrow-complete-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in [
            "escrow.mori",
            "escrow.scenario.json",
            "delivery-false.event.json",
            "delivery-true.event.json",
        ] {
            fs::copy(fixtures().join(name), project.root.join(name)).unwrap();
        }
        project
    }

    fn assert_case(&self, name: &str, args: &[&str], attempted: &str, committed: &str) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args([
                "--color", "never", "simulate", "escrow.mori", "--action", "deliver",
            ])
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            output.stderr,
            fs::read(fixtures().join("empty.stderr.expected.bin")).unwrap(),
            "{name}: exact empty stderr"
        );
        assert!(output.stdout.is_ascii(), "{name}: ASCII stdout");
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_report(&actual, name, attempted, committed);
        assert_eq!(
            output.stdout,
            fs::read(fixtures().join(format!("{name}.stdout.expected.json"))).unwrap(),
            "{name}: exact public stdout bytes"
        );
        actual
    }

    fn save_actual_continuation(&self, continuation: &Value) {
        fs::write(
            self.root.join("actual-continuation.json"),
            serde_json::to_vec(continuation).unwrap(),
        )
        .unwrap();
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn escrow_cli_start_false_observation_and_delivery_match_complete_public_bytes() {
    let project = Project::new();
    let started = project.assert_case(
        "start",
        &["--scenario", "escrow.scenario.json", "--logical-cap", "11"],
        "5",
        "5",
    );
    let first_continuation = started["continuation"].clone();
    project.save_actual_continuation(&started["continuation"]);
    let observed = project.assert_case(
        "delivery-false",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-false.event.json",
        ],
        "2",
        "5",
    );
    assert_eq!(observed["continuation"], first_continuation);
    assert_eq!(observed["advanced"], false);
    assert!(observed["segment"].is_null());
    project.save_actual_continuation(&observed["continuation"]);
    let completed = project.assert_case(
        "delivery-true",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-true.event.json",
        ],
        "6",
        "11",
    );
    assert_eq!(
        completed["segment"]["execution"]["pre"],
        observed["continuation"]["boundary"]
    );
    assert!(completed["residual"].as_array().unwrap().is_empty());
    let closing_duty = &completed["segment"]["execution"]["post"]["liabilities"][0];
    assert_eq!(closing_duty["name"], "release");
    assert_eq!(closing_duty["amount"], "0");
}
