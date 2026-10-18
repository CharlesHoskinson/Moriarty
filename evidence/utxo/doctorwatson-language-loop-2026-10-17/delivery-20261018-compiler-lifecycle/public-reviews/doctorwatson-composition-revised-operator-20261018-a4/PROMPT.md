FRESH independent Gemini4ArgonHigh HIGH OPERATOR review of SAME frozen candidate. Source only; one420s context, no product/build/test/network execution or file writes. Return ONE complete JSON review with reviewToken=doctorwatson-composition-revised-operator-20261018-a4-whole-review. Request six concise fields: reviewToken, verdict, summary, decisionVotes, findings, limitations. Required eight votes D1-D5/RV1-RV3, each with substantive source/API-specific reason. Aim summary<=1200chars, reason<=300chars, findings<=5x600, limitations<=5x400; these are concise writing targets, NOT schema rejection maxima. Extra formatting fields/length are tolerated so do not waste time rewriting to satisfy incidental maxima. Produce a whole substantive review. Decide independently; no rubberstamp, no source/API invention, no missing-golden-as-pass, no new semantics. Full16candidatecode/literal8choices/relevantactualAPI/normative and complete material dissent findings follow. Original151whole inputs are available under a3-inputs/ and frozen main source basis is explicit. Do not repeat fullpatch/delta or read a duplicate file list; inspect additional whole source only if needed. Security/constraint/proof/resource-authorization questions stay unanswered for designated owner. Conditional adoption is source design only; original operators/exits/oracles/runtime gaps stay separate.

# Focused independent operator review a4 — SAME frozen16-file candidate
Review source only, no product/build/test/network execution, no writes or security/constraint/proof/resource-authority judgment. This is one fresh HIGH context,420s. Source basis actual main9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb via read-only git show; old API/program/consumer source equals current main, consolidated current requirements include adopted ordinary Core2 clauses. All151 a3 physical inputs are unchanged under a3-inputs/, including whole API/contracts, original57 inputs, fullpatch/delta, oracle contracts and raw full prior reviews. Exact pins are in a3-inputs/MANIFEST.json. all16 COMPLETE candidate files, literal8choices, actual used APIs, relevant normative clauses and complete material dissent findings are inline here. Inspect actual changed code; choose independently. Existing a3 consumer completed exit0 with all8conditional adopt votes; it is not your vote. Both a1timeouts, a2operator137/revision+consumer0/revision, a3operator124/no-valid-review are preserved without rewriting/salvage. No second vote/adoption yet.
Predicate: substantively assess D1-D5/RV1-RV3 for the declared bounded local draft, identify actual source/API/requirement mismatches where present, and keep missing oracle/runtime/full-operator coverage explicit. Six concise fields/eight votes requested.
Original obligations separately OPEN: sequential Step (separate escrow oracle), full disjoint parallel including same-asset/disjoint-account and native Split, shared-state interleaving, atomic synchronization Join, asynchronous Pending/messages. Candidate supports only stronger exact-asset/state/duty-disjoint local product. Fixed starts[0,1]; case right-first means first resume. No source review/fullstate sum substitutes for native composition or any original parent exit U0-U7/M1-M17/MC/SP/G, failures or caps. Public developer validity stays permissionless.

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/approval-true.json
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/mori-tests.json
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/quote43.json
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/receipt-true.json
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/reserve-scenario.json
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/reserve.mori
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/settlement-true.json
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/two-deliveries-scenario.json
<<<
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

>>>

WHOLE CANDIDATE crates/fixtures/core-disjoint-product/two-deliveries.mori
<<<
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

>>>

WHOLE CANDIDATE crates/mori/Cargo.toml
<<<
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
core2-disjoint-draft = ["mori-core/core2-disjoint-draft"]
default = ["native"]
native = [
    "dep:mori-backend",
    "dep:mori-ledger",
    "dep:futures",
    "dep:rand",
    "dep:midnight-ledger",
    "dep:midnight-base-crypto",
    "dep:midnight-coin-structure",
    "dep:midnight-onchain-state",
    "dep:midnight-onchain-runtime",
    "dep:midnight-storage",
    "dep:midnight-transient-crypto",
    "dep:midnight-serialize",
]

[dependencies]
anstream.workspace = true
anstyle.workspace = true
clap.workspace = true
ignore.workspace = true
mori-ast.workspace = true
mori-checker.workspace = true
mori-core.workspace = true
mori-backend = { workspace = true, optional = true }
mori-ledger = { workspace = true, optional = true }
mori-diagnostics = { workspace = true, features = ["fancy"] }
mori-fmt.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-span.workspace = true
rayon.workspace = true
futures = { workspace = true, optional = true }
rand = { workspace = true, optional = true }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
terminal_size.workspace = true
thiserror.workspace = true

midnight-ledger = { workspace = true, optional = true }
midnight-base-crypto = { workspace = true, optional = true }
midnight-coin-structure = { workspace = true, optional = true }
midnight-onchain-state = { workspace = true, optional = true }
midnight-onchain-runtime = { workspace = true, optional = true }
midnight-storage = { workspace = true, optional = true }
midnight-transient-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
indoc.workspace = true
insta.workspace = true

[[test]]
name = "check_compile_io"
required-features = ["native"]

[[test]]
name = "check_static_compile_io"
required-features = ["native"]

[[test]]
name = "prepare_call"
required-features = ["native"]

[[test]]
name = "installed_call"
required-features = ["native"]

[[test]]
name = "signed_installed_call"
required-features = ["native"]

[[test]]
name = "native_proofs"
required-features = ["native"]

[[test]]
name = "signed_intents"
required-features = ["native"]

[[test]]
name = "replay"
required-features = ["native"]

[[test]]
name = "replay_challenge"
required-features = ["native"]

[[test]]
name = "enforcement_report"
required-features = ["native"]

[[test]]
name = "mapped_role_static_io"
required-features = ["native"]

>>>

WHOLE CANDIDATE crates/mori/src/commands/test/disjoint_test.rs
<<<
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

>>>

WHOLE CANDIDATE crates/mori/src/commands/test.rs
<<<
use super::core_io;
use crate::shell::Shell;
use serde::Deserialize;
use std::{path::PathBuf, process::ExitCode};

mod stage_test;
#[cfg(feature = "core2-disjoint-draft")]
mod disjoint_test;

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
        return Err(core_io::operation("disjoint draft requires feature core2-disjoint-draft"));
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

>>>

WHOLE CANDIDATE crates/mori/tests/disjoint_test_public.rs
<<<
#![cfg(feature = "core2-disjoint-draft")]
use serde_json::Value;
use std::{fs, path::PathBuf, process::Command, sync::atomic::{AtomicUsize, Ordering}};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-disjoint-test-public-{}-{}", std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../fixtures/core-disjoint-product");
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
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}


fn expand(actual: &Value, tables: &Value) -> Value {
    let mut actual = actual.clone();
    assert_eq!(actual["format"], "moriarty-local-disjoint-referenced-draft/1");
    for name in ["history", "effects"] {
        assert_eq!(actual[name]["table"], name);
        let length = usize::try_from(actual[name]["prefix_len"].as_u64().unwrap()).unwrap();
        let table = tables[name].as_array().unwrap();
        actual[name] = Value::Array(table.get(..length).unwrap().to_vec());
    }
    let format = actual.as_object_mut().unwrap().remove("expanded_format").unwrap();
    actual["format"] = format;
    actual
}

#[test]
fn public_consumer_observes_both_disjoint_domains_for_two_explicit_schedules() {
    let project = Project::new();
    // The aggregate and all ten stage reports are independent literal inputs.
    let expected = fs::read(project.0.join("public-report.json")).unwrap();
    let expected_json: Value = serde_json::from_slice(&expected).unwrap();
    let manifest: Value = serde_json::from_slice(
        &fs::read(project.0.join("mori-tests.json")).unwrap(),
    ).unwrap();
    let expected_stages: Vec<Vec<Value>> = manifest["cases"].as_array().unwrap().iter().map(|case| {
        let mut paths = vec![case["expected"].as_str().unwrap()];
        paths.extend(case["events"].as_array().unwrap().iter().map(|event| {
            event["expected"].as_str().unwrap()
        }));
        paths.into_iter().map(|path| {
            serde_json::from_slice(&fs::read(project.0.join(path)).unwrap()).unwrap()
        }).collect()
    }).collect();
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&project.0).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, expected_json);
    assert_eq!(output.stdout, expected);
    assert!(String::from_utf8_lossy(&output.stderr).contains("2 passed; 0 failed"));
    // Expand each explicit prefix reference and compare the original whole
    // independent report. Every frame, duty, original envelope and effect stays
    // observable; the shared tables do not normalize or truncate expectations.
    for (case, expected_stages) in actual["cases"].as_array().unwrap().iter().zip(expected_stages) {
        assert_eq!(case["format"], "moriarty-disjoint-case-observations-draft/1");
        assert_eq!(case["stages"].as_array().unwrap().len(), expected_stages.len());
        for (stage, expected_stage) in case["stages"].as_array().unwrap().iter().zip(expected_stages) {
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

>>>

WHOLE CANDIDATE crates/mori-core/Cargo.toml
<<<
[package]
name = "mori-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[features]
core2-disjoint-draft = []
# Opt-in and non-default: enables only the Midnight-native signing-frame
# compatibility test. Ordinary mori-core consumers and SDK-free test
# selection never pull these dependencies.
midnight-compat-tests = ["dep:midnight-base-crypto", "dep:midnight-serialize"]

[dependencies]
mori-lexer.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
sha2.workspace = true
k256.workspace = true
# Optional and off by default; pulled in only by the midnight-compat-tests
# feature for the native compatibility test.
midnight-base-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
proptest.workspace = true

>>>

WHOLE CANDIDATE crates/mori-core/src/stage/disjoint.rs
<<<
//! Local product of two staged actions with disjoint state and asset domains.
use super::{PendingFrame, StageEvent, StageOutcome, StageRejection, StagedProgram};
use crate::{AssetId, Effect, Scenario, Snapshot, Statement, UInt};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub const MAX_INVOCATIONS: usize = 256;

pub struct Member {
    pub program: StagedProgram,
    pub action: String,
    pub scenario: Scenario,
    pub cap: UInt,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("local disjoint product: {0}")]
    Shape(String),
    #[error("member {member}: {rejection}")]
    Branch {
        member: usize,
        rejection: StageRejection,
        /// Start may have attempted an earlier member before this refusal.
        attempted_before: Vec<UInt>,
        earlier_start_committed: Vec<UInt>,
        pair_attempted_before: Option<UInt>,
        pair_committed_before: Option<[UInt; 2]>,
    },
    #[error("local disjoint product {phase}: {message}")]
    AfterInvocation {
        phase: &'static str,
        message: String,
        member: Option<usize>,
        attempted: Vec<Option<UInt>>,
        local_committed: Vec<UInt>,
        pair_attempted_before: Option<UInt>,
        pair_committed_before: Option<[UInt; 2]>,
    },
}

impl Error {
    /// Complete diagnostic data; no product boundary, effects or continuation
    /// is published by an error. Original stage refusals retain their envelope.
    pub fn public_report(&self) -> Value {
        let detail = match self {
            Self::Shape(message) => json!({"kind":"before-invocation","message":message}),
            Self::Branch {
                member, rejection, attempted_before, earlier_start_committed,
                pair_attempted_before, pair_committed_before,
            } => json!({
                "kind":"stage-rejection", "member":member,
                "rejection":super::outcome_report(Err(rejection.clone())),
                "earlier_start_attempted":attempted_before,
                "earlier_start_local_committed":earlier_start_committed,
                "pair_attempted_before":pair_attempted_before,
                "pair_committed_before":pair_committed_before,
            }),
            Self::AfterInvocation {
                phase, message, member, attempted, local_committed,
                pair_attempted_before, pair_committed_before,
            } => json!({
                "kind":"composition-after-invocation", "phase":phase,
                "message":message, "member":member,
                "invocation_attempted":attempted,
                "candidate_local_committed":local_committed,
                "pair_attempted_before":pair_attempted_before,
                "pair_committed_before":pair_committed_before,
            }),
        };
        json!({
            "format":"moriarty-local-disjoint-error-draft/1", "status":"rejected",
            "qualification":"local-reference-stage", "authentication":"none",
            "consumption":"untracked", "detail":detail,
        })
    }
}

#[derive(Clone)]
struct Observation {
    member: usize,
    outcome: StageOutcome,
}

/// Opaque local state: member boundaries come only from actual stage outputs.
/// It has no decoder, authentication or consumption tracking.
pub struct Pair {
    programs: [StagedProgram; 2],
    caps: [UInt; 2],
    cap_total: UInt,
    attempted_total: UInt,
    opening: Snapshot,
    latest: [StageOutcome; 2],
    history: Vec<Observation>,
}

fn shape(message: &str) -> Error {
    Error::Shape(message.into())
}

fn sum(left: UInt, right: UInt) -> Result<UInt, Error> {
    left.0.checked_add(right.0).map(UInt).ok_or_else(|| shape("logical sum exceeds UInt"))
}

fn declared_domains(program: &StagedProgram) -> (BTreeSet<&str>, BTreeSet<&AssetId>, BTreeSet<&str>) {
    let program = program.program();
    let states = program.states.iter().map(|state| state.name.as_str()).collect();
    let assets = program.assets.iter().map(|asset| &asset.identity).collect();
    let duties = program.actions.iter().flat_map(|action| &action.statements).filter_map(|stmt| {
        match stmt {
            Statement::Liability { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }).collect();
    (states, assets, duties)
}

fn disjoint(left: &StagedProgram, right: &StagedProgram) -> Result<(), Error> {
    let (left_state, left_assets, left_duties) = declared_domains(left);
    let (right_state, right_assets, right_duties) = declared_domains(right);
    if !left_state.is_disjoint(&right_state)
        || !left_assets.is_disjoint(&right_assets)
        || !left_duties.is_disjoint(&right_duties)
    {
        return Err(shape("requires disjoint declared state names, exact assets and duty names"));
    }
    Ok(())
}

fn merge(left: &Snapshot, right: &Snapshot) -> Result<Snapshot, Error> {
    let mut state = left.state.clone();
    for (name, value) in &right.state {
        if state.insert(name.clone(), value.clone()).is_some() {
            return Err(shape("state domains overlap"));
        }
    }
    let mut balances = left.balances.clone();
    balances.extend(right.balances.iter().cloned());
    balances.sort_by(|a, b| (&a.asset, &a.account).cmp(&(&b.asset, &b.account)));
    if balances.windows(2).any(|rows| rows[0].asset == rows[1].asset && rows[0].account == rows[1].account) {
        return Err(shape("balance domains overlap"));
    }
    let mut liabilities = left.liabilities.clone();
    liabilities.extend(right.liabilities.iter().cloned());
    liabilities.sort_by(|a, b| a.name.cmp(&b.name));
    if liabilities.windows(2).any(|rows| rows[0].name == rows[1].name) {
        return Err(shape("duty domains overlap"));
    }
    Ok(Snapshot { state, balances, liabilities })
}

fn boundary(outcome: &StageOutcome) -> &Snapshot {
    match outcome {
        StageOutcome::Waiting { continuation, .. } => &continuation.boundary,
        StageOutcome::Complete { segment, .. } => &segment.execution.post,
    }
}

fn start_pre(outcome: &StageOutcome) -> Result<&Snapshot, Error> {
    match outcome {
        StageOutcome::Waiting { segment: Some(segment), .. }
        | StageOutcome::Complete { segment, .. } => Ok(&segment.execution.pre),
        _ => Err(shape("start did not supply a committed segment")),
    }
}

fn attempted(outcome: &StageOutcome) -> UInt {
    match outcome {
        StageOutcome::Waiting { attempted, .. } | StageOutcome::Complete { attempted, .. } => *attempted,
    }
}

fn committed(outcome: &StageOutcome) -> UInt {
    match outcome {
        StageOutcome::Waiting { continuation, .. } => continuation.logical.committed,
        StageOutcome::Complete { committed, .. } => *committed,
    }
}

fn frame(outcome: &StageOutcome) -> PendingFrame {
    match outcome {
        StageOutcome::Waiting { continuation, .. } => PendingFrame {
            site: continuation.site,
            captures: continuation.captures.clone(),
        },
        StageOutcome::Complete { segment, .. } => segment.frame_post.clone(),
    }
}

fn effects(outcome: &StageOutcome) -> &[Effect] {
    match outcome {
        StageOutcome::Waiting { segment: Some(segment), .. }
        | StageOutcome::Complete { segment, .. } => &segment.execution.effects,
        StageOutcome::Waiting { segment: None, .. } => &[],
    }
}

impl Pair {
    /// Start uses a declared left-then-right reference linearization. Neither
    /// member receives the other's boundary, effects, captures or inputs.
    pub fn start([left, right]: [Member; 2]) -> Result<Self, Error> {
        disjoint(&left.program, &right.program)?;
        let caps = [left.cap, right.cap];
        let cap_total = sum(caps[0], caps[1])?;
        let programs = [left.program, right.program];
        let first = super::start(&programs[0], &left.action, left.scenario, Some(caps[0]))
            .map_err(|rejection| Error::Branch {
                member: 0, rejection, attempted_before: vec![], earlier_start_committed: vec![],
                pair_attempted_before: None, pair_committed_before: None,
            })?;
        let second = super::start(&programs[1], &right.action, right.scenario, Some(caps[1]))
            .map_err(|rejection| Error::Branch {
                member: 1, rejection, attempted_before: vec![attempted(&first)],
                earlier_start_committed: vec![committed(&first)],
                pair_attempted_before: None, pair_committed_before: None,
            })?;
        let prepared = (|| {
            let attempted_total = sum(attempted(&first), attempted(&second))?;
            let opening = merge(start_pre(&first)?, start_pre(&second)?)?;
            merge(boundary(&first), boundary(&second))?;
            Ok((attempted_total, opening))
        })();
        let (attempted_total, opening) = prepared.map_err(|error: Error| Error::AfterInvocation {
            phase: "start", message: error.to_string(), member: None,
            attempted: vec![Some(attempted(&first)), Some(attempted(&second))],
            local_committed: vec![committed(&first), committed(&second)],
            pair_attempted_before: None, pair_committed_before: None,
        })?;
        let history = vec![
            Observation { member: 0, outcome: first.clone() },
            Observation { member: 1, outcome: second.clone() },
        ];
        Ok(Self { programs, caps, cap_total, attempted_total, opening, latest: [first, second], history })
    }

    /// Resume one member from its own actual continuation. Publication replaces
    /// only that member after its stage call and complete frame merge succeed.
    pub fn resume(&mut self, member: usize, event: StageEvent) -> Result<(), Error> {
        if self.history.len() >= MAX_INVOCATIONS {
            return Err(shape("invocation limit reached"));
        }
        let Some(StageOutcome::Waiting { continuation, .. }) = self.latest.get(member) else {
            return Err(shape("member must identify a pending branch"));
        };
        let outcome = super::resume(
            &self.programs[member], continuation.clone(), event, Some(&continuation.action), None,
        ).map_err(|rejection| Error::Branch {
            member, rejection, attempted_before: vec![], earlier_start_committed: vec![],
            pair_attempted_before: Some(self.attempted_total),
            pair_committed_before: Some(self.latest.each_ref().map(committed)),
        })?;
        let mut latest = self.latest.clone();
        latest[member] = outcome.clone();
        let prepared = (|| {
            merge(boundary(&latest[0]), boundary(&latest[1]))?;
            sum(self.attempted_total, attempted(&outcome))
        })();
        let attempted_total = prepared.map_err(|error: Error| {
            let mut invocation_attempted = vec![None, None];
            invocation_attempted[member] = Some(attempted(&outcome));
            Error::AfterInvocation {
                phase: "resume", message: error.to_string(), member: Some(member),
                attempted: invocation_attempted,
                local_committed: latest.each_ref().map(committed).to_vec(),
                pair_attempted_before: Some(self.attempted_total),
                pair_committed_before: Some(self.latest.each_ref().map(committed)),
            }
        })?;
        self.latest = latest;
        self.attempted_total = attempted_total;
        self.history.push(Observation { member, outcome });
        Ok(())
    }

    /// The report contains the complete product frame and all original member
    /// envelopes. Effects retain invocation order, then source statement order.
    pub fn report(&self) -> Result<Value, Error> {
        let post = merge(boundary(&self.latest[0]), boundary(&self.latest[1]))?;
        let committed = [committed(&self.latest[0]), committed(&self.latest[1])];
        let committed_total = sum(committed[0], committed[1])?;

        let ordered_effects: Vec<_> = self.history.iter().flat_map(|item| effects(&item.outcome)).collect();
        let history: Vec<_> = self.history.iter().map(|item| {
            json!({"member":item.member,"outcome":super::outcome_report(Ok(item.outcome.clone()))})
        }).collect();
        let members: Vec<_> = self.latest.iter().map(|outcome| {
            super::outcome_report(Ok(outcome.clone()))
        }).collect();
        let frames = self.latest.each_ref().map(frame);
        let waiting = self.latest.iter().any(|outcome| matches!(outcome, StageOutcome::Waiting { .. }));
        Ok(json!({
            "format":"moriarty-local-disjoint-draft/1",
            "status":if waiting { "waiting" } else { "complete" },
            "qualification":"local-reference-stage",
            "composition_qualification":"unadopted-local-disjoint-product",
            "authentication":"none", "consumption":"untracked",
            "ordering":{"start":[0,1],"effects":"invocation-then-source-order"},
            "pre":self.opening, "post":post, "frames":frames,
            "members":members, "history":history, "effects":ordered_effects,
            "logical":{"caps":self.caps,"cap_total":self.cap_total,
                "committed":committed,"committed_total":committed_total,
                "attempted_total":self.attempted_total},
            "residual":super::residual(&post),
        }))
    }
}

>>>

WHOLE CANDIDATE crates/mori-core/src/stage/mod.rs
<<<
//! Staged `moriarty/2` programs and their local continuation interface.
//!
//! Local and unsigned only. Event binders and captures are caller-chosen values
//! (no authentication), a continuation can be resumed any number of times (no
//! consumption tracking), and no stage authority exists. Outputs carry
//! `STAGE_LABELS` so these limits stay visible.
mod admit;
mod cost;
#[cfg(feature = "core2-disjoint-draft")]
pub mod disjoint;
pub mod decode;
mod digest;
mod plan;
mod profile;
mod report;
mod validate;

pub use crate::LOGICAL_COST_PROFILE_2;
pub use admit::{resume, start};
pub use plan::{ActionPlan, AwaitSite, CaptureSlot, Origin, StageBinder, StagePlan};
pub use profile::{require_core1, require_core2};
pub use report::outcome_report;

use crate::{Error, Execution, Liability, Program, Snapshot, UInt, Value};
use serde::{Deserialize, Serialize};

pub const LANGUAGE_2: &str = "moriarty/2";
pub const SEMANTICS_2: &str = "moriarty-core/2";
pub const STAGE_PLAN_FORMAT: &str = "moriarty-stage-plan/1";
pub const CONTINUATION_FORMAT: &str = "moriarty-continuation/1";
pub const STAGE_EVENT_FORMAT: &str = "moriarty-stage-event/1";
pub const MAX_AWAITS_PER_ACTION: usize = 16;
pub const MAX_EVENT_BINDERS: usize = 16;
pub const MAX_CAPTURE_SLOTS: usize = 256;
pub const MAX_CONTINUATION_BYTES: usize = 4 * 1_048_576;
pub const MAX_EVENT_BYTES: usize = 65_536;
/// Inserted verbatim into every `/2` output object.
pub const STAGE_LABELS: [(&str, &str); 3] = [
    ("qualification", "local-reference-stage"),
    ("authentication", "none"),
    ("consumption", "untracked"),
];

/// A `/2` program that passed profile, validation, plan invariants and static
/// costing, with its canonical digests. Only `load` constructs one; there is
/// no public constructor, `Default` or `Deserialize`.
#[derive(Debug, Clone)]
pub struct StagedProgram {
    program: Program,
    plan: StagePlan,
    program_hash: [u8; 32],
    plan_hash: [u8; 32],
}
impl StagedProgram {
    pub fn program(&self) -> &Program {
        &self.program
    }
    pub fn plan(&self) -> &StagePlan {
        &self.plan
    }
    pub fn program_hash(&self) -> [u8; 32] {
        self.program_hash
    }
    pub fn plan_hash(&self) -> [u8; 32] {
        self.plan_hash
    }
    /// Declaration index of an action; plan and program actions share it.
    pub(crate) fn action_index(&self, name: &str) -> Option<usize> {
        self.program.actions.iter().position(|a| a.name == name)
    }
}

/// Parses, admits and plans a `moriarty/2` source. Legacy `parse`/`validate`
/// remain `/1`-only, so a `/2` source fails every legacy consumer.
pub fn load(src: &str) -> Result<StagedProgram, Error> {
    let program = crate::parser::parse_v2(src)?;
    profile::require_core2(&program)?;
    validate::validate_v2(&program)?;
    let mut plan = plan::build(&program)?;
    validate::plan_invariants(&program, &plan)?;
    cost::fill(&program, &mut plan)?;
    validate::cost_invariants(&program, &plan)?;
    let program_hash = digest::program_hash(&program)?;
    plan.program_hash = digest::hex(&program_hash);
    let plan_hash = digest::plan_hash(&plan)?;
    Ok(StagedProgram {
        program,
        plan,
        program_hash,
        plan_hash,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedValue {
    pub name: String,
    pub value: Value,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageLogical {
    pub profile: String,
    /// Mandatory key; `None` is written as `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub cap: Option<UInt>,
    pub committed: UInt,
}
/// Resume input. `site` and `captures` are the executing action's pending
/// frame; `boundary` is the legacy Snapshot. Nothing here is authenticated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Continuation {
    pub format: String,
    pub program_hash: String,
    pub plan_hash: String,
    pub action: String,
    pub site: u16,
    pub captures: Vec<NamedValue>,
    pub logical: StageLogical,
    pub boundary: Snapshot,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageEvent {
    pub format: String,
    pub action: String,
    pub label: String,
    pub values: Vec<NamedValue>,
}
/// Local frame of one action. `site == 0` exactly when `captures` is empty
/// and the action is idle; no padding is represented locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingFrame {
    pub site: u16,
    pub captures: Vec<NamedValue>,
}
impl PendingFrame {
    pub(crate) fn idle() -> Self {
        Self {
            site: 0,
            captures: Vec::new(),
        }
    }
}
/// Output only. Segment history is never decoded or trusted on resume, so no
/// `Deserialize` exists over the shared Await-bearing `Statement`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StageSegment {
    pub action: String,
    pub frame_pre: PendingFrame,
    pub frame_post: PendingFrame,
    pub execution: Execution,
}
/// `residual` lists every post-boundary liability with a positive amount, in
/// the existing complete `Liability` representation.
// Public outcome; boxing `Waiting` would change every caller's pattern.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum StageOutcome {
    Waiting {
        advanced: bool,
        segment: Option<StageSegment>,
        continuation: Continuation,
        attempted: UInt,
        residual: Vec<Liability>,
    },
    Complete {
        segment: StageSegment,
        committed: UInt,
        attempted: UInt,
        residual: Vec<Liability>,
    },
}
/// A rejected invocation publishes no segment, effects or continuation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, thiserror::Error)]
#[error("{error}")]
pub struct StageRejection {
    pub error: Error,
    pub attempted: UInt,
    pub committed: UInt,
}

/// Positive residual duties of a boundary, in its (name-sorted) order.
pub(crate) fn residual(snapshot: &Snapshot) -> Vec<Liability> {
    snapshot
        .liabilities
        .iter()
        .filter(|l| l.amount.0 > 0)
        .cloned()
        .collect()
}

>>>

LITERAL COMPLETE D1-D5 / RV1-RV3 a3-inputs/revised/ADOPTION-ORIGINAL.md
<<<
# Required adoption decisions for this proposed local source operator

These decisions are not supplied by the current API. The current high-level composition requirements name the operator but do not define this local consumer's exact product representation. Until adopted, the feature and `moriarty-tests/disjoint-draft/1` format remain unadvertised drafts. Required non-author ordinary source review and the existing consequential-decision rule remain applicable; designated Opus/fallback owns any security/resource interpretation or requested hostile tests. Reviewer process never becomes a public program-validity predicate.

**D1 — Exact admitted domain.** Propose exactly two already loaded Core/2 programs, each with one selected action and mandatory finite per-member cap. Require disjoint declared state names, disjoint complete exact AssetId sets and disjoint source-registered liability names across the complete programs, including unused actions. This deliberately stronger asset-disjoint subset excludes same-asset disjoint-account composition; it does not remove that original required coverage. The complete product boundary is the flat union of both admitted member Snapshots, with state ordered by its existing map, balances ordered by `(exact AssetId,account)`, liabilities by name. No row is renamed, netted, omitted or placed in a separate hidden domain. There is no arbitrary unowned third frame in this first slice: untouched rows are explicitly present inside the two complete member domains.

**D2 — Ordering and meaning of parallel.** Propose deterministic **logical** parallel composition with an explicit reference linearization: start member0 then member1 from independent supplied scenarios; after construction each manifest event chooses only one pending member. A member never reads the other member's boundary. Both schedules are source test inputs; there are no threads or claims of simultaneous physical execution. Effects use actual invocation order, then source order inside the real segment. Initial history therefore records 0 then1. Same final state does not imply identical ordered trace, proof history or target behavior. Extending to arbitrary schedules/shared state needs a separate rule.

**D3 — Publication and failures.** Propose publish the initial pair only after both real starts and complete frame union succeed. A branch start rejection returns its unchanged original StageRejection, selected member and any earlier member's attempted work; no product boundary/effects are returned. A resume uses the selected member's actual retained continuation and changes the opaque Pair only after successful Core/2 return, merge and attempted-work addition. A false observation remains Waiting and retains its original continuation; all complete branches and prior history remain stored. Original branch errors/counters are not changed. The thin CLI positive manifest mode stops on a consumer error; this packet does not design or claim complete aggregate rejection reporting. Failure/cleanup/hostile acceptance and target-phase rules are not established by these proposed local rules and retain their current owners.

**D4 — Work and history bounds.** Propose retain each supplied finite cap unchanged in its own original stage calls. The source example uses cap16 and cap36; a descriptive cap_total52 is their checked sum, and committed totals are checked sums of original returned committed values. No member borrows unused cap from another or resets its continuation. attempted_total is a checked sum of actual attempted values across original invocations, distinct from committed work; nonadvancing observations may add attempted work without committed work. Dispatch/merge/serialization overhead has no new target/logical charge certified here. Propose reuse the existing test harness maximum256 underlying invocations: two starts plus at most254 resumes. The Pair retains full actual history; memory/output effects of that representation require the applicable review, not a resource adequacy judgment from this ordinary author. Existing per-document readers/exporters retain their limits. No native resource contract, failure/cap or allowance changes.

**D5 — Qualifications, ownership and exposure.** Propose non-default feature `core2-disjoint-draft` in mori-core, forwarded by mori; default native settings unchanged. The existing public `mori test` selects a new `moriarty-tests/disjoint-draft/1` manifest only with that feature; disabled builds state the missing draft feature. Root report format `moriarty-local-disjoint-draft/1` explicitly labels the construction unadopted/local, authentication none and consumption untracked; nested original whole stage envelopes retain their original labels. Root `complete` means both local actions completed, even with positive residual liabilities; it means neither settlement nor native acceptance. Root members are the latest complete original envelopes; history contains every original invocation envelope; cumulative effects include only committed segment effects in invocation order. This proposed local operator cannot replace required native `Split`, signed authority, lifecycle measures or a proof relation.

Finite integration ownership: four existing source interfaces are touched (`stage/mod.rs`, the two Cargo feature sections and `mori` test dispatcher); the integrator owns adoption and shared-interface integration. All remaining files are additive. No old fixture/oracle/program is edited. New source copies under crates/fixtures preserve exact bytes and provenance; new Rust source reads no fixture outside crates. No compiler or SDK dependency is added. Source comments explain behavior only; authorship/review/not-run metadata lives in this packet. Existing source/fixture guideline deviations remain untouched.

>>>

LITERAL COMPLETE D1-D5 / RV1-RV3 a3-inputs/revised/ADOPTION-DELTA.md
<<<
# Unadopted revision choices RV1–RV3

Read original ADOPTION.md verbatim in the preserved original proposal or compact full-context/proposal/. D1–D5 are not adopted. This revision retains the stronger full-program asset-disjoint subset, fixed start order [0,1], unchanged stage cap semantics, original latest-invocation envelopes (false observation has segment:null), and source publication behavior. The following additional choices require fresh independent substantive agreement; no prior review approves changed bytes.

RV1 — Lossless error diagnostic. The library exposes public_report() with format moriarty-local-disjoint-error-draft/1 and the original local/none/untracked labels. A StageRejection is included through original stage::outcome_report(Err(rejection.clone())), retaining every original error/attempted/committed value and qualification. Earlier calls in the same initial start have separately named attempted/local-committed diagnostics; prior published Pair attempted/committed values use distinct fields on resume. A late composition error includes phase, optional selected member and exact returned per-member attempted/candidate-local-committed values. A member not invoked in that operation has attempted:null rather than a fabricated zero-work result. Those local counters do not publish a product boundary, effects or continuation and do not declare a native debit/refund policy. Existing mori test passes this complete Value through core_io::operation, whose command diagnostic uses its JSON Display instead of Error's lossy summary Display. Independent expected diagnostic bytes are still absent/unobserved.

RV2 — Lossless case-level wire tables. Every complete Pair report still exists and is compared against a complete independent stage golden before any transformation. The public case now has format moriarty-disjoint-case-observations-draft/1 and tables.history/tables.effects arrays. Each stage preserves every other original whole product-report field but uses format moriarty-local-disjoint-referenced-draft/1, saves the exact original format as expanded_format and replaces only the two cumulative arrays with {table:history|effects,prefix_len:N}. N names the complete original array prefix, not a truncation rule. Prefix extension is checked by full equality; a rewritten/decreased earlier prefix errors before aggregate publication. All original history envelopes/effects are stored in the case tables, including the final complete arrays. Expansion substitutes exactly those prefixes, restores expanded_format and removes that representation-only key. The proposed public binary test compares every expansion against its original whole literal golden, in addition to exact independently literal aggregate bytes.

RV3 — Existing bounds and unresolved evidence. The per-case aggregate exporter and input/invocation limits remain unchanged; there is no larger output limit, silent sample, truncation or waiver. Eliminating repeated cumulative arrays removes that source of quadratic serialized duplication but does not certify memory, host cost or byte adequacy for every allowed input. Pair::report remains a complete-report API; constructing/comparing cumulative reports still has repeated host work, which is not a target cost claim. The existing stronger subset and nonawait incoming-duty preadmission rule remain separate D1 review questions; this repair only ensures late failures retain original work metadata. Prior Pair publication/counters remain unchanged on failure, with rejected attempted work separately visible in its error record. No global failure-charge or native lifetime policy is invented.

Full native/financial acceptance and all five operators remain separate. Neither two local sources nor two schedules nor source review closes native Step/Split/Join, shared-state interleaving or asynchronous messaging. All required independent oracles, actual consumer observations and original parent exits stay open.

>>>

COMPLETE a2 operator FINDINGS (unmodified strings, full original review physically pinned):
[
  "`crates/mori-core/src/stage/disjoint.rs` (`declared_domains`, `disjoint`, `Pair::start`): `declared_domains` only collects `Statement::Liability`, ignoring `Statement::Discharge` and `Scenario.liabilities` (admitted by `scenario_snapshot` on 0-await actions when `admit::start` skips `canonical_boundary`). `Pair::start` runs both `super::start` calls before `merge`, so duty collisions or `sum` overflows fail with `Error::Shape` after execution and discard `attempted` work. Change: validate `Statement::Discharge`, `scenario.liabilities`, and pre-snapshot disjointness before `super::start`.",
  "`crates/mori-core/src/stage/disjoint.rs` (`Pair::resume`, `Pair::report`, `Error::Branch`): On `Err(rejection)`, `Pair::resume` sets `attempted_before: vec![]` instead of exposing prior `self.attempted_total` and does not accumulate `rejection.attempted`. On a false guard (`Waiting { advanced: false, segment: None, .. }`), `Pair::resume` overwrites `self.latest[member]`, replacing the member's last committed segment in `report()[\"members\"]`, while `report()[\"logical\"]` omits per-member `attempted: [UInt; 2]`. Change: track per-member `attempted` and preserve `attempted_before`.",
  "`crates/mori/src/commands/test/disjoint_test.rs` (`run_case`) & `crates/mori-core/src/stage/disjoint.rs` (`Error::Branch`): `Error::Branch`'s `Display` (`#[error(\"member {member}: {rejection}\")]`) omits `attempted_before`, `rejection.attempted`, and `rejection.committed`, so `core_io::operation` drops work counters and cannot test rejections like `stage_test.rs`. Also, embedding full `Pair::report()` `history` in every stage grows $O(N^2)$ against `mori_core::json::export(&case)`'s 1 MiB cap in `test.rs::aggregate_report`. Change: report rejection work metadata and bound history.",
  "`crates/fixtures/core-disjoint-product/two-deliveries.mori` (`declare`), `mori-tests.json`, & `crates/mori/tests/disjoint_test_public.rs`: Because `decode::canonical_boundary` requires awaiting-action scenario duties to appear in `Statement::Liability`, `two-deliveries.mori` adds a dummy `action declare()` to keep untouched duty `duty`. Also, `disjoint_test_public.rs` unconditionally reads `public-report.json` and `mori-tests.json`'s 10 stage goldens (`left-first-0..4.json`, `right-first-0..4.json`), which are absent. Change: supply the 11 independent oracle files and preimages."
]

COMPLETE a2 consumer FINDINGS (unmodified strings, full original review physically pinned):
[
  "1. [crates/mori-core/src/stage/disjoint.rs: Pair::resume, Pair::report] When super::resume returns StageOutcome::Waiting { advanced: false, segment: None, .. }, Pair::resume unconditionally overwrites self.latest[member] = outcome.clone(). In Pair::report(), members therefore serializes segment: null instead of the member's latest committed segment, contradicting D3/D5. Change Pair::resume to update latest[member]'s attempted/continuation only or preserve the last committed segment in members when advanced is false, and pass Some(self.caps[member]) as cli_cap to super::resume.",
  "2. [crates/mori-core/src/stage/disjoint.rs: Error::Branch; crates/mori/src/commands/test/disjoint_test.rs: run_case] Error::Branch formats as #[error(\"member {member}: {rejection}\")], and StageRejection formats only {error}. When disjoint_test.rs maps Pair::start/resume errors via core_io::operation, rejection.attempted, rejection.committed, and attempted_before are discarded, and Pair::resume sets attempted_before: vec![] on rejection despite prior self.attempted_total. Expose work fields in Error::Branch/report output and record prior attempted work.",
  "3. [crates/mori-core/src/stage/disjoint.rs: declared_domains, disjoint, merge] declared_domains rejects any shared AssetId across the entire left and right Programs and scans all actions' Statement::Liability declarations. This blocks same-asset disjoint-account composition (even though merge() checks exact (asset, account) collisions) and couples admission to unused helper actions like TwoDeliveries::declare, while 0-await actions can still admit undeclared scenario duties. Scope domain checks to selected actions and allow disjoint accounts per AssetId.",
  "4. [crates/mori-core/src/stage/disjoint.rs: Pair::start; crates/fixtures/core-disjoint-product/mori-tests.json] Pair::start hardcodes starting member 0 then member 1 (ordering.start: [0, 1]). Consequently, the right-first case in mori-tests.json still starts member 0 before member 1 (invocation history [0, 1, 1, 0, 1, 0]), testing only post-start resume interleaving rather than right-first start linearization. Parameterize start order in the manifest/Pair::start or explicitly restrict the schedule claim in D2 and mori-tests.json.",
  "5. [crates/mori-core/src/stage/disjoint.rs: Pair::report; crates/mori/src/commands/test/disjoint_test.rs: run_case; crates/mori/src/commands/test.rs: aggregate_report] Pair::report() clones all cumulative history outcomes into every stage report, and run_case stores every stage's full report in stages. Across K events this duplicates O(K^2) StageOutcome envelopes inside one case Value, which hits mori_core::json::export's 1 MiB per-case cap in aggregate_report well below MAX_INVOCATIONS (256), and all 11 fixture golden files required by disjoint_test_public.rs are missing."
]

COMPLETE a3 consumer FINDINGS (unmodified strings, full original review physically pinned; no operator a3 content exists as a valid whole review):
[
  "`crates/mori-core/src/stage/disjoint.rs` (`Error::public_report`, `Error::Branch`, `Error::AfterInvocation`) & `crates/mori/src/commands/test/disjoint_test.rs` (`pair_error`): Passing `error.public_report()` (`moriarty-local-disjoint-error-draft/1`) to `core_io::operation` preserves `stage::outcome_report(Err(rejection))` counters, `earlier_start_attempted`/`earlier_start_local_committed`, and `pair_attempted_before`/`pair_committed_before`, while `disjoint_test.rs` halts on `PairError` as a positive manifest runner.",
  "`crates/mori-core/src/stage/disjoint.rs` (`Pair::resume`, `Error::Shape`): Pre-invocation errors in `Pair::resume` (`history.len() >= MAX_INVOCATIONS` or non-pending `member`) return `Error::Shape`, which `Error::public_report()` serializes as `detail.kind = \"before-invocation\"` with only `message` (omitting `pair_attempted_before` and `pair_committed_before`, unlike `Error::Branch` and `Error::AfterInvocation`). Any future negative diagnostic oracle for resume preflight refusals must account for this distinction.",
  "`crates/mori/src/commands/test/disjoint_test.rs` (`ObservationTables::reference`, `run_case`) & `crates/mori/tests/disjoint_test_public.rs` (`expand`): `run_case` compares the unprojected `Pair::report()` against `expected_start` and each `event.expected` before `ObservationTables::reference` verifies `starts_with` prefix monotonicity and replaces `history`/`effects` with `{\"table\":...,\"prefix_len\":N}` under `moriarty-local-disjoint-referenced-draft/1`. `expand()` losslessly restores `expanded_format` and slices `tables[name][..prefix_len]` to match the 10 full stage goldens.",
  "`crates/mori-core/src/stage/disjoint.rs` (`declared_domains`, `merge`, `Pair::start`): `declared_domains` checks static `Statement::Liability` names across whole programs (matching `decode::registry`), whereas 0-await actions skip `decode::canonical_boundary` in `admit::start` and can admit undeclared `Scenario.liabilities` via `runtime::scenario_snapshot`. If an undeclared 0-await scenario duty collides by name, `merge(start_pre(&first)?, start_pre(&second)?)` rejects before `Pair` publication via `Error::AfterInvocation`, preserving both members' `attempted` and `local_committed` counters."
]

Reconcile dissent independently: whole-program stronger asset disjointness is deliberate D1; same-asset case is not claimed. Starts[0,1] deliberately fixed D2. Existing false Waiting has segment:null, latest means latest invocation, original prior committed envelopes remain complete in history; do not invent a replacement segment. Actual None cli_cap uses stored cap below. Actual unchanged TwoDeliveries declare action registers duty and is not a newly added rewrite. RV1 addresses old lossy Error Display via actual command consumer and preserves late-returned work; D1 0-await preadmission remains explicitly separate. RV2 removes repeated cumulative arrays only from wire, preserving whole comparisons/original frames/effects/duties/history. New error/wire choices unadopted pending your substantive compatible review.

EXACT existing actual false outcome a3-inputs/main/crates/mori-core/src/stage/admit.rs L100-137
<<<
        let action = &sp.program().actions[index].name;
        let segment = usize::from(entry);
        match run.map_err(|error| fail(premise(error)))? {
            SegmentRun::NotAdvanced => {
                // Only a resume evaluates a guard; a false one changes nothing.
                let Some(c) = unchanged else {
                    return Err(fail(Error::new(
                        "stage_site",
                        "A start segment has no guard to observe.",
                    )));
                };
                let dispatch = cost::dispatch_cost(sp.program(), plan, entry).map_err(fail)?;
                cost::require_static(meter, dispatch).map_err(fail)?;
                emit_checked(sp, index, &c).map_err(fail)?;
                let owed = residual(&c.boundary);
                Ok(StageOutcome::Waiting {
                    advanced: false,
                    segment: None,
                    continuation: c,
                    attempted,
                    residual: owed,
                })
            }
            SegmentRun::Ran {
                execution,
                exit_site,
                captures_out,
            } => {
                let planned = *plan.segment_cost.get(segment).ok_or_else(|| {
                    fail(Error::new(
                        "stage_logical",
                        "Segment is absent from the stage plan.",
                    ))
                })?;
                cost::require_static(meter, planned).map_err(fail)?;
                let committed = cost::committed_after(plan, segment).map_err(fail)?;
                let expected_exit = if segment == plan.sites.len() {
                    0

>>>

EXACT existing actual start/resume a3-inputs/main/crates/mori-core/src/stage/admit.rs L222-392
<<<
pub fn start(
    sp: &StagedProgram,
    action: &str,
    scenario: Scenario,
    cap: Option<UInt>,
) -> Result<StageOutcome, StageRejection> {
    let refuse = |error: Error| rejected(error, UInt(0), UInt(0));
    let index = sp
        .action_index(action)
        .ok_or_else(|| refuse(Error::new("action", format!("Unknown action {action:?}."))))?;
    let selected = &sp.program().actions[index];
    let plan = sp
        .plan()
        .actions
        .get(index)
        .filter(|plan| plan.action == action)
        .ok_or_else(|| {
            refuse(Error::new(
                "stage_site",
                "Action is absent from the stage plan.",
            ))
        })?;
    let pre = runtime::scenario_snapshot(sp.program(), selected, &scenario).map_err(refuse)?;
    let limit = cost::start_preflight(plan, cap).map_err(refuse)?;
    let mut inputs = Vec::with_capacity(selected.inputs.len());
    for input in &selected.inputs {
        let value = scenario.inputs.get(&input.name).cloned().ok_or_else(|| {
            refuse(Error::new(
                "input",
                format!("Missing or wrong type for input {:?}.", input.name),
            ))
        })?;
        inputs.push(NamedValue {
            name: input.name.clone(),
            value,
        });
    }
    // A boundary that R7 would refuse must not start an awaiting action:
    // its continuation could never be resumed (H25).
    if !plan.sites.is_empty() {
        decode::canonical_boundary(sp, index, &inputs, &pre).map_err(refuse)?;
    }
    let mut meter = cost::segment_meter(limit);
    let run = runtime::run_segment(
        sp,
        index,
        SegmentEntry::Start { inputs: &inputs },
        pre,
        &mut meter,
    );
    Admitted {
        sp,
        action: index,
        entry: 0,
        committed: UInt(0),
        stored_cap: cap,
        frame_pre: PendingFrame::idle(),
    }
    .finish(run, &meter, None)
}

/// Resumes `c` with event `e`, applying R0 then R1-R7 in order. The first
/// failure rejects with attempted 0 and the input's committed value. A false
/// guard returns the input continuation unchanged, with its stored cap; the
/// CLI cap only checks this invocation.
pub fn resume(
    sp: &StagedProgram,
    c: Continuation,
    e: StageEvent,
    cli_action: Option<&str>,
    cli_cap: Option<UInt>,
) -> Result<StageOutcome, StageRejection> {
    // Echoes the unchanged input; it is unverified when R1-R6 refuse.
    let kept = c.logical.committed;
    let refuse = |code: &str, message: &str| rejected(Error::new(code, message), UInt(0), kept);
    let refused = |error: Error| rejected(error, UInt(0), kept);
    // R0: an explicitly selected action must be the continuation's action.
    if cli_action.is_some_and(|selected| selected != c.action) {
        return Err(refuse(
            "stage_site",
            "Selected action differs from the continuation action.",
        ));
    }
    // R1
    if c.format != CONTINUATION_FORMAT || c.logical.profile != LOGICAL_COST_PROFILE_2 {
        return Err(refuse(
            "stage_format",
            "Unsupported continuation format or logical profile.",
        ));
    }
    // R2
    profile::require_core2(sp.program()).map_err(refused)?;
    if !digest::matches(&c.program_hash, &sp.program_hash())
        || !digest::matches(&c.plan_hash, &sp.plan_hash())
    {
        return Err(refuse(
            "stage_program",
            "Continuation does not belong to this program and stage plan.",
        ));
    }
    // R3
    let Some(index) = sp.action_index(&c.action) else {
        return Err(refuse("stage_site", "Continuation action does not exist."));
    };
    let Some(plan) = sp
        .plan()
        .actions
        .get(index)
        .filter(|plan| plan.action == c.action)
    else {
        return Err(refuse(
            "stage_site",
            "Action is absent from the stage plan.",
        ));
    };
    if c.site == 0 || usize::from(c.site) > plan.sites.len() {
        return Err(refuse(
            "stage_site",
            "Continuation site is not a pending await of this action.",
        ));
    }
    let site = &plan.sites[usize::from(c.site) - 1];
    // R4
    if !layout_matches(&c.captures, site.captures.iter(), |s| {
        (s.name.as_str(), s.ty)
    }) {
        return Err(refuse(
            "stage_capture",
            "Captures differ from the pending site layout.",
        ));
    }
    // R5
    if e.format != STAGE_EVENT_FORMAT
        || e.action != c.action
        || e.label != site.label
        || !layout_matches(&e.values, site.binders.iter(), |b| (b.name.as_str(), b.ty))
    {
        return Err(refuse(
            "stage_event",
            "Event does not match the pending await.",
        ));
    }
    // R6
    let limit = cost::resume_preflight(plan, c.site, &c.logical, cli_cap).map_err(refused)?;
    // R7
    decode::canonical_boundary(sp, index, &c.captures, &c.boundary).map_err(refused)?;
    let mut meter = cost::segment_meter(limit);
    let run = runtime::run_segment(
        sp,
        index,
        SegmentEntry::Resume {
            site: c.site,
            captures: &c.captures,
            event: &e.values,
        },
        c.boundary.clone(),
        &mut meter,
    );
    Admitted {
        sp,
        action: index,
        entry: c.site,
        committed: kept,
        stored_cap: c.logical.cap,
        frame_pre: PendingFrame {
            site: c.site,
            captures: c.captures.clone(),
        },
    }
    .finish(run, &meter, Some(c))
}

>>>

EXACT whole original outcome projection a3-inputs/main/crates/mori-core/src/stage/report.rs L1-39
<<<
//! Complete qualified JSON envelopes for the local Core/2 outcome API.
use super::{STAGE_LABELS, StageOutcome, StageRejection};
use serde_json::{Value, json};

/// Render an outcome or rejection without executing a program or performing I/O.
/// Every envelope retains the explicit local-stage qualifications.
pub fn outcome_report(result: Result<StageOutcome, StageRejection>) -> Value {
    let mut report = match result {
        Ok(StageOutcome::Waiting {
            advanced,
            segment,
            continuation,
            attempted,
            residual,
        }) => {
            let committed = continuation.logical.committed;
            json!({"status":"waiting","advanced":advanced,"segment":segment,
                "continuation":continuation,"attempted":attempted,
                "committed":committed,"residual":residual})
        }
        Ok(StageOutcome::Complete {
            segment,
            committed,
            attempted,
            residual,
        }) => json!({"status":"complete","segment":segment,"attempted":attempted,
                "committed":committed,"residual":residual}),
        Err(rejected) => json!({"status":"rejected","error":rejected.error,
            "attempted":rejected.attempted,"committed":rejected.committed}),
    };
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in STAGE_LABELS {
        object.insert(name.into(), Value::String(value.into()));
    }
    report
}

>>>

EXACT actual complete balance/duty/scenario/snapshot DTOs a3-inputs/main/crates/mori-core/src/ir.rs L187-218
<<<
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub account: String,
    pub asset: AssetId,
    pub amount: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Liability {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub amount: UInt,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub state: BTreeMap<String, Value>,
    pub inputs: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub state: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}

>>>

EXACT stored-cap rule a3-inputs/main/crates/mori-core/src/stage/cost.rs L193-230
<<<
/// invocation only and can never raise the stored cap, which the output
/// continuation carries unchanged. Returns the meter limit of segment `site`.
pub(crate) fn resume_preflight(
    plan: &ActionPlan,
    site: u16,
    logical: &StageLogical,
    cli_cap: Option<UInt>,
) -> Result<UInt, Error> {
    let refuse = |message: &str| Error::new("stage_logical", message);
    let index = usize::from(site);
    let (Some(before), Some(cost)) = (
        plan.committed_before.get(index),
        plan.segment_cost.get(index),
    ) else {
        return Err(refuse("Pending site has no planned segment."));
    };
    if index == 0 || logical.committed != *before {
        return Err(refuse("Committed work differs from the stage plan."));
    }
    let needed = logical
        .committed
        .0
        .checked_add(cost.0)
        .ok_or_else(|| refuse("Committed work overflows u128."))?;
    let effective = match (cli_cap, logical.cap) {
        (Some(cli), Some(stored)) if cli > stored => {
            return Err(refuse("Logical cap cannot exceed the stored cap."));
        }
        (Some(cli), _) => Some(cli),
        (None, stored) => stored,
    };
    if effective.is_some_and(|cap| needed > cap.0) {
        return Err(refuse("Next segment exceeds the logical cap."));
    }
    Ok(*cost)
}

pub(crate) fn segment_meter(limit: UInt) -> Meter {

>>>

EXACT existing diagnostic consumer a3-inputs/main/crates/mori/src/commands/core_io.rs L158-164
<<<
pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    CliError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}

>>>

EXACT adopted ordinary whole Steps/DTO/status observations a3-inputs/main/docs/CORE-STAGED-TESTS.md L175-201
<<<
### Step attachment, values and complete segment snapshots

Every emitted Step SHALL retain exactly index, statement, value, snapshot and effects. Index SHALL be the zero-based original action statement ordinal; it SHALL NOT be renumbered per segment. Statement SHALL be the exact admitted public IR statement, including its numeric ExprId references into that Program's expressions array. Statement amount/value/predicate/guard references SHALL NOT be replaced by evaluated values. This requirement fixes reference meaning and exact comparison, not the parser's global arena allocation.

At start or after an accepted guard, reaching the next await SHALL end the segment before guard evaluation and SHALL emit no Step for that await entry. A normal true resume SHALL begin its segment with exactly one Step for the pending Await: outer value typed Bool true, snapshot equal to the segment pre-boundary and effects:[]. The exact Await statement SHALL retain kind, label, binders and guard ExprId. Its accepted event tuple SHALL NOT replace Step.value. A normal false resume SHALL emit no segment and therefore no public guard Step, value, effects or synthetic snapshot. The next true resume SHALL not repeat the earlier financial prefix. A newly reached later await SHALL again contribute no entry Step.

For each other normally completed statement in an emitted segment, the present outer Step.value SHALL be its typed result: Let's captured RHS; Bool true for Requires/Ensures at their written positions; Next's RHS before assignment; Emit's emitted value; Transfer/Fee's evaluated amount; Liability's added amount; Discharge's discharged amount. UInt SHALL be {type:"UInt",value:canonical unsigned decimal string}; Bool SHALL be {type:"Bool",value:JSON boolean}. Null or omission SHALL NOT replace those results. Liability's result is not its closing amount; Discharge's result is not the remaining duty. This is an adopted Core/2 attachment rule, not inheritance from the Core/1 design or an interest/accrual convention. No Mint, Burn or dedicated accrual construct is introduced; their broader missing financial obligations remain unchanged.

Each Step.snapshot SHALL contain exactly complete state, balances and liabilities after that statement, including preserved rows and present-zero duties. Inputs/captures SHALL NOT become state fields. Balances SHALL retain full account/AssetId/amount rows ordered by exact (asset,account), with AssetId (network,issuer,reference,scale); liabilities SHALL retain full name/AssetId/debtor/creditor/amount rows ordered by name. State values SHALL retain typed values. Empty state SHALL be {}, and empty balance/liability lists []. Segment execution.pre SHALL be the complete starting boundary: the normalized scenario on start, or input continuation.boundary on an accepted resume. Execution.post SHALL equal the last Step.snapshot, or pre if there are no Steps. This local report frame supplies no authenticated-domain or history claim and does not replace boundary admission conditions.

Step.effects SHALL contain only the new statement delta: Let/Requires/Ensures and an accepted Await have []; Next has one State {name,before,after}; Emit one Emit {name,value}; Transfer one Transfer {asset,from,to,amount}; Fee one Fee with those same field names; Liability creation/addition and Discharge each one Liability {name,asset,debtor,creditor,before,after}. The variant name SHALL be the kind field. Effect assets SHALL be full AssetIds, effect amounts/before/after canonical UInt strings; State/Emit values retain typed Value objects. Existing financial transitions determine the values. In already admitted zero/self cases the corresponding effect SHALL remain present even if numeric state does not change. Liability creation SHALL insert its row, addition preserve identity, and discharge retain a closing-zero row; payment remains separate. An effect before zero alone SHALL NOT collapse absent opening duty and present-zero opening duty. Execution.effects SHALL be the ordered concatenation of Step deltas within that segment, retaining repetitions. Cross-segment prefixes are derived histories, not an extra wire field or an authenticated history certificate.

### Status projection and process contract

The public outcome_report envelope SHALL be distinct from derived enum serialization. Every root SHALL have exactly the three qualification members qualification:"local-reference-stage", authentication:"none", consumption:"untracked", plus the applicable members below. All listed members SHALL be present; no others SHALL be added by this projection.

| Typed outcome / ordinary invocation | Additional exact root members and projection |
| --- | --- |
| Waiting after start reaches an await, or true resume reaches a later await | status:"waiting", advanced:true, segment:full StageSegment, continuation:full Continuation, attempted:UInt string, committed:continuation.logical.committed, residual:positive duties of continuation.boundary |
| Waiting after an admitted false resume | status:"waiting", advanced:false, segment:null, continuation:unchanged input Continuation, attempted:UInt string, committed:that continuation.logical.committed, residual:its boundary's positive duties |
| Complete at start or after true resume | status:"complete", segment:full StageSegment, attempted:UInt string, committed:the typed Complete committed amount, residual:positive duties of segment.execution.post; advanced and continuation SHALL be absent, not null |
| Existing StageRejection supplied to the renderer | status:"rejected", error:full typed Error, attempted:its UInt string, committed:its UInt string; advanced, segment, continuation and residual SHALL be absent; no error condition/code/order or refusal-work rule is selected here |

StageSegment SHALL contain exactly action, frame_pre, frame_post and execution. Execution SHALL contain exactly semantics:"moriarty-core/2", action, pre, post, steps and effects. Action names SHALL agree with the selected action. PendingFrame SHALL contain exactly numeric site and ordered captures. Start's frame_pre SHALL be idle; an accepted resume's frame_pre SHALL equal its input pending frame. Waiting frame_post SHALL equal continuation's site/captures and execution.post SHALL equal continuation.boundary. Completion frame_post SHALL be idle. A start with no statements before its first await SHALL still emit a non-null segment with steps:[], effects:[] and pre=post. A normally complete empty action SHALL likewise emit its full empty segment; no synthetic Step is added.

Continuation SHALL retain exactly format:"moriarty-continuation/1", program_hash:String, plan_hash:String, action, site, captures, logical and boundary. StageLogical SHALL retain exactly profile:"moriarty-reference-logical-cost/2", mandatory cap (canonical UInt string or null), and committed (canonical UInt string). A successful waiting start SHALL project the supplied start cap or null; an advancing waiting resume SHALL preserve input continuation.logical.cap, without replacing its reported stored value with a per-call override. This output projection selects no cap admission/editability policy. Positive residual rows SHALL retain the full Liability DTO in boundary order; present-zero rows remain in the complete boundary but are excluded from residual. No report in this scope claims settlement. Error SHALL retain code, message, offset and step, with existing numeric/null representation, without adopting any stage-specific code or admission precedence.


>>>

EXACT adopted ordinary logical work reporting a3-inputs/main/docs/CORE-STAGED-TESTS.md L206-210
<<<
### Local logical schedule and reporting authority

The adopted schedule for moriarty-reference-logical-cost/2 SHALL charge one unit before each ordinary statement dispatch and each distinct visited expression node, with a fresh node cache per statement. Both Boolean operands SHALL retain existing eager evaluation. Reaching the next await SHALL charge entry1 without visiting its guard and without emitting a Step. Resuming SHALL charge a separate dispatch1 plus the guard's distinct nodes. An admitted false guard SHALL report that invocation's attempted guard work and preserve the input logical.committed; a true guard SHALL charge that guard once and then the executed suffix, including any next await entry. No trace length SHALL substitute for this work count.

For an action with n awaits, segment indices SHALL be0..n: segment0 starts at action entry, segment j>0 begins with the guard dispatch of site j, and a successful segment stops before the next await guard while including its entry charge. ActionPlan.segment_cost SHALL hold n+1 successful segment costs in that order. committed_before[j] SHALL be the sum of earlier successful segment costs, with committed_before[0]=0; plan_total SHALL be their full sum. A normally successful segment j SHALL report attempted=segment_cost[j] and committed=committed_before[j]+segment_cost[j]. A waiting envelope SHALL project committed from its continuation; Complete SHALL use its typed field. False observations SHALL not accumulate into committed, although they remain separately attempted work. This is a local successful-prefix reporting convention, not authenticated lifetime-budget, financial commitment or history authority. No cap-editability, admission, resource ceiling or cap-refusal precedence policy is changed; those decisions remain with their owner.

>>>

EXACT all five operator obligations a3-inputs/main/openspec/sprints/sp10-private-handoff-and-bounded-composition.md L33-39
<<<
## SP10.1: Realize every required operator

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP10.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Implement sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging using the RP01 definitions. Check read/write compatibility and explicit ordering. Partition and conserve a global finite work measure.
- [ ] Verify: A required unsupported operator leaves MC06 open. Shared-state conflicts cannot masquerade as disjoint branches; async Pending does not claim atomic completion.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP10` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.

>>>

EXACT bounded history duty preservation a3-inputs/main/openspec/changes/mc06-private-handoff-and-composition/specs/mc06-private-handoff-and-composition/spec.md L18-24
<<<

#### Scenario: Compatible composition
- **WHEN** two independently proved compatible branches undergo the declared split and join
- **THEN** their new proofs discharge dependencies and preserve obligations, residual authority, uniqueness, and global lifecycle bounds.

#### Scenario: Composition attack
- **WHEN** composition duplicates predecessors, reuses outputs, mixes incompatible policies, exceeds fan-in, or resets bounds

>>>

INDEPENDENT ORACLE GAP (never a pass): all10 COMPLETE expanded product-stage goldens (left-first0..4/right-first0..4) plus exact public-report.json aggregate (11), full independently derived Program/StagePlan preimages/digests and derivation/freeze remain ABSENT. Revised diagnostics also lack independent expectation/actual observation. Complete original and revised oracle contracts physically pinned at a3-inputs/revised/ORACLE-ORIGINAL.md and ORACLE-REQUIRED.md govern all full rows, statements/ExprIds/results/frame/capture/work/residual distinctions. Actual complete report equality precedes any wire projection; independent public aggregate must restore and compare each full report, exact case tables/prefixes/format fields/bytes/final LF. No evaluator/compiler/report/projection code or actual outputs may generate expected values. Original sources/scenarios/events/caps16/36/sum52/fixedstarts/twoschedules unchanged. No runtime has been run. Adopt-design means only conditional ordinary local design; missing evidence/allfiveoperators/all original acceptance obligations stay open. No new semantic choices are proposed.
