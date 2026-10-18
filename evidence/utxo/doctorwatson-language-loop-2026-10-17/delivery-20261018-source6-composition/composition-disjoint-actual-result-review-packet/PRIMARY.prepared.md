# PREPARED actual-result review; DISPATCH HELD for root/final integration
Actual checks ran on source4bc5dfe99164b8f7d8840adb2ab5e63b22b54ba4/treea8c8c96d9288a2784be7c0b71f23bb634afd423e, basef7943b1c1caccede6f1db36bfa60815b2a2f7a4d. All36sourcebytes matched source-reviewed candidate. Library1test and publicCLI1test passed, nonefailed/ignored/filtered; oneattempt each under90s+5s/frozenoffline/no-default-features/core2-disjoint-draft/13variable env withoutHOME. Fullterminalstdout/stderr/START/RESULT/pins/runner+cleanup physical. Root owns those executions; this preparer ran no product. Preparation-only olddocumentpin failure occurred BEFOREtemp/child; preservedoriginalscript and exactone-lineknown972guidelinecorrection, notconsumerretry.
ALLfullcode/nineinputs inline below; 19frozenoracleartifacts in lossless504nodeDAG, everyoriginalobjectANDrawbytes verified, raw74payloads preserved. Financial/work values independently source-derived with nonblind/relatedpriororacle exposure; fullwire/schema conditional authoredcodec/narrowfiniteCore premise, not independentcompilerwireproof. PriorSOURCEaudit acceptedconditionally/unrunthen; it is not this freshresultreview. NestedpublicCLIoutput is captured/asserted inside thepassingintegrationtest, not separately dumped asrawstdout; harnesslogs retained. All5fulloperators/parents/exits/R7/stagedgaps/security/native/hostile/resource/proof/target remainopen. No neworacle or canonicalprovideridentity claim.
PENDING exact finalsource/PR214finiteintegration: prospectiveSource6tree7e7a6fc1980b67c06ba9ceda78a6f11eaf682227 is physicallypresent, NOTclaimedmerged. Source-at-check stayshistoricalactualsource. Finalreview needs explicit finitechangedsource/callgraph judgment and rootsignal; no blanketfuturebaseapproval/retest solelyfeature-disabledunselected changes.

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/approval-true.json
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/mori-tests.json
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/quote43.json
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/receipt-true.json
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/reserve-scenario.json
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/reserve.mori
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/settlement-true.json
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/two-deliveries-scenario.json
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

WHOLE ACTUALSOURCE-AT-CHECK crates/fixtures/core-disjoint-product/two-deliveries.mori
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori/Cargo.toml
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori/src/commands/test/disjoint_test.rs
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori/src/commands/test.rs
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori/tests/disjoint_test_public.rs
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori-core/Cargo.toml
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori-core/src/stage/disjoint.rs
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori-core/src/stage/mod.rs
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

WHOLE ACTUALSOURCE-AT-CHECK crates/mori-core/tests/disjoint_program_plan.rs
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

WHOLE LOSSLESS 19ROOT EXPECTEDDATA VIEW
<<<
{"format":"one-off-lossless-review-JSON-value-DAG/1","notProductCode":true,"encoding":["nodes: o=ordered[key,node]members; a=nodeIDs; v=exactJSONscalar","roots:[custodyfilename,nodeID,codec]; codec0=sortedASCIIindent2plusLF; codec1=compactUTF8noLF"],"nodes":[["v","none"],["v","unadopted-local-disjoint-product"],["v","untracked"],["v","7"],["v","independent-staged-test-oracle"],["v","local"],["v","UNIT"],["v",0],["o",[["issuer",4],["network",5],["reference",6],["scale",7]]],["v","alice"],["v","Transfer"],["v","bob"],["o",[["amount",3],["asset",8],["from",9],["kind",10],["to",11]]],["v","UInt"],["v","1"],["o",[["type",13],["value",14]]],["v","0"],["o",[["type",13],["value",16]]],["v","State"],["v","phase"],["o",[["after",15],["before",17],["kind",18],["name",19]]],["v","35"],["o",[["type",13],["value",21]]],["v","40"],["o",[["type",13],["value",23]]],["v","basis"],["o",[["after",22],["before",24],["kind",18],["name",25]]],["v","47"],["v","continuation-reserve-oracle"],["o",[["issuer",28],["network",5],["reference",6],["scale",7]]],["v","5"],["v","target"],["v","reserve"],["v","Liability"],["v","allocation"],["o",[["after",27],["asset",29],["before",30],["creditor",31],["debtor",32],["kind",33],["name",34]]],["a",[12,20,26,35]],["v","moriarty-local-disjoint-draft/1"],["a",[]],["v",1],["o",[["captures",38],["site",39]]],["v","extra"],["v","2"],["o",[["type",13],["value",42]]],["o",[["name",41],["value",43]]],["v","held"],["v","42"],["o",[["type",13],["value",46]]],["o",[["name",45],["value",47]]],["a",[44,48]],["o",[["captures",49],["site",39]]],["a",[40,50]],["v",true],["v","deliver"],["v","13"],["o",[["account",9],["amount",54],["asset",8]]],["o",[["account",11],["amount",3],["asset",8]]],["v","observer"],["v","11"],["o",[["account",57],["amount",58],["asset",8]]],["v","treasury"],["o",[["account",60],["amount",16],["asset",8]]],["a",[55,56,59,61]],["v","duty"],["o",[["amount",30],["asset",8],["creditor",11],["debtor",9],["name",63]]],["a",[64]],["o",[["phase",15]]],["o",[["balances",62],["liabilities",65],["state",66]]],["v","moriarty-continuation/1"],["v","16"],["v","moriarty-reference-logical-cost/2"],["o",[["cap",69],["committed",30],["profile",70]]],["v","d9f7cba42cd4ce1dc1f70544a3592b8a0aba984c8f5cd779c3716d26ca1ac575"],["v","582b7923087e42b46c318576ad4bb786904b7ac0691a6c4580ed2deb187afe44"],["o",[["action",53],["boundary",67],["captures",38],["format",68],["logical",71],["plan_hash",72],["program_hash",73],["site",39]]],["v","local-reference-stage"],["a",[12,20]],["v","20"],["o",[["account",9],["amount",77],["asset",8]]],["o",[["account",11],["amount",16],["asset",8]]],["a",[78,79,59,61]],["o",[["phase",17]]],["o",[["balances",80],["liabilities",65],["state",81]]],["v","moriarty-core/2"],["a",[12]],["o",[["balances",62],["liabilities",65],["state",81]]],["o",[["amount",7],["asset",6],["from",9],["kind",10],["to",11]]],["o",[["type",13],["value",3]]],["o",[["effects",84],["index",7],["snapshot",85],["statement",86],["value",87]]],["a",[20]],["v","Next"],["o",[["kind",90],["state",19],["value",39]]],["o",[["effects",89],["index",39],["snapshot",67],["statement",91],["value",15]]],["a",[88,92]],["o",[["action",53],["effects",76],["post",67],["pre",82],["semantics",83],["steps",93]]],["o",[["captures",38],["site",7]]],["o",[["action",53],["execution",94],["frame_post",40],["frame_pre",95]]],["v","waiting"],["o",[["advanced",52],["attempted",30],["authentication",0],["committed",30],["consumption",2],["continuation",74],["qualification",75],["residual",65],["segment",96],["status",97]]],["o",[["member",7],["outcome",98]]],["v","allocate"],["o",[["account",57],["amount",46],["asset",29]]],["v","100"],["o",[["account",32],["amount",102],["asset",29]]],["o",[["account",31],["amount",16],["asset",29]]],["o",[["account",60],["amount",16],["asset",29]]],["a",[101,103,104,105]],["o",[["amount",27],["asset",29],["creditor",31],["debtor",32],["name",34]]],["a",[107]],["v","77"],["o",[["type",13],["value",109]]],["o",[["allocated",17],["basis",22],["marker",110]]],["o",[["balances",106],["liabilities",108],["state",111]]],["v","36"],["o",[["cap",113],["committed",58],["profile",70]]],["v","634bf5ef487e9f4b2a2828d89a6dd2afe158e0edc9514437fbae04245dbf7ef7"],["v","60ff43afae17f7d7d4f32e734f4cb841aff28552a7ec8223948a55e3b25b550a"],["o",[["action",100],["boundary",112],["captures",49],["format",68],["logical",114],["plan_hash",115],["program_hash",116],["site",39]]],["a",[26,35]],["o",[["amount",30],["asset",29],["creditor",31],["debtor",32],["name",34]]],["a",[119]],["o",[["allocated",17],["basis",24],["marker",110]]],["o",[["balances",106],["liabilities",120],["state",121]]],["v","Let"],["v",2],["o",[["kind",123],["name",45],["value",124]]],["o",[["effects",38],["index",7],["snapshot",122],["statement",125],["value",47]]],["a",[26]],["o",[["balances",106],["liabilities",120],["state",111]]],["v",5],["o",[["kind",90],["state",25],["value",129]]],["o",[["effects",127],["index",39],["snapshot",128],["statement",130],["value",22]]],["a",[35]],["v",6],["v","CREDIT"],["o",[["amount",133],["asset",134],["creditor",31],["debtor",32],["kind",33],["name",34]]],["o",[["effects",132],["index",124],["snapshot",112],["statement",135],["value",47]]],["a",[126,131,136]],["o",[["action",100],["effects",118],["post",112],["pre",122],["semantics",83],["steps",137]]],["o",[["action",100],["execution",138],["frame_post",50],["frame_pre",95]]],["o",[["advanced",52],["attempted",58],["authentication",0],["committed",58],["consumption",2],["continuation",117],["qualification",75],["residual",108],["segment",139],["status",97]]],["o",[["member",39],["outcome",140]]],["a",[99,141]],["v","52"],["a",[69,113]],["a",[30,58]],["o",[["attempted_total",69],["cap_total",143],["caps",144],["committed",145],["committed_total",69]]],["a",[98,140]],["v","invocation-then-source-order"],["a",[7,39]],["o",[["effects",148],["start",149]]],["a",[101,103,104,105,55,56,59,61]],["a",[107,64]],["o",[["allocated",17],["basis",22],["marker",110],["phase",15]]],["o",[["balances",151],["liabilities",152],["state",153]]],["a",[101,103,104,105,78,79,59,61]],["a",[119,64]],["o",[["allocated",17],["basis",24],["marker",110],["phase",17]]],["o",[["balances",155],["liabilities",156],["state",157]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",36],["format",37],["frames",51],["history",142],["logical",146],["members",147],["ordering",150],["post",154],["pre",158],["qualification",75],["residual",152],["status",97]]],["v","3"],["o",[["amount",160],["asset",8],["from",9],["kind",10],["to",11]]],["a",[12,20,26,35,161]],["v","ok"],["v","Bool"],["o",[["type",164],["value",52]]],["o",[["name",163],["value",165]]],["a",[166]],["o",[["captures",167],["site",124]]],["a",[168,50]],["v","10"],["o",[["account",9],["amount",170],["asset",8]]],["o",[["account",11],["amount",170],["asset",8]]],["a",[171,172,59,61]],["o",[["balances",173],["liabilities",65],["state",66]]],["o",[["cap",69],["committed",170],["profile",70]]],["o",[["action",53],["boundary",174],["captures",167],["format",68],["logical",175],["plan_hash",72],["program_hash",73],["site",124]]],["a",[161]],["o",[["name",163],["ty",164]]],["a",[178]],["v","Await"],["v","approval"],["o",[["binders",179],["guard",124],["kind",180],["label",181]]],["o",[["effects",38],["index",124],["snapshot",67],["statement",182],["value",165]]],["v",3],["o",[["amount",184],["asset",6],["from",9],["kind",10],["to",11]]],["o",[["type",13],["value",160]]],["o",[["effects",177],["index",184],["snapshot",174],["statement",185],["value",186]]],["a",[183,187]],["o",[["action",53],["effects",177],["post",174],["pre",67],["semantics",83],["steps",188]]],["o",[["action",53],["execution",189],["frame_post",168],["frame_pre",40]]],["o",[["advanced",52],["attempted",30],["authentication",0],["committed",170],["consumption",2],["continuation",176],["qualification",75],["residual",65],["segment",190],["status",97]]],["o",[["member",7],["outcome",191]]],["a",[99,141,192]],["v","21"],["a",[170,58]],["o",[["attempted_total",194],["cap_total",143],["caps",144],["committed",195],["committed_total",194]]],["a",[191,140]],["a",[101,103,104,105,171,172,59,61]],["o",[["balances",198],["liabilities",152],["state",153]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",162],["format",37],["frames",169],["history",193],["logical",196],["members",197],["ordering",150],["post",199],["pre",158],["qualification",75],["residual",152],["status",97]]],["v","allocated"],["o",[["after",47],["before",17],["kind",18],["name",201]]],["o",[["amount",46],["asset",29],["from",32],["kind",10],["to",31]]],["o",[["after",30],["asset",29],["before",27],["creditor",31],["debtor",32],["kind",33],["name",34]]],["v","78"],["o",[["type",13],["value",205]]],["o",[["after",206],["before",22],["kind",18],["name",25]]],["a",[12,20,26,35,161,202,203,204,207]],["v","quote"],["v","43"],["o",[["type",13],["value",210]]],["o",[["name",209],["value",211]]],["a",[44,48,212]],["o",[["captures",213],["site",124]]],["a",[168,214]],["v","15"],["v","26"],["v","58"],["o",[["account",32],["amount",218],["asset",29]]],["o",[["account",31],["amount",46],["asset",29]]],["a",[101,219,220,105]],["o",[["allocated",47],["basis",206],["marker",110]]],["o",[["balances",221],["liabilities",120],["state",222]]],["o",[["cap",113],["committed",217],["profile",70]]],["o",[["action",100],["boundary",223],["captures",213],["format",68],["logical",224],["plan_hash",115],["program_hash",116],["site",124]]],["a",[202,203,204,207]],["o",[["name",209],["ty",13]]],["a",[227]],["v",9],["o",[["binders",228],["guard",229],["kind",180],["label",181]]],["o",[["effects",38],["index",184],["snapshot",112],["statement",230],["value",165]]],["a",[202]],["v",4],["o",[["allocated",47],["basis",22],["marker",110]]],["o",[["balances",106],["liabilities",108],["state",234]]],["v",10],["o",[["kind",90],["state",201],["value",236]]],["o",[["effects",232],["index",233],["snapshot",235],["statement",237],["value",47]]],["a",[203]],["o",[["balances",221],["liabilities",108],["state",234]]],["v",11],["o",[["amount",241],["asset",134],["from",32],["kind",10],["to",31]]],["o",[["effects",239],["index",129],["snapshot",240],["statement",242],["value",47]]],["a",[204]],["o",[["balances",221],["liabilities",120],["state",234]]],["v",12],["v","Discharge"],["o",[["amount",246],["kind",247],["name",34]]],["o",[["effects",244],["index",133],["snapshot",245],["statement",248],["value",47]]],["a",[207]],["v",7],["v",15],["o",[["kind",90],["state",25],["value",252]]],["o",[["effects",250],["index",251],["snapshot",223],["statement",253],["value",206]]],["a",[231,238,243,249,254]],["o",[["action",100],["effects",226],["post",223],["pre",112],["semantics",83],["steps",255]]],["o",[["action",100],["execution",256],["frame_post",214],["frame_pre",50]]],["o",[["advanced",52],["attempted",216],["authentication",0],["committed",217],["consumption",2],["continuation",225],["qualification",75],["residual",120],["segment",257],["status",97]]],["o",[["member",39],["outcome",258]]],["a",[99,141,192,259]],["a",[170,217]],["o",[["attempted_total",113],["cap_total",143],["caps",144],["committed",261],["committed_total",113]]],["a",[191,258]],["a",[101,219,220,105,171,172,59,61]],["o",[["allocated",47],["basis",206],["marker",110],["phase",15]]],["o",[["balances",264],["liabilities",156],["state",265]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",208],["format",37],["frames",215],["history",260],["logical",262],["members",263],["ordering",150],["post",266],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["after",43],["before",15],["kind",18],["name",19]]],["v","Emit"],["v","done"],["o",[["kind",269],["name",270],["value",43]]],["a",[12,20,26,35,161,202,203,204,207,268,271]],["a",[95,214]],["v","6"],["a",[268,271]],["o",[["phase",43]]],["o",[["balances",173],["liabilities",65],["state",276]]],["v","seen"],["o",[["name",278],["ty",164]]],["a",[279]],["v","receipt"],["o",[["binders",280],["guard",233],["kind",180],["label",281]]],["o",[["effects",38],["index",233],["snapshot",174],["statement",282],["value",165]]],["a",[268]],["o",[["kind",90],["state",19],["value",129]]],["o",[["effects",284],["index",129],["snapshot",277],["statement",285],["value",43]]],["a",[271]],["o",[["kind",269],["name",270],["value",133]]],["o",[["effects",287],["index",133],["snapshot",277],["statement",288],["value",43]]],["a",[283,286,289]],["o",[["action",53],["effects",275],["post",277],["pre",174],["semantics",83],["steps",290]]],["o",[["action",53],["execution",291],["frame_post",95],["frame_pre",168]]],["v","complete"],["o",[["attempted",274],["authentication",0],["committed",69],["consumption",2],["qualification",75],["residual",65],["segment",292],["status",293]]],["o",[["member",7],["outcome",294]]],["a",[99,141,192,259,295]],["a",[69,217]],["o",[["attempted_total",46],["cap_total",143],["caps",144],["committed",297],["committed_total",46]]],["a",[294,258]],["o",[["allocated",47],["basis",206],["marker",110],["phase",43]]],["o",[["balances",264],["liabilities",156],["state",300]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",272],["format",37],["frames",273],["history",296],["logical",298],["members",299],["ordering",150],["post",301],["pre",158],["qualification",75],["residual",156],["status",97]]],["v","Fee"],["o",[["amount",42],["asset",29],["from",32],["kind",303],["to",60]]],["v","settled_quote"],["o",[["kind",269],["name",305],["value",211]]],["a",[12,20,26,35,161,202,203,204,207,268,271,304,306]],["a",[95,95]],["a",[304,306]],["v","56"],["o",[["account",32],["amount",310],["asset",29]]],["o",[["account",60],["amount",42],["asset",29]]],["a",[101,311,220,312]],["o",[["balances",313],["liabilities",120],["state",222]]],["v",8],["v","ready"],["o",[["name",316],["ty",164]]],["a",[317]],["v",16],["v","settlement"],["o",[["binders",318],["guard",319],["kind",180],["label",320]]],["o",[["effects",38],["index",315],["snapshot",223],["statement",321],["value",165]]],["v","Requires"],["v",19],["o",[["kind",323],["predicate",324]]],["o",[["effects",38],["index",229],["snapshot",223],["statement",325],["value",165]]],["a",[304]],["v",20],["o",[["amount",328],["asset",134],["from",32],["kind",303],["to",60]]],["o",[["effects",327],["index",236],["snapshot",314],["statement",329],["value",43]]],["a",[306]],["v",21],["o",[["kind",269],["name",305],["value",332]]],["o",[["effects",331],["index",241],["snapshot",314],["statement",333],["value",211]]],["a",[322,326,330,334]],["o",[["action",100],["effects",309],["post",314],["pre",223],["semantics",83],["steps",335]]],["o",[["action",100],["execution",336],["frame_post",95],["frame_pre",214]]],["o",[["attempted",170],["authentication",0],["committed",113],["consumption",2],["qualification",75],["residual",120],["segment",337],["status",293]]],["o",[["member",39],["outcome",338]]],["a",[99,141,192,259,295,339]],["o",[["attempted_total",143],["cap_total",143],["caps",144],["committed",144],["committed_total",143]]],["a",[294,338]],["a",[101,311,220,312,171,172,59,61]],["o",[["balances",343],["liabilities",156],["state",300]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",307],["format",37],["frames",308],["history",340],["logical",341],["members",342],["ordering",150],["post",344],["pre",158],["qualification",75],["residual",156],["status",293]]],["a",[12,20,26,35,202,203,204,207]],["a",[40,214]],["a",[99,141,259]],["v","31"],["a",[30,217]],["o",[["attempted_total",349],["cap_total",143],["caps",144],["committed",350],["committed_total",349]]],["a",[98,258]],["a",[101,219,220,105,55,56,59,61]],["o",[["balances",353],["liabilities",156],["state",265]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",346],["format",37],["frames",347],["history",348],["logical",351],["members",352],["ordering",150],["post",354],["pre",158],["qualification",75],["residual",156],["status",97]]],["a",[12,20,26,35,202,203,204,207,161]],["a",[99,141,259,192]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",356],["format",37],["frames",215],["history",357],["logical",262],["members",263],["ordering",150],["post",266],["pre",158],["qualification",75],["residual",156],["status",97]]],["a",[12,20,26,35,202,203,204,207,161,304,306]],["a",[168,95]],["a",[99,141,259,192,339]],["v","46"],["a",[170,113]],["o",[["attempted_total",362],["cap_total",143],["caps",144],["committed",363],["committed_total",362]]],["a",[191,338]],["o",[["balances",343],["liabilities",156],["state",265]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",359],["format",37],["frames",360],["history",361],["logical",364],["members",365],["ordering",150],["post",366],["pre",158],["qualification",75],["residual",156],["status",97]]],["a",[12,20,26,35,202,203,204,207,161,304,306,268,271]],["a",[99,141,259,192,339,295]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",368],["format",37],["frames",308],["history",369],["logical",341],["members",342],["ordering",150],["post",344],["pre",158],["qualification",75],["residual",156],["status",293]]],["v","moriarty-disjoint-case-observations-draft/1"],["v","left-first"],["v","effects"],["o",[["prefix_len",233],["table",373]]],["v","moriarty-local-disjoint-referenced-draft/1"],["v","history"],["o",[["prefix_len",124],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",374],["expanded_format",37],["format",375],["frames",51],["history",377],["logical",146],["members",147],["ordering",150],["post",154],["pre",158],["qualification",75],["residual",152],["status",97]]],["o",[["actual",378],["index",7],["passed",52]]],["o",[["prefix_len",129],["table",373]]],["o",[["prefix_len",184],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",380],["expanded_format",37],["format",375],["frames",169],["history",381],["logical",196],["members",197],["ordering",150],["post",199],["pre",158],["qualification",75],["residual",152],["status",97]]],["o",[["actual",382],["index",39],["passed",52]]],["o",[["prefix_len",229],["table",373]]],["o",[["prefix_len",233],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",384],["expanded_format",37],["format",375],["frames",215],["history",385],["logical",262],["members",263],["ordering",150],["post",266],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",386],["index",124],["passed",52]]],["o",[["prefix_len",241],["table",373]]],["o",[["prefix_len",129],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",388],["expanded_format",37],["format",375],["frames",273],["history",389],["logical",298],["members",299],["ordering",150],["post",301],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",390],["index",184],["passed",52]]],["v",13],["o",[["prefix_len",392],["table",373]]],["o",[["prefix_len",133],["table",376]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",393],["expanded_format",37],["format",375],["frames",308],["history",394],["logical",341],["members",342],["ordering",150],["post",344],["pre",158],["qualification",75],["residual",156],["status",293]]],["o",[["actual",395],["index",233],["passed",52]]],["a",[379,383,387,391,396]],["o",[["effects",307],["history",340]]],["o",[["format",371],["name",372],["passed",52],["stages",397],["tables",398]]],["v","right-first"],["o",[["prefix_len",315],["table",373]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",401],["expanded_format",37],["format",375],["frames",347],["history",381],["logical",351],["members",352],["ordering",150],["post",354],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",402],["index",39],["passed",52]]],["o",[["authentication",0],["composition_qualification",1],["consumption",2],["effects",388],["expanded_format",37],["format",375],["frames",360],["history",389],["logical",364],["members",365],["ordering",150],["post",366],["pre",158],["qualification",75],["residual",156],["status",97]]],["o",[["actual",404],["index",184],["passed",52]]],["a",[379,403,387,405,396]],["o",[["effects",368],["history",369]]],["o",[["format",371],["name",400],["passed",52],["stages",406],["tables",407]]],["a",[399,408]],["v","passed"],["o",[["cases",409],["status",410]]],["a",[86,91,182,185,282,285,288]],["o",[["inputs",38],["name",53],["statements",412]]],["v","declare"],["o",[["amount",251],["asset",6],["creditor",11],["debtor",9],["kind",33],["name",63]]],["a",[415]],["o",[["inputs",38],["name",414],["statements",416]]],["a",[413,417]],["o",[["identity",8],["name",6]]],["a",[419]],["v","Literal"],["o",[["kind",421],["value",87]]],["o",[["kind",421],["value",15]]],["v","Name"],["o",[["kind",424],["name",163]]],["o",[["kind",421],["value",186]]],["o",[["kind",424],["name",278]]],["o",[["kind",421],["value",43]]],["o",[["kind",424],["name",19]]],["o",[["kind",421],["value",17]]],["a",[422,423,425,426,427,428,429,430]],["v","moriarty/2"],["v","TwoDeliveries"],["o",[["initial",17],["name",19],["ty",13]]],["a",[434]],["o",[["actions",418],["assets",420],["expressions",431],["language",432],["name",433],["semantics",83],["states",435]]],["a",[16,30,170]],["a",[30,30,274]],["o",[["binders",179],["captures",38],["label",181],["ordinal",124],["site",39]]],["v","Event"],["o",[["name",163],["ordinal",124],["origin",440],["ty",164]]],["a",[441]],["o",[["binders",280],["captures",442],["label",281],["ordinal",233],["site",124]]],["a",[439,443]],["o",[["action",53],["committed_before",437],["plan_total",69],["segment_cost",438],["sites",444],["statement_count",251]]],["a",[16]],["a",[42]],["o",[["action",414],["committed_before",446],["plan_total",42],["segment_cost",447],["sites",38],["statement_count",39]]],["a",[445,448]],["v","moriarty-stage-plan/1"],["o",[["actions",449],["format",450],["program_hash",73]]],["o",[["name",41],["ty",13]]],["a",[452]],["a",[125,130,135,230,237,242,248,253,321,325,329,333]],["o",[["inputs",453],["name",100],["statements",454]]],["a",[455]],["o",[["identity",29],["name",134]]],["a",[457]],["o",[["kind",424],["name",25]]],["o",[["kind",424],["name",41]]],["v","Binary"],["v","Add"],["o",[["kind",461],["lhs",7],["op",462],["rhs",39]]],["o",[["type",13],["value",30]]],["o",[["kind",421],["value",464]]],["v","Sub"],["o",[["kind",461],["lhs",184],["op",466],["rhs",233]]],["o",[["kind",424],["name",45]]],["o",[["kind",424],["name",209]]],["v","Ge"],["o",[["kind",461],["lhs",251],["op",470],["rhs",315]]],["v",14],["o",[["kind",461],["lhs",392],["op",462],["rhs",472]]],["o",[["kind",424],["name",316]]],["v","50"],["o",[["type",13],["value",475]]],["o",[["kind",421],["value",476]]],["v",17],["v","Le"],["v",18],["o",[["kind",461],["lhs",478],["op",479],["rhs",480]]],["a",[459,460,463,459,465,467,468,469,468,471,468,468,468,459,469,473,474,469,477,481,460,469]],["v","ReserveContinuationOracle"],["o",[["initial",24],["name",25],["ty",13]]],["o",[["initial",17],["name",201],["ty",13]]],["v","marker"],["o",[["initial",110],["name",486],["ty",13]]],["a",[484,485,487]],["o",[["actions",456],["assets",458],["expressions",482],["language",432],["name",483],["semantics",83],["states",488]]],["a",[16,58,217]],["a",[58,216,170]],["v","Input"],["o",[["name",41],["ordinal",7],["origin",492],["ty",13]]],["o",[["name",45],["ordinal",7],["origin",123],["ty",13]]],["a",[493,494]],["o",[["binders",228],["captures",495],["label",181],["ordinal",184],["site",39]]],["o",[["name",209],["ordinal",184],["origin",440],["ty",13]]],["a",[493,494,497]],["o",[["binders",318],["captures",498],["label",320],["ordinal",315],["site",124]]],["a",[496,499]],["o",[["action",100],["committed_before",490],["plan_total",113],["segment_cost",491],["sites",500],["statement_count",246]]],["a",[501]],["o",[["actions",502],["format",450],["program_hash",116]]]],"roots":[["left-first-0.json",159,0],["left-first-1.json",200,0],["left-first-2.json",267,0],["left-first-3.json",302,0],["left-first-4.json",345,0],["right-first-0.json",159,0],["right-first-1.json",355,0],["right-first-2.json",358,0],["right-first-3.json",367,0],["right-first-4.json",370,0],["public-report.json",411,0],["two-deliveries-program.json",436,0],["two-deliveries-program.preimage.bin",436,1],["two-deliveries-plan.json",451,0],["two-deliveries-plan.preimage.bin",451,1],["reserve-program.json",489,0],["reserve-program.preimage.bin",489,1],["reserve-plan.json",503,0],["reserve-plan.preimage.bin",503,1]]}

>>>

COMPLETE DTO/keyset/attachment codec condensation; fullauthoredoriginal physical
<<<
# Compact complete DTO/representation contract for result review

SOURCE-DERIVED FIELD CODEC, NOT independent wire proof or new semantics. Complete original16,340B authored codec remains at independent-custody/basis/input-packet/PROPOSED-FIELD-CODEC.md; adopted exact clauses and owner M03 are beside it. This is a lossless prose condensation of keysets/presence/attachment; all19rawexpectedobjects/bytes remain unchanged and fully represented by the review DAG. It supplies no new expected values.

Scalars: UInt=canonical decimal JSON string in0..2^128-1, no leadingzeros except0. Value exactly{type,value}: UInt/string or Bool/boolean. Numeric JSON indices include ExprId, original Step.index, member, Pair stage index, site/ordinal/statement_count, prefix_len; scale u8. Null, absent, present-zero duty, empty[] and{} differ. Object keys lexically rendered; arrays retain specified order.

AssetId={network,issuer,reference,scale}; Balance={account,asset,amount}; Liability={name,asset,debtor,creditor,amount}. Snapshot={state,balances,liabilities}, ALL affected/untouched rows including present-zero duties. State maps all declared names to typed Value, excluding inputs/captures. Balance order exact(network,issuer,reference,scale),then account; duty order name. Scenario={state,inputs,balances,liabilities}; exact nineinputbytes normalized under supplied clauses, never expected outputs.

NamedValue={name,value}; PendingFrame={site,captures}. Idle=site0/captures[]; pending site may have[] too. StageLogical={profile,cap,committed}, profile=moriarty-reference-logical-cost/2, mandatory cap UInt|null, committed UInt. Continuation={format,program_hash,plan_hash,action,site,captures,logical,boundary}, format=moriarty-continuation/1, boundary complete LOCALmember snapshot. Captures in exact adopted input/let/prior-event order with captured values retained; pending event binders excluded.

StageSegment={action,frame_pre,frame_post,execution}; Execution={semantics,action,pre,post,steps,effects}, semantics=moriarty-core/2. Step={index,statement,value,snapshot,effects}: original action ordinal, exact admitted Statement/ExprIds into child's whole Program, present adopted typed result, complete poststatement LOCALsnapshot and only newdelta. Awaitentry produces noStep; trueguard first emits Await/Booltrue at preboundary, noeffects; falseguard nosegment/publicStep. Local execution.pre/post and Step.snapshot/residual include ONLYmember domain, never padded with othermember rows. Segmentframes retain actual pending/idle semantics; captures/inputvalues do not become state.

Effect={kind,...}: State{name,before,after} with typedValues; Emit{name,value} typedValue; Transfer/Fee{asset,from,to,amount}; Liability{name,asset,debtor,creditor,before,after}. Assets completeIDs, amounts/before/after UInt. Discharge uses Liability effect and preserves zero row. Effects are source-order new deltas with repeats/zero/self effects retained; Execution.effects concatenates them. No Mint/Burn/dedicatedaccrual or new semantic operation.

Every original member outcome root has qualification:local-reference-stage,authentication:none,consumption:untracked. Waiting additionally exactly status:waiting,advanced:boolean,segment:StageSegment|null,continuation,attempted:UInt,committed:UInt,residual:Liability[]. Complete additionally exactly status:complete,segment,attempted,committed,residual; advanced/continuation ABSENT. StageRejection additionally exactly status:rejected,error,attempted,committed; advanced/segment/continuation/residual ABSENT. Error={code,message,offset,step}, lasttwo numeric|null. FalseWaiting advancedfalse/segmentnull, continuation and committed unchanged, actualguard attempted retained. Storedcap remains; Pair resumes None override uses existingstoredcap. Residual strictlypositive LOCALduties in canonicalboundaryorder; zero rows staycompleteboundary.

Unprojected Pair root EXACTLY format,status,qualification,composition_qualification,authentication,consumption,ordering,pre,post,frames,members,history,effects,logical,residual. Literals format:moriarty-local-disjoint-draft/1; composition_qualification:unadopted-local-disjoint-product; threeoriginallocalqualifications. Statuswaiting iff any latestmemberWaiting, otherwisecomplete, even positiveresidual. ordering={start:[0,1],effects:invocation-then-source-order}. Starts fixed0then1, case left/right-first means firstRESUME. pre=completeflatunion bothnormalizedstart.pre; post=completeflatunion latestmemberboundary. Globalstate/balances/duties preserved/sorted, no netting/renaming/hidden third domain. Strongwholeprogram state/exactAssetId/source-duty disjointsubset; actualmerges also check everyrow collision beforepublication, includingincoming0-awaitduties. No new preadmission rule.

frames exactly2currentPendingFrames/memberorder. members exactly2LATEST ORIGINAL INVOCATION envelopes; false/null stayslatest, not substitutedlastcommitted. history complete{member,outcome}array: bothstarts then eachpublishedscheduledoriginaloutcome, falseobservations included. Priorcommittedsegments stayintactinhistory. effects completeorderedconcat segmentexecution.effects invocation-then-sourceorder; false addsnoeffect but remainsinhistory. logical={caps,cap_total,committed,committed_total,attempted_total}, twoUIntarrays caps/committed/memberorder, descriptivecheckedsumcap_total, sums of latestcommitted and ALLsuccessfullypublished attemptedoutcome values (falseincluded). Failedresumes leavePair/history/countersunchanged; rejectedattemptedwork separatelydiagnostic. No parentlifetime/dynamicborrow/refund/nativepolicy. Pairresidual strictlypositive GLOBALpost duties, fullrows/canonicalorder; completiondoesnoteraseanyduty.

Case root EXACTLY format,name,passed,stages,tables, format:moriarty-disjoint-case-observations-draft/1. Manifestcaseorder retained. stages=[{index,passed,actual},...], index0afterBOTHstarts thenmanifesteventorder; passed comparesWHOLEunprojectedPair againstliteral BEFOREprojection, casepassed isconjunction. tables={history,effects}, finalfullarraysoneperCASE. Referencedactual retainsALLPairfields except format:moriarty-local-disjoint-referenced-draft/1, adds expanded_format:originalformat, replaces history/effects each{table:history|effects,prefix_len:N}. N is complete observedarraylength; slice[0,N), zeroempty/fulltablelengthfull. Prefix must extendentirepreviousprefix byexactequality, never rewrite/decrease/reorder. Expansion restoresbothcompleteprefixarrays, restoresformat, REMOVESexpanded_format; allotherfieldsunchanged, mustequalfullindependentgolden. No fields/snapshots/duties/history/effects truncated. Outeraggregate EXACTLY{cases,status}, NOformatfield. Casesandstagesinmanifestorder, statuspassediffallcomparisonsmatch elsefailed. Expectedflagsareindependentexpectations, notobservations.

Publicbytes: recursivelylexicalobjectkeys, arrayorder, ASCIIprettyJSON, two-spaceindent/LFentries, colonspace/nonfinalcomma, empty{}or[], notrailingspaces/BOM/extradocument. Stringquotes/backslash escaped; shortcontrolescapes; otherU+0000-001F/U+007F lowerfour-digitUnicode; nonASCIIlowerUTF16escapes/surrogatepair; otherASCII/slashliteral. Aggregateembedsfullyexportedcases withaddedindentandONEfinalLF. Existingpercase1MiB/depth32 retained, noaggregateclipping/newadequacyclaim. Librarypretty/perinvocation/aggregatebyteevidence stayseparate.

Pairerrorroot EXACTLY format,status,qualification,authentication,consumption,detail; format:moriarty-local-disjoint-error-draft/1/status:rejected/threelocalqualifications. NOproductframe/snapshot/effects/continuation. detailvariants:
- kind:before-invocation,message ONLY; no inventedzero/nullpriorworkcounter.
- kind:stage-rejection,member,rejection,earlier_start_attempted,earlier_start_local_committed,pair_attempted_before,pair_committed_before. rejection=completeoriginalStageRejectionenvelope. Earlier-startvectors only earliercallsinSAMEinitialstart. PriorPairfields nullbeforepublication; rejectedresume originalpriorattemptedtotal/twocommittedvalues.
- kind:composition-after-invocation,phase,message,member,invocation_attempted,candidate_local_committed,pair_attempted_before,pair_committed_before. phase:start|resume, membernullstart/selectedresume; invocation_attempted TWO UInt|null, nullforNOTCALLED, notfabricatedzero; candidate_local_committed TWOactualcandidatevalues; priorPairfields null/actualpublished asabove. Latefailureretainsreturnedworkwithoutpublishing/mutatingPair. ExistingcommandreceivesfullValue diagnostic, commandcode mori::command and renderedmessage; notpassedstagereport/aggregate. No selectedfalse/errorcaseor independentlyexpecteddiagnosticbytes inthispositivepacket.

Program exact{language,semantics,name,states,assets,expressions,actions}, completewhole-source/unusedactions, adoptedM01freshglobalpostorder. StateDecl{name,ty,initial}; AssetDecl{name,identity}; InputDecl/StageBinder{name,ty}; Action{name,inputs,statements}; Value asabove. Expr kind+fields: Literal{value},Name/LiabilityPresent/LiabilityAmount{name},Not{value:ExprId},Binary{op,lhs,rhs}. Statement kind+fields: Let{name,value},Requires/Ensures{predicate},Next{state,value},Emit{name,value},Transfer/Fee{asset,from,to,amount},Liability{name,asset,debtor,creditor,amount},Discharge{name,amount},Await{label,binders,guard}; expressionfields numericExprIds, financialstatementasset alias, outputeffectasset completeID.

StagePlan={format,program_hash,actions}, format:moriarty-stage-plan/1,FILLEDchildprogram_hash. ActionPlan={action,statement_count,sites,segment_cost,committed_before,plan_total}, allactionsinProgramorder. AwaitSite={site,ordinal,label,binders,captures}; CaptureSlot={name,ty,origin,ordinal}, Origin Input/Let/Event. SitesONLYactualawaits/order, no syntheticidle0row; noawait sites[] butonecostedsegment. Adoptedcosts have n+1segmentcost/prefix entries, staticnode/dispatch/entryschedule and true/falseworkdistinctions, nottracelength.

OwnerM03 preimages usecompactUTF8 JSON recursivelyobjectkeysorted/arraysunchanged, no domain/lengthprefix/whitespace/LF; exactUInt/type/enum/keysets asadopted. SHA256→64lowercaseASCIIhex. Planfullyfilledcosts+childprogramhashbeforehashing. Not publicASCIIpretty/sourcebytes/Scenario/selectedaction/reportstate. CanonicalProgram/Planpreimages differfromprettyfixturefiles. NO Pair hash/preimage exists. Source/custodyhashesidentifybytesonly. Qualifyingtheseobservationalidentities addsnosecurity/proof/nativeclaim.

>>>

EXACT COMPLETE INDEPENDENT MEMBER FINANCIAL/WORK/FRAME/CAPTURE AND BOTH SCHEDULE DERIVATION
<<<
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

WHOLE ACTUAL RESULT.json
<<<
{
  "recordedAt": "2026-10-18T10:48:54.082442+00:00",
  "sourceCommit": "4bc5dfe99164b8f7d8840adb2ab5e63b22b54ba4",
  "sourceTree": "a8c8c96d9288a2784be7c0b71f23bb634afd423e",
  "steps": [
    {
      "step": "library-program-plan",
      "finishedAt": "2026-10-18T10:48:08.815012+00:00",
      "exitCode": 0,
      "wallSeconds": 26.510785313999804,
      "sourceCommitAfter": "4bc5dfe99164b8f7d8840adb2ab5e63b22b54ba4",
      "sourceTreeAfter": "a8c8c96d9288a2784be7c0b71f23bb634afd423e",
      "sourceStatusAfter": "",
      "stdoutSha256": "b1a106c34b2788852c4d643b05247f488c575704e2c6ef3c1cceec8f293c6c84",
      "stderrSha256": "f75117938294269b1fead0d1da36315bb6750b36f55cfb37bfff414aaa1515c4",
      "attemptConsumed": true
    },
    {
      "step": "public-cli-two-schedules",
      "finishedAt": "2026-10-18T10:48:53.968050+00:00",
      "exitCode": 0,
      "wallSeconds": 45.03889508299835,
      "sourceCommitAfter": "4bc5dfe99164b8f7d8840adb2ab5e63b22b54ba4",
      "sourceTreeAfter": "a8c8c96d9288a2784be7c0b71f23bb634afd423e",
      "sourceStatusAfter": "",
      "stdoutSha256": "7766834ee498ca2fa2a6aea96f97928ca0661a300071e9e30a94bbe0ee5202d5",
      "stderrSha256": "f8f55ff1dc3121618d0fabac24106096e4ce3bc2c4ed520749e69b2460a400a8",
      "attemptConsumed": true
    }
  ],
  "temporaryRootExistsAfter": false,
  "allStepsCompleted": true,
  "allExitZero": true,
  "scope": "Actual finite ordinary library+public CLI comparisons only; full operators/authentication/native/proof/ledger remain unobserved."
}

>>>

WHOLE ACTUAL CLEANUP.json
<<<
{
  "recordedAt": "2026-10-18T10:48:54.082270+00:00",
  "ownedTemporaryRoot": "/tmp/doctorwatson-disjoint-public-yxrznesu",
  "existsAfter": false,
  "scope": "Only owned temporary root removed, public cache symlinks not traversed."
}

>>>

WHOLE ACTUAL PREPARATION-FAILURE-a1.json
<<<
{
  "recordedAt": "2026-10-18T10:47:42.112241+00:00",
  "exitCode": 1,
  "phase": "Source-pin preparation before temp root/child/product execution",
  "failedPath": "RUST_GUIDELINES.md",
  "cause": "Inherited PR212 test cycle pins retain old pre-PR210 guideline; actual known merged guideline differs and is already pinned in source review BEFORE-PINS.",
  "productChildStarted": false,
  "temporaryRootCreated": false,
  "productAllocationConsumed": false,
  "originalScriptSha256": "d12280fde6a1a61cc5ea2fe7961ab79397c37b134ac76df4aba5a7a3ba5d9914",
  "correction": "Replace only document pin with independently verified exact candidate review BEFORE-PINS current hash, leaving all code and oracle pins checked."
}

>>>

WHOLE TERMINAL HARNESS STDOUT library-program-plan
<<<

running 1 test
test complete_child_programs_and_costed_plans_match_independent_literals ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


>>>

COMPLETE STDERR physically at actual-check-terminal/library-program-plan/STDERR.log SHA256 f75117938294269b1fead0d1da36315bb6750b36f55cfb37bfff414aaa1515c4; original inspected, compilationprogress/finish/target lines, no omitted runtimeerror. Final2lines:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 26.48s
     Running tests/disjoint_program_plan.rs (/tmp/doctorwatson-disjoint-public-yxrznesu/target/debug/deps/disjoint_program_plan-e018c586d45741e9)

WHOLE TERMINAL HARNESS STDOUT public-cli-two-schedules
<<<

running 1 test
test public_consumer_observes_both_disjoint_domains_for_two_explicit_schedules ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s


>>>

COMPLETE STDERR physically at actual-check-terminal/public-cli-two-schedules/STDERR.log SHA256 f8f55ff1dc3121618d0fabac24106096e4ce3bc2c4ed520749e69b2460a400a8; original inspected, compilationprogress/finish/target lines, no omitted runtimeerror. Final2lines:
    Finished `test` profile [unoptimized + debuginfo] target(s) in 44.79s
     Running tests/disjoint_test_public.rs (/tmp/doctorwatson-disjoint-public-yxrznesu/target/debug/deps/disjoint_test_public-5fc2732fdd9981c0)

WHOLE PRIOR SOURCEAUDIT SCOPE/HASH/QUALIFICATION RECORD; full bothwholeobjects physical
<<<
{
  "candidatePath": "/home/charl/moriarty-c2-opus55-restored-20261017/composition-disjoint-complete-source-proposal",
  "candidateFiles": 36,
  "baseMainCommit": "588d811dbbd9e88d22d3ebe90b0544672870caa0",
  "candidatePatchSha256": "490b374aee6369386f4bce21cbf8328a34fed7df74c3400dbfb90e31321ca3f1",
  "candidatePatchBytes": 2126301,
  "sourceManifestSha256": "3d7779bdc4eaf975879cb39f739739e5143ef946d7f97623d0a9d7402019f2bd",
  "sourceInputsVerified": 141,
  "sourceInputsChangedAfter": [],
  "helperUnchanged": true,
  "originalAdopted16SourceSemanticChoices": "D1-D5/RV1-RV3 retained; Rust formatting only and additive independent literals/library regression",
  "custodyFreezeSha256": "12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848",
  "all19PlacedOracleBytesUnchanged": true,
  "publicAggregateBytes": 839110,
  "review": {
    "id": "doctorwatson-composition-complete-source-ordinary-20261018-a1",
    "requestedModel": "gemini-4-argon-high",
    "requestedEffort": "high",
    "canonicalActualModel": "UNESTABLISHED; no top-level provider identity or assistant-envelope model fields",
    "providerExitCode": 0,
    "startedAt": "2026-10-18T10:41:54.865113+00:00",
    "endedAt": "2026-10-18T10:44:13.692683+00:00",
    "processSha256": "779c9bab5e68b2d2987bd1e9fcff3cc42091d925bedf8a03b2f4e758c6f072df",
    "wholeReviewArraySha256": "d407fbfcc071b0dd807987ce66eea83036ced5394c7be0cfdd1e6c3fcec19c6c",
    "retainedWholeObjects": 2,
    "allSixFieldsEqual": true,
    "countedContexts": 1,
    "verdictEach": "accept-source",
    "allFiveChecks": "satisfied-source",
    "blockingSourceFindings": 0,
    "canonicalSelection": false
  },
  "actualProductCompilerCLIProgramPairTestsExecuted": false,
  "formattingOnly": "rustfmt exit0 on6ownedcandidateRustfiles",
  "sourceAcceptance": "conditional ordinary source only; full raw reviews retained",
  "requiredNext": "Root-owned apply/reviewed source integration and existing bounded SDK-free execution for disjoint_program_plan and disjoint_test_public, followed by fresh result audit. No outcome inferred here.",
  "limitations": [
    "financial/work/value derivation independent but nonblind/source and related-oracle exposure disclosed",
    "full field/wire/bytes conditional authored codec and narrow finiteCoreoperation premise; not independent compiler-wire proof",
    "positive true-guard schedules only; no false/error/Shape diagnostic actual observation",
    "strong whole-program asset/state/source-duty-disjoint local two-member subset only",
    "allfivefulloperators/alloriginalparents/stagedgaps/R7/security/hostile/resource/constraint/proof/native/target questions remain open"
  ]
}

>>>

FINAL FINITE-INTEGRATION SECTION RESERVED; finalsource/mergedPR214/rootdispatchsignal NOT yet available; no providerjob created.
