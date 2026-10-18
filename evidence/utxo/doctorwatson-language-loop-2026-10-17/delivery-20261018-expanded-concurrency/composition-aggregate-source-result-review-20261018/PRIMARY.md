# Ordinary source/result review input — not dispatched

The actual candidate is commit `2f1b969ae4153c4f4f05ffe27a285a12e20f09cd`, tree `892b5934ddd9fe7dfe5e9999f992ae4ac8842bff`, direct parent main E4D `e4d7a2b5709caa18a94f5256d03a0302364b8235`. Exactly one test file changes by62 lines. After-file SHA256 is `b522275389d57247a68c36c3e28e7e174a6035d644b5b7cbaa6b592a865b4c41`; patch SHA256 is `6dc69fdeeb0c4200d269cd3deaa31c6cc88d81ba72933c81c275a2b3a6b3fccb`. Prepared K823 source-check changes are absent from this tested subject. Do not use ambient J HEAD: the worktree has since switched branches. All source bodies here came from exact immutable Git objects.

ACTUAL OBSERVATION: root ran one cold command with a90-second timeout and5-second kill grace, frozen/offline,13 environment variables and no HOME. Outer exit0; wall56.859117s. The Rust harness reports0.24s: one new function passes, zero fail, and one existing function is filtered. This passing function spawned the real public `mori test`, required inner exit1 and one case passed/one failed, compared the complete failed aggregate JSON and exact bytes, then expanded all ten actual Pair reports against ten complete original independent goldens. Counts are one CLI invocation, two unchanged cases/schedules/programs, twelve stage calls, ten Pair observations and one aggregate. This is not a new financial scenario.

Root's physical stdout is the Rust harness output, not the raw inner CLI report. Inner stdout/stderr were captured and asserted inside the passing test but were not separately dumped. The physical `DERIVED-FAILED-PUBLIC-REPORT.json` is the EXPECTED DERIVATIVE,839112 bytes, SHA256 `96ed426bef4d9804205d009fca5036b39592f2fdfab5754fdf02c2a68ca7a6d8`. Never relabel that file as captured actual output. Only `/status`, `/cases/0/passed`, `/cases/0/stages/2/passed` differ from the original839110-byte aggregate. No financial field differs.

Original independent custody freeze SHA256: `12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848`. All74 payloads and freeze are physical under independent-custody/. Financial/work values were independently source-derived with disclosed nonblind related-source/producer exposure and a narrow finite Core premise. Full wire/schema bytes remain conditional on the authored codec, not an independent compiler-wire proof. No new financial values or goldens were generated. The complete two sources/nine inputs, ten whole reports, original aggregate, four Program/Plan JSONs and four preimages are physical under actual-source/crates/fixtures/core-disjoint-product. The four child objects/preimages are NOT directly compared by this new test.

All89 selected owned source bodies are physical under actual-source/. Root's full817-source inventory is retained;81 selected body hashes intersect and match it. The other eight are documents/metadata outside that compiled-source inventory. No claim is made that all817 source bodies were inspected here. Full goldens are deliberately UNEMBEDDED, not absent. The view below contains complete financial heads, frames and work; it does not replace the full original Execution/Step/effect/history envelopes physically retained.

Root reports draft PR228 created/pushed; publication was not queried here and no merge attestation is claimed. Original root INTEGRATION.json saying UNRUN is timestamped before the command; the later terminal RESULT supplies the actual observation. PR-BODY reports a rustfmt pass, but this packet contains no separate formatter receipt. No provider has reviewed or voted on this packet. All five full operators and parent/native/proof/resource/ledger/target/security/authority exits remain open; SI1–SI4 remain unadopted.

## Complete actual changed integration-test file
Path: actual-source/crates/mori/tests/disjoint_test_public.rs
SHA256: b522275389d57247a68c36c3e28e7e174a6035d644b5b7cbaa6b592a865b4c41
```text
#![cfg(feature = "core2-disjoint-draft")]
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-disjoint-test-public-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        let fixtures =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core-disjoint-product");
        for entry in fs::read_dir(fixtures).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                fs::copy(entry.path(), path.join(entry.file_name())).unwrap();
            }
        }
        Self(path)
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn expand(actual: &Value, tables: &Value) -> Value {
    let mut actual = actual.clone();
    assert_eq!(
        actual["format"],
        "moriarty-local-disjoint-referenced-draft/1"
    );
    for name in ["history", "effects"] {
        assert_eq!(actual[name]["table"], name);
        let length = usize::try_from(actual[name]["prefix_len"].as_u64().unwrap()).unwrap();
        let table = tables[name].as_array().unwrap();
        actual[name] = Value::Array(table.get(..length).unwrap().to_vec());
    }
    let format = actual
        .as_object_mut()
        .unwrap()
        .remove("expanded_format")
        .unwrap();
    actual["format"] = format;
    actual
}

#[test]
fn public_consumer_observes_both_disjoint_domains_for_two_explicit_schedules() {
    let project = Project::new();
    // The aggregate and all ten stage reports are independent literal inputs.
    let expected = fs::read(project.0.join("public-report.json")).unwrap();
    let expected_json: Value = serde_json::from_slice(&expected).unwrap();
    let manifest: Value =
        serde_json::from_slice(&fs::read(project.0.join("mori-tests.json")).unwrap()).unwrap();
    let expected_stages: Vec<Vec<Value>> = manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let mut paths = vec![case["expected"].as_str().unwrap()];
            paths.extend(
                case["events"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|event| event["expected"].as_str().unwrap()),
            );
            paths
                .into_iter()
                .map(|path| {
                    serde_json::from_slice(&fs::read(project.0.join(path)).unwrap()).unwrap()
                })
                .collect()
        })
        .collect();
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&project.0)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, expected_json);
    assert_eq!(output.stdout, expected);
    assert!(String::from_utf8_lossy(&output.stderr).contains("2 passed; 0 failed"));
    // Expand each explicit prefix reference and compare the original whole
    // independent report. Every frame, duty, original envelope and effect stays
    // observable; the shared tables do not normalize or truncate expectations.
    for (case, expected_stages) in actual["cases"]
        .as_array()
        .unwrap()
        .iter()
        .zip(expected_stages)
    {
        assert_eq!(
            case["format"],
            "moriarty-disjoint-case-observations-draft/1"
        );
        assert_eq!(
            case["stages"].as_array().unwrap().len(),
            expected_stages.len()
        );
        for (stage, expected_stage) in case["stages"]
            .as_array()
            .unwrap()
            .iter()
            .zip(expected_stages)
        {
            assert_eq!(expand(&stage["actual"], &case["tables"]), expected_stage);
        }
    }
    // Equality here supplements the independently literal complete reports.
    // It does not replace their separate ordered histories or member effects.
    assert_eq!(
        actual["cases"][0]["stages"][4]["actual"]["post"],
        actual["cases"][1]["stages"][4]["actual"]["post"],
    );
}

#[test]
fn ordinary_expected_frame_mismatch_fails_and_retains_all_actual_reports() {
    let project = Project::new();
    // Keep the complete original observations before changing one expectation.
    // Source, scenarios, events and actual continuations are unchanged.
    let baseline = fs::read(project.0.join("public-report.json")).unwrap();
    let mut expected: Value = serde_json::from_slice(&baseline).unwrap();
    let expected_stages: Vec<Vec<Value>> = ["left-first", "right-first"]
        .into_iter()
        .map(|case| {
            (0..5)
                .map(|index| {
                    serde_json::from_slice(
                        &fs::read(project.0.join(format!("{case}-{index}.json"))).unwrap(),
                    )
                    .unwrap()
                })
                .collect()
        })
        .collect();
    // Model an ordinary transcription error in an untouched state value.
    let mut wrong = expected_stages[0][2].clone();
    assert_eq!(wrong["post"]["state"]["marker"]["value"], "77");
    wrong["post"]["state"]["marker"]["value"] = Value::String("78".into());
    fs::write(
        project.0.join("left-first-2.json"),
        serde_json::to_vec_pretty(&wrong).unwrap(),
    )
    .unwrap();
    // Only comparison flags change in the public aggregate; all actual data
    // and the later stage/case observations must remain exactly the baseline.
    expected["status"] = Value::String("failed".into());
    expected["cases"][0]["passed"] = Value::Bool(false);
    expected["cases"][0]["stages"][2]["passed"] = Value::Bool(false);
    let mut expected_bytes = serde_json::to_vec_pretty(&expected).unwrap();
    expected_bytes.push(b'\n');
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&project.0)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(output.stdout, expected_bytes);
    assert!(String::from_utf8_lossy(&output.stderr).contains("1 passed; 1 failed"));
    let cases = actual["cases"].as_array().unwrap();
    assert_eq!(cases.len(), expected_stages.len());
    for (case, expected_stages) in cases.iter().zip(expected_stages) {
        let stages = case["stages"].as_array().unwrap();
        assert_eq!(stages.len(), expected_stages.len());
        for (stage, expected_stage) in stages.iter().zip(expected_stages) {
            assert_eq!(expand(&stage["actual"], &case["tables"]), expected_stage);
        }
    }
}

```

## Full actual public test dispatch and aggregate exporter
Path: actual-source/crates/mori/src/commands/test.rs
SHA256: 6d79546f1bf9c6271ff45ca4e58b514cc2fc6f5a6b970b55f6b3df4e6788aec8
```text
use super::core_io;
use crate::shell::Shell;
use serde::Deserialize;
use std::{path::PathBuf, process::ExitCode};

#[cfg(feature = "core2-disjoint-draft")]
mod disjoint_test;
mod stage_test;

#[derive(Debug, clap::Args)]
pub struct Args {
    pub dir: PathBuf,
}
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
    expected: Expected,
}
#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum Expected {
    Accepted {
        post: mori_core::Snapshot,
        effects: Vec<mori_core::Effect>,
    },
    Rejected {
        code: String,
    },
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let value: serde_json::Value = core_io::read_json(&args.dir.join("mori-tests.json"))?;
    if value.get("profile").and_then(serde_json::Value::as_str) == Some("moriarty-tests/2") {
        return stage_test::run(&args.dir, value, shell);
    }
    if value.get("profile").and_then(serde_json::Value::as_str)
        == Some("moriarty-tests/disjoint-draft/1")
    {
        #[cfg(feature = "core2-disjoint-draft")]
        return disjoint_test::run(&args.dir, value, shell);
        #[cfg(not(feature = "core2-disjoint-draft"))]
        return Err(core_io::operation(
            "disjoint draft requires feature core2-disjoint-draft",
        ));
    }
    let manifest: Manifest = serde_json::from_value(value).map_err(core_io::operation)?;
    if manifest.profile != "moriarty-tests/1"
        || manifest.cases.is_empty()
        || manifest.cases.len() > 256
    {
        return Err(core_io::operation(
            "expected a nonempty bounded moriarty-tests/1 manifest",
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    let mut reports = Vec::new();
    let mut failed = 0;
    for case in manifest.cases {
        if case.name.is_empty() || !names.insert(case.name.clone()) {
            return Err(core_io::operation(
                "test case names must be nonempty and unique",
            ));
        }
        let source = core_io::read_source(&args.dir.join(&case.source))?;
        let scenario: mori_core::Scenario = core_io::read_json(&args.dir.join(&case.scenario))?;
        let result = mori_core::parse(&source)
            .and_then(|p| mori_core::simulate(&p, &case.action, &scenario));
        let passed = match (&case.expected, &result) {
            (Expected::Accepted { post, effects }, Ok(actual)) => {
                post == &actual.post && effects == &actual.effects
            }
            (Expected::Rejected { code }, Err(error)) => code == &error.code,
            _ => false,
        };
        if !passed {
            failed += 1;
        }
        reports.push(serde_json::json!({"name":case.name,"passed":passed,"actual":result}));
    }
    shell.status(
        "Tested",
        format!("{} passed; {} failed", reports.len() - failed, failed),
    );
    let report = aggregate_report(failed == 0, reports)?;
    println!("{report}");
    Ok(if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Pretty report identical to exporting the whole value, without applying the
/// single-document 1 MiB cap to the aggregate. Every case is exported, bounded
/// and escaped by the unchanged mori_core::json::export; only trusted
/// structural bytes are added here. Nothing is written until all cases export.
fn aggregate_report(passed: bool, mut cases: Vec<serde_json::Value>) -> super::Result<String> {
    // Field order follows serde_json::Map exactly as the legacy json! wrapper did.
    let mut order = serde_json::Map::new();
    order.insert("status".into(), serde_json::Value::Null);
    order.insert("cases".into(), serde_json::Value::Null);
    let mut out = String::from("{");
    for (field, key) in order.keys().enumerate() {
        out.push_str(if field == 0 { "\n  " } else { ",\n  " });
        if key == "status" {
            out.push_str(if passed {
                r#""status": "passed""#
            } else {
                r#""status": "failed""#
            });
            continue;
        }
        out.push_str(r#""cases": ["#);
        let count = cases.len();
        for (index, case) in std::mem::take(&mut cases).into_iter().enumerate() {
            let case = mori_core::json::export(&case).map_err(core_io::operation)?;
            out.push_str(if index == 0 { "\n    " } else { ",\n    " });
            // Pretty JSON escapes newlines inside strings, so every raw newline
            // is structural and takes the enclosing array-element indentation.
            out.push_str(&case.replace('\n', "\n    "));
        }
        out.push_str(if count == 0 { "]" } else { "\n  ]" });
    }
    out.push_str("\n}");
    Ok(out)
}

#[cfg(test)]
mod report_tests {
    use super::aggregate_report;
    use serde_json::{Value, json};

    const HOSTILE: &str = "x\n}\"\\\r\u{1b}[31m\u{7f}\u{85}\u{202e}\u{1f9ea}";

    fn case(index: usize, padding: usize) -> Value {
        json!({
            "name": format!("case-{index:03}{HOSTILE}"),
            "passed": index.is_multiple_of(2),
            "actual": {"Ok": {
                "pad": "\u{202e}".repeat(padding),
                "empty": [],
                "none": {},
                "nested": [[{"k": [1, -2, 3.5, null, true, HOSTILE]}]]
            }}
        })
    }

    fn whole(passed: bool, cases: &[Value]) -> Value {
        json!({"status": if passed { "passed" } else { "failed" }, "cases": cases})
    }

    #[test]
    fn small_aggregate_is_byte_identical_to_whole_export() {
        for passed in [true, false] {
            for count in [1, 2, 3] {
                let cases: Vec<_> = (0..count).map(|index| case(index, 1)).collect();
                let legacy = mori_core::json::export(&whole(passed, &cases)).unwrap();
                assert_eq!(aggregate_report(passed, cases).unwrap(), legacy);
            }
        }
    }

    #[test]
    fn full_256_aggregate_exceeds_whole_cap_and_round_trips_safely() {
        let cases: Vec<_> = (0..256).map(|index| case(index, 1024)).collect();
        let expected = whole(false, &cases);
        assert!(mori_core::json::export(&expected).is_err());
        let text = aggregate_report(false, cases).unwrap();
        assert!(text.len() > 1_048_576);
        assert!(text.is_ascii());
        assert!(text.bytes().all(|b| b == b'\n' || !b.is_ascii_control()));
        assert!(text.ends_with("\n}"));
        assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), expected);
    }

    #[test]
    fn any_individually_oversized_case_fails_before_output() {
        // Raw pretty size over 1 MiB, then raw size under 1 MiB but escaped size over it.
        for pad in [
            json!("x".repeat(1_048_576)),
            json!("\u{202e}".repeat(200_000)),
        ] {
            let mut cases: Vec<_> = (0..256).map(|index| case(index, 1)).collect();
            cases[255]["actual"]["Ok"]["pad"] = pad;
            assert!(mori_core::json::export(&cases[255]).is_err());
            assert!(aggregate_report(true, cases).is_err());
        }
    }

    #[test]
    fn per_case_depth_limit_applies_and_aggregate_is_not_reader_profile() {
        // Case root is depth 0 under export, so the innermost scalar of
        // `actual` sits at nested + 1 and the reader accepts depth 32.
        let deep = |nested: usize| {
            let actual = (0..nested).fold(json!(0), |inner, _| json!([inner]));
            vec![json!({"name": "deep", "passed": true, "actual": actual})]
        };
        let cases = deep(31);
        assert!(mori_core::json::export(&whole(true, &cases)).is_err());
        let text = aggregate_report(true, cases.clone()).unwrap();
        assert!(mori_core::json::parse(&text).is_err());
        assert_eq!(
            serde_json::from_str::<Value>(&text).unwrap(),
            whole(true, &cases)
        );
        assert!(aggregate_report(true, deep(32)).is_err());
    }
}

```

## Full actual disjoint public consumer
Path: actual-source/crates/mori/src/commands/test/disjoint_test.rs
SHA256: ce01456416e68b37457f20f3cd12ae4b715716d2be3c801d3cc0c2ddada88ce1
```text
use crate::{commands::core_io, shell::Shell};
use mori_core::stage::{
    self,
    disjoint::{Error as PairError, MAX_INVOCATIONS, Member, Pair},
};
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
        || manifest.cases.is_empty()
        || manifest.cases.len() > 256
    {
        return Err(core_io::operation(
            "expected a bounded disjoint-draft manifest",
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    for case in &manifest.cases {
        if case.name.is_empty()
            || !names.insert(&case.name)
            || case.events.len() > MAX_INVOCATIONS - 2
            || case.events.iter().any(|event| event.member >= 2)
        {
            return Err(core_io::operation(
                "invalid disjoint case name, member or invocation count",
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
    println!("{}", super::aggregate_report(failed == 0, reports)?);
    Ok(if failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn member(dir: &Path, input: MemberPath) -> Result<Member> {
    let source = core_io::read_source(&dir.join(input.source))?;
    let scenario: mori_core::Scenario = core_io::read_json(&dir.join(input.scenario))?;
    Ok(Member {
        program: stage::load(&source).map_err(core_io::operation)?,
        action: input.action,
        scenario,
        cap: input.logical_cap,
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
        let object = actual
            .as_object_mut()
            .ok_or_else(|| core_io::operation("pair report must be an object"))?;
        let history = object
            .remove("history")
            .ok_or_else(|| core_io::operation("pair report has no history"))?;
        let effects = object
            .remove("effects")
            .ok_or_else(|| core_io::operation("pair report has no effects"))?;
        let (Value::Array(history), Value::Array(effects)) = (history, effects) else {
            return Err(core_io::operation("pair report tables must be arrays"));
        };
        if !history.starts_with(&self.history) || !effects.starts_with(&self.effects) {
            return Err(core_io::operation(
                "pair report rewrote an earlier observed table prefix",
            ));
        }
        let history_length = history.len();
        let effects_length = effects.len();
        let previous_history = self.history.len();
        let previous_effects = self.effects.len();
        self.history.extend_from_slice(&history[previous_history..]);
        self.effects.extend_from_slice(&effects[previous_effects..]);
        let expanded_format = object
            .remove("format")
            .ok_or_else(|| core_io::operation("pair report has no format"))?;
        object.insert(
            "format".into(),
            json!("moriarty-local-disjoint-referenced-draft/1"),
        );
        object.insert("expanded_format".into(), expanded_format);
        object.insert(
            "history".into(),
            json!({"table":"history","prefix_len":history_length}),
        );
        object.insert(
            "effects".into(),
            json!({"table":"effects","prefix_len":effects_length}),
        );
        Ok(actual)
    }
}

fn run_case(dir: &Path, case: Case) -> Result<Value> {
    // Freeze complete expected observations before invoking either program.
    let expected_start: Value = core_io::read_json(&dir.join(case.expected))?;
    let expected_events: Vec<Value> = case
        .events
        .iter()
        .map(|event| core_io::read_json(&dir.join(&event.expected)))
        .collect::<Result<_>>()?;
    let events: Vec<_> = case
        .events
        .iter()
        .map(|event| {
            stage::decode::read_bounded(
                &dir.join(&event.event),
                stage::decode::MAX_EVENT_BYTES,
                stage::decode::EVENT_TOO_LARGE,
            )
            .and_then(|text| stage::decode::event(&text))
            .map_err(core_io::operation)
        })
        .collect::<Result<_>>()?;
    let [left, right] = case.members;
    let members = [member(dir, left)?, member(dir, right)?];
    let mut pair = Pair::start(members).map_err(pair_error)?;
    let actual = pair.report().map_err(pair_error)?;
    let mut passed = actual == expected_start;
    let mut tables = ObservationTables::default();
    let mut stages = vec![json!({"index":0,"passed":passed,"actual":tables.reference(actual)?})];
    for (index, ((step, event), expected)) in case
        .events
        .iter()
        .zip(events)
        .zip(expected_events)
        .enumerate()
    {
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

```

## Whole actual CHECK-PLAN.json
Path: actual-records/CHECK-PLAN.json
SHA256: 2678845edc28bd014966075849399fc30ca4c3b37537964cd3c156d079b5fcd6
```text
{
  "recordedAt": "2026-10-18T13:09:54.175173+00:00",
  "head": "2f1b969ae4153c4f4f05ffe27a285a12e20f09cd",
  "tree": "892b5934ddd9fe7dfe5e9999f992ae4ac8842bff",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--features",
    "core2-disjoint-draft",
    "--test",
    "disjoint_test_public",
    "ordinary_expected_frame_mismatch_fails_and_retains_all_actual_reports",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "environment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-composition-aggregate-tx8x5khc/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-composition-aggregate-tx8x5khc/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-composition-aggregate-tx8x5khc/tmp"
  },
  "ownedTemporaryRoot": "/tmp/doctorwatson-composition-aggregate-tx8x5khc",
  "absentAncestorConfigs": [
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/.cargo/config",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/.cargo/config.toml",
    "/home/charl/Moriarty/.worktrees/.cargo/config",
    "/home/charl/Moriarty/.worktrees/.cargo/config.toml",
    "/home/charl/Moriarty/.cargo/config",
    "/home/charl/Moriarty/.cargo/config.toml",
    "/home/charl/.cargo/config",
    "/home/charl/.cargo/config.toml",
    "/home/.cargo/config",
    "/home/.cargo/config.toml",
    "/.cargo/config",
    "/.cargo/config.toml",
    "/tmp/doctorwatson-composition-aggregate-tx8x5khc/cwd/.cargo/config",
    "/tmp/doctorwatson-composition-aggregate-tx8x5khc/cwd/.cargo/config.toml",
    "/tmp/doctorwatson-composition-aggregate-tx8x5khc/.cargo/config",
    "/tmp/doctorwatson-composition-aggregate-tx8x5khc/.cargo/config.toml",
    "/tmp/.cargo/config",
    "/tmp/.cargo/config.toml"
  ],
  "toolPins": [
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
      "sha256": "e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
      "sha256": "f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
      "sha256": "a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"
    },
    {
      "path": "/usr/bin/timeout",
      "sha256": "f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"
    }
  ],
  "scope": "One ordinary public composition comparison/aggregate regression; no SDK/native/key/proof/wallet/ledger call. Existing financial expectations unchanged; no financial allocation reset.",
  "expected": "Exactly one selected test passes, old positive test filtered; complete failed aggregate and ten expanded reports equal frozen goldens."
}

```

## Whole actual START.json
Path: actual-records/START.json
SHA256: c4329fdf54c35df3e8da40f09ac00ce9b8a2acac00ec987c79fabd05b8266427
```text
{
  "startedAt": "2026-10-18T13:09:54.222599+00:00",
  "head": "2f1b969ae4153c4f4f05ffe27a285a12e20f09cd",
  "tree": "892b5934ddd9fe7dfe5e9999f992ae4ac8842bff",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--features",
    "core2-disjoint-draft",
    "--test",
    "disjoint_test_public",
    "ordinary_expected_frame_mismatch_fails_and_retains_all_actual_reports",
    "--",
    "--exact",
    "--test-threads=1"
  ]
}

```

## Whole actual RESULT.json
Path: actual-records/RESULT.json
SHA256: 507b5d28352e71e2f8dd3f9babc6f2f97756066ea64f3daba6397405463bfd77
```text
{
  "finishedAt": "2026-10-18T13:10:51.081693+00:00",
  "exitCode": 0,
  "wallSeconds": 56.859117027997854,
  "stdoutSha256": "465cfe3d789d2a03d34108a9ef2e1a3f12a2ed206c8633c1f0c53cbfcfbbd221",
  "stderrSha256": "1105a81c4ed1e2aafb4a3f960e8c6a54abb298127840f79ca36151cd2117f1aa",
  "ownedSessionRemaining": [],
  "head": "2f1b969ae4153c4f4f05ffe27a285a12e20f09cd",
  "tree": "892b5934ddd9fe7dfe5e9999f992ae4ac8842bff",
  "sourcePinsUnchanged": true,
  "workingTreeClean": true
}

```

## Whole actual CHILD.json
Path: actual-records/CHILD.json
SHA256: 5cd25be585ddf1e10105704e1d3fd45cd6f35a25c2101a763f87b518d1b0ac37
```text
{
  "pid": 737863,
  "session": 737863
}

```

## Whole actual CLEANUP.json
Path: actual-records/CLEANUP.json
SHA256: 6d4f37a2fd8cd4edf154976478e7d37f32afbb5b2be9c3cc07cffd75b584cfda
```text
{
  "recordedAt": "2026-10-18T13:10:51.179318+00:00",
  "ownedRoot": "/tmp/doctorwatson-composition-aggregate-tx8x5khc",
  "existsAfter": false,
  "resultRecorded": true
}

```

## Whole actual STDOUT.log
Path: actual-records/STDOUT.log
SHA256: 465cfe3d789d2a03d34108a9ef2e1a3f12a2ed206c8633c1f0c53cbfcfbbd221
```text

running 1 test
test ordinary_expected_frame_mismatch_fails_and_retains_all_actual_reports ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.24s


```

## Whole actual STDERR.log
Path: actual-records/STDERR.log
SHA256: 1105a81c4ed1e2aafb4a3f960e8c6a54abb298127840f79ca36151cd2117f1aa
```text
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling syn v3.0.6
   Compiling cfg-if v1.0.5
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling typenum v1.20.1
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling zeroize v1.9.1
   Compiling writeable v0.6.4
   Compiling version_check v0.9.5
   Compiling libc v0.2.190
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling icu_locale_fallback_data v2.3.0
   Compiling rustix v1.1.5
   Compiling litemap v0.8.3
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling bitflags v2.13.2
   Compiling subtle v2.6.1
   Compiling linux-raw-sys v0.12.1
   Compiling const-oid v0.9.6
   Compiling icu_segmenter_data v2.3.0
   Compiling icu_provider v2.3.1
   Compiling utf8_iter v1.0.4
   Compiling icu_collections v2.3.0
   Compiling icu_locale_fallback v2.3.0
   Compiling smallvec v1.16.2
   Compiling icu_segmenter v2.3.0
   Compiling unicode-width v0.2.2
   Compiling is_ci v1.2.0
   Compiling supports-color v3.0.2
   Compiling textwrap v0.16.4
   Compiling terminal_size v0.4.4
   Compiling crypto-common v0.1.6
   Compiling block-buffer v0.10.4
   Compiling getrandom v0.2.17
   Compiling owo-colors v4.4.0
   Compiling supports-unicode v3.0.0
   Compiling supports-hyperlinks v3.2.0
   Compiling unicode-width v0.1.14
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-span)
   Compiling der v0.7.10
   Compiling memchr v2.8.3
   Compiling crossbeam-utils v0.8.23
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling crossbeam-epoch v0.9.21
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-lexer)
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling crossbeam-deque v0.8.8
   Compiling base16ct v0.2.0
   Compiling serde_core v1.0.229
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling utf8parse v0.2.2
   Compiling once_cell v1.21.4
   Compiling zmij v1.0.23
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling anstyle-query v1.1.5
   Compiling serde v1.0.229
   Compiling is_terminal_polyfill v1.70.2
   Compiling rayon-core v1.13.0
   Compiling const-oid v0.10.2
   Compiling thiserror v2.0.21
   Compiling getrandom v0.4.3
   Compiling cpufeatures v0.2.17
   Compiling anstyle v1.0.14
   Compiling serde_json v1.0.151
   Compiling colorchoice v1.0.5
   Compiling regex-syntax v0.8.11
   Compiling regex-automata v0.4.18
   Compiling anstream v1.0.0
   Compiling sha2 v0.10.9
   Compiling digest v0.11.3
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling thiserror-impl v2.0.21
   Compiling serde_derive v1.0.229
   Compiling itoa v1.0.18
   Compiling strsim v0.11.1
   Compiling same-file v1.0.6
   Compiling similar v2.7.0
   Compiling log v0.4.34
   Compiling heck v0.5.0
   Compiling cpufeatures v0.3.1
   Compiling clap_lex v1.1.1
   Compiling clap_builder v4.6.7
   Compiling sha2 v0.11.0
   Compiling clap_derive v4.6.7
   Compiling globset v0.4.20
   Compiling walkdir v2.5.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-parser)
   Compiling either v1.19.0
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling rayon v1.12.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 56.59s
     Running tests/disjoint_test_public.rs (/tmp/doctorwatson-composition-aggregate-tx8x5khc/target/debug/deps/disjoint_test_public-5fc2732fdd9981c0)

```

## EXPECTATION-DERIVATION.json
Path: EXPECTATION-DERIVATION.json
SHA256: 511f88f91bef59c55da05f972e1768dc333cec41aca3d92449bdc8251c459fc9
```text
{
  "baseAggregateSha256": "0f51b2c2c1eb3d259d3c5b69689924761822e7f9b50809a4497450b2037153f5",
  "baseCustodyFreezeSha256": "12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848",
  "unchangedIndependentWholeReports": [
    "left-first-0.json",
    "left-first-1.json",
    "left-first-2.json",
    "left-first-3.json",
    "left-first-4.json",
    "right-first-0.json",
    "right-first-1.json",
    "right-first-2.json",
    "right-first-3.json",
    "right-first-4.json",
    "public-report.json"
  ],
  "all11CurrentFixtureBytesEqualCustody": true,
  "sourceTestInputChange": {
    "file": "temporary Project/left-first-2.json",
    "jsonPointer": "/post/state/marker/value",
    "before": "77",
    "after": "78",
    "scope": "only expected comparison input, no producer or actual continuation change"
  },
  "expectedAggregateChanges": [
    {
      "jsonPointer": "/status",
      "value": "failed"
    },
    {
      "jsonPointer": "/cases/0/passed",
      "value": false
    },
    {
      "jsonPointer": "/cases/0/stages/2/passed",
      "value": false
    }
  ],
  "allOtherExpectedAggregateValuesUnchanged": true,
  "derivedFailedAggregateBytes": 839112,
  "derivedFailedAggregateSha256": "96ed426bef4d9804205d009fca5036b39592f2fdfab5754fdf02c2a68ca7a6d8",
  "independenceQualification": "No new financial values. Reuses accepted independent finance/work custody with original related-source exposure and implementation-derived wire qualification. Failure-wrapper flags are literal ordinary expected-test contract, not new independent execution goldens.",
  "execution": false
}

```

## FINANCIAL-EXPECTATION-VIEW.json
Path: FINANCIAL-EXPECTATION-VIEW.json
SHA256: 5e5f8a1f8173d6e9b807e7b00b39c1cd942259a58fe78ce33b2a842c18375354
```text
{
  "qualification": "Extracted review view of unchanged existing independent goldens, not new evaluator output or full report replacement",
  "observations": [
    {
      "case": "left-first",
      "stage": 0,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "100",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "13",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "7",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "47",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "0"
          },
          "basis": {
            "type": "UInt",
            "value": "35"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "1"
          }
        }
      },
      "frames": [
        {
          "captures": [],
          "site": 1
        },
        {
          "captures": [
            {
              "name": "extra",
              "value": {
                "type": "UInt",
                "value": "2"
              }
            },
            {
              "name": "held",
              "value": {
                "type": "UInt",
                "value": "42"
              }
            }
          ],
          "site": 1
        }
      ],
      "logical": {
        "attempted_total": "16",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "5",
          "11"
        ],
        "committed_total": "16"
      },
      "residual": [
        {
          "amount": "47",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 2,
      "effectsLength": 4
    },
    {
      "case": "left-first",
      "stage": 1,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "100",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "47",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "0"
          },
          "basis": {
            "type": "UInt",
            "value": "35"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "1"
          }
        }
      },
      "frames": [
        {
          "captures": [
            {
              "name": "ok",
              "value": {
                "type": "Bool",
                "value": true
              }
            }
          ],
          "site": 2
        },
        {
          "captures": [
            {
              "name": "extra",
              "value": {
                "type": "UInt",
                "value": "2"
              }
            },
            {
              "name": "held",
              "value": {
                "type": "UInt",
                "value": "42"
              }
            }
          ],
          "site": 1
        }
      ],
      "logical": {
        "attempted_total": "21",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "10",
          "11"
        ],
        "committed_total": "21"
      },
      "residual": [
        {
          "amount": "47",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 3,
      "effectsLength": 5
    },
    {
      "case": "left-first",
      "stage": 2,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "58",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "5",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "42"
          },
          "basis": {
            "type": "UInt",
            "value": "78"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "1"
          }
        }
      },
      "frames": [
        {
          "captures": [
            {
              "name": "ok",
              "value": {
                "type": "Bool",
                "value": true
              }
            }
          ],
          "site": 2
        },
        {
          "captures": [
            {
              "name": "extra",
              "value": {
                "type": "UInt",
                "value": "2"
              }
            },
            {
              "name": "held",
              "value": {
                "type": "UInt",
                "value": "42"
              }
            },
            {
              "name": "quote",
              "value": {
                "type": "UInt",
                "value": "43"
              }
            }
          ],
          "site": 2
        }
      ],
      "logical": {
        "attempted_total": "36",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "10",
          "26"
        ],
        "committed_total": "36"
      },
      "residual": [
        {
          "amount": "5",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 4,
      "effectsLength": 9
    },
    {
      "case": "left-first",
      "stage": 3,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "58",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "5",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "42"
          },
          "basis": {
            "type": "UInt",
            "value": "78"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "2"
          }
        }
      },
      "frames": [
        {
          "captures": [],
          "site": 0
        },
        {
          "captures": [
            {
              "name": "extra",
              "value": {
                "type": "UInt",
                "value": "2"
              }
            },
            {
              "name": "held",
              "value": {
                "type": "UInt",
                "value": "42"
              }
            },
            {
              "name": "quote",
              "value": {
                "type": "UInt",
                "value": "43"
              }
            }
          ],
          "site": 2
        }
      ],
      "logical": {
        "attempted_total": "42",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "16",
          "26"
        ],
        "committed_total": "42"
      },
      "residual": [
        {
          "amount": "5",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 5,
      "effectsLength": 11
    },
    {
      "case": "left-first",
      "stage": 4,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "56",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "2",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "5",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "42"
          },
          "basis": {
            "type": "UInt",
            "value": "78"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "2"
          }
        }
      },
      "frames": [
        {
          "captures": [],
          "site": 0
        },
        {
          "captures": [],
          "site": 0
        }
      ],
      "logical": {
        "attempted_total": "52",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "16",
          "36"
        ],
        "committed_total": "52"
      },
      "residual": [
        {
          "amount": "5",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 6,
      "effectsLength": 13
    },
    {
      "case": "right-first",
      "stage": 0,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "100",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "13",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "7",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "47",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "0"
          },
          "basis": {
            "type": "UInt",
            "value": "35"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "1"
          }
        }
      },
      "frames": [
        {
          "captures": [],
          "site": 1
        },
        {
          "captures": [
            {
              "name": "extra",
              "value": {
                "type": "UInt",
                "value": "2"
              }
            },
            {
              "name": "held",
              "value": {
                "type": "UInt",
                "value": "42"
              }
            }
          ],
          "site": 1
        }
      ],
      "logical": {
        "attempted_total": "16",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "5",
          "11"
        ],
        "committed_total": "16"
      },
      "residual": [
        {
          "amount": "47",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 2,
      "effectsLength": 4
    },
    {
      "case": "right-first",
      "stage": 1,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "58",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "13",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "7",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "5",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "42"
          },
          "basis": {
            "type": "UInt",
            "value": "78"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "1"
          }
        }
      },
      "frames": [
        {
          "captures": [],
          "site": 1
        },
        {
          "captures": [
            {
              "name": "extra",
              "value": {
                "type": "UInt",
                "value": "2"
              }
            },
            {
              "name": "held",
              "value": {
                "type": "UInt",
                "value": "42"
              }
            },
            {
              "name": "quote",
              "value": {
                "type": "UInt",
                "value": "43"
              }
            }
          ],
          "site": 2
        }
      ],
      "logical": {
        "attempted_total": "31",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "5",
          "26"
        ],
        "committed_total": "31"
      },
      "residual": [
        {
          "amount": "5",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 3,
      "effectsLength": 8
    },
    {
      "case": "right-first",
      "stage": 2,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "58",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "5",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "42"
          },
          "basis": {
            "type": "UInt",
            "value": "78"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "1"
          }
        }
      },
      "frames": [
        {
          "captures": [
            {
              "name": "ok",
              "value": {
                "type": "Bool",
                "value": true
              }
            }
          ],
          "site": 2
        },
        {
          "captures": [
            {
              "name": "extra",
              "value": {
                "type": "UInt",
                "value": "2"
              }
            },
            {
              "name": "held",
              "value": {
                "type": "UInt",
                "value": "42"
              }
            },
            {
              "name": "quote",
              "value": {
                "type": "UInt",
                "value": "43"
              }
            }
          ],
          "site": 2
        }
      ],
      "logical": {
        "attempted_total": "36",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "10",
          "26"
        ],
        "committed_total": "36"
      },
      "residual": [
        {
          "amount": "5",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 4,
      "effectsLength": 9
    },
    {
      "case": "right-first",
      "stage": 3,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "56",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "2",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "5",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "42"
          },
          "basis": {
            "type": "UInt",
            "value": "78"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "1"
          }
        }
      },
      "frames": [
        {
          "captures": [
            {
              "name": "ok",
              "value": {
                "type": "Bool",
                "value": true
              }
            }
          ],
          "site": 2
        },
        {
          "captures": [],
          "site": 0
        }
      ],
      "logical": {
        "attempted_total": "46",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "10",
          "36"
        ],
        "committed_total": "46"
      },
      "residual": [
        {
          "amount": "5",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 5,
      "effectsLength": 11
    },
    {
      "case": "right-first",
      "stage": 4,
      "post": {
        "balances": [
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "reserve",
            "amount": "56",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "target",
            "amount": "42",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "2",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "alice",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "11",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          },
          {
            "account": "treasury",
            "amount": "0",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "5",
            "asset": {
              "issuer": "continuation-reserve-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "target",
            "debtor": "reserve",
            "name": "allocation"
          },
          {
            "amount": "5",
            "asset": {
              "issuer": "independent-staged-test-oracle",
              "network": "local",
              "reference": "UNIT",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "alice",
            "name": "duty"
          }
        ],
        "state": {
          "allocated": {
            "type": "UInt",
            "value": "42"
          },
          "basis": {
            "type": "UInt",
            "value": "78"
          },
          "marker": {
            "type": "UInt",
            "value": "77"
          },
          "phase": {
            "type": "UInt",
            "value": "2"
          }
        }
      },
      "frames": [
        {
          "captures": [],
          "site": 0
        },
        {
          "captures": [],
          "site": 0
        }
      ],
      "logical": {
        "attempted_total": "52",
        "cap_total": "52",
        "caps": [
          "16",
          "36"
        ],
        "committed": [
          "16",
          "36"
        ],
        "committed_total": "52"
      },
      "residual": [
        {
          "amount": "5",
          "asset": {
            "issuer": "continuation-reserve-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "target",
          "debtor": "reserve",
          "name": "allocation"
        },
        {
          "amount": "5",
          "asset": {
            "issuer": "independent-staged-test-oracle",
            "network": "local",
            "reference": "UNIT",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "alice",
          "name": "duty"
        }
      ],
      "historyLength": 6,
      "effectsLength": 13
    }
  ]
}

```

## OBSERVED-SCOPE.json
Path: OBSERVED-SCOPE.json
SHA256: 7faf5e84d01c6aba4e4347c009671899c91f5c09c25c249403fb91fd3e6f2001
```text
{
  "actualSubject": "2f1b969ae4153c4f4f05ffe27a285a12e20f09cd",
  "actualTree": "892b5934ddd9fe7dfe5e9999f992ae4ac8842bff",
  "rootTerminalResult": {
    "finishedAt": "2026-10-18T13:10:51.081693+00:00",
    "exitCode": 0,
    "wallSeconds": 56.859117027997854,
    "stdoutSha256": "465cfe3d789d2a03d34108a9ef2e1a3f12a2ed206c8633c1f0c53cbfcfbbd221",
    "stderrSha256": "1105a81c4ed1e2aafb4a3f960e8c6a54abb298127840f79ca36151cd2117f1aa",
    "ownedSessionRemaining": [],
    "head": "2f1b969ae4153c4f4f05ffe27a285a12e20f09cd",
    "tree": "892b5934ddd9fe7dfe5e9999f992ae4ac8842bff",
    "sourcePinsUnchanged": true,
    "workingTreeClean": true
  },
  "counts": {
    "RustSelectedFunctions": 1,
    "RustPassed": 1,
    "RustFailed": 0,
    "oldPositiveRustFunctionFiltered": 1,
    "actualChildCLIInvocationsAssertedByPassingTest": 1,
    "unchangedInputCases": 2,
    "unchangedSourcePrograms": 2,
    "unchangedExplicitSchedules": 2,
    "fullPairObservationsCompared": 10,
    "fullLosslessExpansionsCompared": 10,
    "wholeFailedAggregateJSONAndBytesCompared": 1,
    "actualStageCallsAcrossBothCases": 12,
    "newFinancialScenarios": 0,
    "newProgramPlanFixtureComparisons": 0,
    "newSemanticsOrSIChoices": 0
  },
  "outerColdWrapperSeconds": 56.859117027997854,
  "RustHarnessReportedSeconds": 0.24,
  "innerCLIResult": "exit1 and 1passed1failed substring plus complete JSON/byte expectations asserted inside passing Rust function; not separately dumped raw stdout/stderr",
  "physicalAggregate": "DERIVED-FAILED-PUBLIC-REPORT.json is expected derivative839112bytes, NEVER relabeled as captured actual CLI stdout",
  "sourcePinIntersectionMatchedToRootInventory": 81,
  "providerDispatch": false,
  "reviewVerdict": "NOT REQUESTED FROM PROVIDER / NO RESULT REVIEW YET",
  "publication": "Root reports draft PR228 created/pushed; no remote GitHub query/merge attestation performed here",
  "formatterEvidence": "PR-BODY.md reports rustfmt check passed; this packet has no separate formatter command receipt, so only root-reported formatting status",
  "retainedLimits": [
    "Original nonblind related-source and narrow Core premise qualifications; wire conditional on authored codec",
    "No new finance scenario/complete ProgramPlan observation",
    "No StageRejection/false-guard behavior, owner security/authentication or new shared-state semantics judgment",
    "All5fulloperators/parents/native/proof/resource/ledger/target exits remain open"
  ]
}

```

## ACTUAL-RUNTIME-PATH.md
Path: ACTUAL-RUNTIME-PATH.md
SHA256: 0d4a28c180a4efec0210e9cbb836dedc4ddc52f8d722b2ef5b98e121d166d9a9
```text
# Exact actual source runtime path

Subject 2f1b969ae4153c4f4f05ffe27a285a12e20f09cd/tree892b5934ddd9fe7dfe5e9999f992ae4ac8842bff, direct parent e4d7a2b5709caa18a94f5256d03a0302364b8235. These are immutable Git objects, not ambient J/K branch state. Complete selected owned source bodies are in actual-source/ and entry pins in SOURCE-CALL-MATRIX.md. Third-party/std/build scripts/proc macros are not newly audited.

1. The selected Rust test uses existing `Project::new`: it creates one owned temp directory and copies root regular public fixtures. It reads independent expected JSON, changes only the one temporary expected marker value and constructs the failed wrapper before any command. `std::process::Command` starts the actual `CARGO_BIN_EXE_mori --color never test <Project>` exactly once. No replacement evaluator, test-only success wrapper or library call substitutes for the binary.
2. `main` runs clap Cli::parse, constructs Shell using normal verbosity/colorNever, and calls commands::run. Its Command::Test arm calls test::run. The result expected here is Ok(ExitCode::FAILURE), so the command-error renderer is not this mismatch path.
3. `test::run` reads only this directory's mori-tests.json using core_io::read_json. Source reads use a bounded std File reader; JSON goes through mori_core::json::parse and serde schema decoding. Exact profile moriarty-tests/disjoint-draft/1 selects the feature-enabled disjoint_test::run. Generic `/1` and staged `/2` consumers are not selected.
4. `disjoint_test::run` decodes/validates its bounded manifest and iterates both cases. `run_case` preloads complete expected reports, then bounded event strings and strict typed StageEvents. Each member reads its actual source/scenario and calls public stage::load. No expected file provides runtime state, captures or a continuation.
5. `stage::load` calls parser::parse_v2 → parse_profile → mori_lexer::lex. The lexer uses its owned bounded scanner, token/span, number and string functions. Parser builds the explicit `/2` Program. Admission then calls require_core2, validate_v2, plan::build, plan_invariants, cost::fill, cost_invariants and existing program/plan digests. Type/asset/identifier/liability-read checks reuse runtime helpers. Digest uses only existing intent::canonical/hex and SHA256; no intent authentication/signature or proof operation is called.
6. `Pair::start` checks whole-program disjoint declared state/exact assets/duty names, selects the fixed left-then-right local ordering, and calls stage::start for each real action with its own scenario/cap. Start performs scenario_snapshot validation/canonical ordering, stored plan-cap preflight, input collection and canonical_boundary before local segment execution. `run_segment` clones the actual boundary, seeds state/input locals and invokes the same scalar execute_statement/evaluate/eval_cached/amount/move_funds helpers. Local Meter::new/used/charge accounts logical work. No FeeCapability, federation authorization, source sponsorship, replay, Sherlock/VM, ledger or native operation is selected.
7. `Admitted::finish` requires exact static cost and source-plan exit/capture invariants. Waiting continuations are serialized/strictly decoded and canonical-boundary checked by emit_checked. It retains actual segment/Execution/Step/value/snapshot/effect/frame data and positive residual duties. This source path is unchanged.
8. Each explicit event drives Pair::resume → existing stage::resume. It keeps that member's own continuation and R0–R7 checks/stored cap, then uses the same local segment path. Pair updates only the selected member after a complete merge check and appends the original outcome. There is no boundary rebasing or shared-state operator.
9. Pair::report merges both complete disjoint boundaries, obtains actual frames, counts committed/attempted work, concatenates full ordered effects and renders original latest/history StageOutcome objects via outcome_report. JSON object construction does not run a second evaluator.
10. The whole actual report is compared to each expected file before ObservationTables::reference removes only repeated cumulative arrays into case-local tables and full prefix references. The deliberate expected typo makes only that comparison false. `passed &= matched` retains the case failure while later real events still execute. The other case still runs.
11. `disjoint_test::run` counts one failed case, emits Shell::status “1 passed; 1 failed”, and passes every case to existing aggregate_report. That exporter uses the unchanged per-case mori_core::json::export, strict JSON bounds, ASCII rendering and existing structural indentation. Only after all case exports succeed does println publish the full aggregate plus LF. It returns FAILURE, which main returns as process exit1. The test requires parseable whole stdout and exact bytes, not merely an exit code.
12. The Rust test compares the full failed aggregate/bytes, explicitly checks case/stage membership, then expands all ten actual report prefixes against their full original independent reports. Existing Project Drop attempts owned-directory cleanup. Actual child/group/temp cleanup remains something root must observe during its authorized command; this source inspection does not claim it occurred.

Actual E4D→2f changes exactly the one integration-test file by62 lines. The prepared K823 check_source adapter is absent from this tested subject; its three source-context changes were never part of this actual command. The selected test path and all unchanged local composition/serialization contracts are bound to actual2f and its full89 physical source bodies. No future source/base is preapproved.

The selected feature forwards only core2-disjoint-draft to the core empty feature. Native/default, source6-expansion and midnight-compat-tests are unselected in the retained actual manifests. Actual root compilation/stdout/stderr are retained separately; this worker executed no product/provider check.

```

## SOURCE-CALL-MATRIX.md
Path: SOURCE-CALL-MATRIX.md
SHA256: e7bfbad881867633e38f7530fd8320db125bd8226b51a8bb6ff63aea41b8da9f
```text
# Actual2f source entry pins

Complete file bytes live in actual-source/; hashes identify source evidence, not a compiled artifact, ABI or native identity.

| Source | Line | SHA256 |
|---|---:|---|
| `crates/mori/src/main.rs` | 16 | `35988592bec7038a7674403a0e897184d1099dd4164e89a3ca0aa22ab0b3a232` |
| `crates/mori/src/commands/mod.rs` | 62 | `46a297c1a63b440a3fff1fddacac3953c36e39cc3e3b088ba229494005e8d091` |
| `crates/mori/src/commands/test.rs` | 41 | `6d79546f1bf9c6271ff45ca4e58b514cc2fc6f5a6b970b55f6b3df4e6788aec8` |
| `crates/mori/src/commands/test.rs` | 107 | `6d79546f1bf9c6271ff45ca4e58b514cc2fc6f5a6b970b55f6b3df4e6788aec8` |
| `crates/mori/src/commands/core_io.rs` | 237 | `6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594` |
| `crates/mori/src/commands/test/disjoint_test.rs` | 45 | `ce01456416e68b37457f20f3cd12ae4b715716d2be3c801d3cc0c2ddada88ce1` |
| `crates/mori/src/commands/test/disjoint_test.rs` | 158 | `ce01456416e68b37457f20f3cd12ae4b715716d2be3c801d3cc0c2ddada88ce1` |
| `crates/mori/src/commands/test/disjoint_test.rs` | 114 | `ce01456416e68b37457f20f3cd12ae4b715716d2be3c801d3cc0c2ddada88ce1` |
| `crates/mori/src/commands/test/disjoint_test.rs` | 195 | `ce01456416e68b37457f20f3cd12ae4b715716d2be3c801d3cc0c2ddada88ce1` |
| `crates/mori/src/commands/test/disjoint_test.rs` | 196 | `ce01456416e68b37457f20f3cd12ae4b715716d2be3c801d3cc0c2ddada88ce1` |
| `crates/mori-core/src/stage/mod.rs` | 75 | `e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23` |
| `crates/mori-core/src/stage/disjoint.rs` | 242 | `5e7aac9b6053657b06362957b14a09b7db5ccf74027f2b97ddeec112923047f1` |
| `crates/mori-core/src/stage/disjoint.rs` | 304 | `5e7aac9b6053657b06362957b14a09b7db5ccf74027f2b97ddeec112923047f1` |
| `crates/mori-core/src/stage/disjoint.rs` | 353 | `5e7aac9b6053657b06362957b14a09b7db5ccf74027f2b97ddeec112923047f1` |
| `crates/mori-core/src/stage/admit.rs` | 222 | `12474f92ad0752cfeee1a6054c5b251370dc08d8a17949f7747878ada60450c3` |
| `crates/mori-core/src/stage/admit.rs` | 287 | `12474f92ad0752cfeee1a6054c5b251370dc08d8a17949f7747878ada60450c3` |
| `crates/mori-core/src/runtime.rs` | 942 | `eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930` |
| `crates/mori-core/src/stage/report.rs` | 7 | `25e2b88e527a3f9b90951a92fc92ba8c17dd6f16040529dcac0356bb36707b4f` |
| `crates/mori-core/src/json.rs` | 117 | `faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183` |
| `crates/mori/src/shell.rs` | 41 | `b2116fcd9bb80abfb175f6dc2da6327b4d9787cfa3d56481fde705e62d94fff1` |
| `crates/mori/tests/disjoint_test_public.rs` | 138 | `b522275389d57247a68c36c3e28e7e174a6035d644b5b7cbaa6b592a865b4c41` |
| `docs/NATIVE-CORE.md` | 236 | `904bddfaa9eabe49f5e935a6e704ca65500a2edb975e17500bd56088b783adce` |

```

## SCOPE.md
Path: SCOPE.md
SHA256: 8f64fbdf0b8ed290cb54df60e4b1a0fefd69c9034b80e6b1ebe915495f577d9c
```text
Actual selected result: one new Rust function PASS, one existing function filtered, one inner public CLI invocation, two unchanged cases/schedules over two existing programs, ten fullPair comparisons, ten fullprefixexpansions and one839112byte completefailedaggregate comparison. Source onefile62lines only. No new financialscenario or direct completeProgramPlanfixture observation.

Sequential: existinglocalstage behavior unchanged; no new fulloperator/nativeStep claim. Disjointparallel: this ordinary comparison-failure consumer result observes the existing qualified local Pair only. Shared-stateinterleaving: no sharedheadoperator implemented; SI1–SI4 unadopted. Atomicsynchronization: no Join/sameintentmulti-action operation observed. Asynchronousmessaging: no message/pending/unknown/settled/recovery behavior observed. Allfivefulloperators remainseparately required/open.

Original independentvalues retain nonblindrelated-sourceexposure/finiteCorepremise and authoredcodecconditionalwire qualification. All originalattempts/caps/failures/campaigns/parents/U0–U7/M1–M17/MC/SP/G/R7/native/proof/resource/ledger/target exits remainunchanged/open. Security/authentication/hostile-input judgments areoutside this ordinaryreview.

```

## Exact existing input two-deliveries.mori
Path: actual-source/crates/fixtures/core-disjoint-product/two-deliveries.mori
SHA256: b1f89331a3992e8f5135d9af6361a2347d0b45c0be74b0962c63ceb48de640c7
```text
language "moriarty/2";
agreement TwoDeliveries {
  state phase: UInt = 0;
  asset UNIT = { network: "local", issuer: "independent-staged-test-oracle", reference: "UNIT", scale: 0 };
  action deliver() {
    transfer(UNIT, "alice", "bob", 7);
    next phase = 1;
    await approval(ok: Bool) when ok;
    transfer(UNIT, "alice", "bob", 3);
    await receipt(seen: Bool) when seen;
    next phase = 2;
    emit "done"(phase);
  }
  action declare() { liability("duty", UNIT, "alice", "bob", 0); }
}

```

## Exact existing input reserve.mori
Path: actual-source/crates/fixtures/core-disjoint-product/reserve.mori
SHA256: 260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530
```text
language "moriarty/2";
agreement ReserveContinuationOracle {
  asset CREDIT = { network: "local", issuer: "continuation-reserve-oracle", reference: "UNIT", scale: 0 };
  state basis: UInt = 40;
  state allocated: UInt = 0;
  state marker: UInt = 77;
  action allocate(extra: UInt) {
    let held = basis + extra;
    next basis = basis - 5;
    liability("allocation", CREDIT, "reserve", "target", held);
    await approval(quote: UInt) when quote >= held;
    next allocated = held;
    transfer(CREDIT, "reserve", "target", held);
    discharge("allocation", held);
    next basis = basis + quote;
    await settlement(ready: Bool) when ready;
    requires quote <= 50;
    fee(CREDIT, "reserve", "treasury", extra);
    emit "settled_quote"(quote);
  }
}

```

## Exact existing input two-deliveries-scenario.json
Path: actual-source/crates/fixtures/core-disjoint-product/two-deliveries-scenario.json
SHA256: 5081ad889c98bed4e0ccd31f8f741ac856188523e6885d54832f60681e42e514
```text
{
  "balances": [
    {
      "account": "alice",
      "amount": "20",
      "asset": {
        "issuer": "independent-staged-test-oracle",
        "network": "local",
        "reference": "UNIT",
        "scale": 0
      }
    },
    {
      "account": "bob",
      "amount": "0",
      "asset": {
        "issuer": "independent-staged-test-oracle",
        "network": "local",
        "reference": "UNIT",
        "scale": 0
      }
    },
    {
      "account": "observer",
      "amount": "11",
      "asset": {
        "issuer": "independent-staged-test-oracle",
        "network": "local",
        "reference": "UNIT",
        "scale": 0
      }
    },
    {
      "account": "treasury",
      "amount": "0",
      "asset": {
        "issuer": "independent-staged-test-oracle",
        "network": "local",
        "reference": "UNIT",
        "scale": 0
      }
    }
  ],
  "inputs": {},
  "liabilities": [
    {
      "amount": "5",
      "asset": {
        "issuer": "independent-staged-test-oracle",
        "network": "local",
        "reference": "UNIT",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "alice",
      "name": "duty"
    }
  ],
  "state": {
    "phase": {
      "type": "UInt",
      "value": "0"
    }
  }
}

```

## Exact existing input reserve-scenario.json
Path: actual-source/crates/fixtures/core-disjoint-product/reserve-scenario.json
SHA256: acddee29960a1d8850d0df27b0151d24da3247f2c8a37bbe9a8297a17b3971a4
```text
{
  "state": {
    "basis": {
      "type": "UInt",
      "value": "40"
    },
    "allocated": {
      "type": "UInt",
      "value": "0"
    },
    "marker": {
      "type": "UInt",
      "value": "77"
    }
  },
  "inputs": {
    "extra": {
      "type": "UInt",
      "value": "2"
    }
  },
  "balances": [
    {
      "asset": {
        "network": "local",
        "issuer": "continuation-reserve-oracle",
        "reference": "UNIT",
        "scale": 0
      },
      "account": "observer",
      "amount": "42"
    },
    {
      "asset": {
        "network": "local",
        "issuer": "continuation-reserve-oracle",
        "reference": "UNIT",
        "scale": 0
      },
      "account": "reserve",
      "amount": "100"
    },
    {
      "asset": {
        "network": "local",
        "issuer": "continuation-reserve-oracle",
        "reference": "UNIT",
        "scale": 0
      },
      "account": "target",
      "amount": "0"
    },
    {
      "asset": {
        "network": "local",
        "issuer": "continuation-reserve-oracle",
        "reference": "UNIT",
        "scale": 0
      },
      "account": "treasury",
      "amount": "0"
    }
  ],
  "liabilities": [
    {
      "name": "allocation",
      "asset": {
        "network": "local",
        "issuer": "continuation-reserve-oracle",
        "reference": "UNIT",
        "scale": 0
      },
      "debtor": "reserve",
      "creditor": "target",
      "amount": "5"
    }
  ]
}

```

## Exact existing input approval-true.json
Path: actual-source/crates/fixtures/core-disjoint-product/approval-true.json
SHA256: 33bb3b6a2b2f6c0175f9c6398668ba7124db736d1da41e6424c70f3a0ad14cbc
```text
{
  "format": "moriarty-stage-event/1",
  "action": "deliver",
  "label": "approval",
  "values": [
    {
      "name": "ok",
      "value": {
        "type": "Bool",
        "value": true
      }
    }
  ]
}

```

## Exact existing input receipt-true.json
Path: actual-source/crates/fixtures/core-disjoint-product/receipt-true.json
SHA256: a730a5b45e532dfeb4e26860cc0d15265b893bbaa81bf95c79d1214ce92feee2
```text
{
  "format": "moriarty-stage-event/1",
  "action": "deliver",
  "label": "receipt",
  "values": [
    {
      "name": "seen",
      "value": {
        "type": "Bool",
        "value": true
      }
    }
  ]
}

```

## Exact existing input quote43.json
Path: actual-source/crates/fixtures/core-disjoint-product/quote43.json
SHA256: 43093a5c32bdadea307bc89149d17a472dedcd12d55d762cf28d35227b0b9b6f
```text
{
  "format": "moriarty-stage-event/1",
  "action": "allocate",
  "label": "approval",
  "values": [
    {
      "name": "quote",
      "value": {
        "type": "UInt",
        "value": "43"
      }
    }
  ]
}

```

## Exact existing input settlement-true.json
Path: actual-source/crates/fixtures/core-disjoint-product/settlement-true.json
SHA256: a2fa104de81f75dd66b094334426badaf30939c250acd2f6abe3043520ceb07a
```text
{
  "format": "moriarty-stage-event/1",
  "action": "allocate",
  "label": "settlement",
  "values": [
    {
      "name": "ready",
      "value": {
        "type": "Bool",
        "value": true
      }
    }
  ]
}

```

## Exact existing input mori-tests.json
Path: actual-source/crates/fixtures/core-disjoint-product/mori-tests.json
SHA256: 76371519e3759ac87a85a63be6187a9eb6bacf84c499d112b5dd6c9c99c1654a
```text
{
  "profile": "moriarty-tests/disjoint-draft/1",
  "cases": [
    {
      "name": "left-first",
      "members": [
        {
          "source": "two-deliveries.mori",
          "scenario": "two-deliveries-scenario.json",
          "action": "deliver",
          "logical_cap": "16"
        },
        {
          "source": "reserve.mori",
          "scenario": "reserve-scenario.json",
          "action": "allocate",
          "logical_cap": "36"
        }
      ],
      "expected": "left-first-0.json",
      "events": [
        {
          "member": 0,
          "event": "approval-true.json",
          "expected": "left-first-1.json"
        },
        {
          "member": 1,
          "event": "quote43.json",
          "expected": "left-first-2.json"
        },
        {
          "member": 0,
          "event": "receipt-true.json",
          "expected": "left-first-3.json"
        },
        {
          "member": 1,
          "event": "settlement-true.json",
          "expected": "left-first-4.json"
        }
      ]
    },
    {
      "name": "right-first",
      "members": [
        {
          "source": "two-deliveries.mori",
          "scenario": "two-deliveries-scenario.json",
          "action": "deliver",
          "logical_cap": "16"
        },
        {
          "source": "reserve.mori",
          "scenario": "reserve-scenario.json",
          "action": "allocate",
          "logical_cap": "36"
        }
      ],
      "expected": "right-first-0.json",
      "events": [
        {
          "member": 1,
          "event": "quote43.json",
          "expected": "right-first-1.json"
        },
        {
          "member": 0,
          "event": "approval-true.json",
          "expected": "right-first-2.json"
        },
        {
          "member": 1,
          "event": "settlement-true.json",
          "expected": "right-first-3.json"
        },
        {
          "member": 0,
          "event": "receipt-true.json",
          "expected": "right-first-4.json"
        }
      ]
    }
  ]
}

```

## Existing intended contract, unchanged
Actual docs/NATIVE-CORE.md234–241 requires every exportablecase/result, mismatchstatusfailed/exit1, matchingpassed/exit0, originalprettybytesandper-casebounds. Actual disjointrun_case comparesfullactual beforeprefixreference, foldsfalsewhilecontinuinglatersteps; outerrun iterateslatercases, printscompleteaggregate andreturnsFAILURE. Old stagedprofile2 regression concerns a differentdispatcherbranch anddoesnotsatisfy thisone. Author expectedtypo touches onlytemporarycomparisonfileleft-first-2.json/post/state/marker/value77→78, notsource/scenario/event/actualcontinuation. Existingactualfullgoldensunchangedbeforefirstinvocation. Marker77, observers42/11, positiveallocation5/duty5andallframes/caps/steps/history/effects remain in everywholecomparison. No StageRejection/falseguard/securitytest isauthoredornewlyobserved.

Review exactactualsource andterminalscope; donot inventstandaloneinnerrawoutput, K823execution, ornewfinancialcoverage. Full89sourcebodies/all74custodypayloads/completegoldensandfullrootrecords availablephysically. No provider ornewcheck dispatch authorizedbythispacket.
