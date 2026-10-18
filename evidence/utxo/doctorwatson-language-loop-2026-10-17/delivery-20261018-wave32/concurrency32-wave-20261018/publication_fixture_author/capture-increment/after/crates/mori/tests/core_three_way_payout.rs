//! Complete ordinary Core/1 payout frames and real public CLI observations.
//! Independent source-specific IR/byte premises are retained with the fixtures.
use serde::Serialize;
use serde_json::Value;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{
        OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/three-way-payout-contract")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

static OBSERVATIONS: OnceLock<Option<PathBuf>> = OnceLock::new();

fn observations() -> Option<&'static Path> {
    OBSERVATIONS
        .get_or_init(|| {
            std::env::var_os("MORI_PAYOUT_OBSERVATIONS").map(|value| {
                let root = PathBuf::from(value);
                assert!(root.is_absolute(), "MORI_PAYOUT_OBSERVATIONS must be absolute");
                fs::create_dir(&root).expect(
                    "MORI_PAYOUT_OBSERVATIONS must be fresh with an existing parent directory",
                );
                root
            })
        })
        .as_deref()
}

fn retain_bytes(name: &str, bytes: &[u8]) {
    if let Some(root) = observations() {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(name))
            .unwrap();
        file.write_all(bytes).unwrap();
    }
}

fn retain_json(name: &str, value: &impl Serialize) {
    if observations().is_some() {
        let mut bytes = serde_json::to_vec_pretty(value).unwrap();
        bytes.push(b'\n');
        retain_bytes(name, &bytes);
    }
}

fn program(name: &str) -> mori_core::Program {
    let source = fs::read_to_string(fixtures().join("inputs/three-way-payout.mori")).unwrap();
    let parsed = mori_core::parse(&source);
    retain_json(&format!("{name}.parse-result.json"), &parsed);
    let program = parsed.expect("payout source must parse");
    retain_json(&format!("{name}.program.json"), &program);
    program
}

#[test]
fn payout_source_matches_complete_independent_program_candidate() {
    assert_eq!(
        serde_json::to_value(program("source")).unwrap(),
        json_fixture("three-way-payout.program.expected.json"),
        "complete source-specific Program, including the unused duty declaration"
    );
}

#[test]
fn payout_execution_matches_full_prefixes_effects_frames_and_work() {
    let program = program("execution");
    let scenario = serde_json::from_slice(
        &fs::read(fixtures().join("inputs/three-way-payout.scenario.json")).unwrap(),
    )
    .unwrap();
    let expected_cost = json_fixture("three-way-payout.cost.expected.json");
    let cap = serde_json::from_value(expected_cost["cap"].clone()).unwrap();
    let ordinary_result = mori_core::simulate(&program, "payout", &scenario);
    let metered_result = mori_core::simulate_metered(&program, "payout", &scenario, cap);
    retain_json("ordinary-call-result.json", &ordinary_result);
    retain_json("metered-call-result.json", &metered_result);
    if let Ok(execution) = &ordinary_result {
        retain_json("ordinary-execution.json", execution);
    }
    if let Ok(metered) = &metered_result {
        retain_json("metered-result.json", metered);
    }
    let actual = ordinary_result.unwrap();
    let metered = metered_result.unwrap();
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

    fn run_case(&self, case: &Value) -> std::io::Result<Output> {
        let stem = case["name"].as_str().unwrap();
        let mut argv = [
            "--color",
            "never",
            "simulate",
            "three-way-payout.mori",
            "--action",
            "payout",
            "--scenario",
            "three-way-payout.scenario.json",
        ]
        .map(str::to_owned)
        .to_vec();
        if let Some(cap) = case["logical_cap"].as_str() {
            argv.push(format!("--logical-cap={cap}"));
        }
        let result = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args(&argv)
            .output();
        let process = match &result {
            Ok(output) => {
                retain_bytes(&format!("{stem}.stdout.bin"), &output.stdout);
                retain_bytes(&format!("{stem}.stderr.bin"), &output.stderr);
                serde_json::json!({
                    "executable": env!("CARGO_BIN_EXE_mori"),
                    "argv": argv,
                    "cwd": self.root,
                    "status_code": output.status.code(),
                    "success": output.status.success(),
                    "status_debug": format!("{:?}", output.status),
                    "spawn_error": null
                })
            }
            Err(error) => serde_json::json!({
                "executable": env!("CARGO_BIN_EXE_mori"),
                "argv": argv,
                "cwd": self.root,
                "status_code": null,
                "success": false,
                "status_debug": null,
                "spawn_error": {
                    "message": error.to_string(),
                    "kind": format!("{:?}", error.kind()),
                    "raw_os_error": error.raw_os_error()
                }
            }),
        };
        retain_json(&format!("{stem}.process.json"), &process);
        result
    }

    fn assert_case(&self, case: &Value, output: &Output) -> Value {
        let stem = case["name"].as_str().unwrap();
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
    let ordinary_output = project.run_case(&cases[0]);
    let metered_output = project.run_case(&cases[1]);
    let ordinary_output = ordinary_output.unwrap();
    let metered_output = metered_output.unwrap();
    let ordinary = project.assert_case(&cases[0], &ordinary_output);
    let metered = project.assert_case(&cases[1], &metered_output);
    assert_eq!(ordinary["execution"], metered["execution"]);
    assert_eq!(
        ordinary["execution"],
        json_fixture("three-way-payout.execution.expected.json")
    );
}
