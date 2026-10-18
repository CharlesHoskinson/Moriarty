//! Complete local staged observations of two installments under distinct awaits.
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core2-partial-delivery-complete")
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

fn frame(continuation: &Continuation) -> PendingFrame {
    PendingFrame {
        site: continuation.site,
        captures: continuation.captures.clone(),
    }
}

fn event(name: &str) -> stage::StageEvent {
    stage::decode::event(&fs::read_to_string(fixtures().join(name)).unwrap()).unwrap()
}

#[test]
fn partial_delivery_public_load_matches_complete_program_and_filled_plan() {
    let source = fs::read_to_string(fixtures().join("partial-delivery.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        json_fixture("partial-delivery.program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        json_fixture("partial-delivery.plan.expected.json")
    );
}

#[test]
fn partial_delivery_library_retains_the_partial_duty_at_the_second_condition() {
    let source = fs::read_to_string(fixtures().join("partial-delivery.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    let scenario: Scenario =
        serde_json::from_value(json_fixture("partial-delivery.scenario.json")).unwrap();
    let first = waiting(
        stage::start(&program, "deliver", scenario, None).unwrap(),
        "start",
    );
    let first_boundary = first.boundary.clone();
    let first_frame = frame(&first);
    let approved = stage::resume(
        &program,
        first,
        event("approval-true.event.json"),
        Some("deliver"),
        None,
    )
    .unwrap();
    let StageOutcome::Waiting {
        segment: Some(segment),
        ..
    } = &approved
    else {
        panic!("approval must advance to the next pending condition");
    };
    assert_eq!(segment.execution.pre, first_boundary);
    assert_eq!(segment.frame_pre, first_frame);
    let second = waiting(approved, "approval-true");
    let before_observation = second.clone();
    let observed = stage::resume(
        &program,
        second,
        event("receipt-false.event.json"),
        Some("deliver"),
        None,
    )
    .unwrap();
    let pending = waiting(observed, "receipt-false");
    assert_eq!(pending, before_observation);
    let partial_boundary = pending.boundary.clone();
    let partial_frame = frame(&pending);
    let completed = stage::resume(
        &program,
        pending,
        event("receipt-true.event.json"),
        Some("deliver"),
        None,
    )
    .unwrap();
    let StageOutcome::Complete { segment, .. } = &completed else {
        panic!("receipt must finish the remaining installment");
    };
    assert_eq!(segment.execution.pre, partial_boundary);
    assert_eq!(segment.frame_pre, partial_frame);
    assert_report(&stage::outcome_report(Ok(completed)), "receipt-true");
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core2-partial-delivery-complete-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in [
            "partial-delivery.mori",
            "partial-delivery.scenario.json",
            "approval-true.event.json",
            "receipt-false.event.json",
            "receipt-true.event.json",
        ] {
            fs::copy(fixtures().join(name), project.root.join(name)).unwrap();
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
                "partial-delivery.mori",
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
        assert_eq!(output.stderr, Vec::<u8>::new(), "{name}: empty stderr");
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

    fn save_actual_continuation(&self, continuation: &Value) {
        fs::write(
            self.root.join("actual-continuation.json"),
            serde_json::to_vec(continuation).unwrap(),
        )
        .unwrap();
    }

    fn resume(&self, previous: &Value, name: &str, event: &str) -> Value {
        self.save_actual_continuation(&previous["continuation"]);
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
fn partial_delivery_cli_retains_the_partial_duty_and_matches_complete_public_bytes() {
    let project = Project::new();
    let started = project.assert_case("start", &["--scenario", "partial-delivery.scenario.json"]);
    let approved = project.resume(&started, "approval-true", "approval-true.event.json");
    assert_eq!(
        approved["segment"]["execution"]["pre"],
        started["continuation"]["boundary"]
    );
    assert_eq!(
        approved["segment"]["frame_pre"]["captures"],
        started["continuation"]["captures"]
    );
    assert_eq!(
        approved["segment"]["frame_pre"]["site"],
        started["continuation"]["site"]
    );
    let observed = project.resume(&approved, "receipt-false", "receipt-false.event.json");
    assert_eq!(observed["continuation"], approved["continuation"]);
    let completed = project.resume(&observed, "receipt-true", "receipt-true.event.json");
    assert_eq!(
        completed["segment"]["execution"]["pre"],
        observed["continuation"]["boundary"]
    );
    assert_eq!(
        completed["segment"]["frame_pre"]["captures"],
        observed["continuation"]["captures"]
    );
    assert_eq!(
        completed["segment"]["frame_pre"]["site"],
        observed["continuation"]["site"]
    );
}
