You are a FRESH independent Gemini4ArgonHigh HIGH ordinary consumer design reviewer. One420s context. Do not execute product/build/tests/network or modify any input. Public source only. Return exactly one complete JSON matching schema, reviewToken=doctorwatson-composition-revised-consumer-20261018-a3-whole-review. Six fields only; every D1-D5/RV1-RV3 needs its own substantive vote and code/API-specific reason. Summary<=1200 chars, findings<=5x600 chars, limitations<=5x400 chars. No previews, duplicate whole answers, approval by missing evidence, rubberstamp or invented API. Do not spend response budget restating the packet. Main actual APIs are pinned below and whole in main/; all full candidate code below. Review actual changed code, not only author summaries. Full three-file delta is revised/delta-from-original.patch. Lens consumer: emphasize real existing mori test dispatch/error/output consumer, lossless full expansion and complete oracle prerequisites. Both reviewers must still decide all eight choices. You may adopt conditional local draft design while explicitly retaining unobserved/missing-oracle limitations, or revise with concrete source mismatches. Consequential design needs two compatible substantive votes; no runtime or native/resource acceptance follows. Inspect additional pinned whole source only if needed.

# Full revised candidate: independent ordinary design review a3
Source only, no execution or product acceptance. Current implementation basis is read-only git show main 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb. All 31 old public API/program/consumer files are byte-identical; only consolidated requirements changed. Current whole source and complete requirements are in main/. Historical full57 inputs remain pinned in historical-context/; both a1 timeouts and a2 operator137/consumer0 revisions remain preserved; no historical result is a new vote. revised/ contains all16 candidate files plus complete14,020B three-file delta, original requirements/oracles, reconciliation and both unmodified full reviews.
Falsifiable review predicate: every retained D1-D5 and new RV1-RV3 choice is either substantively accepted as this explicitly scoped local draft or rejected with an actual code/API/requirement mismatch; neither missing goldens nor source arithmetic becomes observed success.
All eight choices UNADOPTED. Full changed code and unchanged source programs appear below. Read every candidate file and all eight decisions. Review ordinary composition/API/report design, not security/constraints/proofs/resource approval or hostile tests. Route those unanswered to designated owner. Your reasoned dissent is required when justified; earlier review text is evidence, not instruction.
All five operator obligations remain distinct: sequential Step (separate escrow oracle lane), full disjoint parallel including same-asset/account-disjoint and native Split, shared-state interleaving, atomic synchronization Join, asynchronous Pending/messages. This proposed stronger asset-disjoint local product addresses only a finite building block for operator2. Starts fixed[0,1]; right-first refers only to resumes. No native or full composition claim.

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/approval-true.json (209B SHA256 33bb3b6a2b2f6c0175f9c6398668ba7124db736d1da41e6424c70f3a0ad14cbc)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/mori-tests.json (2034B SHA256 76371519e3759ac87a85a63be6187a9eb6bacf84c499d112b5dd6c9c99c1654a)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/quote43.json (213B SHA256 43093a5c32bdadea307bc89149d17a472dedcd12d55d762cf28d35227b0b9b6f)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/receipt-true.json (210B SHA256 a730a5b45e532dfeb4e26860cc0d15265b893bbaa81bf95c79d1214ce92feee2)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/reserve-scenario.json (1459B SHA256 acddee29960a1d8850d0df27b0151d24da3247f2c8a37bbe9a8297a17b3971a4)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/reserve.mori (758B SHA256 260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/settlement-true.json (215B SHA256 a2fa104de81f75dd66b094334426badaf30939c250acd2f6abe3043520ceb07a)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/two-deliveries-scenario.json (1259B SHA256 5081ad889c98bed4e0ccd31f8f741ac856188523e6885d54832f60681e42e514)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/fixtures/core-disjoint-product/two-deliveries.mori (497B SHA256 b1f89331a3992e8f5135d9af6361a2347d0b45c0be74b0962c63ceb48de640c7)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/mori/Cargo.toml (2674B SHA256 44d3d5a6ccb8c5aa0cd9d6a7d60f7c6578b10a0952f668c85c95f78297cfd18a)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/mori/src/commands/test/disjoint_test.rs (6446B SHA256 998b4aae87aa3beef7df6b68487bc55b51286727f5fdd9c2dd72fac0067edfaf)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/mori/src/commands/test.rs (8142B SHA256 23b9a35834a5951543a9d932d15c4a13f356a435a0173ae8718f22448a869135)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/mori/tests/disjoint_test_public.rs (4036B SHA256 cc52bd16d413293ddf0116a3109383e4c33740468c7cb2c1cf2e14ed33e54873)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/mori-core/Cargo.toml (881B SHA256 93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/mori-core/src/stage/disjoint.rs (13326B SHA256 e6967611d3ec5375bbc9a019b31357baf82e294d77522d06603f0ea2a34e1df7)
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

WHOLE COMPLETE CANDIDATE: revised/proposed/crates/mori-core/src/stage/mod.rs (6382B SHA256 284ec48670778b37b9fe35797514812af5ad97b4d5d6a09decddaff3cb2b76fc)
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

WHOLE LITERAL DECISIONS/ORACLE: revised/ADOPTION-ORIGINAL.md (5839B SHA256 d9a8016d4fa47ca9dc72a78f6b9ca5a97766b94d485e3ddfbc1a9bf7345b027f)
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

WHOLE LITERAL DECISIONS/ORACLE: revised/ADOPTION-DELTA.md (3983B SHA256 994c7f4122faff9f354ff10d4ce726b56aff144d5729f31de03aeb6a735e41ab)
<<<
# Unadopted revision choices RV1–RV3

Read original ADOPTION.md verbatim in the preserved original proposal or compact full-context/proposal/. D1–D5 are not adopted. This revision retains the stronger full-program asset-disjoint subset, fixed start order [0,1], unchanged stage cap semantics, original latest-invocation envelopes (false observation has segment:null), and source publication behavior. The following additional choices require fresh independent substantive agreement; no prior review approves changed bytes.

RV1 — Lossless error diagnostic. The library exposes public_report() with format moriarty-local-disjoint-error-draft/1 and the original local/none/untracked labels. A StageRejection is included through original stage::outcome_report(Err(rejection.clone())), retaining every original error/attempted/committed value and qualification. Earlier calls in the same initial start have separately named attempted/local-committed diagnostics; prior published Pair attempted/committed values use distinct fields on resume. A late composition error includes phase, optional selected member and exact returned per-member attempted/candidate-local-committed values. A member not invoked in that operation has attempted:null rather than a fabricated zero-work result. Those local counters do not publish a product boundary, effects or continuation and do not declare a native debit/refund policy. Existing mori test passes this complete Value through core_io::operation, whose command diagnostic uses its JSON Display instead of Error's lossy summary Display. Independent expected diagnostic bytes are still absent/unobserved.

RV2 — Lossless case-level wire tables. Every complete Pair report still exists and is compared against a complete independent stage golden before any transformation. The public case now has format moriarty-disjoint-case-observations-draft/1 and tables.history/tables.effects arrays. Each stage preserves every other original whole product-report field but uses format moriarty-local-disjoint-referenced-draft/1, saves the exact original format as expanded_format and replaces only the two cumulative arrays with {table:history|effects,prefix_len:N}. N names the complete original array prefix, not a truncation rule. Prefix extension is checked by full equality; a rewritten/decreased earlier prefix errors before aggregate publication. All original history envelopes/effects are stored in the case tables, including the final complete arrays. Expansion substitutes exactly those prefixes, restores expanded_format and removes that representation-only key. The proposed public binary test compares every expansion against its original whole literal golden, in addition to exact independently literal aggregate bytes.

RV3 — Existing bounds and unresolved evidence. The per-case aggregate exporter and input/invocation limits remain unchanged; there is no larger output limit, silent sample, truncation or waiver. Eliminating repeated cumulative arrays removes that source of quadratic serialized duplication but does not certify memory, host cost or byte adequacy for every allowed input. Pair::report remains a complete-report API; constructing/comparing cumulative reports still has repeated host work, which is not a target cost claim. The existing stronger subset and nonawait incoming-duty preadmission rule remain separate D1 review questions; this repair only ensures late failures retain original work metadata. Prior Pair publication/counters remain unchanged on failure, with rejected attempted work separately visible in its error record. No global failure-charge or native lifetime policy is invented.

Full native/financial acceptance and all five operators remain separate. Neither two local sources nor two schedules nor source review closes native Step/Split/Join, shared-state interleaving or asynchronous messaging. All required independent oracles, actual consumer observations and original parent exits stay open.

>>>

WHOLE LITERAL DECISIONS/ORACLE: revised/ORACLE-ORIGINAL.md (6675B SHA256 da18517bd2d0d6635a0cf241b194b4b912ebcd1f56f6de813bf3aa6772920396)
<<<
# Required independent positive oracle before execution

No expected output is authored or generated in this packet. The existing sequential escrow lane is separate and its pending goldens are neither inputs nor consulted. An independent ordinary oracle author, separate from this source author and blinded to future evaluator output, must first derive and freeze this exact draft's complete observations after D1–D5 adoption. Do not use evaluator/compiler/report code, actual-output capture, shared helper replay or this implementation to compute expected financial results.

Freeze all proposed fixture/source inputs and the complete candidate patch by full SHA-256 and byte size. Freeze independent complete Program and StagePlan representation preimages for **both unchanged copied source files**, their independently computed canonical program/plan digests, statement/expression IDs, all unused actions/nodes, capture layouts and costs. Disclose source-derived parser/serialization/admission premises and the derivation method as premises, not normative proof results. Preserve all input/event and normal-work/finance provenance. Hashing independently literal preimages is allowed; importing Moriarty to generate them is not.

Required literal new members under `crates/fixtures/core-disjoint-product/`:

- `left-first-0.json` through `left-first-4.json`: five whole Pair reports after initial pair start and each of the four `left-first` manifest events.
- `right-first-0.json` through `right-first-4.json`: five whole Pair reports for the distinct `right-first` schedule. The two index0 values share identical declared start ordering; later histories/effects are independently derived for their actual schedules.
- `public-report.json`: exact public `mori test` aggregate with cases in manifest order, all ten stage results, all original nested whole values, status passed and one final newline. Derive it as literal aggregate assembly of frozen reports, not from runtime output.
- Independent Program/Plan preimages, `DERIVATION.md` with full normal arithmetic/history and disclosed premises, `INDEPENDENT-FREEZE.json` listing exact input/source/premise/golden hashes/sizes, actual author/reviewer model identities/effort/status/dissent/independence limits. These are provenance members, not replacement expected outputs.

Each Pair report must contain every actual proposed root key: format, local qualification fields, status, explicit ordering, complete opening/pre and current/post Snapshots, both current frames, both latest original whole member reports, ordered full invocation history (including both starts), cumulative ordered effects, both caps and their sum, both committed counters and their sum, attempted_total and every positive residual duty. Each original nested report must retain its complete original shape/labels, action/program/plan identities, segment/frame_pre/frame_post, all captured typed values/cap/site/boundary fields, every complete Execution pre/post/step, exact statement IR/expression indices/value/snapshot/effects, ordered segment effects, residual and attempted/committed/null/absence distinctions. No key projections, normalized expected arrays, expected-driven continuation, host success stub or final-only oracle is sufficient.

Exact input branch0 is copied TwoDeliveries.deliver: UNIT `(local,independent-staged-test-oracle,UNIT,0)`, cap16; phase0; alice20,bob0,observer11,treasury0; original duty5 alice→bob remains. branch1 is copied ReserveContinuationOracle.allocate: CREDIT `(local,continuation-reserve-oracle,UNIT,0)`, cap36; basis40,allocated0,marker77; extra2; observer42,reserve100,target0,treasury0; incoming allocation5 reserve→target. Branch1 scenario is new literal input specified here; it is not claimed to be an independently reviewed old oracle copy. Events are ordinary true approval/receipt, reserve quote43 and ready=true, exactly as proposed manifest files.

Normal source arithmetic, supplied for independent checking rather than as an independent full oracle: branch0 transfers7 then3 to bob, sets phase1 then2 and emits done(2), retaining all unrelated UNIT rows and duty5. branch1 captures held42 from basis40+extra2, changes basis35 and allocation5→47; approval43 sets allocated42, transfers42 CREDIT to target, discharges42 leaving allocation5, sets basis78; final readiness verifies quote≤50, charges fee2 to treasury and emits settled_quote(43). Final UNIT rows alice10,bob10,observer11,treasury0 sum31, exact fee0 and gross debit alice10. Final CREDIT rows observer42,reserve56,target42,treasury2 sum142; reserve gross44=transfer42+fee2, target net42 and treasury fee2. marker77 and both observers/UNIT treasury0 remain present. Both allocation5 and duty5 remain positive residuals with exact debtor/creditor/asset identities; debt is not token supply. Preserve every intermediate snapshot/effect and source statement order, not only these terminal values.

Independent work enumeration: TwoDeliveries segments `[5,5,6]`, committed `[5,10,16]` with fixed cap16; Reserve segments `[11,15,10]`, committed `[11,26,36]` with fixed cap36. Initial Pair start commits16 (5+11). Left-first aggregate committed/attempted totals after each report are `[16,21,36,42,52]`; right-first `[16,31,36,46,52]`. These positive schedules have no false observation and attempted_total equals committed_total at each report. No underlying cap changes or discarded prior work is allowed. If the independent derivation disagrees, preserve the discrepancy and resolve the spec/source premise before execution; never capture runtime to choose an expected number.

Ordering premise: start history `[0,1]`; left-first complete invocation history `[0,1,0,1,0,1]`; right-first `[0,1,1,0,1,0]`. Root cumulative effects concatenate only committed effects in that order and the actual source order within each original segment. Global sorted state/balance/liability storage order is separate from effect/history order. Both final Snapshots match, but the final complete reports intentionally differ in history/effect order. Neither schedule is a shared-state interleaving or an atomic Join proof.

Before execution: independent oracle and exact candidate/adoption review must exist; all hash pins remain current; actual consumer invocation stays within its separately applicable execution boundary. This assignment grants no execution. After a permitted observation, retain actual command/executable/features/source/golden pins, exit, stdout/stderr/output digests and independently reviewed differences. No passing test can close native/proof/ledger/private composition or any other retained operator or parent exit.

>>>

WHOLE LITERAL DECISIONS/ORACLE: revised/ORACLE-REQUIRED.md (3238B SHA256 a536fd1db0f7102882c63fbae93dbb74903bd860a54cbb9c53687e3c4f2039cd)
<<<
# Independent oracle contract for the revised source

All original independent-oracle requirements in the original proposal ORACLE-REQUIRED.md remain in force: independently derived complete source Program/StagePlan/preimages/digests, exact source/input/event hashes, ten complete product reports (left-first-0..4.json, right-first-0..4.json), derivation/premise/identity/freeze records and an exact public aggregate. None exists as observed evidence here. Do not import producer/evaluator/compiler/report/projection code or capture actual output to compute expected values. The separate escrow oracle lane is not consulted.

The positive source programs, inputs, caps16/36, start order[0,1], resume schedules, original stage outcomes, complete per-step financial boundaries, all duties and expected ten full Pair reports are unchanged by the intended positive behavior of this revision. That is a source proposal, not a passing result. Supply/verify those ten full reports independently before execution. They remain literal full expanded reports with history/effects arrays and original product format; run_case compares actual complete reports with them before projection.

public-report.json must be newly and independently authored for the explicit RV2 case-table wire. For each case freeze every stage's complete retained product fields and exact history/effect prefix references, the shared tables containing every original complete invocation envelope/effect in order, changed case and referenced-report format identifiers, expanded_format values, result flags, case/stage order and canonical public aggregate bytes/newline. Independently expand every reference to the corresponding full literal golden and compare every field, including frames, statement IR/value/snapshot/effects, complete product state/balance/liability domains, captures, work, all residual duties and present-zero/absent distinctions. Never regenerate a full expected report from a producer projection. Literal independent golden prefix assembly/checking must disclose its data-only method and remain separate from the code under review.

New error format/diagnostic bytes also lack an independent oracle and actual observation. Before executing an ordinary business-rejection check, an independent ordinary oracle author must freeze the specific unchanged source, supplied event/scenario, expected exact original StageRejection(error/attempted/committed), all prior published Pair work fields and any earlier-in-start local work values, and the exact public command diagnostic. A post-invocation composition error similarly needs independently specified phase/member/raw work vectors and no product effects/boundary/continuation. No hostile/security case is authored in this packet; those remain designated-owner work. Do not infer a failed-attempt global fee/debit/refund policy from these descriptive raw counters.

All consumer/build/tests/runtime remain NOT RUN; changed bytes require fresh source/design reviews, and missing goldens cannot be replaced by a source hash, model verdict, same-evaluator replay, host success wrapper, empty fixture, ignored test or weakened assertion. No execution or native/resource authority is granted by this contract.

>>>

RECONCILIATION TO TEST, NOT AN AUTHOR VETO: stronger whole-program exact-asset disjointness, fixed starts, latest invocation including false/null, and resume cli_cap=None are deliberate original choices. In actual cost::resume_preflight None uses stored cap. Check independently before treating these as bugs. Original source declare action was unchanged, not added by this proposal. Complete history retains prior committed segment envelopes even when latest observation is false/null. Both prior full dissents remain verbatim below, including recommendations author disputes. Demonstrated public counter loss and repeated cumulative serialized arrays motivated RV1/RV2; those new representations require fresh votes. 0-await incoming-duty preadmission remains an explicit D1 question; late union collision is checked before Pair publication and revised errors retain attempted metadata. Source-only algebra does not establish byte/resource adequacy.

WHOLE UNMODIFIED COMPLETE a2 DISSENT: revised/reviews/operator-REVIEW.json (7033B SHA256 6c0f67f58095b03a5a6c8f2fe728506f5453b98c68f0036df1f96f59287e79df)
<<<
{
  "decisionVotes": {
    "D1": {
      "reason": "`disjoint.rs::declared_domains` only checks `Statement::Liability`, omitting `Statement::Discharge` and `Scenario.liabilities` (admitted on 0-await actions without `canonical_boundary`). Whole-program `AssetId` disjointness also excludes required same-asset disjoint-account composition under SP10.1.",
      "vote": "revise-design"
    },
    "D2": {
      "reason": "`Pair::start` and `Pair::resume` in `disjoint.rs` deterministically linearize starts as `[0,1]`, step one pending member per event without cross-branch state reads, and order `effects` by invocation then source order, keeping distinct schedules non-equivalent despite equal final `post` state.",
      "vote": "adopt-design"
    },
    "D3": {
      "reason": "`Pair::start` runs both `stage::start` calls before `merge`/`sum`, so `Error::Shape` discards executed `attempted` work. `Pair::resume` sets `attempted_before: vec![]` and drops `rejection.attempted`. `disjoint_test.rs` maps `Error::Branch` via `Display`, losing `attempted_before` and `committed`.",
      "vote": "revise-design"
    },
    "D4": {
      "reason": "`Pair::report` tracks `caps` and `committed` per member plus `attempted_total`, but omits per-member cumulative `attempted` and rejected resume work. `disjoint_test.rs` duplicates growing `history` across `stages` under the 1 MiB per-case `json::export` cap; resource adequacy is routed to owner.",
      "vote": "revise-design"
    },
    "D5": {
      "reason": "Feature gating (`core2-disjoint-draft`) and local-only labels (`unadopted-local-disjoint-product`, `authentication: none`, `consumption: untracked`) are sound, but `disjoint_test_public.rs` fails until missing goldens exist and false-guard `resume` overwrites `members[i]` with `segment: None`.",
      "vote": "revise-design"
    }
  },
  "findings": [
    "`crates/mori-core/src/stage/disjoint.rs` (`declared_domains`, `disjoint`, `Pair::start`): `declared_domains` only collects `Statement::Liability`, ignoring `Statement::Discharge` and `Scenario.liabilities` (admitted by `scenario_snapshot` on 0-await actions when `admit::start` skips `canonical_boundary`). `Pair::start` runs both `super::start` calls before `merge`, so duty collisions or `sum` overflows fail with `Error::Shape` after execution and discard `attempted` work. Change: validate `Statement::Discharge`, `scenario.liabilities`, and pre-snapshot disjointness before `super::start`.",
    "`crates/mori-core/src/stage/disjoint.rs` (`Pair::resume`, `Pair::report`, `Error::Branch`): On `Err(rejection)`, `Pair::resume` sets `attempted_before: vec![]` instead of exposing prior `self.attempted_total` and does not accumulate `rejection.attempted`. On a false guard (`Waiting { advanced: false, segment: None, .. }`), `Pair::resume` overwrites `self.latest[member]`, replacing the member's last committed segment in `report()[\"members\"]`, while `report()[\"logical\"]` omits per-member `attempted: [UInt; 2]`. Change: track per-member `attempted` and preserve `attempted_before`.",
    "`crates/mori/src/commands/test/disjoint_test.rs` (`run_case`) & `crates/mori-core/src/stage/disjoint.rs` (`Error::Branch`): `Error::Branch`'s `Display` (`#[error(\"member {member}: {rejection}\")]`) omits `attempted_before`, `rejection.attempted`, and `rejection.committed`, so `core_io::operation` drops work counters and cannot test rejections like `stage_test.rs`. Also, embedding full `Pair::report()` `history` in every stage grows $O(N^2)$ against `mori_core::json::export(&case)`'s 1 MiB cap in `test.rs::aggregate_report`. Change: report rejection work metadata and bound history.",
    "`crates/fixtures/core-disjoint-product/two-deliveries.mori` (`declare`), `mori-tests.json`, & `crates/mori/tests/disjoint_test_public.rs`: Because `decode::canonical_boundary` requires awaiting-action scenario duties to appear in `Statement::Liability`, `two-deliveries.mori` adds a dummy `action declare()` to keep untouched duty `duty`. Also, `disjoint_test_public.rs` unconditionally reads `public-report.json` and `mori-tests.json`'s 10 stage goldens (`left-first-0..4.json`, `right-first-0..4.json`), which are absent. Change: supply the 11 independent oracle files and preimages."
  ],
  "limitations": [
    "All 10 stage golden files (`left-first-0..4.json`, `right-first-0..4.json`), `public-report.json`, and independent `Program`/`StagePlan` preimages (`DERIVATION.md`, `INDEPENDENT-FREEZE.json`) required by `ORACLE-REQUIRED.md` are absent, and no build, Cargo, `mori`, or test execution was run (`SOURCE-ONLY` review). Missing goldens and unrun tests are not a pass; D1\u2013D5 remain unadopted.",
    "`Pair` is a local, unauthenticated, untracked source consumer for asset-disjoint pairs only. All 5 operators remain separately open under MC06/SP10.1/U4: sequential (`Step` and the separate sequential escrow lane), full disjoint parallel (same-asset disjoint-account and native circuit `Split`), shared-state interleaving, atomic synchronization (`Join`), and async messaging (`Pending`).",
    "No security, constraint, proof, target-phase, or resource authorization judgment is made here: memory/output adequacy of retaining 256 full outcomes in `Pair::history`, merged `Snapshot` bounds (up to 8,192 balances/2,048 liabilities), dispatch/merge/serialization cost certification, and hostile test rules are flagged unanswered for the designated Opus/fallback owner.",
    "All existing parent exits, failure records, and logical/native caps remain unchanged and open; `Pair` keeps static per-member caps (`16` and `36`, descriptive sum `52`) without dynamic global budget partitioning, native head lifetime conservation (`revision + remaining = allocatedLifetime(head)`), predecessor proof verification, or authority attenuation."
  ],
  "reviewToken": "doctorwatson-composition-disjoint-compact-operator-20261018-a2-whole-review",
  "summary": "Independent source-only operator review of the 16-file `core2-disjoint-draft` candidate (`Pair` in `crates/mori-core/src/stage/disjoint.rs`, CLI runner `crates/mori/src/commands/test/disjoint_test.rs`, `crates/mori/src/commands/test.rs`, `crates/mori/tests/disjoint_test_public.rs`, and `crates/fixtures/core-disjoint-product/*`). Verdict: `revise-design` (D1\u2013D5 remain unadopted). D2's local reference linearization and `ORACLE-REQUIRED.md`'s stage arithmetic/work totals (`[16,21,36,42,52]` and `[16,31,36,46,52]`) are consistent, but concrete defects require revision: (1) `declared_domains` omits `Statement::Discharge` and `Scenario.liabilities`, and `Pair::start` executes both starts before `merge`, losing attempted work on `Error::Shape`; (2) `Pair::resume` drops rejected-attempt work/`attempted_before` and overwrites `latest[member]` on false guards; (3) `disjoint_test.rs` discards `Error::Branch` work counters via `Display` and lacks rejection support; (4) all 10 stage goldens, `public-report.json`, and independent preimages are missing and unrun.",
  "verdict": "revise-design"
}

>>>

WHOLE UNMODIFIED COMPLETE a2 DISSENT: revised/reviews/consumer-REVIEW.json (7171B SHA256 44c010b5000cde73c23950110417a9bf28f08fe7491351c03ea7f346aa7cb90d)
<<<
{
  "decisionVotes": {
    "D1": {
      "reason": "disjoint.rs::declared_domains bans shared AssetIds across whole StagedPrograms (including unused actions like TwoDeliveries::declare), blocking same-asset disjoint-account composition that merge() supports by (asset,account), and ignores 0-await scenario duties.",
      "vote": "revise-design"
    },
    "D2": {
      "reason": "disjoint.rs::Pair::start hardcodes [0,1] start order, so right-first in mori-tests.json still runs member 0 segment 0 before member 1 ([0,1,1,0,1,0]). Local Pair is a reference product only and does not realize native Split or the other four required composition operators.",
      "vote": "revise-design"
    },
    "D3": {
      "reason": "disjoint.rs::Pair::resume overwrites latest[member] on Waiting{advanced:false}, replacing its last committed segment with null in report().members, and sets attempted_before:vec![] on resume error. disjoint_test.rs maps Error::Branch via Display, dropping StageRejection.{attempted,committed}.",
      "vote": "revise-design"
    },
    "D4": {
      "reason": "Pair::resume passes cli_cap=None instead of Some(self.caps[member]), and cap_total only sums child caps without a conserved parent budget. Cloning full history into each Pair::report() makes disjoint_test.rs stage arrays grow O(N^2), hitting test.rs's 1 MiB per-case cap well before 256 calls.",
      "vote": "revise-design"
    },
    "D5": {
      "reason": "Feature gating (core2-disjoint-draft) and moriarty-local-disjoint-draft/1 labels mark local untracked status, but crates/mori/tests/disjoint_test_public.rs and mori-tests.json reference 10 stage goldens and public-report.json absent from the candidate, blocking consumer verification.",
      "vote": "revise-design"
    }
  },
  "findings": [
    "1. [crates/mori-core/src/stage/disjoint.rs: Pair::resume, Pair::report] When super::resume returns StageOutcome::Waiting { advanced: false, segment: None, .. }, Pair::resume unconditionally overwrites self.latest[member] = outcome.clone(). In Pair::report(), members therefore serializes segment: null instead of the member's latest committed segment, contradicting D3/D5. Change Pair::resume to update latest[member]'s attempted/continuation only or preserve the last committed segment in members when advanced is false, and pass Some(self.caps[member]) as cli_cap to super::resume.",
    "2. [crates/mori-core/src/stage/disjoint.rs: Error::Branch; crates/mori/src/commands/test/disjoint_test.rs: run_case] Error::Branch formats as #[error(\"member {member}: {rejection}\")], and StageRejection formats only {error}. When disjoint_test.rs maps Pair::start/resume errors via core_io::operation, rejection.attempted, rejection.committed, and attempted_before are discarded, and Pair::resume sets attempted_before: vec![] on rejection despite prior self.attempted_total. Expose work fields in Error::Branch/report output and record prior attempted work.",
    "3. [crates/mori-core/src/stage/disjoint.rs: declared_domains, disjoint, merge] declared_domains rejects any shared AssetId across the entire left and right Programs and scans all actions' Statement::Liability declarations. This blocks same-asset disjoint-account composition (even though merge() checks exact (asset, account) collisions) and couples admission to unused helper actions like TwoDeliveries::declare, while 0-await actions can still admit undeclared scenario duties. Scope domain checks to selected actions and allow disjoint accounts per AssetId.",
    "4. [crates/mori-core/src/stage/disjoint.rs: Pair::start; crates/fixtures/core-disjoint-product/mori-tests.json] Pair::start hardcodes starting member 0 then member 1 (ordering.start: [0, 1]). Consequently, the right-first case in mori-tests.json still starts member 0 before member 1 (invocation history [0, 1, 1, 0, 1, 0]), testing only post-start resume interleaving rather than right-first start linearization. Parameterize start order in the manifest/Pair::start or explicitly restrict the schedule claim in D2 and mori-tests.json.",
    "5. [crates/mori-core/src/stage/disjoint.rs: Pair::report; crates/mori/src/commands/test/disjoint_test.rs: run_case; crates/mori/src/commands/test.rs: aggregate_report] Pair::report() clones all cumulative history outcomes into every stage report, and run_case stores every stage's full report in stages. Across K events this duplicates O(K^2) StageOutcome envelopes inside one case Value, which hits mori_core::json::export's 1 MiB per-case cap in aggregate_report well below MAX_INVOCATIONS (256), and all 11 fixture golden files required by disjoint_test_public.rs are missing."
  ],
  "limitations": [
    "1. Source-only review: no build, cargo, rustc, mori CLI, native, proof, VM, or test execution was performed; all candidate code and crates/mori/tests/disjoint_test_public.rs remain UNRUN.",
    "2. Absent independent oracle evidence: all 10 stage golden files (left-first-0..4.json, right-first-0..4.json), public-report.json, independent Program/StagePlan preimages and digests, DERIVATION.md, and INDEPENDENT-FREEZE.json are missing and cannot be treated as passing.",
    "3. Operator and lane separation: this local unadopted Pair consumer covers only a restricted local reference product; all five composition operators (sequential, disjoint parallel, shared-state interleaving, atomic synchronization, asynchronous messaging), native Split/Join, parent exits/caps, and the separate sequential escrow oracle lane remain open.",
    "4. Unanswered security, constraint, proof, and resource-authorization judgments (including memory/output bounds for 256-step cumulative history serialization, uncharged host dispatch/merge overhead, and hostile rejection coverage) are outside this ordinary consumer review and flagged for the designated owner."
  ],
  "reviewToken": "doctorwatson-composition-disjoint-compact-consumer-20261018-a2-whole-review",
  "summary": "Independent consumer design review of the 16-file core2-disjoint-draft candidate yields revise-design. While crates/mori-core/src/stage/disjoint.rs and crates/mori/src/commands/test/disjoint_test.rs wire an opaque two-member local product into mori test behind non-default feature core2-disjoint-draft, concrete consumer, domain, publication, and bound defects require revision: (1) Pair::resume overwrites latest[member] on non-advancing Waiting outcomes (erasing the last committed segment from members) and passes cli_cap=None; (2) Error::Branch Display and disjoint_test.rs .map_err(core_io::operation) discard StageRejection.{attempted,committed} and attempted_before, and resume rejections report attempted_before=[] despite prior pair work; (3) declared_domains rejects shared AssetIds across whole programs (blocking same-asset disjoint-account composition and colliding on unused actions like TwoDeliveries::declare); (4) Pair::start hardcodes [0,1] start order so right-first cannot start member 1 first; (5) embedding full history in every stage report causes quadratic case growth against test.rs::aggregate_report's 1 MiB per-case export cap, and all 11 fixture goldens are absent.",
  "verdict": "revise-design"
}

>>>

EXACT actual start/resume and outcome construction: main/crates/mori-core/src/stage/admit.rs L100-137 (whole SHA256 12474f92ad0752cfeee1a6054c5b251370dc08d8a17949f7747878ada60450c3)
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

EXACT actual start/resume and outcome construction: main/crates/mori-core/src/stage/admit.rs L222-392 (whole SHA256 12474f92ad0752cfeee1a6054c5b251370dc08d8a17949f7747878ada60450c3)
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

EXACT actual public outcome_report: main/crates/mori-core/src/stage/report.rs L1-39 (whole SHA256 25e2b88e527a3f9b90951a92fc92ba8c17dd6f16040529dcac0356bb36707b4f)
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

EXACT exact asset/statements/snapshots/effects DTOs: main/crates/mori-core/src/ir.rs L187-270 (whole SHA256 0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d)
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Effect {
    State {
        name: String,
        before: Value,
        after: Value,
    },
    Transfer {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Fee {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Liability {
        name: String,
        asset: AssetId,
        debtor: String,
        creditor: String,
        before: UInt,
        after: UInt,
    },
    Emit {
        name: String,
        value: Value,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub index: u16,
    pub statement: Statement,
    pub value: Option<Value>,
    pub snapshot: Snapshot,
    pub effects: Vec<Effect>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub semantics: String,
    pub action: String,
    pub pre: Snapshot,
    pub post: Snapshot,
    pub steps: Vec<Step>,
    pub effects: Vec<Effect>,
}

>>>

EXACT actual stored-cap preflight: main/crates/mori-core/src/stage/cost.rs L193-230 (whole SHA256 eeb5c9fbd1c58c5b46f346a0e62484c1bb366e548c50824295d833648ba6bcc1)
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

EXACT actual existing command diagnostic consumer: main/crates/mori/src/commands/core_io.rs L158-164 (whole SHA256 6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594)
<<<
pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    CliError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}

>>>

EXACT current adopted ordinary Core2 observation contract: main/docs/CORE-STAGED-TESTS.md L175-212 (whole SHA256 4ad9df1dc5111535800862cb6921751fb65d0db54c969a8a29bda9d7178315c3)
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

For a successfully printed ordinary public simulate waiting/complete invocation within the existing bounded exporter, stdout SHALL be one complete ASCII JSON report followed by exactly one final LF, with no ordinary status text on stderr, and exit0. Report objects SHALL order member names lexicographically by their original string keys; arrays SHALL retain their prescribed order. Nonempty objects/arrays SHALL use LF-separated entries, two ASCII spaces of indentation per nesting level, comma at each nonfinal entry, colon then one space for object values, and a closing delimiter at the parent's indentation. Empty containers SHALL be {} or []. No trailing spaces, BOM or extra document SHALL appear. Numbers for u16 fields and scale SHALL be unsigned decimal JSON numbers; UInt fields remain strings, booleans lowercase and null literal null. Strings SHALL escape quote/backslash, use the short JSON escapes for backspace/form-feed/LF/CR/tab, encode remaining U+0000–001F and U+007F with lowercase four-digit Unicode escapes, and encode every non-ASCII scalar as lowercase UTF-16 Unicode escape(s), including a surrogate pair when required. Other ASCII characters, including slash, SHALL remain literal. These printing rules concern the report only, not digest preimages. They are adopted independently inspectable rendering choices; no output bytes are generated here. Existing exporter/input bounds and failure handling remain unchanged.

The staged test harness SHALL compare entire JSON values from the same projection while passing actual continuations, not expectations, to successive events. Its aggregate is a separate consumer; no per-invocation byte golden SHALL transfer to the aggregate or vice versa. The existing library pretty-byte check remains separate from public ASCII-export/stdout evidence. Rejected-envelope key projection above adopts no error-byte/process or hostile-case result. Complete stdout/exit/stderr conformance requires its own independently frozen bytes and authorized actual consumer observation.

### Local logical schedule and reporting authority

The adopted schedule for moriarty-reference-logical-cost/2 SHALL charge one unit before each ordinary statement dispatch and each distinct visited expression node, with a fresh node cache per statement. Both Boolean operands SHALL retain existing eager evaluation. Reaching the next await SHALL charge entry1 without visiting its guard and without emitting a Step. Resuming SHALL charge a separate dispatch1 plus the guard's distinct nodes. An admitted false guard SHALL report that invocation's attempted guard work and preserve the input logical.committed; a true guard SHALL charge that guard once and then the executed suffix, including any next await entry. No trace length SHALL substitute for this work count.

For an action with n awaits, segment indices SHALL be0..n: segment0 starts at action entry, segment j>0 begins with the guard dispatch of site j, and a successful segment stops before the next await guard while including its entry charge. ActionPlan.segment_cost SHALL hold n+1 successful segment costs in that order. committed_before[j] SHALL be the sum of earlier successful segment costs, with committed_before[0]=0; plan_total SHALL be their full sum. A normally successful segment j SHALL report attempted=segment_cost[j] and committed=committed_before[j]+segment_cost[j]. A waiting envelope SHALL project committed from its continuation; Complete SHALL use its typed field. False observations SHALL not accumulate into committed, although they remain separately attempted work. This is a local successful-prefix reporting convention, not authenticated lifetime-budget, financial commitment or history authority. No cap-editability, admission, resource ceiling or cap-refusal precedence policy is changed; those decisions remain with their owner.

The previously frozen escrow arithmetic (attempted5/2/6 and committed5/5/11) remains historically CONDITIONAL in its original partial freeze. The [2026-10-18 design adoption](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json) establishes this schedule and prefix convention for a NEW independent freeze; it does not upgrade that original record or claim an observed meter pass. A distinct independent custodian SHALL derive and freeze complete new work expectations before an authorized comparison. Original conditional records SHALL remain unchanged. Logical interpretation units SHALL remain separate from circuit cost, DUST, service fees, recovery reserves, wall time and allocation.

>>>

Original exact operator requirements below are also byte-identical on current main. Historical labels do not mean stale APIs.
EXACT-RETAINED-NORMATIVE base/openspec/REPORT-RECONCILIATION-2026-09-07.md (whole-file SHA256 a1a1661fc0d554d8e469651fdbd42c41e7c9e2115c8550f369742dcee94bd5da; all ranges below are original line numbers)
LINES 67-67
<<<
All five composition operators receive separate dispositions: sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging. MC06 owns their interface/authority/history contracts with MC07's financial cases. A proof split/join does not establish another operator. Unsupported required behavior remains an open completion obligation; bounded foreign-domain modeling does not create a foreign adapter deliverable.
>>>

EXACT-RETAINED-NORMATIVE base/openspec/sprints/sp10-private-handoff-and-bounded-composition.md (whole-file SHA256 f4a2b54deaf074f2784f96df0b76b1538b74cf180452175602c72674e8742005; all ranges below are original line numbers)
LINES 33-39
<<<
## SP10.1: Realize every required operator

- [ ] Bind inputs, exact file ownership and independent expected results in this task or the existing `openspec/sprints/execution/SP10.md` note. Reuse sufficient records; routine edits do not require another packet or design vote.
- [ ] Implement sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging using the RP01 definitions. Check read/write compatibility and explicit ordering. Partition and conserve a global finite work measure.
- [ ] Verify: A required unsupported operator leaves MC06 open. Shared-state conflicts cannot masquerade as disjoint branches; async Pending does not claim atomic completion.
- [ ] Retain commands, outputs, resource use and exact source/profile digests under `SP10` in the owning package evidence.
- [ ] Obtain current scoped reviews and commit the accepted task without changing unrelated files.
>>>

EXACT-RETAINED-NORMATIVE base/openspec/changes/pcd-ledger-anchored-acceptance/specs/pcd-composition/spec.md (whole-file SHA256 fef2572484566673c80252d171cf9c69b286358d93b60f7d229cc47c3ac020fc; all ranges below are original line numbers)
LINES 3-8
<<<
### Requirement: Ledger-atomic split and join
`Split` and `Join` within one contract SHALL be ledger-atomic. Child head identities SHALL be computed in the circuit. Budgets SHALL be conserved, obligations partitioned and authority attenuated. The lifetime invariant SHALL hold per head: `revision + remaining = allocatedLifetime(head)`, fixed when the head is created. `Join` SHALL require distinct heads and compatible policies. Fan-in and fan-out SHALL be at most 2 until the bounds freeze sets them.

#### Scenario: Split then join
- **WHEN** head A splits into B and C and they later join into D
- **THEN** budgets, residual obligations and authority are conserved and each head's lifetime invariant holds.
>>>
LINES 36-45
<<<
### Requirement: Composition operators under head discipline
Sequential composition SHALL map to `Step`, disjoint parallel composition to `Split`, and atomic synchronization to same-contract `Join` or a claimed cross-contract call. Shared-state interleaving, asynchronous messaging and Pending SHALL receive a reviewed rule under head discipline before any SP10 campaign advertises them.

#### Scenario: Mapped operators
- **WHEN** a composition uses sequential, disjoint parallel or atomic synchronization
- **THEN** it runs through the mapped entry points under the same acceptance lineage.

#### Scenario: Operator without a rule
- **WHEN** an operator has no accepted rule
- **THEN** it is unavailable, is not advertised, and MC06 stays open.
>>>

EXACT-RETAINED-NORMATIVE base/openspec/changes/mc06-private-handoff-and-composition/specs/mc06-private-handoff-and-composition/spec.md (whole-file SHA256 58121600ae1da0c05d2747fc19b01d4c5cd47ced4980987258fb81ec0c44fcad; all ranges below are original line numbers)
LINES 16-21
<<<
### Requirement: Bounded history composition
Split and join SHALL preserve obligations, authority limits, output uniqueness, and the original global lifecycle measure.

#### Scenario: Compatible composition
- **WHEN** two independently proved compatible branches undergo the declared split and join
- **THEN** their new proofs discharge dependencies and preserve obligations, residual authority, uniqueness, and global lifecycle bounds.
>>>
LINES 78-83
<<<
### Requirement: Composition operators and witness ownership
Composition SHALL state whether it is sequential, disjoint, shared-state atomic or asynchronous. Private handoff SHALL enumerate required backend artifacts and preserve liabilities, residual authority and a conserved global work budget under genuine predecessor proof verification.

#### Scenario: Report requirement omitted
- **WHEN** a candidate omits the applicable requirement or substitutes an earlier narrower experiment
- **THEN** acceptance remains pending under [the report reconciliation](../../../../REPORT-RECONCILIATION-2026-09-07.md).
>>>


MISSING INDEPENDENT ORACLES: all10 complete stage JSON goldens plus public-report.json (11 total), complete independently derived Program/StagePlan preimages/digests, derivation/freeze and revised error diagnostic/representation oracles remain absent/unrun. No reviewer should produce goldens from evaluator output. Adopt-design means only the conditional ordinary design, not an oracle or observed pass. All original U0-U7/M1-M17/MC/SP/G exits/failures/caps and other operators stay open. Full original partial-conditional EARS and product contract are preserved whole in main/ and historical-context/. No source hashes or reviews close acceptance.
