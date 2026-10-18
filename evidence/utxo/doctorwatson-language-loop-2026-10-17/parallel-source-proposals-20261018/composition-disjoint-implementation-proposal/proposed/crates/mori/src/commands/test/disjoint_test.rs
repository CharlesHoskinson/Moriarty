use crate::{commands::core_io, shell::Shell};
use mori_core::stage::{self, disjoint::{Member, Pair, MAX_INVOCATIONS}};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{path::{Path, PathBuf}, process::ExitCode};

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
    members: [MemberPath; 2],
    expected: PathBuf,
    events: Vec<Event>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MemberPath {
    source: PathBuf,
    scenario: PathBuf,
    action: String,
    logical_cap: mori_core::UInt,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Event {
    member: usize,
    event: PathBuf,
    expected: PathBuf,
}

pub(super) fn run(dir: &Path, value: Value, shell: &mut Shell) -> Result<ExitCode> {
    let manifest: Manifest = serde_json::from_value(value).map_err(core_io::operation)?;
    if manifest.profile != "moriarty-tests/disjoint-draft/1"
        || manifest.cases.is_empty() || manifest.cases.len() > 256
    {
        return Err(core_io::operation("expected a bounded disjoint-draft manifest"));
    }
    let mut names = std::collections::BTreeSet::new();
    for case in &manifest.cases {
        if case.name.is_empty() || !names.insert(&case.name)
            || case.events.len() > MAX_INVOCATIONS - 2
            || case.events.iter().any(|event| event.member >= 2)
        {
            return Err(core_io::operation("invalid disjoint case name, member or invocation count"));
        }
    }
    let mut reports = Vec::new();
    let mut failed = 0;
    for case in manifest.cases {
        let report = run_case(dir, case)?;
        if report["passed"] != true { failed += 1; }
        reports.push(report);
    }
    shell.status("Tested", format!("{} passed; {} failed", reports.len() - failed, failed));
    println!("{}", super::aggregate_report(failed == 0, reports)?);
    Ok(if failed == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}

fn member(dir: &Path, input: MemberPath) -> Result<Member> {
    let source = core_io::read_source(&dir.join(input.source))?;
    let scenario: mori_core::Scenario = core_io::read_json(&dir.join(input.scenario))?;
    Ok(Member {
        program: stage::load(&source).map_err(core_io::operation)?,
        action: input.action, scenario, cap: input.logical_cap,
    })
}

fn run_case(dir: &Path, case: Case) -> Result<Value> {
    // Freeze complete expected observations before invoking either program.
    let expected_start: Value = core_io::read_json(&dir.join(case.expected))?;
    let expected_events: Vec<Value> = case.events.iter().map(|event| {
        core_io::read_json(&dir.join(&event.expected))
    }).collect::<Result<_>>()?;
    let events: Vec<_> = case.events.iter().map(|event| {
        stage::decode::read_bounded(
            &dir.join(&event.event), stage::decode::MAX_EVENT_BYTES,
            stage::decode::EVENT_TOO_LARGE,
        ).and_then(|text| stage::decode::event(&text)).map_err(core_io::operation)
    }).collect::<Result<_>>()?;
    let [left, right] = case.members;
    let members = [member(dir, left)?, member(dir, right)?];
    let mut pair = Pair::start(members).map_err(core_io::operation)?;
    let actual = pair.report().map_err(core_io::operation)?;
    let mut passed = actual == expected_start;
    let mut stages = vec![json!({"index":0,"passed":passed,"actual":actual})];
    for (index, ((step, event), expected)) in case.events.iter().zip(events).zip(expected_events).enumerate() {
        pair.resume(step.member, event).map_err(core_io::operation)?;
        let actual = pair.report().map_err(core_io::operation)?;
        let matched = actual == expected;
        passed &= matched;
        stages.push(json!({"index":index+1,"passed":matched,"actual":actual}));
    }
    Ok(json!({"name":case.name,"passed":passed,"stages":stages}))
}
