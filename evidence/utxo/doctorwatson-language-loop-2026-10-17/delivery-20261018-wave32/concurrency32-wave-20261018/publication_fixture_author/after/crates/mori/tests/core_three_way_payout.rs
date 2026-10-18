//! Complete ordinary Core/1 payout frames and real public CLI observations.
//! Independent source-specific IR/byte premises are retained with the fixtures.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/three-way-payout-contract")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn program() -> mori_core::Program {
    let source = fs::read_to_string(fixtures().join("inputs/three-way-payout.mori")).unwrap();
    mori_core::parse(&source).expect("payout source must parse")
}

#[test]
fn payout_source_matches_complete_independent_program_candidate() {
    assert_eq!(
        serde_json::to_value(program()).unwrap(),
        json_fixture("three-way-payout.program.expected.json"),
        "complete source-specific Program, including the unused duty declaration"
    );
}

#[test]
fn payout_execution_matches_full_prefixes_effects_frames_and_work() {
    let program = program();
    let scenario = serde_json::from_slice(
        &fs::read(fixtures().join("inputs/three-way-payout.scenario.json")).unwrap(),
    )
    .unwrap();
    let actual = mori_core::simulate(&program, "payout", &scenario).unwrap();
    assert_eq!(
        serde_json::to_value(&actual).unwrap(),
        json_fixture("three-way-payout.execution.expected.json"),
        "complete pre/post, statement IDs, typed values, all snapshots and deltas"
    );
    let prefixes = json_fixture("three-way-payout.effect-prefixes.expected.json");
    let prefixes = prefixes.as_array().unwrap();
    assert_eq!(actual.steps.len(), prefixes.len());
    let mut effects = Vec::new();
    for (step, expected_prefix) in actual.steps.iter().zip(prefixes) {
        effects.extend_from_slice(&step.effects);
        assert_eq!(
            serde_json::to_value(&effects).unwrap(),
            *expected_prefix,
            "complete ordered effect prefix after statement {}",
            step.index
        );
    }
    assert_eq!(actual.effects, effects);
    assert_eq!(actual.post, actual.steps.last().unwrap().snapshot);

    let expected_cost = json_fixture("three-way-payout.cost.expected.json");
    let cap = serde_json::from_value(expected_cost["cap"].clone()).unwrap();
    let metered = mori_core::simulate_metered(&program, "payout", &scenario, cap).unwrap();
    assert_eq!(
        metered.execution, actual,
        "complete metered Execution equality"
    );
    assert_eq!(serde_json::to_value(metered.cost).unwrap(), expected_cost);
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-three-way-payout-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in ["three-way-payout.mori", "three-way-payout.scenario.json"] {
            fs::copy(fixtures().join("inputs").join(name), project.root.join(name)).unwrap();
        }
        project
    }

    fn assert_case(&self, case: &Value) -> Value {
        let stem = case["name"].as_str().unwrap();
        let cap = case["logical_cap"].as_str();
        let mut command = Command::new(env!("CARGO_BIN_EXE_mori"));
        command.current_dir(&self.root).args([
            "--color",
            "never",
            "simulate",
            "three-way-payout.mori",
            "--action",
            "payout",
            "--scenario",
            "three-way-payout.scenario.json",
        ]);
        if let Some(cap) = cap {
            command.arg(format!("--logical-cap={cap}"));
        }
        let output = command.output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{stem}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let expected_stderr = fs::read(fixtures().join(case["stderr"].as_str().unwrap())).unwrap();
        assert!(expected_stderr.is_empty());
        assert_eq!(output.stderr, expected_stderr, "{stem}: exact empty stderr");
        let expected_stdout =
            fs::read(fixtures().join(case["stdout"].as_str().unwrap())).unwrap();
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            actual,
            json_fixture(case["expected_report"].as_str().unwrap()),
            "{stem}: entire public JSON envelope"
        );
        assert_eq!(
            output.stdout, expected_stdout,
            "{stem}: exact public stdout bytes"
        );
        actual
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn payout_cli_matches_complete_ordinary_and_metered_candidates() {
    let project = Project::new();
    let cases = json_fixture("CASES.json");
    let cases = cases["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 2);
    let ordinary = project.assert_case(&cases[0]);
    let metered = project.assert_case(&cases[1]);
    assert_eq!(ordinary["execution"], metered["execution"]);
    assert_eq!(
        ordinary["execution"],
        json_fixture("three-way-payout.execution.expected.json")
    );
}
