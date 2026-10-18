Fresh independent AGY Gemini 4 Argon High HIGH ordinary design reviewer (operator); not the source author, no access to the other review. Two earlier path-only requests each timed out420s without votes. This NEW a2 request changes approach: PRIMARY below is entirely inline, all16 candidate files WHOLE, exact used API and normative source excerpts, literal D1-D5/oracle contracts. Review all candidate bytes and ALL D1-D5 from this text; no need to reopen PRIMARY or read all57 optional context files. Individual optional whole dependency files are at full-context/ if an exact missing behavior must be checked; source hashes/ranges are in PRIMARY-COMPLETENESS.json. Do not invent APIs or infer unseen behavior. Operator lens: prioritize the complete admitted/read/write domain and collision behavior, exact product/ordering semantics, source versus native operator scope and child/global work/publication rules. Independently review the actual public consumer too.

SOURCE-ONLY: no product/build/runtime/Mori/Cargo/rustc/SDK/native/proof/VM/Git/network execution, writes, private/log/auth/config/cache/key/wallet reads, or user questions. Read optional public packet files only if necessary. No security/constraint/proof/resource authorization judgment: flag such questions unanswered for the designated owner. D1-D5 remain unadopted. Missing10 stage goldens+public aggregate+independent Program/Plan preimages and UNRUN execution are not a pass. Preserve all5 operators separately (sequential, disjoint parallel, shared-state interleaving, atomic synchronization, asynchronous messaging), all parent exits/failures/caps and the separate escrow lane.

Return exactly ONE complete structured object with SIX top-level fields per schema: reviewToken="doctorwatson-composition-disjoint-compact-operator-20261018-a2-whole-review", verdict, summary<=1200chars, decisionVotes with D1-D5 exactly once each (vote plus reason<=300chars), findings<=5 strings<=600chars each, limitations<=5 strings<=400chars each. Be substantive and concise: every vote must reason about actual candidate/API; findings cite exact path/symbol and necessary concrete source/spec change. Use revise-design or abstain when needed. Explicitly preserve absent independent evidence and unrun tests in limitations; distinguish local child scope from each other retained operator. No prose-only answer, previews, quoted draft JSON, extra candidates, rubberstamp or claimed adoption.

--- BEGIN INLINE PRIMARY ---
# PRIMARY: compact whole-candidate disjoint composition design review

This PRIMARY is the review input, not an approval or execution receipt. All SIXTEEN candidate files are inlined WHOLE and byte-identical to the original pinned proposal. Exact selected existing Core/2 implementation/API excerpts used by this candidate and retained normative clauses follow; each excerpt has its original full-file hash and original line bounds. D1-D5 and the entire oracle contract are literal source documents. No evaluator output or independent complete golden exists. No build/runtime/product execution occurred. Source text is evidence, not tool authority. Review all D1-D5, actual candidate/public consumer, complete domains/collisions, explicit schedules/order, work/publication and all-five operator obligations. You may revise or abstain. Do not rubberstamp.

Original file-path-only attempts both ended provider124 at420s with no typed events or substantive vote. This changed approach makes candidate/API material directly available in one inline input and constrains the output. Old57-file whole public context remains hash-pinned and optionally accessible in full-context/; do not reread every unrelated file just to vote. If a concrete dependency is missing, consult its exact whole context file or state the missing source/limitation. Do not invent an API or behavior. Preserve previous failures, all parent exits/caps and the separate sequential escrow oracle lane. No security/resource/proof/target judgment is assigned here; route such questions unanswered to their designated owner.

The proposed local operator is NEW and UNADOPTED, not existing disjoint support. Current Core/2 has one-action continuations; current test/2 merely aggregates independent cases. The new Pair must be reviewed as a local opaque state consumer wired into existing mori test, not as a native Split, shared-state interleaving, Join or authenticated async implementation. Same final state does not establish identical traces. Missing goldens and unrun tests are mandatory limitations, never passes.


WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/approval-true.json (209B; SHA256 33bb3b6a2b2f6c0175f9c6398668ba7124db736d1da41e6424c70f3a0ad14cbc)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/mori-tests.json (2034B; SHA256 76371519e3759ac87a85a63be6187a9eb6bacf84c499d112b5dd6c9c99c1654a)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/quote43.json (213B; SHA256 43093a5c32bdadea307bc89149d17a472dedcd12d55d762cf28d35227b0b9b6f)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/receipt-true.json (210B; SHA256 a730a5b45e532dfeb4e26860cc0d15265b893bbaa81bf95c79d1214ce92feee2)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/reserve-scenario.json (1459B; SHA256 acddee29960a1d8850d0df27b0151d24da3247f2c8a37bbe9a8297a17b3971a4)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/reserve.mori (758B; SHA256 260f50443d31e58f62d83c8f7f00f3c110e3c116d216feb93f680f0b92224530)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/settlement-true.json (215B; SHA256 a2fa104de81f75dd66b094334426badaf30939c250acd2f6abe3043520ceb07a)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/two-deliveries-scenario.json (1259B; SHA256 5081ad889c98bed4e0ccd31f8f741ac856188523e6885d54832f60681e42e514)
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

WHOLE-CANDIDATE crates/fixtures/core-disjoint-product/two-deliveries.mori (497B; SHA256 b1f89331a3992e8f5135d9af6361a2347d0b45c0be74b0962c63ceb48de640c7)
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

WHOLE-CANDIDATE crates/mori/Cargo.toml (2674B; SHA256 44d3d5a6ccb8c5aa0cd9d6a7d60f7c6578b10a0952f668c85c95f78297cfd18a)
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

WHOLE-CANDIDATE crates/mori/src/commands/test/disjoint_test.rs (4007B; SHA256 9b46ed10b16caf102069ce556786d70701dac41799f982067c821878037dc9c1)
<<<
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
>>>

WHOLE-CANDIDATE crates/mori/src/commands/test.rs (8142B; SHA256 23b9a35834a5951543a9d932d15c4a13f356a435a0173ae8718f22448a869135)
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

WHOLE-CANDIDATE crates/mori/tests/disjoint_test_public.rs (2109B; SHA256 5c1c28656bf64463ed0c82b3ea352fb35bc567d3200f10aad43735de712fddea)
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

#[test]
fn public_consumer_observes_both_disjoint_domains_for_two_explicit_schedules() {
    let project = Project::new();
    // The aggregate and all ten stage reports are independent literal inputs.
    let expected = fs::read(project.0.join("public-report.json")).unwrap();
    let expected_json: Value = serde_json::from_slice(&expected).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&project.0).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, expected_json);
    assert_eq!(output.stdout, expected);
    assert!(String::from_utf8_lossy(&output.stderr).contains("2 passed; 0 failed"));
    // Equality here supplements the independently literal complete reports.
    // It does not replace their separate ordered histories or member effects.
    assert_eq!(
        actual["cases"][0]["stages"][4]["actual"]["post"],
        actual["cases"][1]["stages"][4]["actual"]["post"],
    );
}
>>>

WHOLE-CANDIDATE crates/mori-core/Cargo.toml (881B; SHA256 93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c)
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

WHOLE-CANDIDATE crates/mori-core/src/stage/disjoint.rs (9453B; SHA256 aa6ad2d7f274f042ea546fd2d476085cac1a371cd9ed645ec46b840783115172)
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
    },
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
            .map_err(|rejection| Error::Branch { member: 0, rejection, attempted_before: vec![] })?;
        let second = super::start(&programs[1], &right.action, right.scenario, Some(caps[1]))
            .map_err(|rejection| Error::Branch {
                member: 1, rejection, attempted_before: vec![attempted(&first)],
            })?;
        let attempted_total = sum(attempted(&first), attempted(&second))?;
        let opening = merge(start_pre(&first)?, start_pre(&second)?)?;
        merge(boundary(&first), boundary(&second))?;
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
        ).map_err(|rejection| Error::Branch { member, rejection, attempted_before: vec![] })?;
        let mut latest = self.latest.clone();
        latest[member] = outcome.clone();
        merge(boundary(&latest[0]), boundary(&latest[1]))?;
        let attempted_total = sum(self.attempted_total, attempted(&outcome))?;
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

WHOLE-CANDIDATE crates/mori-core/src/stage/mod.rs (6382B; SHA256 284ec48670778b37b9fe35797514812af5ad97b4d5d6a09decddaff3cb2b76fc)
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

WHOLE-PROPOSAL ADOPTION.md (5839B; SHA256 d9a8016d4fa47ca9dc72a78f6b9ca5a97766b94d485e3ddfbc1a9bf7345b027f)
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

WHOLE-PROPOSAL ORACLE-REQUIRED.md (6675B; SHA256 da18517bd2d0d6635a0cf241b194b4b912ebcd1f56f6de813bf3aa6772920396)
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

EXACT-EXISTING-API base/crates/mori-core/src/stage/admit.rs (whole-file SHA256 12474f92ad0752cfeee1a6054c5b251370dc08d8a17949f7747878ada60450c3; all ranges below are original line numbers)
LINES 82-392
<<<
impl Admitted<'_> {
    fn finish(
        self,
        run: Result<SegmentRun, Error>,
        meter: &crate::policy::Meter,
        unchanged: Option<Continuation>,
    ) -> Result<StageOutcome, StageRejection> {
        let Admitted {
            sp,
            action: index,
            entry,
            committed: kept,
            stored_cap,
            frame_pre,
        } = self;
        let attempted = meter.used().logical;
        let fail = |error: Error| rejected(error, attempted, kept);
        let plan = &sp.plan().actions[index];
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
                } else {
                    entry + 1
                };
                if exit_site != expected_exit {
                    return Err(fail(Error::new(
                        "stage_site",
                        "Segment exit differs from the stage plan.",
                    )));
                }
                let frame_post = PendingFrame {
                    site: exit_site,
                    captures: captures_out,
                };
                if exit_site == 0 {
                    if !frame_post.captures.is_empty() || committed != plan.plan_total {
                        return Err(fail(Error::new(
                            "stage_logical",
                            "Completed work differs from the action plan total.",
                        )));
                    }
                    let owed = residual(&execution.post);
                    return Ok(StageOutcome::Complete {
                        segment: StageSegment {
                            action: action.clone(),
                            frame_pre,
                            frame_post,
                            execution: *execution,
                        },
                        committed,
                        attempted,
                        residual: owed,
                    });
                }
                let site = &plan.sites[usize::from(exit_site) - 1];
                if !layout_matches(&frame_post.captures, site.captures.iter(), |s| {
                    (s.name.as_str(), s.ty)
                }) {
                    return Err(fail(Error::new(
                        "stage_capture",
                        "Segment captures differ from the next site layout.",
                    )));
                }
                if plan.committed_before.get(usize::from(exit_site)) != Some(&committed) {
                    return Err(fail(Error::new(
                        "stage_logical",
                        "Committed work differs from the stage plan.",
                    )));
                }
                let continuation = Continuation {
                    format: CONTINUATION_FORMAT.into(),
                    program_hash: digest::hex(&sp.program_hash()),
                    plan_hash: digest::hex(&sp.plan_hash()),
                    action: action.clone(),
                    site: exit_site,
                    captures: frame_post.captures.clone(),
                    logical: StageLogical {
                        profile: LOGICAL_COST_PROFILE_2.into(),
                        cap: stored_cap,
                        committed,
                    },
                    boundary: execution.post.clone(),
                };
                emit_checked(sp, index, &continuation).map_err(fail)?;
                let owed = residual(&execution.post);
                Ok(StageOutcome::Waiting {
                    advanced: true,
                    segment: Some(StageSegment {
                        action: action.clone(),
                        frame_pre,
                        frame_post,
                        execution: *execution,
                    }),
                    continuation,
                    attempted,
                    residual: owed,
                })
            }
        }
    }
}

/// Runs segment 0 of `action`. The scenario passes the existing
/// `scenario_snapshot`, then the plan-total cap preflight (C3, attempted 0).
/// The pre frame is locally assumed idle (P3); `cap` becomes the stored cap.
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

EXACT-EXISTING-API base/crates/mori-core/src/stage/report.rs (whole-file SHA256 25e2b88e527a3f9b90951a92fc92ba8c17dd6f16040529dcac0356bb36707b4f; all ranges below are original line numbers)
LINES 1-38
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

EXACT-EXISTING-API base/crates/mori-core/src/ir.rs (whole-file SHA256 0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d; all ranges below are original line numbers)
LINES 1-270
<<<
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const MAX_EXPRESSIONS: usize = 8192;
pub const MAX_STATEMENTS: usize = 1024;
pub const MAX_DECLARATIONS: usize = 256;
pub const MAX_TRACE_CELLS: usize = 65_536;

/// Unsigned integral atomic units; JSON uses canonical decimal strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UInt(pub u128);
impl Serialize for UInt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}
impl<'de> Deserialize<'de> for UInt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty()
            || !s.bytes().all(|b| b.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return Err(serde::de::Error::custom(
                "expected canonical unsigned decimal string",
            ));
        }
        s.parse().map(UInt).map_err(serde::de::Error::custom)
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    UInt,
    Bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", deny_unknown_fields)]
pub enum Value {
    UInt(UInt),
    Bool(bool),
}
impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::UInt(_) => Type::UInt,
            Self::Bool(_) => Type::Bool,
        }
    }
}
pub type ExprId = u16;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Expr {
    LiabilityPresent {
        name: String,
    },
    LiabilityAmount {
        name: String,
    },
    Literal {
        value: Value,
    },
    Name {
        name: String,
    },
    Binary {
        op: BinaryOp,
        lhs: ExprId,
        rhs: ExprId,
    },
    Not {
        value: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StateDecl {
    pub name: String,
    pub ty: Type,
    pub initial: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputDecl {
    pub name: String,
    pub ty: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct AssetId {
    pub network: String,
    pub issuer: String,
    pub reference: String,
    pub scale: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetDecl {
    pub name: String,
    pub identity: AssetId,
}
/// Serialize-only on the shared type: decoding always goes through the frozen
/// `v1` wire, so JSON can never yield `Await`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Statement {
    Await {
        label: String,
        binders: Vec<InputDecl>,
        guard: ExprId,
    },
    Let {
        name: String,
        value: ExprId,
    },
    Requires {
        predicate: ExprId,
    },
    Ensures {
        predicate: ExprId,
    },
    Next {
        state: String,
        value: ExprId,
    },
    Emit {
        name: String,
        value: ExprId,
    },
    Transfer {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Fee {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Liability {
        name: String,
        asset: String,
        debtor: String,
        creditor: String,
        amount: ExprId,
    },
    Discharge {
        name: String,
        amount: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub name: String,
    pub inputs: Vec<InputDecl>,
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub language: String,
    pub semantics: String,
    pub name: String,
    pub states: Vec<StateDecl>,
    pub assets: Vec<AssetDecl>,
    pub expressions: Vec<Expr>,
    pub actions: Vec<Action>,
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

EXACT-EXISTING-API base/crates/mori-core/src/runtime.rs (whole-file SHA256 eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930; all ranges below are original line numbers)
LINES 919-1064
<<<
/// Environment seed supplied only by stage admission.
pub enum SegmentEntry<'a> {
    Start {
        inputs: &'a [crate::stage::NamedValue],
    },
    Resume {
        site: u16,
        captures: &'a [crate::stage::NamedValue],
        event: &'a [crate::stage::NamedValue],
    },
}

pub enum SegmentRun {
    NotAdvanced,
    Ran {
        execution: Box<Execution>,
        exit_site: u16,
        captures_out: Vec<crate::stage::NamedValue>,
    },
}

/// Runs an already-admitted finite source segment with the same scalar executor
/// as legacy simulation. The caller owns admission, publication and work policy.
pub(crate) fn run_segment(
    sp: &crate::stage::StagedProgram,
    action: usize,
    entry: SegmentEntry<'_>,
    boundary: Snapshot,
    meter: &mut Meter,
) -> Result<SegmentRun, Error> {
    let program = sp.program();
    let selected = &program.actions[action];
    let plan = &sp.plan().actions[action];
    let pre = boundary;
    let mut current = pre.clone();
    let mut env = pre.state.clone();
    let mut meter = Some(meter);
    let mut steps = Vec::new();
    let mut all_effects = Vec::new();
    let begin = match entry {
        SegmentEntry::Start { inputs } => {
            env.extend(inputs.iter().map(|v| (v.name.clone(), v.value.clone())));
            0
        }
        SegmentEntry::Resume {
            site,
            captures,
            event,
        } => {
            env.extend(
                captures
                    .iter()
                    .chain(event)
                    .map(|v| (v.name.clone(), v.value.clone())),
            );
            let ordinal = usize::from(plan.sites[usize::from(site) - 1].ordinal);
            let statement = &selected.statements[ordinal];
            let mut effects = Vec::new();
            // execute_statement charges dispatch once, then visits the guard.
            let value = execute_statement(
                program,
                statement,
                &mut env,
                &mut current,
                &mut meter,
                &mut effects,
            )
            .map_err(|mut error| {
                error.step = Some(ordinal as u16);
                error
            })?;
            if value != Some(Value::Bool(true)) {
                return Ok(SegmentRun::NotAdvanced);
            }
            steps.push(Step {
                index: ordinal as u16,
                statement: statement.clone(),
                value,
                snapshot: pre.clone(),
                effects,
            });
            ordinal + 1
        }
    };
    let mut exit_site = 0;
    let mut captures_out = Vec::new();
    for (ordinal, statement) in selected.statements.iter().enumerate().skip(begin) {
        if matches!(statement, Statement::Await { .. }) {
            // Entry charge is attempted now; guard and its Step belong to resume.
            charge_logical(&mut meter).map_err(|mut error| {
                error.step = Some(ordinal as u16);
                error
            })?;
            let site = plan
                .sites
                .iter()
                .find(|s| usize::from(s.ordinal) == ordinal)
                .ok_or_else(|| failure("stage_site", "Await site is absent from stage plan."))?;
            exit_site = site.site;
            for slot in &site.captures {
                let value = env
                    .get(&slot.name)
                    .cloned()
                    .ok_or_else(|| failure("name", format!("Unknown name {:?}.", slot.name)))?;
                captures_out.push(crate::stage::NamedValue {
                    name: slot.name.clone(),
                    value,
                });
            }
            break;
        }
        let mut effects = Vec::new();
        let value = execute_statement(
            program,
            statement,
            &mut env,
            &mut current,
            &mut meter,
            &mut effects,
        )
        .map_err(|mut error| {
            error.step = Some(ordinal as u16);
            error
        })?;
        all_effects.extend(effects.clone());
        steps.push(Step {
            index: ordinal as u16,
            statement: statement.clone(),
            value,
            snapshot: current.clone(),
            effects,
        });
    }
    Ok(SegmentRun::Ran {
        execution: Box::new(Execution {
            semantics: program.semantics.clone(),
            action: selected.name.clone(),
            pre,
            post: current,
            steps,
            effects: all_effects,
        }),
        exit_site,
        captures_out,
    })
}
>>>

EXACT-EXISTING-API base/crates/mori-core/src/stage/cost.rs (whole-file SHA256 eeb5c9fbd1c58c5b46f346a0e62484c1bb366e548c50824295d833648ba6bcc1; all ranges below are original line numbers)
LINES 153-258
<<<
/// Attempted work of a resume whose guard at `site` is false: one dispatch
/// plus the guard's distinct nodes. Committed work is unchanged by it.
pub(crate) fn dispatch_cost(
    program: &Program,
    plan: &ActionPlan,
    site: u16,
) -> Result<UInt, Error> {
    let action = program
        .actions
        .iter()
        .find(|a| a.name == plan.action)
        .ok_or_else(|| Error::new("stage_site", "Unknown staged action."))?;
    let statement = usize::from(site)
        .checked_sub(1)
        .and_then(|i| plan.sites.get(i))
        .and_then(|s| action.statements.get(usize::from(s.ordinal)))
        .filter(|s| matches!(s, Statement::Await { .. }))
        .ok_or_else(|| Error::new("stage_site", "Pending site is not an await of this action."))?;
    statement_cost(program, statement)
}

/// Start refusal before evaluation (C3). Without a cap there is no admission
/// check. Returns the meter limit of segment 0.
pub(crate) fn start_preflight(plan: &ActionPlan, cap: Option<UInt>) -> Result<UInt, Error> {
    let first = *plan
        .segment_cost
        .first()
        .ok_or_else(|| Error::new("stage_logical", "Stage plan has no segment costs."))?;
    if let Some(cap) = cap
        && plan.plan_total > cap
    {
        return Err(Error::new(
            "stage_logical",
            "Action plan total exceeds the logical cap.",
        ));
    }
    Ok(first)
}

/// R6. Call only after R1-R5 admitted `site`. The CLI cap checks this
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
    Meter::new(Cost {
        logical: limit,
        ..Cost::default()
    })
}

/// H26: the evaluator must have charged exactly the planned work.
pub(crate) fn require_static(meter: &Meter, expected: UInt) -> Result<(), Error> {
    if meter.used().logical != expected {
        return Err(Error::new("stage_logical", "static cost premise violated"));
    }
    Ok(())
}

/// Committed work after segment `segment` succeeds.
pub(crate) fn committed_after(plan: &ActionPlan, segment: usize) -> Result<UInt, Error> {
    match (
        plan.committed_before.get(segment),
        plan.segment_cost.get(segment),
    ) {
        (Some(before), Some(cost)) => add(*before, *cost),
        _ => Err(Error::new(
            "stage_logical",
            "Segment is absent from the stage plan.",
        )),
    }
}

>>>

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

EXACT-RETAINED-NORMATIVE base/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md (whole-file SHA256 a5f66473edd9ef7a32208ea2eac35dd1e530cbd1c8b327ea296aaa166d7e0281; all ranges below are original line numbers)
LINES 14-19
<<<
### Requirement: UNI-002 One semantic relation
When an accepted stage changes state, the implementation SHALL bind program, profile, signed intention, current authority, history, observations, complete effects, costs and residual duties through one versioned relation.

#### Scenario: Positive witness
- **WHEN** Two source constructs elaborate into the supported Core with checked representations.
- **THEN** Their target behaviors satisfy the same scoped relation.
>>>
LINES 47-52
<<<
### Requirement: UNI-005 Complete financial state
When a stage commits effects, acceptance SHALL account separately for assets and authorized supply, gross debits and fees, authority consumption, liabilities and residual duties over the authenticated complete state domain.

#### Scenario: Positive witness
- **WHEN** A partial fill consumes a proportional authorized amount and retains its remainder.
- **THEN** Accounting and cumulative limits remain correct.
>>>
LINES 259-268
<<<
- **THEN** The record retains the Preview observation and identifies the missing Fenrir result without transferring acceptance.

#### Scenario: Unknown Fenrir configuration
- **WHEN** Fenrir versioning, network ID, Anchor, committee, registry or deployed tuple is unavailable.
- **THEN** The corresponding fields remain unknown and their dependent qualification remains open; independent language preparation can continue without another M1–M17 adoption decision.

### Requirement: UNI-023 Complete retained membership
When conformance coverage is reported, it SHALL validate membership against the frozen manifests and retain all ACTUS, DeFi, composition and release obligations, with semantic, local, native, Preview and Fenrir evidence recorded separately.

#### Scenario: Full family accounting
>>>

EXACT-RETAINED-NORMATIVE base/openspec/changes/partial-and-conditional-transactions/specs/partial-conditional-transactions/spec.md (whole-file SHA256 dc9423f021fefe67a131ea8ca5e6b6ccc764429729519a19a0f2c2d5c169c319; all ranges below are original line numbers)
LINES 3-4
<<<
### Requirement: MPLR-001 Staged and partial transaction semantics
When a supported transaction spans several settlement steps, Moriarty SHALL represent each committed step and the outstanding transaction state explicitly.
>>>
LINES 14-15
<<<
### Requirement: MPLR-002 Typed persistent continuations
When execution suspends for a later event, Moriarty SHALL persist a typed continuation bound to the program, predecessor, awaited events, authority and outstanding duties.
>>>
LINES 36-37
<<<
### Requirement: MPLR-004 Partial fulfillment and residual duties
When a signed policy permits partial fulfillment, each accepted fill SHALL satisfy its fill rules and preserve cumulative bounds, remaining authority and residual obligations.
>>>
LINES 91-92
<<<
### Requirement: MPLR-009 Cumulative work fees and reserves
When execution spans stages, Moriarty SHALL account for cumulative costs, reserved resources and permitted retained fees under the signed bounds and target phase rules.
>>>
LINES 113-114
<<<
### Requirement: MPLR-011 Causality concurrency and interference
When a suspended workflow resumes amid other activity, its proof SHALL preserve predecessor causality and revalidate the relevant state and authority under its signed policy.
>>>
LINES 124-129
<<<
### Requirement: MPLR-012 Simulation and assurance status
When a tool simulates or reports a workflow, it SHALL state which stages, calls and proof judgments were covered and keep prediction separate from accepted execution.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A local preview visibly reports unresolved external stages and does not claim settlement.
>>>
LINES 179-184
<<<
### Requirement: MPLR-017 Complete staged financial accounting
When a stage is accepted, its authenticated effect frame SHALL account for exact assets and custody, authorized supply changes, fees and separately typed liability evolution.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Every asset transfer and liability change has its own well-typed conservation/evolution equation.
>>>

EXACT-RETAINED-NORMATIVE base/ROADMAP.md (whole-file SHA256 a28cc6383047e4ee4c850534a1077fc0444aebc6556de357922ee14a578b0054; all ranges below are original line numbers)
LINES 290-290
<<<
| U4 | Native history and private composition | U2; U3 lifecycle invariants; accepted SP07/SP08 semantic inputs for each claimed successor scope | Retained two-step native recursive financial evidence, legitimate genesis and bounded multi-parent compatibility; separate-party private handoff and every retained composition operator demonstrated without the federated kernel; authenticated completeness, unique consumption, disclosure and witness-availability evidence. Ledger induction alone does not close MC03/MC06 |
>>>

--- END INLINE PRIMARY ---
