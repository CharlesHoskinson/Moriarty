use crate::{commands::core_io, shell::Shell};
use mori_core::stage::{self, disjoint::{Error as PairError, Member, Pair, MAX_INVOCATIONS}};
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


fn pair_error(error: PairError) -> mori_diagnostics::Report {
    // Value's Display preserves every original rejection/work field in the
    // existing command diagnostic path instead of Error's summary Display.
    core_io::operation(error.public_report())
}

#[derive(Default)]
struct ObservationTables {
    history: Vec<Value>,
    effects: Vec<Value>,
}

impl ObservationTables {
    /// Lossless wire representation: cumulative arrays live once at case scope;
    /// each stage names the complete prefix it observed. No item is truncated.
    fn reference(&mut self, mut actual: Value) -> Result<Value> {
        let object = actual.as_object_mut()
            .ok_or_else(|| core_io::operation("pair report must be an object"))?;
        let history = object.remove("history")
            .ok_or_else(|| core_io::operation("pair report has no history"))?;
        let effects = object.remove("effects")
            .ok_or_else(|| core_io::operation("pair report has no effects"))?;
        let (Value::Array(history), Value::Array(effects)) = (history, effects) else {
            return Err(core_io::operation("pair report tables must be arrays"));
        };
        if !history.starts_with(&self.history) || !effects.starts_with(&self.effects) {
            return Err(core_io::operation("pair report rewrote an earlier observed table prefix"));
        }
        let history_length = history.len();
        let effects_length = effects.len();
        let previous_history = self.history.len();
        let previous_effects = self.effects.len();
        self.history.extend_from_slice(&history[previous_history..]);
        self.effects.extend_from_slice(&effects[previous_effects..]);
        let expanded_format = object.remove("format")
            .ok_or_else(|| core_io::operation("pair report has no format"))?;
        object.insert("format".into(), json!("moriarty-local-disjoint-referenced-draft/1"));
        object.insert("expanded_format".into(), expanded_format);
        object.insert("history".into(), json!({"table":"history","prefix_len":history_length}));
        object.insert("effects".into(), json!({"table":"effects","prefix_len":effects_length}));
        Ok(actual)
    }
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
    let mut pair = Pair::start(members).map_err(pair_error)?;
    let actual = pair.report().map_err(pair_error)?;
    let mut passed = actual == expected_start;
    let mut tables = ObservationTables::default();
    let mut stages = vec![json!({"index":0,"passed":passed,"actual":tables.reference(actual)?})];
    for (index, ((step, event), expected)) in case.events.iter().zip(events).zip(expected_events).enumerate() {
        pair.resume(step.member, event).map_err(pair_error)?;
        let actual = pair.report().map_err(pair_error)?;
        let matched = actual == expected;
        passed &= matched;
        stages.push(json!({"index":index+1,"passed":matched,"actual":tables.reference(actual)?}));
    }
    Ok(json!({
        "format":"moriarty-disjoint-case-observations-draft/1",
        "name":case.name,"passed":passed,"stages":stages,
        "tables":{"history":tables.history,"effects":tables.effects},
    }))
}
