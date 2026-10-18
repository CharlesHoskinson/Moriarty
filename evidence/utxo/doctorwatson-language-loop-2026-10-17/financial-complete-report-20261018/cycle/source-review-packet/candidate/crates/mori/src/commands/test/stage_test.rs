//! Additive ordinary test-manifest tooling for the existing local Core/2 API.
//! Expected reports are whole independently supplied JSON values. They never
//! supply runtime state, captures, continuation hashes, or work counters.
use crate::{commands::core_io, shell::Shell};
use mori_core::stage::{self, Continuation, StageOutcome, StageRejection};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

type Result<T> = crate::commands::Result<T>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    profile: String,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    name: String,
    source: PathBuf,
    scenario: PathBuf,
    action: String,
    /// Explicit null means no start cap; the key is mandatory.
    #[serde(deserialize_with = "Option::deserialize")]
    logical_cap: Option<mori_core::UInt>,
    expected: PathBuf,
    events: Vec<Event>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event {
    event: PathBuf,
    expected: PathBuf,
}

pub(super) fn run(dir: &Path, value: Value, shell: &mut Shell) -> Result<ExitCode> {
    let manifest: Manifest = serde_json::from_value(value).map_err(core_io::operation)?;
    if manifest.profile != "moriarty-tests/2"
        || manifest.cases.is_empty()
        || manifest.cases.len() > 256
    {
        return Err(core_io::operation(
            "expected a nonempty bounded moriarty-tests/2 manifest",
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    for case in &manifest.cases {
        if case.name.is_empty() || !names.insert(&case.name) {
            return Err(core_io::operation(
                "test case names must be nonempty and unique",
            ));
        }
        // This bounds invocations of the test harness, not the language plan.
        if case.events.len() >= 256 {
            return Err(core_io::operation(
                "a staged test case permits at most 256 invocations including start",
            ));
        }
    }
    let mut reports = Vec::new();
    let mut failed = 0;
    for case in manifest.cases {
        let report = run_case(dir, case)?;
        if report["passed"] != true {
            failed += 1;
        }
        reports.push(report);
    }
    shell.status(
        "Tested",
        format!("{} passed; {} failed", reports.len() - failed, failed),
    );
    // Reuse the existing individually bounded aggregate writer. All actual
    // staged reports retain the Core/2 qualifications below.
    let report = super::aggregate_report(failed == 0, reports)?;
    println!("{report}");
    Ok(if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn run_case(dir: &Path, case: Case) -> Result<Value> {
    // Read every expected value before invoking the program. Keep the shared
    // strict 1 MiB JSON reader: no field projection or expected-value defaults.
    let expected_start: Value = core_io::read_json(&dir.join(&case.expected))?;
    let expected_events: Vec<Value> = case
        .events
        .iter()
        .map(|event| core_io::read_json(&dir.join(&event.expected)))
        .collect::<Result<_>>()?;
    let source = core_io::read_source(&dir.join(&case.source))?;
    let program = stage::load(&source);
    let started = match &program {
        Ok(program) => scenario(&dir.join(&case.scenario))
            .and_then(|scenario| stage::start(program, &case.action, scenario, case.logical_cap)),
        Err(error) => Err(no_work(error.clone())),
    };
    let mut pending = continuation(&started);
    let actual = stage::outcome_report(started);
    let passed = actual == expected_start;
    let mut stages = vec![json!({"index":0,"passed":passed,"actual":actual})];
    let mut all_passed = passed;
    for (index, (event, expected)) in case.events.iter().zip(expected_events).enumerate() {
        let previous = pending.take().ok_or_else(|| {
            core_io::operation("a staged test event requires an actual pending continuation")
        })?;
        let program = program.as_ref().map_err(core_io::operation)?;
        let resumed = stage::decode::read_bounded(
            &dir.join(&event.event),
            stage::decode::MAX_EVENT_BYTES,
            stage::decode::EVENT_TOO_LARGE,
        )
        .and_then(|text| stage::decode::event(&text))
        .map_err(no_work)
        .and_then(|event| {
            stage::resume(program, previous.clone(), event, Some(&case.action), None)
        });
        pending = match &resumed {
            // Rejection publishes no new boundary. Retain the actual input for
            // an explicitly listed retry; the local API tracks no consumption.
            Err(_) => Some(previous),
            _ => continuation(&resumed),
        };
        let actual = stage::outcome_report(resumed);
        let passed = actual == expected;
        all_passed &= passed;
        stages.push(json!({"index":index+1,"passed":passed,"actual":actual}));
    }
    Ok(json!({"name":case.name,"passed":all_passed,"stages":stages}))
}

fn scenario(path: &Path) -> std::result::Result<mori_core::Scenario, StageRejection> {
    let text = stage::decode::read_bounded(path, 1_048_576, "Scenario JSON exceeds 1 MiB")
        .map_err(no_work)?;
    let value = mori_core::json::parse(&text).map_err(no_work)?;
    serde_json::from_value(value).map_err(|error| {
        no_work(mori_core::Error {
            code: "mori::core::json".into(),
            message: error.to_string(),
            offset: None,
            step: None,
        })
    })
}

fn no_work(error: mori_core::Error) -> StageRejection {
    StageRejection {
        error,
        attempted: mori_core::UInt(0),
        committed: mori_core::UInt(0),
    }
}

fn continuation(
    result: &std::result::Result<StageOutcome, StageRejection>,
) -> Option<Continuation> {
    match result {
        Ok(StageOutcome::Waiting { continuation, .. }) => Some(continuation.clone()),
        _ => None,
    }
}
