# Complete ordinary source audit:36-file disjoint candidate, no execution
Exact main588d811dbbd9e88d22d3ebe90b0544672870caa0. Root adopted conditional ordinary D1-D5/RV1-RV3 on originalpatch3a04eaaf…01b865; chosen operator/codec semantics/draftlabels remain unchanged. New candidate adds verbatim frozen goldens and one meaningful publicstage::load regression. Fullcode and allnineinputs below;19expectedobjects/preimages are encoded in one lossless valueDAG with exactrawbyte roundtrips, raw originals always physically retained. independent-custody/ contains ALL74payloads+freeze12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848; aggregate839110B is not mandatory inline. Grammar: nodes[tag,payload], o=ordered[key,nodeID]members,a=nodeIDarray,v=exactscalar; roots[filename,nodeID,codec],0=sortedASCIIprettyindent2+LF,1=compactUTF8noLF. All19wholevalues+rawbytes verified, no producthelper added.
Financial/work/value expectations independently source-derived but nonblind/relatedpriororacle exposure disclosed. Full wire/schema/bytes conditional on implementation-derived authored codec and narrow finite sharedCoreoperation premise; NOT independent compiler-wire proof. Actualtests/consumer/Cargo/compiler/SDK/native UNRUN. Source-only text/patch/custodychecks and rustfmt on6ownedfiles only. Security/hostile/resource/constraint/proof/target judgments stay designatedowner. All five fulloperators and originalU0-U7/M1-M17/MC/SP/G exits/failures/caps/gaps/R7 remain open. Source audit may accept source, never claim observed behavior. before/ exact APIs, adoption/decisions and all raw expectations physical; inspect if needed, not duplicate full unrelated readlists.

WHOLE after/crates/fixtures/core-disjoint-product/approval-true.json
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

WHOLE after/crates/fixtures/core-disjoint-product/mori-tests.json
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

WHOLE after/crates/fixtures/core-disjoint-product/quote43.json
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

WHOLE after/crates/fixtures/core-disjoint-product/receipt-true.json
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

WHOLE after/crates/fixtures/core-disjoint-product/reserve-scenario.json
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

WHOLE after/crates/fixtures/core-disjoint-product/reserve.mori
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

WHOLE after/crates/fixtures/core-disjoint-product/settlement-true.json
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

WHOLE after/crates/fixtures/core-disjoint-product/two-deliveries-scenario.json
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

WHOLE after/crates/fixtures/core-disjoint-product/two-deliveries.mori
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

WHOLE after/crates/mori/Cargo.toml
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

WHOLE after/crates/mori/src/commands/test/disjoint_test.rs
<<<
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

>>>

WHOLE after/crates/mori/src/commands/test.rs
<<<
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

>>>

WHOLE after/crates/mori/tests/disjoint_test_public.rs
<<<
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

>>>

WHOLE after/crates/mori-core/Cargo.toml
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

WHOLE after/crates/mori-core/src/stage/disjoint.rs
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
                member,
                rejection,
                attempted_before,
                earlier_start_committed,
                pair_attempted_before,
                pair_committed_before,
            } => json!({
                "kind":"stage-rejection", "member":member,
                "rejection":super::outcome_report(Err(rejection.clone())),
                "earlier_start_attempted":attempted_before,
                "earlier_start_local_committed":earlier_start_committed,
                "pair_attempted_before":pair_attempted_before,
                "pair_committed_before":pair_committed_before,
            }),
            Self::AfterInvocation {
                phase,
                message,
                member,
                attempted,
                local_committed,
                pair_attempted_before,
                pair_committed_before,
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
    left.0
        .checked_add(right.0)
        .map(UInt)
        .ok_or_else(|| shape("logical sum exceeds UInt"))
}

fn declared_domains(
    program: &StagedProgram,
) -> (BTreeSet<&str>, BTreeSet<&AssetId>, BTreeSet<&str>) {
    let program = program.program();
    let states = program
        .states
        .iter()
        .map(|state| state.name.as_str())
        .collect();
    let assets = program.assets.iter().map(|asset| &asset.identity).collect();
    let duties = program
        .actions
        .iter()
        .flat_map(|action| &action.statements)
        .filter_map(|stmt| match stmt {
            Statement::Liability { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    (states, assets, duties)
}

fn disjoint(left: &StagedProgram, right: &StagedProgram) -> Result<(), Error> {
    let (left_state, left_assets, left_duties) = declared_domains(left);
    let (right_state, right_assets, right_duties) = declared_domains(right);
    if !left_state.is_disjoint(&right_state)
        || !left_assets.is_disjoint(&right_assets)
        || !left_duties.is_disjoint(&right_duties)
    {
        return Err(shape(
            "requires disjoint declared state names, exact assets and duty names",
        ));
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
    if balances
        .windows(2)
        .any(|rows| rows[0].asset == rows[1].asset && rows[0].account == rows[1].account)
    {
        return Err(shape("balance domains overlap"));
    }
    let mut liabilities = left.liabilities.clone();
    liabilities.extend(right.liabilities.iter().cloned());
    liabilities.sort_by(|a, b| a.name.cmp(&b.name));
    if liabilities
        .windows(2)
        .any(|rows| rows[0].name == rows[1].name)
    {
        return Err(shape("duty domains overlap"));
    }
    Ok(Snapshot {
        state,
        balances,
        liabilities,
    })
}

fn boundary(outcome: &StageOutcome) -> &Snapshot {
    match outcome {
        StageOutcome::Waiting { continuation, .. } => &continuation.boundary,
        StageOutcome::Complete { segment, .. } => &segment.execution.post,
    }
}

fn start_pre(outcome: &StageOutcome) -> Result<&Snapshot, Error> {
    match outcome {
        StageOutcome::Waiting {
            segment: Some(segment),
            ..
        }
        | StageOutcome::Complete { segment, .. } => Ok(&segment.execution.pre),
        _ => Err(shape("start did not supply a committed segment")),
    }
}

fn attempted(outcome: &StageOutcome) -> UInt {
    match outcome {
        StageOutcome::Waiting { attempted, .. } | StageOutcome::Complete { attempted, .. } => {
            *attempted
        }
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
        StageOutcome::Waiting {
            segment: Some(segment),
            ..
        }
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
                member: 0,
                rejection,
                attempted_before: vec![],
                earlier_start_committed: vec![],
                pair_attempted_before: None,
                pair_committed_before: None,
            })?;
        let second = super::start(&programs[1], &right.action, right.scenario, Some(caps[1]))
            .map_err(|rejection| Error::Branch {
                member: 1,
                rejection,
                attempted_before: vec![attempted(&first)],
                earlier_start_committed: vec![committed(&first)],
                pair_attempted_before: None,
                pair_committed_before: None,
            })?;
        let prepared = (|| {
            let attempted_total = sum(attempted(&first), attempted(&second))?;
            let opening = merge(start_pre(&first)?, start_pre(&second)?)?;
            merge(boundary(&first), boundary(&second))?;
            Ok((attempted_total, opening))
        })();
        let (attempted_total, opening) =
            prepared.map_err(|error: Error| Error::AfterInvocation {
                phase: "start",
                message: error.to_string(),
                member: None,
                attempted: vec![Some(attempted(&first)), Some(attempted(&second))],
                local_committed: vec![committed(&first), committed(&second)],
                pair_attempted_before: None,
                pair_committed_before: None,
            })?;
        let history = vec![
            Observation {
                member: 0,
                outcome: first.clone(),
            },
            Observation {
                member: 1,
                outcome: second.clone(),
            },
        ];
        Ok(Self {
            programs,
            caps,
            cap_total,
            attempted_total,
            opening,
            latest: [first, second],
            history,
        })
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
            &self.programs[member],
            continuation.clone(),
            event,
            Some(&continuation.action),
            None,
        )
        .map_err(|rejection| Error::Branch {
            member,
            rejection,
            attempted_before: vec![],
            earlier_start_committed: vec![],
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
                phase: "resume",
                message: error.to_string(),
                member: Some(member),
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

        let ordered_effects: Vec<_> = self
            .history
            .iter()
            .flat_map(|item| effects(&item.outcome))
            .collect();
        let history: Vec<_> = self.history.iter().map(|item| {
            json!({"member":item.member,"outcome":super::outcome_report(Ok(item.outcome.clone()))})
        }).collect();
        let members: Vec<_> = self
            .latest
            .iter()
            .map(|outcome| super::outcome_report(Ok(outcome.clone())))
            .collect();
        let frames = self.latest.each_ref().map(frame);
        let waiting = self
            .latest
            .iter()
            .any(|outcome| matches!(outcome, StageOutcome::Waiting { .. }));
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

WHOLE after/crates/mori-core/src/stage/mod.rs
<<<
//! Staged `moriarty/2` programs and their local continuation interface.
//!
//! Local and unsigned only. Event binders and captures are caller-chosen values
//! (no authentication), a continuation can be resumed any number of times (no
//! consumption tracking), and no stage authority exists. Outputs carry
//! `STAGE_LABELS` so these limits stay visible.
mod admit;
mod cost;
pub mod decode;
mod digest;
#[cfg(feature = "core2-disjoint-draft")]
pub mod disjoint;
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

WHOLE after/crates/mori-core/tests/disjoint_program_plan.rs
<<<
#![cfg(feature = "core2-disjoint-draft")]

use mori_core::{intent, stage};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

#[test]
fn complete_child_programs_and_costed_plans_match_independent_literals() {
    let fixtures =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core-disjoint-product");
    let expected: Vec<_> = ["two-deliveries", "reserve"]
        .into_iter()
        .map(|name| {
            let program: Value = serde_json::from_slice(
                &fs::read(fixtures.join(format!("{name}-program.json"))).unwrap(),
            )
            .unwrap();
            let plan: Value = serde_json::from_slice(
                &fs::read(fixtures.join(format!("{name}-plan.json"))).unwrap(),
            )
            .unwrap();
            let program_preimage =
                fs::read(fixtures.join(format!("{name}-program.preimage.bin"))).unwrap();
            let plan_preimage =
                fs::read(fixtures.join(format!("{name}-plan.preimage.bin"))).unwrap();
            (name, program, plan, program_preimage, plan_preimage)
        })
        .collect();

    // Whole independent objects include unused actions and all expression nodes.
    for (name, program, plan, program_preimage, plan_preimage) in expected {
        let source = fs::read_to_string(fixtures.join(format!("{name}.mori"))).unwrap();
        let actual = stage::load(&source).unwrap();
        assert_eq!(
            serde_json::to_value(actual.program()).unwrap(),
            program,
            "{name} Program"
        );
        assert_eq!(
            serde_json::to_value(actual.plan()).unwrap(),
            plan,
            "{name} Plan"
        );
        assert_eq!(
            intent::canonical(actual.program()).unwrap(),
            program_preimage,
            "{name} Program preimage"
        );
        assert_eq!(
            intent::canonical(actual.plan()).unwrap(),
            plan_preimage,
            "{name} Plan preimage"
        );
        let program_hash: [u8; 32] = Sha256::digest(&program_preimage).into();
        let plan_hash: [u8; 32] = Sha256::digest(&plan_preimage).into();
        assert_eq!(
            actual.program_hash(),
            program_hash,
            "{name} local Program digest"
        );
        assert_eq!(actual.plan_hash(), plan_hash, "{name} local Plan digest");
    }
}

>>>

WHOLE ONEOFF LOSSLESS EXPECTED-DATA VIEW (all19fullraw roots retained separately)
<<<
{"format":"one-off-lossless-review-JSON-value-DAG/1","notProductCode":true,"encoding":["nodes: o=ordered[key,node]members; a=nodeIDs; v=exactJSONscalar","roots:[custodyfilename,nodeID,codec]; codec0=sortedASCIIindent2plusLF; codec1=compactUTF8noLF"],"nodes":[["v","none"],["v","unadopted-local-disjoint-product"],["v","untracked"],["v","7"],["v","independent-staged-test-oracle"],["v","local"],["v","UNIT"],["v",0],["o",[["issuer",4],["network",5],["reference",6],["scale",7]]],["v","alice"],["v","Transfer"],["v","bob"],["o",[["amount",3],["asset",8],["from",9],["kind",10],["to",11]]],["v","UInt"],["v","1"],["o",[["type",13],["value",14]]],["v","0"],["o",[["type",13],["value",16]]],["v","State"],["v","phase"],["o",[["after",15],["before",17],["kind",18],["name",19]]],["v","35"],["o",[["type",13],["value",21]]],["v","40"],["o",[["type",13],["value",23]]],["v","basis"],["o",[["after",22],["before",24],["kind",18],["name",25]]],["v","47"],["v","continuation-reserve-oracle"],["o",[["issuer",28],["network",5],["reference",6],["scale",7]]],["v","5"],["v","target"],["v","reserve"],["v","Liability"],["v","allocation"],["o",[["after",27],["asset",29],["before",30],["creditor",31],["debtor",32],["kind",33],["name",34]]],["a",[12,20,26,35]],["v","moriarty-local-disjoint-draft/1"],["a",[]],["v",1],["o",[["captures",38],["site",39]]],["v","extra"],["v","2"],["o",[["type",13],["value",42]]],["o",[["name",41],["value",43]]],["v","held"],["v","42"],["o",[["type",13],["value",46]]],["o",[["name",45],["value",47]]],["a",[44,48]],["o",[["captures",49],["site",39]]],["a",[40,50]],["v",true],["v","deliver"],["v","13"],["o",[["account",9],["amount",54],["asset",8]]],["o",[["account",11],["amount",3],["asset",8]]],["v","observer"],["v","11"],["o",[["account",57],["amount",58],["asset",8]]],["v","treasury"],["o",[["account",60],["amount",16],["asset",8]]],["a",[55,56,59,61]],["v","duty"],["o",[["amount",30],["asset",8],["creditor",11],["debtor",9],["name",63]]],["a",[64]],["o",[["phase",15]]],["o",[["balances",62],["liabilities",65],["state",66]]],["v","moriarty-continuation/1"],["v","16"],["v","moriarty-reference-logical-cost/2"],["o",[["cap",69],["committed",30],["profile",70]]],["v","d9f7cba42cd4ce1dc1f70544a3592b8a0aba984c8f5cd779c3716d26ca1ac575"],["v","582b7923087e42b46c318576ad4bb786904b7ac0691a6c4580ed2deb187afe44"],["o",[["action",53],["boundary",67],["captures",38],["format",68],["logical",71],["plan_hash",72],["program_hash",73],["site",39]]],["v","local-reference-stage"],["a",[12,20]],["v","20"],["o",[["account",9],["amount",77],["asset",8]]],["o",[["account",11],["amount",16],["asset",8]]],["a",[78,79,59,61]],["o",[["phase",17]]],["o",[["balances",80],["liabilities",65],["state",81]]],["v","moriarty-core/2"],["a",[12]],["o",[["balances",62],["liabilities",65],["state",81]]],["o",[["amount",7],["asset",6],["from",9],["kind",10],["to",11]]],["o",[["type",13],["value",3]]],["o",[["effects",84],["index",7],["snapshot",85],["statement",86],["value",87]]],["a",[20]],["v","Next"],["o",[["kind",90],["state",19],["value",39]]],["o",[["effects",89],["index",39],["snapshot",67],["statement",91],["value",15]]],["a",[88,92]],["o",[["action",53],["effects",76],["post",67],["pre",82],["semantics",83],["steps",93]]],["o",[["captures",38],["site",7]]],["o",[["action",53],["execution",94],["frame_post",40],["frame_pre",95]]],["v","waiting"],["o",[["advanced",52],["attempted",30],["authentication",0],["committed",30],["consumption",2],["continuation",74],["qualification",75],["residual",65],["segment",96],["status",97]]],["o",[["member",7],["outcome",98]]],["v","allocate"],["o",[["account",57],["amount",46],["asset",29]]],["v","100"],["o",[["account",32],["amount",102],["asset",29]]],["o",[["account",31],["amount",16],["asset",29]]],["o",[["account",60],["amount",16],["asset",29]]],["a",[101,103,104,105]],["o",[["amount",27],["asset",29],["creditor",31],["debtor",32],["name",34]]],["a",[107]],["v","77"],["o",[["type",13],["value",109]]],["o",[["allocated",17],["basis",22],["marker",110]]],["o",[["balances",106],["liabilities",108],["state",111]]],["v","36"],["o",[["cap",113],["committed",58],["profile",70]]],["v","634bf5ef487e9f4b2a2828d89a6dd2afe158e0edc9514437fbae04245dbf7ef7"],["v","60ff43afae17f7d7d4f32e734f4cb841aff28552a7ec8223948a55e3b25b550a"],["o",[["action",100],["boundary",112],["captures",49],["format",68],["logical",114],["plan_hash",115],["program_hash",116],["site",39]]],["a",[26,35]],["o",[["amount",30],["asset",29],["creditor",31],["debtor",32],["name",34]]],["a",[119]],["o",[["allocated",17],["basis",24],["marker",110]]],["o",[["balances",106],["liabilities",120],["state",121]]],["v","Let"],["v",2],["o",[["kind",123],["name",45],["value",124]]],["o",[["effects",38],["index",7],["snapshot",122],["statement",125],["value",47]]],["a",[26]],["o",[["balances",106],["liabilities",120],["state",111]]],["v",5],["o",[["kind",90],["state",25],["value",129]]],["o",[["effects",127],["index",39],["snapshot",128],["statement",130],["value",22]]],["a",[35]],["v",6],["v","CREDIT"],["o",[["amount",133],["asset",134],["creditor",31],["debtor",32],["kind",33],["name",34]]],["o",[["effects",132],["index",124],["snapshot",112],["statement",135],["value",47]]],["a",[126,131,136]],["o",[["action",100],["effects",118],["post",112],["pre",122],["semantics",83],["steps",137]]],["o",[["action",100],["execution",138],["frame_post",50],["frame_pre",95]]],["o",[["advanced",52],["attempted",58],["authentication",0],["committed",58],["consumption",2],["continuation",117],["qualification",75],["residual",108],["segment",139],["status",97]]],["o",[["member",39],["outcome",140]]],["a",[99,141]],["v","52"],["a",[69,113]],["a",[30,58]],["o",[["attempted_total",69],["cap_total",143],["caps",144],["committed",145],["committed_total",69]]],["a",[98,140]],["v","invocation-then-source-order"],["a",[7,39]],["o",[["effects",148],["start",149]]],["a",[101,103,104,105,55,56,59,61]],["a",[107,64]],["o",[["allocated",17],["basis",22],["marker",110],["phase",15]]],["o",[["balances",151],["liabilities",152],["state",153]]],["a",[101,103,104,105,78,79,59,61]],["a",[119,64]],["o",[["allocated",17],["basis",24],["marker",110],["phase",17]]],["o",[["balances",155],["liabilities",156],["state",157]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",36],["format",37],["frames",51],["history",142],["logical",146],["members",147],["ordering",150],["post",154],["pre",158],["qualification",75],["residual",152],["status",97]]],["v","3"],["o",[["amount",160],["asset",8],["from",9],["kind",10],["to",11]]],["a",[12,20,26,35,161]],["v","ok"],["v","Bool"],["o",[["type",164],["value",52]]],["o",[["name",163],["value",165]]],["a",[166]],["o",[["captures",167],["site",124]]],["a",[168,50]],["v","10"],["o",[["account",9],["amount",170],["asset",8]]],["o",[["account",11],["amount",170],["asset",8]]],["a",[171,172,59,61]],["o",[["balances",173],["liabilities",65],["state",66]]],["o",[["cap",69],["committed",170],["profile",70]]],["o",[["action",53],["boundary",174],["captures",167],["format",68],["logical",175],["plan_hash",72],["program_hash",73],["site",124]]],["a",[161]],["o",[["name",163],["ty",164]]],["a",[178]],["v","Await"],["v","approval"],["o",[["binders",179],["guard",124],["kind",180],["label",181]]],["o",[["effects",38],["index",124],["snapshot",67],["statement",182],["value",165]]],["v",3],["o",[["amount",184],["asset",6],["from",9],["kind",10],["to",11]]],["o",[["type",13],["value",160]]],["o",[["effects",177],["index",184],["snapshot",174],["statement",185],["value",186]]],["a",[183,187]],["o",[["action",53],["effects",177],["post",174],["pre",67],["semantics",83],["steps",188]]],["o",[["action",53],["execution",189],["frame_post",168],["frame_pre",40]]],["o",[["advanced",52],["attempted",30],["authentication",0],["committed",170],["consumption",2],["continuation",176],["qualification",75],["residual",65],["segment",190],["status",97]]],["o",[["member",7],["outcome",191]]],["a",[99,141,192]],["v","21"],["a",[170,58]],["o",[["attempted_total",194],["cap_total",143],["caps",144],["committed",195],["committed_total",194]]],["a",[191,140]],["a",[101,103,104,105,171,172,59,61]],["o",[["balances",198],["liabilities",152],["state",153]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",162],["format",37],["frames",169],["history",193],["logical",196],["members",197],["ordering",150],["post",199],["pre",158],["qualification",75],["residual",152],["status",97]]],["v","allocated"],["o",[["after",47],["before",17],["kind",18],["name",201]]],["o",[["amount",46],["asset",29],["from",32],["kind",10],["to",31]]],["o",[["after",30],["asset",29],["before",27],["creditor",31],["debtor",32],["kind",33],["name",34]]],["v","78"],["o",[["type",13],["value",205]]],["o",[["after",206],["before",22],["kind",18],["name",25]]],["a",[12,20,26,35,161,202,203,204,207]],["v","quote"],["v","43"],["o",[["type",13],["value",210]]],["o",[["name",209],["value",211]]],["a",[44,48,212]],["o",[["captures",213],["site",124]]],["a",[168,214]],["v","15"],["v","26"],["v","58"],["o",[["account",32],["amount",218],["asset",29]]],["o",[["account",31],["amount",46],["asset",29]]],["a",[101,219,220,105]],["o",[["allocated",47],["basis",206],["marker",110]]],["o",[["balances",221],["liabilities",120],["state",222]]],["o",[["cap",113],["committed",217],["profile",70]]],["o",[["action",100],["boundary",223],["captures",213],["format",68],["logical",224],["plan_hash",115],["program_hash",116],["site",124]]],["a",[202,203,204,207]],["o",[["name",209],["ty",13]]],["a",[227]],["v",9],["o",[["binders",228],["guard",229],["kind",180],["label",181]]],["o",[["effects",38],["index",184],["snapshot",112],["statement",230],["value",165]]],["a",[202]],["v",4],["o",[["allocated",47],["basis",22],["marker",110]]],["o",[["balances",106],["liabilities",108],["state",234]]],["v",10],["o",[["kind",90],["state",201],["value",236]]],["o",[["effects",232],["index",233],["snapshot",235],["statement",237],["value",47]]],["a",[203]],["o",[["balances",221],["liabilities",108],["state",234]]],["v",11],["o",[["amount",241],["asset",134],["from",32],["kind",10],["to",31]]],["o",[["effects",239],["index",129],["snapshot",240],["statement",242],["value",47]]],["a",[204]],["o",[["balances",221],["liabilities",120],["state",234]]],["v",12],["v","Discharge"],["o",[["amount",246],["kind",247],["name",34]]],["o",[["effects",244],["index",133],["snapshot",245],["statement",248],["value",47]]],["a",[207]],["v",7],["v",15],["o",[["kind",90],["state",25],["value",252]]],["o",[["effects",250],["index",251],["snapshot",223],["statement",253],["value",206]]],["a",[231,238,243,249,254]],["o",[["action",100],["effects",226],["post",223],["pre",112],["semantics",83],["steps",255]]],["o",[["action",100],["execution",256],["frame_post",214],["frame_pre",50]]],["o",[["advanced",52],["attempted",216],["authentication",0],["committed",217],["consumption",2],["continuation",225],["qualification",75],["residual",120],["segment",257],["status",97]]],["o",[["member",39],["outcome",258]]],["a",[99,141,192,259]],["a",[170,217]],["o",[["attempted_total",113],["cap_total",143],["caps",144],["committed",261],["committed_total",113]]],["a",[191,258]],["a",[101,219,220,105,171,172,59,61]],["o",[["allocated",47],["basis",206],["marker",110],["phase",15]]],["o",[["balances",264],["liabilities",156],["state",265]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",208],["format",37],["frames",215],["history",260],["logical",262],["members",263],["ordering",150],["post",266],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["after",43],["before",15],["kind",18],["name",19]]],["v","Emit"],["v","done"],["o",[["kind",269],["name",270],["value",43]]],["a",[12,20,26,35,161,202,203,204,207,268,271]],["a",[95,214]],["v","6"],["a",[268,271]],["o",[["phase",43]]],["o",[["balances",173],["liabilities",65],["state",276]]],["v","seen"],["o",[["name",278],["ty",164]]],["a",[279]],["v","receipt"],["o",[["binders",280],["guard",233],["kind",180],["label",281]]],["o",[["effects",38],["index",233],["snapshot",174],["statement",282],["value",165]]],["a",[268]],["o",[["kind",90],["state",19],["value",129]]],["o",[["effects",284],["index",129],["snapshot",277],["statement",285],["value",43]]],["a",[271]],["o",[["kind",269],["name",270],["value",133]]],["o",[["effects",287],["index",133],["snapshot",277],["statement",288],["value",43]]],["a",[283,286,289]],["o",[["action",53],["effects",275],["post",277],["pre",174],["semantics",83],["steps",290]]],["o",[["action",53],["execution",291],["frame_post",95],["frame_pre",168]]],["v","complete"],["o",[["attempted",274],["authentication",0],["committed",69],["consumption",2],["qualification",75],["residual",65],["segment",292],["status",293]]],["o",[["member",7],["outcome",294]]],["a",[99,141,192,259,295]],["a",[69,217]],["o",[["attempted_total",46],["cap_total",143],["caps",144],["committed",297],["committed_total",46]]],["a",[294,258]],["o",[["allocated",47],["basis",206],["marker",110],["phase",43]]],["o",[["balances",264],["liabilities",156],["state",300]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",272],["format",37],["frames",273],["history",296],["logical",298],["members",299],["ordering",150],["post",301],["pre",158],["qualification",75],["residual",156],["status",97]]],["v","Fee"],["o",[["amount",42],["asset",29],["from",32],["kind",303],["to",60]]],["v","settled_quote"],["o",[["kind",269],["name",305],["value",211]]],["a",[12,20,26,35,161,202,203,204,207,268,271,304,306]],["a",[95,95]],["a",[304,306]],["v","56"],["o",[["account",32],["amount",310],["asset",29]]],["o",[["account",60],["amount",42],["asset",29]]],["a",[101,311,220,312]],["o",[["balances",313],["liabilities",120],["state",222]]],["v",8],["v","ready"],["o",[["name",316],["ty",164]]],["a",[317]],["v",16],["v","settlement"],["o",[["binders",318],["guard",319],["kind",180],["label",320]]],["o",[["effects",38],["index",315],["snapshot",223],["statement",321],["value",165]]],["v","Requires"],["v",19],["o",[["kind",323],["predicate",324]]],["o",[["effects",38],["index",229],["snapshot",223],["statement",325],["value",165]]],["a",[304]],["v",20],["o",[["amount",328],["asset",134],["from",32],["kind",303],["to",60]]],["o",[["effects",327],["index",236],["snapshot",314],["statement",329],["value",43]]],["a",[306]],["v",21],["o",[["kind",269],["name",305],["value",332]]],["o",[["effects",331],["index",241],["snapshot",314],["statement",333],["value",211]]],["a",[322,326,330,334]],["o",[["action",100],["effects",309],["post",314],["pre",223],["semantics",83],["steps",335]]],["o",[["action",100],["execution",336],["frame_post",95],["frame_pre",214]]],["o",[["attempted",170],["authentication",0],["committed",113],["consumption",2],["qualification",75],["residual",120],["segment",337],["status",293]]],["o",[["member",39],["outcome",338]]],["a",[99,141,192,259,295,339]],["o",[["attempted_total",143],["cap_total",143],["caps",144],["committed",144],["committed_total",143]]],["a",[294,338]],["a",[101,311,220,312,171,172,59,61]],["o",[["balances",343],["liabilities",156],["state",300]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",307],["format",37],["frames",308],["history",340],["logical",341],["members",342],["ordering",150],["post",344],["pre",158],["qualification",75],["residual",156],["status",293]]],["a",[12,20,26,35,202,203,204,207]],["a",[40,214]],["a",[99,141,259]],["v","31"],["a",[30,217]],["o",[["attempted_total",349],["cap_total",143],["caps",144],["committed",350],["committed_total",349]]],["a",[98,258]],["a",[101,219,220,105,55,56,59,61]],["o",[["balances",353],["liabilities",156],["state",265]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",346],["format",37],["frames",347],["history",348],["logical",351],["members",352],["ordering",150],["post",354],["pre",158],["qualification",75],["residual",156],["status",97]]],["a",[12,20,26,35,202,203,204,207,161]],["a",[99,141,259,192]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",356],["format",37],["frames",215],["history",357],["logical",262],["members",263],["ordering",150],["post",266],["pre",158],["qualification",75],["residual",156],["status",97]]],["a",[12,20,26,35,202,203,204,207,161,304,306]],["a",[168,95]],["a",[99,141,259,192,339]],["v","46"],["a",[170,113]],["o",[["attempted_total",362],["cap_total",143],["caps",144],["committed",363],["committed_total",362]]],["a",[191,338]],["o",[["balances",343],["liabilities",156],["state",265]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",359],["format",37],["frames",360],["history",361],["logical",364],["members",365],["ordering",150],["post",366],["pre",158],["qualification",75],["residual",156],["status",97]]],["a",[12,20,26,35,202,203,204,207,161,304,306,268,271]],["a",[99,141,259,192,339,295]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",368],["format",37],["frames",308],["history",369],["logical",341],["members",342],["ordering",150],["post",344],["pre",158],["qualification",75],["residual",156],["status",293]]],["v","moriarty-disjoint-case-observations-draft/1"],["v","left-first"],["v","effects"],["o",[["prefix_len",233],["table",373]]],["v","moriarty-local-disjoint-referenced-draft/1"],["v","history"],["o",[["prefix_len",124],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",374],["expanded_format",37],["format",375],["frames",51],["history",377],["logical",146],["members",147],["ordering",150],["post",154],["pre",158],["qualification",75],["residual",152],["status",97]]],["o",[["actual",378],["index",7],["passed",52]]],["o",[["prefix_len",129],["table",373]]],["o",[["prefix_len",184],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",380],["expanded_format",37],["format",375],["frames",169],["history",381],["logical",196],["members",197],["ordering",150],["post",199],["pre",158],["qualification",75],["residual",152],["status",97]]],["o",[["actual",382],["index",39],["passed",52]]],["o",[["prefix_len",229],["table",373]]],["o",[["prefix_len",233],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",384],["expanded_format",37],["format",375],["frames",215],["history",385],["logical",262],["members",263],["ordering",150],["post",266],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",386],["index",124],["passed",52]]],["o",[["prefix_len",241],["table",373]]],["o",[["prefix_len",129],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",388],["expanded_format",37],["format",375],["frames",273],["history",389],["logical",298],["members",299],["ordering",150],["post",301],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",390],["index",184],["passed",52]]],["v",13],["o",[["prefix_len",392],["table",373]]],["o",[["prefix_len",133],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",393],["expanded_format",37],["format",375],["frames",308],["history",394],["logical",341],["members",342],["ordering",150],["post",344],["pre",158],["qualification",75],["residual",156],["status",293]]],["o",[["actual",395],["index",233],["passed",52]]],["a",[379,383,387,391,396]],["o",[["effects",307],["history",340]]],["o",[["format",371],["name",372],["passed",52],["stages",397],["tables",398]]],["v","right-first"],["o",[["prefix_len",315],["table",373]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",401],["expanded_format",37],["format",375],["frames",347],["history",381],["logical",351],["members",352],["ordering",150],["post",354],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",402],["index",39],["passed",52]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",388],["expanded_format",37],["format",375],["frames",360],["history",389],["logical",364],["members",365],["ordering",150],["post",366],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",404],["index",184],["passed",52]]],["a",[379,403,387,405,396]],["o",[["effects",368],["history",369]]],["o",[["format",371],["name",400],["passed",52],["stages",406],["tables",407]]],["a",[399,408]],["v","passed"],["o",[["cases",409],["status",410]]],["a",[86,91,182,185,282,285,288]],["o",[["inputs",38],["name",53],["statements",412]]],["v","declare"],["o",[["amount",251],["asset",6],["creditor",11],["debtor",9],["kind",33],["name",63]]],["a",[415]],["o",[["inputs",38],["name",414],["statements",416]]],["a",[413,417]],["o",[["identity",8],["name",6]]],["a",[419]],["v","Literal"],["o",[["kind",421],["value",87]]],["o",[["kind",421],["value",15]]],["v","Name"],["o",[["kind",424],["name",163]]],["o",[["kind",421],["value",186]]],["o",[["kind",424],["name",278]]],["o",[["kind",421],["value",43]]],["o",[["kind",424],["name",19]]],["o",[["kind",421],["value",17]]],["a",[422,423,425,426,427,428,429,430]],["v","moriarty/2"],["v","TwoDeliveries"],["o",[["initial",17],["name",19],["ty",13]]],["a",[434]],["o",[["actions",418],["assets",420],["expressions",431],["language",432],["name",433],["semantics",83],["states",435]]],["a",[16,30,170]],["a",[30,30,274]],["o",[["binders",179],["captures",38],["label",181],["ordinal",124],["site",39]]],["v","Event"],["o",[["name",163],["ordinal",124],["origin",440],["ty",164]]],["a",[441]],["o",[["binders",280],["captures",442],["label",281],["ordinal",233],["site",124]]],["a",[439,443]],["o",[["action",53],["committed_before",437],["plan_total",69],["segment_cost",438],["sites",444],["statement_count",251]]],["a",[16]],["a",[42]],["o",[["action",414],["committed_before",446],["plan_total",42],["segment_cost",447],["sites",38],["statement_count",39]]],["a",[445,448]],["v","moriarty-stage-plan/1"],["o",[["actions",449],["format",450],["program_hash",73]]],["o",[["name",41],["ty",13]]],["a",[452]],["a",[125,130,135,230,237,242,248,253,321,325,329,333]],["o",[["inputs",453],["name",100],["statements",454]]],["a",[455]],["o",[["identity",29],["name",134]]],["a",[457]],["o",[["kind",424],["name",25]]],["o",[["kind",424],["name",41]]],["v","Binary"],["v","Add"],["o",[["kind",461],["lhs",7],["op",462],["rhs",39]]],["o",[["type",13],["value",30]]],["o",[["kind",421],["value",464]]],["v","Sub"],["o",[["kind",461],["lhs",184],["op",466],["rhs",233]]],["o",[["kind",424],["name",45]]],["o",[["kind",424],["name",209]]],["v","Ge"],["o",[["kind",461],["lhs",251],["op",470],["rhs",315]]],["v",14],["o",[["kind",461],["lhs",392],["op",462],["rhs",472]]],["o",[["kind",424],["name",316]]],["v","50"],["o",[["type",13],["value",475]]],["o",[["kind",421],["value",476]]],["v",17],["v","Le"],["v",18],["o",[["kind",461],["lhs",478],["op",479],["rhs",480]]],["a",[459,460,463,459,465,467,468,469,468,471,468,468,468,459,469,473,474,469,477,481,460,469]],["v","ReserveContinuationOracle"],["o",[["initial",24],["name",25],["ty",13]]],["o",[["initial",17],["name",201],["ty",13]]],["v","marker"],["o",[["initial",110],["name",486],["ty",13]]],["a",[484,485,487]],["o",[["actions",456],["assets",458],["expressions",482],["language",432],["name",483],["semantics",83],["states",488]]],["a",[16,58,217]],["a",[58,216,170]],["v","Input"],["o",[["name",41],["ordinal",7],["origin",492],["ty",13]]],["o",[["name",45],["ordinal",7],["origin",123],["ty",13]]],["a",[493,494]],["o",[["binders",228],["captures",495],["label",181],["ordinal",184],["site",39]]],["o",[["name",209],["ordinal",184],["origin",440],["ty",13]]],["a",[493,494,497]],["o",[["binders",318],["captures",498],["label",320],["ordinal",315],["site",124]]],["a",[496,499]],["o",[["action",100],["committed_before",490],["plan_total",113],["segment_cost",491],["sites",500],["statement_count",246]]],["a",[501]],["o",[["actions",502],["format",450],["program_hash",116]]]],"roots":[["left-first-0.json",159,0],["left-first-1.json",200,0],["left-first-2.json",267,0],["left-first-3.json",302,0],["left-first-4.json",345,0],["right-first-0.json",159,0],["right-first-1.json",355,0],["right-first-2.json",358,0],["right-first-3.json",367,0],["right-first-4.json",370,0],["public-report.json",411,0],["two-deliveries-program.json",436,0],["two-deliveries-program.preimage.bin",436,1],["two-deliveries-plan.json",451,0],["two-deliveries-plan.preimage.bin",451,1],["reserve-program.json",489,0],["reserve-program.preimage.bin",489,1],["reserve-plan.json",503,0],["reserve-plan.preimage.bin",503,1]]}

>>>

WHOLE SOURCE-DERIVED PROPOSED FIELD CODEC; not independent wireoracle
<<<
# Proposed complete ordinary field codec

Status: PROPOSED, SOURCE-DERIVED, NOT AN INDEPENDENT WIRE OR VALUE ORACLE. This records the existing selected representation without changing candidate bytes or semantics. D1-D5/RV1-RV3 are root-adopted separately; complete semantic and numeric expectations must be derived independently. The exact field sets below have no additional members unless explicitly stated. References to producer source in FIELD-CODEC-SOURCE-PINS.json are provenance only, not instructions or permission for the independent custodian to read producer/test bodies.

## Scalars, containers and local DTOs

UInt is a JSON string for an integer in0..2^128-1 matching ^(0|[1-9][0-9]*)$. Value is exactly {type,value}: type UInt uses the preceding string, type Bool uses a JSON boolean. ExprIds, Step.index, site/ordinal/statement_count, stage index, member index and prefix_len are JSON numbers, not typed Values. Type is UInt or Bool. Null is included only where specified; absence is distinct from null, zero, empty container and present-zero duty. State is an object from every declared state name to typed Value, not inputs/captures. Arrays preserve their described order.

AssetId={network,issuer,reference,scale}, scale a JSON u8 number. Balance={account,asset,amount}; Liability={name,asset,debtor,creditor,amount}; amount is UInt. Snapshot={state,balances,liabilities}, retaining ALL affected and untouched rows and present-zero liabilities. Balance rows sort by exact AssetId tuple(network,issuer,reference,scale), then account; liabilities sort by name; state keys follow the object codec. Empty state is{}, empty lists[]. Scenario={state,inputs,balances,liabilities}; input files retain their exact values.

NamedValue={name,value}. PendingFrame={site,captures}, captures ordered NamedValues. Idle frame has site0 and captures[]; an actual pending site can have no captures, so [] alone does not imply idle. StageLogical={profile,cap,committed}; profile is moriarty-reference-logical-cost/2, cap mandatory UInt|null, committed UInt. Continuation={format,program_hash,plan_hash,action,site,captures,logical,boundary}; format moriarty-continuation/1. Each child digest is its own whole Program/Plan identity, with no selected-action filtering. Boundary is the complete MEMBER snapshot.

StageSegment={action,frame_pre,frame_post,execution}. Execution={semantics,action,pre,post,steps,effects}; semantics moriarty-core/2. Step={index,statement,value,snapshot,effects}; index is the original action statement ordinal, statement is the complete admitted Statement including ExprIds into the child's whole Program. Await entry emits no Step; true resume starts with original Await typed Bool true at its pre-boundary; false resume has no segment or public guard Step. Present results, snapshots, effect variants and ordered deltas follow exact adopted Core2 clauses. No renumbering, synthetic success Step or member/global snapshot substitution is allowed.

Effect variants contain kind plus: State{name,before,after} with typed Values; Emit{name,value} with typed Value; Transfer/Fee{asset,from,to,amount}; Liability{name,asset,debtor,creditor,before,after}. Numeric amounts/before/after are UInt, not Values. Discharge uses Liability effect, preserving before/after and duty identity. These are original new deltas; ordered concatenation gives each member segment's effects. Statement amount/value/predicate/guard fields remain ExprIds, not evaluated replacements.

## Exact original member outcome projection

Every outcome envelope includes qualification:local-reference-stage, authentication:none, consumption:untracked.

| Variant | All other exact root fields |
|---|---|
| Waiting | status:waiting, advanced:boolean, segment:StageSegment|null, continuation:Continuation, attempted:UInt, committed:UInt, residual:Liability[] |
| Complete | status:complete, segment:StageSegment, attempted:UInt, committed:UInt, residual:Liability[]; advanced and continuation ABSENT |
| Original StageRejection | status:rejected, error:Error, attempted:UInt, committed:UInt; advanced/segment/continuation/residual ABSENT |

Error={code,message,offset,step}; offset and step retain existing numeric|null form. No error condition, precedence, code or charged failure policy is chosen by this field codec. Residual is every strictly positive duty in the MEMBER post-boundary, in boundary order; complete snapshots keep zero rows. Start/true-wait segment is present. False waiting has advanced:false, segment:null, unchanged input continuation and unchanged committed, but actual attempted guard work remains visible. Cap is stored in Continuation.logical and is not replaced by a per-call override; Pair starts supply each source input's cap, resumes pass no override and retain stored cap semantics. Do not derive committed from trace length.

## Complete Pair report before case projection

Root fields exactly: format,status,qualification,composition_qualification,authentication,consumption,ordering,pre,post,frames,members,history,effects,logical,residual. Literal format=moriarty-local-disjoint-draft/1; qualification=local-reference-stage; composition_qualification=unadopted-local-disjoint-product; authentication=none; consumption=untracked. Status is waiting if any child latest outcome is Waiting, otherwise complete; positive residual duties do not change that local action-completion meaning.

ordering={start,effects}: start[0,1], effects string invocation-then-source-order. Starts always occur in fixed order before the first recorded stage; left-first/right-first case names refer only to which member RESUMES first. Member indexes0 and1 follow manifest members array, never an asset/name sort.

pre is the flat union of BOTH normalized starting member execution.pre snapshots. post is the flat union of BOTH latest actual member boundaries, taking waiting boundary from continuation.boundary and complete boundary from segment.execution.post. Union preserves every member row, including untouched state, observer/treasury balances and incoming duties, with canonical row sorting. State names, exact asset domains and source-declared duty names are strongly whole-program disjoint under D1; actual row merges additionally refuse duplicate state keys, (asset,account) rows and duty names before Pair publication. No caller or expected boundary supplies execution state. A scenario's incoming nonawait duty not in static declarations remains subject to actual complete merge; the selected source checks do not invent pre-call admission or global financial authority.

frames is exactly two current PendingFrames in member order. members is exactly two COMPLETE ORIGINAL envelopes for each member's LATEST INVOCATION, including false/null if most recent invocation was false. Do not replace latest with last committed segment. history is the full invocation sequence of {member,outcome}, each outcome the original complete member envelope. It starts with both original starts in order, then each successfully published scheduled event observation, including a normally returned false waiting outcome. Prior committed segments remain intact in history even if latest has segment:null.

MEMBER execution.pre/post/Step.snapshot/boundary/residual describe only that member's complete declared/local scenario domain. They must NOT be padded with the other member's rows. Pair pre/post/residual are the complete GLOBAL flat union. The unselected member's boundary survives each selected-member resume unchanged in Pair post; local member snapshots still retain their own untouched rows. Neither member receives the other's state, captures, effects or inputs.

effects is the complete ordered concatenation of each invocation's segment execution.effects in history order, retaining repeats and zero/self deltas. A false/null segment appends no effects but still appears in history. logical={caps,cap_total,committed,committed_total,attempted_total}: caps and committed are two UInt arrays in member order; cap_total is the checked descriptive sum of child caps, committed_total the checked sum of latest child committed values, attempted_total the checked sum of attempted values for successfully published outcome observations (both starts plus published resumes, including false attempts). This is not a dynamic parent budget or a charge/refund/lifetime rule. Rejected resumes leave Pair/history/counters unchanged; their attempted work remains separate diagnostic data. residual is strictly positive GLOBAL post liabilities, preserving full rows and canonical order; zero duties stay in post.liabilities.

## Lossless public case references and aggregate

The unprojected Pair report above is each of the ten independent stage goldens. Every such complete golden is compared in full before projection. A case report has exactly format,name,passed,stages,tables. Format=moriarty-disjoint-case-observations-draft/1. name comes from manifest. stages is in recorded observation order: index0 after BOTH starts; each later index follows its event in manifest order. Stage={index,passed,actual}. passed is the full original report comparison result; case passed is conjunction of its stages. This specifies the comparison, not its independently expected value.

tables={history,effects}: two cumulative original arrays shared once per CASE, never across cases. Each stage actual keeps all original Pair fields except: format=moriarty-local-disjoint-referenced-draft/1; expanded_format retains original format; history={table:history,prefix_len:N}; effects={table:effects,prefix_len:M}. Each prefix_len is the COMPLETE ARRAY LENGTH observed at that stage, zero-based slicing [0,length), not the index of one item or a truncation budget. History references index the case history array of {member,outcome}; effect references index the case complete effect array. Prefix0 means[]; prefix equal table length means full table. Both arrays must extend the entire previous observed prefix by exact equality, without rewriting/decreasing/reordering. Empty effect deltas still preserve the correct prefix attachment.

Expansion substitutes tables.history[0,N) and tables.effects[0,M), restores format from expanded_format and REMOVES expanded_format. All other original Pair fields remain byte/value constituents. Expansion must equal the full independent stage golden, not a reconstruction generated by the producer. Final tables retain every original envelope/effect in complete order. No frame/duty/snapshot/history/effect is sampled or discarded.

The outer aggregate has exactly {cases,status}, with NO format field. cases follow manifest case order, each case retaining its stage/event order. status is passed iff all case comparisons pass, otherwise failed. The public aggregate independently expected bytes must represent the complete expanded-recoverable case reports, exact per-case table references and flags, canonical ASCII pretty rendering below, plus one final LF. The aggregate remains a separate consumer from library/per-invocation byte printing.

Object keys are lexical; arrays retain order; two ASCII spaces per nesting; LF entries; colon-space and nonfinal-entry comma; no trailing spaces/BOM/extra document. Empty objects/arrays are{} and[]. Strings escape quote/backslash, short control escapes, remaining U+0000-001F and U+007F as lowercase four-digit Unicode escapes, all non-ASCII as lowercase UTF-16 escape(s), surrogate pair when needed; other ASCII including slash literal. The aggregate embeds each fully exported case with correct additional indentation; the existing per-case1MiB/depth32 exporter bounds stay unchanged, and the aggregate is not silently clipped. No general size adequacy claim is made. Positive process uses exit0 when comparisons all match, otherwise1; ordinary stderr status uses the existing shell status with passed/failed counts. Exact diagnostic/error process bytes are not specified for an unselected error case by this packet.

## Pair error diagnostics: no fabricated attempt or state

Pair error report root exactly format,status,qualification,authentication,consumption,detail; format=moriarty-local-disjoint-error-draft/1, status=rejected and the same three local qualifications. There is no product frame, snapshot, effects or continuation in a failed Pair error report.

| detail.kind | Exact remaining detail fields |
|---|---|
| before-invocation | message only. No attempted/committed/prior-pair counters are invented as0 or null. |
| stage-rejection | member, rejection, earlier_start_attempted, earlier_start_local_committed, pair_attempted_before, pair_committed_before. rejection is the COMPLETE original StageRejection envelope above. Earlier-start vectors contain only earlier calls in that SAME initial start, empty where none, not all prior Pair history. Prior-pair fields are null before initial publication; on rejected resume they contain prior published attempted_total and two committed values. |
| composition-after-invocation | phase, message, member, invocation_attempted, candidate_local_committed, pair_attempted_before, pair_committed_before. phase=start or resume; member null for start or the selected resume member. invocation_attempted has TWO UInt|null entries, null means that member was NOT called in this operation, never fabricated zero-work. candidate_local_committed has TWO actual candidate local committed values. Prior-pair fields use the same null/published distinction. |

Late union/sum failure preserves returned child work without publishing candidate Pair state or mutating prior Pair. Original StageRejection attempted/committed stays original; prior Pair and earlier-start diagnostics are separate. Before-invocation Shape has message-only detail; do not manufacture rejection/work metadata to make its shape resemble later errors. This is distinct from original stage's rejected envelope and from ordinary full outcome reports.

Existing command error adapter receives the complete Pair error Value as its diagnostic payload, using the ordinary command-error path (code mori::command, rendered diagnostic message). It does not add this error as a passed case stage or emit a successful aggregate. No selected input here produces an independent error fixture; exact error/event/process expectations need independently bound ordinary inputs before any future check. No new refusal, failure-charge, or hostile case is authored here.

## Program/Plan preimages and absence of a Pair digest

Use the designated-owner M03 F4 EXACT contract for whole Program and cost-filled StagePlan canonical_bytes and SHA256 encoding. Whole Program retains unused actions/declarations and fresh global expression occurrences under M01. Whole StagePlan retains actions in Program order, with exact action/statement_count/sites/segment_cost/committed_before/plan_total fields. AwaitSite={site,ordinal,label,binders,captures}; StageBinder={name,ty}; CaptureSlot={name,ty,origin,ordinal}. Sites contains actual source await sites in order; it does NOT insert an idle site0 row. A no-await action therefore has sites[], while its segment_cost/committed_before arrays still describe its one action segment. PendingFrame idle site0 is separate from StagePlan.sites. This no-idle-row detail is explicitly source-derived field codec, not an independently established result.

Program and Plan preimages are compact UTF-8, recursively object-key sorted, arrays unchanged, no prefix/length framing/whitespace/final LF; use the exact owner scalar/enum/key sets. Standard compact UTF-8 escaping differs from PUBLIC ASCII pretty output; do not hash pretty reports, source bytes, selected action, Scenario, Pair state, flattened histories or projected case tables as a child Program/Plan identity. StagePlan preimage contains its FILLED child program_hash and complete static costs, never an empty placeholder. Digests are SHA256 over exact preimage bytes,64 lowercase ASCII hex chars.

There is NO Pair program_hash, plan_hash, separate Pair digest or Pair identity preimage field in the chosen wire. Do not invent one. Child Continuations carry their own child identities; public Pair/report/case qualification is not a new identity mechanism. Source-file/packet hashes identify custody bytes only and do not substitute for semantic Program/Plan digest values.

>>>

EXACT INDEPENDENT FINANCIAL/WORK/PROGRAM/PLAN/SCHEDULE DERIVATION; all raw expectations complete in view
<<<
## Complete child Programs and identities

TwoDeliveries has state phase/UInt/initial0, exact asset `(local,independent-staged-test-oracle,UNIT,scale0)`, selected deliver() and unused declare(). M01 gives nodes0..7: Literal7, Literal1, Name ok, Literal3, Name seen, Literal2, Name phase, Literal0. Deliver roots0,1,2,3,4,5,6 are Transfer, Next, Await approval, Transfer, Await receipt, Next, Emit done. Unused declare retains Liability duty/UNIT/alice/bob/root7. Initial state literals are direct declaration values, not arena nodes. Keeping unused declare is necessary to whole Program identity and source-wide duty identity; it is never executed.

ReserveContinuationOracle declares basis40,allocated0,marker77 and exact asset `(local,continuation-reserve-oracle,UNIT,scale0)` aliased CREDIT. Its sole allocate(extra:UInt) has22 fresh nodes0..21: basis,extra,Add(0,1); basis,Literal5,Sub(3,4); held; quote,held,Ge(7,8); held; held; held; basis,quote,Add(13,14); ready; quote,Literal50,Le(17,18); extra; quote. Statement roots are2,5,6,9,10,11,12,15,16,19,20,21. Every repeated name occurrence remains a separate node. Full metadata, state/action/input/asset arrays and all expression edges are frozen in the complete Program JSONs.

TwoDeliveries plan includes both actions. Deliver sites1/ordinal2/approval/ok:Bool with captures[], then2/ordinal4/receipt/seen:Bool with prior Event ok at ordinal2. It excludes pending seen. Deliver costs[5,5,6], committed_before[0,5,10], total16. Unused declare has sites[], costs[2], committed_before[0], total2. Its static cost is in the whole Plan identity but not the selected execution budget.

Reserve sites are1/ordinal3/approval/quote:UInt and2/ordinal8/settlement/ready:Bool. Site1 captures Input extra/UInt/ordinal0 then Let held/UInt/ordinal0. Site2 appends prior Event quote/UInt/ordinal3. State fields are excluded; pending quote/ready are excluded from their own sites. There is no liveness pruning. Actual captures are extra2,held42, then quote43. Plan costs[11,15,10], committed_before[0,11,26], total36. No synthetic idle row is inserted in either sites array under the explicit authored codec premise; PendingFrame idle0 remains separate.

M03 canonical preimages are compact UTF8 recursively key-sorted JSON, arrays unchanged, canonical UInt strings/u16-u8 numbers, no prefix/framing/whitespace/final LF. StagePlan is fully filled with child program_hash and all actions/costs before hashing. Every chosen value is ASCII, so independent standard JSON encoding has no non-ASCII escape ambiguity. Independent SHA256 gives:

| Child object | Preimage bytes | Local digest |
| --- | --- | --- |
| TwoDeliveries Program |1301|582b7923087e42b46c318576ad4bb786904b7ac0691a6c4580ed2deb187afe44|
| TwoDeliveries Plan |618|d9f7cba42cd4ce1dc1f70544a3592b8a0aba984c8f5cd779c3716d26ca1ac575|
| Reserve Program |2089|60ff43afae17f7d7d4f32e734f4cb841aff28552a7ec8223948a55e3b25b550a|
| Reserve Plan |744|634bf5ef487e9f4b2a2828d89a6dd2afe158e0edc9514437fbae04245dbf7ef7|

IDENTITY-DIGESTS names every exact object/preimage/source file. Pretty object file hashes differ from semantic digest preimages. These local diagnostic identities are not proof/authentication keys. **No Pair hash or Pair preimage is selected or invented.** The proposed positive test loads only eleven report files; separate actual consumers are needed to observe these four complete Program/Plan objects and canonical identities.

## Independent member values, complete frames and work

TwoDeliveries opens phase0, balances alice20,bob0,observer11,treasury0 and incoming duty5/alice→bob. Transfer7 gives alice13/bob7; Next phase1 updates immediately; await approval entry stops without a Step. Its first segment has indices0,1, values UInt7/UInt1 and deltas Transfer7/State0→1. Work2+2+entry1=5. Pending frame is site1/captures[], which is not idle despite empty captures.

Approval ok=true emits original Await index2, Booltrue, unchanged member boundary/effects[]. Transfer3 at index3 yields alice10/bob10, UInt3 and Transfer3. Await receipt entry emits no Step. Work guard2+transfer2+entry1=5; committed10. Pending site2 captures ok=true. Receipt seen=true emits Await index4/Booltrue, then Next phase2 at5/UInt2/State1→2 and Emit done2 at6/UInt2. Work2+2+2=6; committed16. Full balances close alice10,bob10,observer11,treasury0, total31. Duty5 is never discharged or reset by unused declare; it remains positive on completion. Observer11/treasury0 survive every prefix. No fee is charged.

Reserve opens basis40,allocated0,marker77, balances observer42,reserve100,target0,treasury0 and present allocation5/reserve→target. Let held=40+2 captures42 without changing state. Next basis=40−5 sets35. Liability adds42 to the existing duty5, yielding47; its Step.value is **42**, not47. Start indices0,1,2 have values42,35,42 and deltas[],State40→35,Liability5→47. Work4+4+2+entry1=11; pending approval captures extra2/held42.

Quote43≥held42 is true. Approval emits Await index3/Booltrue with pre-boundary/effects[]. Next allocated42 at4, Transfer42 at5, Discharge42 at6 and Next basis=35+43=78 at7 follow. Discharge Step.value is **42**, not closing5. Balances become reserve58,target42; duty returns47−42=5. Work guard4+2+2+2+4+next-entry1=15; committed26. Pending settlement captures extra2/held42/quote43. Held remains42 rather than being recaptured from basis35 or78.

Settlement ready=true emits Await index8/Booltrue. Requires quote43≤50 at9 emits Booltrue with unchanged state/effects[]. Fee extra2 at10 leaves reserve56/treasury2, emits UInt2/Fee2; Emit settled_quote43 at11 emits UInt43/Emit43. Work2+4+2+2=10; committed36. Final state basis78,allocated42,marker77. Balance sum142 stays142: observer42,reserve56,target42,treasury2. Gross reserve debit44 includes transfer42 and fee2; target net42, treasury net2. Allocation opening5+added42−discharged42=closing5 stays positive. Closing reserve56 is unspent balance, not rounding dust or an automatically protected recovery reserve.

FULL-PREFIX-SNAPSHOTS-AND-WORK and MEMBER-EXPECTED-SEGMENTS retain all19 unique source Step records, full local snapshots/deltas, every captured value and all six complete child outcomes per case. A member's execution/pre/post/steps/continuation/residual contain only its own complete supplied domain, never padding with the other asset/state. All19 source statements execute once per schedule; unused declare is excluded. No state map contains an input/let/event capture. All mathematical work is charged by adopted /2 statement/node/entry rules, not Step count or target cost.

## Both schedules and complete Pair fields

Whole-program state names, exact asset sets and source duty names are disjoint. Shared account strings observer/treasury are safe distinct rows here because their exact AssetIds differ. Equal reference UNIT and scale0 do not permit cross-asset addition. D1's strong subset excludes same-asset disjoint-account composition; it does not retire that broader required case. There is no hidden third domain.

Pair pre is the full union of both normalized original pre-snapshots. Pair post is the full union of both current member boundaries. It preserves four typed state fields, eight full balance rows and two duties, with canonical asset/account/name order. The other complete member boundary survives each selected member resume unchanged. Pair frames/members are in manifest order; members are latest original whole invocation envelopes. History retains starts0 then1 followed by the exact successful schedule observations; it is not reconstructed from final state.

All selected guards are true. The nine input files include no false or error event; concrete false/error goldens are therefore unselected rather than invented. Generic adopted false semantics remain: actual guard work contributes attempted, committed/continuation stay unchanged, latest segment is null and history still retains the observation. This positive packet does not observe that case or any failure diagnostic.

The underlying invocation schedules are starts0,1 then0,1,0,1 for left-first, or starts0,1 then1,0,1,0 for right-first. Each case has six member invocations and five Pair observations. Both reach52 attempted/committed units, caps16/36 unchanged and descriptive cap_total52; no member borrows capacity or resets work. Prefixes are:

| Case | Pair attempted/committed totals | History lengths | Effect prefix lengths |
| --- | --- | --- | --- |
| left-first |16,21,36,42,52|2,3,4,5,6|4,5,9,11,13|
| right-first |16,31,36,46,52|2,3,4,5,6|4,8,9,11,13|

The ten complete reports retain every original Pair/member field, full local and global frames, history envelope and ordered effect. Root members are latest invocations, not cumulative synthesized segments; original attempted fields remain per-invocation while Pair attempted_total sums all published calls. Committed values are per-child successful prefixes, summed separately. Start publication occurs only after both successful starts/union; no extra public Pair after the first start is invented.

Both final Pair snapshots and positive residual sets agree; ordered histories and effects differ. Root complete means both local actions completed while **allocation5 and duty5 remain positive**. No settlement, native acceptance or history proof follows. Explicit fee2 and logical52 are distinct from unmeasured DUST/service/native/circuit/wall-time/allocation costs; no interest/day-count/accrual convention, refund, mint, burn or rounding operation is added.


>>>

EXACT existing actual public canonical API before/crates/mori-core/src/intent.rs L72-88
<<<
pub fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    fn sort(v: serde_json::Value) -> serde_json::Value {
        match v {
            serde_json::Value::Object(map) => {
                let sorted: std::collections::BTreeMap<_, _> =
                    map.into_iter().map(|(k, v)| (k, sort(v))).collect();
                serde_json::Value::Object(sorted.into_iter().collect())
            }
            serde_json::Value::Array(a) => {
                serde_json::Value::Array(a.into_iter().map(sort).collect())
            }
            v => v,
        }
    }
    let value = serde_json::to_value(value).map_err(|e| Error::new("intent", e.to_string()))?;
    serde_json::to_vec(&sort(value)).map_err(|e| Error::new("intent", e.to_string()))
}

>>>

EXACT existing actual None->stored-cap preflight before/crates/mori-core/src/stage/cost.rs L193-230
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
