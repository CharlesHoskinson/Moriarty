//! Complete local observations of one heterogeneous conjunctive await guard.
use mori_core::{
    Scenario,
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core2-multi-condition-complete")
}

fn inputs() -> PathBuf {
    fixtures().join("input-custody")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn assert_report(actual: &Value, name: &str) {
    assert_eq!(
        actual,
        &json_fixture(&format!("{name}.report.expected.json")),
        "{name}: complete report"
    );
}

fn waiting(outcome: StageOutcome, name: &str) -> Continuation {
    assert_report(&stage::outcome_report(Ok(outcome.clone())), name);
    let StageOutcome::Waiting { continuation, .. } = outcome else {
        panic!("{name}: expected pending computation");
    };
    continuation
}

fn event(name: &str) -> stage::StageEvent {
    stage::decode::event(&fs::read_to_string(inputs().join(name)).unwrap()).unwrap()
}

#[test]
fn multi_condition_public_load_matches_complete_program_and_filled_plan() {
    let source = fs::read_to_string(inputs().join("multi-condition.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        json_fixture("multi-condition.program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        json_fixture("multi-condition.plan.expected.json")
    );
}

#[test]
fn multi_condition_library_requires_both_observations_in_the_same_event() {
    let source = fs::read_to_string(inputs().join("multi-condition.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    let scenario: Scenario = serde_json::from_slice(
        &fs::read(inputs().join("multi-condition.scenario.json")).unwrap(),
    )
    .unwrap();
    let mut pending = waiting(
        stage::start(&program, "deliver", scenario, None).unwrap(),
        "start",
    );
    for (name, file) in [
        ("document-only", "document-only.event.json"),
        ("amount-only", "amount-only.event.json"),
    ] {
        let before_observation = pending.clone();
        let observed =
            stage::resume(&program, pending, event(file), Some("deliver"), None).unwrap();
        pending = waiting(observed, name);
        assert_eq!(pending, before_observation, "{name}: actual continuation");
    }
    let pending_boundary = pending.boundary.clone();
    let pending_frame = PendingFrame {
        site: pending.site,
        captures: pending.captures.clone(),
    };
    let completed = stage::resume(
        &program,
        pending,
        event("both.event.json"),
        Some("deliver"),
        None,
    )
    .unwrap();
    let StageOutcome::Complete { segment, .. } = &completed else {
        panic!("both observations must complete the computation");
    };
    assert_eq!(segment.execution.pre, pending_boundary);
    assert_eq!(segment.frame_pre, pending_frame);
    assert_report(&stage::outcome_report(Ok(completed)), "both");
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core2-multi-condition-complete-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in [
            "multi-condition.mori",
            "multi-condition.scenario.json",
            "document-only.event.json",
            "amount-only.event.json",
            "both.event.json",
        ] {
            fs::copy(inputs().join(name), project.root.join(name)).unwrap();
        }
        project
    }

    fn assert_case(&self, name: &str, args: &[&str]) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args([
                "--color",
                "never",
                "simulate",
                "multi-condition.mori",
                "--action",
                "deliver",
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
            fs::read(fixtures().join(format!("{name}.stderr.expected.txt"))).unwrap(),
            "{name}: exact empty stderr"
        );
        assert!(output.stdout.is_ascii(), "{name}: ASCII stdout");
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_report(&actual, name);
        assert_eq!(
            output.stdout,
            fs::read(fixtures().join(format!("{name}.stdout.expected.json"))).unwrap(),
            "{name}: exact public stdout bytes"
        );
        actual
    }

    fn resume(&self, previous: &Value, name: &str, event: &str) -> Value {
        fs::write(
            self.root.join("actual-continuation.json"),
            serde_json::to_vec(&previous["continuation"]).unwrap(),
        )
        .unwrap();
        self.assert_case(
            name,
            &["--resume", "actual-continuation.json", "--event", event],
        )
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn multi_condition_cli_requires_both_observations_and_matches_complete_public_bytes() {
    let project = Project::new();
    let mut previous = project.assert_case("start", &["--scenario", "multi-condition.scenario.json"]);
    for (name, file) in [
        ("document-only", "document-only.event.json"),
        ("amount-only", "amount-only.event.json"),
    ] {
        let observed = project.resume(&previous, name, file);
        assert_eq!(
            observed["continuation"], previous["continuation"],
            "{name}: actual continuation"
        );
        previous = observed;
    }
    let completed = project.resume(&previous, "both", "both.event.json");
    assert_eq!(
        completed["segment"]["execution"]["pre"],
        previous["continuation"]["boundary"]
    );
    assert_eq!(
        completed["segment"]["frame_pre"]["captures"],
        previous["continuation"]["captures"]
    );
    assert_eq!(
        completed["segment"]["frame_pre"]["site"],
        previous["continuation"]["site"]
    );
}
