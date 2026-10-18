You are a fresh nonauthor Gemini HIGH ordinary ACTUAL-RESULT reviewer, independent of the separate live source-review context. The immutable full physical packet is /home/charl/moriarty-c2-opus55-restored-20261017/core2-complete-result-review-packet; MANIFEST SHA256 7f8c56ecbbf4c24a28391da583d8075068800774da8f91a8bb97aa07e90481ae. The primary burden is INLINE BELOW: 18 complete exact raw file bodies with lengths/SHA/terminal-LF flags. INLINE-BYTE-CUSTODY.json proves each body reconstructs its original file exactly. No expected JSON key/value/array or actual capture is omitted. Begin with these inline bodies, not745 unchanged source files. All other full source/adopted docs/receipts/43payload freeze/provenance/closure remain physically pinned and available ONLY as needed. Full-source review is separately pending under doctorwatson-core2-complete-source-ordinary-20261018-a1; do not infer its approval. Condition any merge recommendation on that independent audit being resolved; no cancellation/restart or logical dependence on its findings.

Audit THIS finite observation: exact718b1b020718c6b23a12846d32d5d2db9f14de7c/tree1e3fc1912705ce349353accaee7924644e684012, formatted sourcec8a48dbccc77169822b57246b91d842573d274af31d54930d65a4d06012ddfbc. Root executed exactly one distinct90s+5s selected ordinary offline/frozen-j1 --no-default-features mori target from a fresh owned cold target with exact13-variable Cargo-child environment/noHOME. Captured actual exit0/wall54.704761924s, all3 named tests passed0failed/ignored/filtered, clean unchanged source and cleanup removed its owned root. Allocation is CONSUMED; no rerun or additional execution is requested. Verify against actual captured metadata/stdout/stderr below rather than trusting this summary.

The3 predicates are whole five-node Program+filled Plan; real public library stage::start/resume/outcome_report3whole reports; real CARGO_BIN_EXE_mori3process calls with entire JSON/exact public ASCII stdout/exit0/empty stderr. Check every complete statement/ExprId/typed value/frame/snapshot/delta/ordered effects/duty/digest/member/null/absence. Both flows carry ONLY the actual immediately preceding waiting continuation. False observation must leave the entire continuation unchanged and no segment; true must retain release0 with empty positive residual. Attempted work5/2/6, committed5/5/11. Library pretty bytes are intentionally outside the selected contract. Actual CLI stdout JSONs were captured and compared inside the passing Rust test, not separately archived as CLI process documents: do not claim an independent manual readback of each actual CLI body. Complete process logs are below. No mock/success-flag/oracle generation/filter/sort/normalization is acceptable.

Independent freeze SHA2fd5c05c3a25af78a8f71ab74056f9c6a4342ab42f303a3a0090c360cb6e5f22 and all43 payload copies are unchanged; original-partial frozen history and no-LF compact preimages remain. Custodian exposure is disclosed: post-adoption public DTO/normative reads incidentally displayed residual helper and InputDecl-to-StageBinder conversion; never call it blind/preimplementation/wholly-unexposed. Source/test author is implementation-exposed. Goldens were not selected from product output. Exact RUSTFMT before/after/diff below changes only one argv array's whitespace; no golden changed. Physical whitespace receipt preserves two exact immutable normative-excerpt blank-EOF exceptions, no global waiver.

Source-only dependency/full50 audit remains separate, not a burden to reread745 files here. Owner M03 integration audit is separate; do not author or reassess identity/security conditions. All14 staged gaps remain OPEN with original dispositions, and no parent/U0–U7/M1–M17/MC/SP/G, general composition/finance/private/native/proof/ledger/authentication/consumption claim follows. Public developer use remains permissionless. Ordinary Cargo/toolchain/public cache/system linker trust assumptions and lack of syscall tracing/aggregate process-memory bound remain. START does not enumerate public cache symlink targets; intended mappings are in source/PROPOSAL.json, not independently observed trace evidence.

Use no shell/commands, parser, Cargo/rustfmt/CLI/compiler/model/SDK/proof/network or mutation. You need not read unchanged closure files; optional read-only packet inspection is allowed if it resolves a concrete gap. Return ONE complete six-field JSON matching OUTPUT-SCHEMA: reviewToken=doctorwatson-core2-complete-result-ordinary-20261018-a1-whole-review, verdict approve-actual-result/changes-required/abstain. Concise output: summary<=1200chars; at most5findings<=600chars each; at most5limitations<=400chars each; filesRead<=20. Cite primary inline file names or PROMPT.md inline bundle honestly plus any actual physical reads. Do not claim to have read all745 files. No partial narrowed pass; note exact evidence limits and pending independent source-audit condition. You have420s wrapper wall budget.

# Exact inline primary evidence

Each body is copied unchanged. Byte offsets/counts/hashes are independently checkable in INLINE-BYTE-CUSTODY.json. Framing is not part of any input, golden or actual capture.

===== BEGIN FILE source/candidate/crates/mori/tests/core2_escrow_complete.rs | bytes=7374 | sha256=c8a48dbccc77169822b57246b91d842573d274af31d54930d65a4d06012ddfbc | terminalLF=True =====
//! Complete local Core/2 escrow observations through public consumers.
use mori_core::{
    Scenario, UInt,
    stage::{self, Continuation, PendingFrame, StageOutcome},
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core2-escrow-complete")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn assert_report(actual: &Value, name: &str, attempted: &str, committed: &str) {
    assert_eq!(
        actual,
        &json_fixture(&format!("{name}.report.expected.json")),
        "{name}: complete report"
    );
    assert_eq!(actual["attempted"], attempted, "{name}: attempted work");
    assert_eq!(actual["committed"], committed, "{name}: committed work");
}

fn assert_outcome(
    outcome: StageOutcome,
    name: &str,
    attempted: &str,
    committed: &str,
) -> Option<Continuation> {
    let continuation = match &outcome {
        StageOutcome::Waiting { continuation, .. } => Some(continuation.clone()),
        StageOutcome::Complete { .. } => None,
    };
    let actual = stage::outcome_report(Ok(outcome));
    assert_report(&actual, name, attempted, committed);
    continuation
}

#[test]
fn escrow_public_load_matches_complete_program_and_filled_plan() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        json_fixture("escrow.program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        json_fixture("escrow.plan.expected.json")
    );
}

#[test]
fn escrow_library_start_false_observation_and_delivery_match_complete_reports() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    let scenario: Scenario = serde_json::from_value(json_fixture("escrow.scenario.json")).unwrap();
    let false_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-false.event.json")).unwrap(),
    )
    .unwrap();
    let true_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-true.event.json")).unwrap(),
    )
    .unwrap();

    let started = stage::start(&program, "deliver", scenario, Some(UInt(11))).unwrap();
    let first = assert_outcome(started, "start", "5", "5").unwrap();
    let before_observation = first.clone();
    let observed = stage::resume(&program, first, false_event, Some("deliver"), None).unwrap();
    let pending = assert_outcome(observed, "delivery-false", "2", "5").unwrap();
    assert_eq!(pending, before_observation);

    let funded_boundary = pending.boundary.clone();
    let pending_frame = PendingFrame {
        site: pending.site,
        captures: pending.captures.clone(),
    };
    let completed = stage::resume(&program, pending, true_event, Some("deliver"), None).unwrap();
    let StageOutcome::Complete {
        segment, residual, ..
    } = &completed
    else {
        panic!("delivery must complete");
    };
    assert_eq!(segment.frame_pre, pending_frame);
    assert_eq!(segment.execution.pre, funded_boundary);
    assert!(residual.is_empty());
    assert_eq!(segment.execution.post.liabilities[0].name, "release");
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(0));
    assert!(assert_outcome(completed, "delivery-true", "6", "11").is_none());
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core2-escrow-complete-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in [
            "escrow.mori",
            "escrow.scenario.json",
            "delivery-false.event.json",
            "delivery-true.event.json",
        ] {
            fs::copy(fixtures().join(name), project.root.join(name)).unwrap();
        }
        project
    }

    fn assert_case(&self, name: &str, args: &[&str], attempted: &str, committed: &str) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args([
                "--color",
                "never",
                "simulate",
                "escrow.mori",
                "--action",
                "deliver",
            ])
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            output.stderr,
            fs::read(fixtures().join("empty.stderr.expected.bin")).unwrap(),
            "{name}: exact empty stderr"
        );
        assert!(output.stdout.is_ascii(), "{name}: ASCII stdout");
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_report(&actual, name, attempted, committed);
        assert_eq!(
            output.stdout,
            fs::read(fixtures().join(format!("{name}.stdout.expected.json"))).unwrap(),
            "{name}: exact public stdout bytes"
        );
        actual
    }

    fn save_actual_continuation(&self, continuation: &Value) {
        fs::write(
            self.root.join("actual-continuation.json"),
            serde_json::to_vec(continuation).unwrap(),
        )
        .unwrap();
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn escrow_cli_start_false_observation_and_delivery_match_complete_public_bytes() {
    let project = Project::new();
    let started = project.assert_case(
        "start",
        &["--scenario", "escrow.scenario.json", "--logical-cap", "11"],
        "5",
        "5",
    );
    let first_continuation = started["continuation"].clone();
    project.save_actual_continuation(&started["continuation"]);
    let observed = project.assert_case(
        "delivery-false",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-false.event.json",
        ],
        "2",
        "5",
    );
    assert_eq!(observed["continuation"], first_continuation);
    assert_eq!(observed["advanced"], false);
    assert!(observed["segment"].is_null());
    project.save_actual_continuation(&observed["continuation"]);
    let completed = project.assert_case(
        "delivery-true",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-true.event.json",
        ],
        "6",
        "11",
    );
    assert_eq!(
        completed["segment"]["execution"]["pre"],
        observed["continuation"]["boundary"]
    );
    assert!(completed["residual"].as_array().unwrap().is_empty());
    let closing_duty = &completed["segment"]["execution"]["post"]["liabilities"][0];
    assert_eq!(closing_duty["name"], "release");
    assert_eq!(closing_duty["amount"], "0");
}
===== END FILE source/candidate/crates/mori/tests/core2_escrow_complete.rs =====

===== BEGIN FILE source/formatting/AUTHOR-ORIGINAL.rs | bytes=7294 | sha256=9e4a373b37b874cfb8540cb23b939a429f7852ffde0a2da26b6fdf6bb6edca7d | terminalLF=True =====
//! Complete local Core/2 escrow observations through public consumers.
use mori_core::{
    Scenario, UInt,
    stage::{self, Continuation, PendingFrame, StageOutcome},
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core2-escrow-complete")
}

fn json_fixture(name: &str) -> Value {
    serde_json::from_slice(&fs::read(fixtures().join(name)).unwrap()).unwrap()
}

fn assert_report(actual: &Value, name: &str, attempted: &str, committed: &str) {
    assert_eq!(
        actual,
        &json_fixture(&format!("{name}.report.expected.json")),
        "{name}: complete report"
    );
    assert_eq!(actual["attempted"], attempted, "{name}: attempted work");
    assert_eq!(actual["committed"], committed, "{name}: committed work");
}

fn assert_outcome(
    outcome: StageOutcome,
    name: &str,
    attempted: &str,
    committed: &str,
) -> Option<Continuation> {
    let continuation = match &outcome {
        StageOutcome::Waiting { continuation, .. } => Some(continuation.clone()),
        StageOutcome::Complete { .. } => None,
    };
    let actual = stage::outcome_report(Ok(outcome));
    assert_report(&actual, name, attempted, committed);
    continuation
}

#[test]
fn escrow_public_load_matches_complete_program_and_filled_plan() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    assert_eq!(
        serde_json::to_value(program.program()).unwrap(),
        json_fixture("escrow.program.expected.json")
    );
    assert_eq!(
        serde_json::to_value(program.plan()).unwrap(),
        json_fixture("escrow.plan.expected.json")
    );
}

#[test]
fn escrow_library_start_false_observation_and_delivery_match_complete_reports() {
    let source = fs::read_to_string(fixtures().join("escrow.mori")).unwrap();
    let program = stage::load(&source).unwrap();
    let scenario: Scenario = serde_json::from_value(json_fixture("escrow.scenario.json")).unwrap();
    let false_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-false.event.json")).unwrap(),
    )
    .unwrap();
    let true_event = stage::decode::event(
        &fs::read_to_string(fixtures().join("delivery-true.event.json")).unwrap(),
    )
    .unwrap();

    let started = stage::start(&program, "deliver", scenario, Some(UInt(11))).unwrap();
    let first = assert_outcome(started, "start", "5", "5").unwrap();
    let before_observation = first.clone();
    let observed = stage::resume(&program, first, false_event, Some("deliver"), None).unwrap();
    let pending = assert_outcome(observed, "delivery-false", "2", "5").unwrap();
    assert_eq!(pending, before_observation);

    let funded_boundary = pending.boundary.clone();
    let pending_frame = PendingFrame {
        site: pending.site,
        captures: pending.captures.clone(),
    };
    let completed = stage::resume(&program, pending, true_event, Some("deliver"), None).unwrap();
    let StageOutcome::Complete {
        segment, residual, ..
    } = &completed
    else {
        panic!("delivery must complete");
    };
    assert_eq!(segment.frame_pre, pending_frame);
    assert_eq!(segment.execution.pre, funded_boundary);
    assert!(residual.is_empty());
    assert_eq!(segment.execution.post.liabilities[0].name, "release");
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(0));
    assert!(assert_outcome(completed, "delivery-true", "6", "11").is_none());
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core2-escrow-complete-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let project = Self { root };
        for name in [
            "escrow.mori",
            "escrow.scenario.json",
            "delivery-false.event.json",
            "delivery-true.event.json",
        ] {
            fs::copy(fixtures().join(name), project.root.join(name)).unwrap();
        }
        project
    }

    fn assert_case(&self, name: &str, args: &[&str], attempted: &str, committed: &str) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args([
                "--color", "never", "simulate", "escrow.mori", "--action", "deliver",
            ])
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            output.stderr,
            fs::read(fixtures().join("empty.stderr.expected.bin")).unwrap(),
            "{name}: exact empty stderr"
        );
        assert!(output.stdout.is_ascii(), "{name}: ASCII stdout");
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_report(&actual, name, attempted, committed);
        assert_eq!(
            output.stdout,
            fs::read(fixtures().join(format!("{name}.stdout.expected.json"))).unwrap(),
            "{name}: exact public stdout bytes"
        );
        actual
    }

    fn save_actual_continuation(&self, continuation: &Value) {
        fs::write(
            self.root.join("actual-continuation.json"),
            serde_json::to_vec(continuation).unwrap(),
        )
        .unwrap();
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn escrow_cli_start_false_observation_and_delivery_match_complete_public_bytes() {
    let project = Project::new();
    let started = project.assert_case(
        "start",
        &["--scenario", "escrow.scenario.json", "--logical-cap", "11"],
        "5",
        "5",
    );
    let first_continuation = started["continuation"].clone();
    project.save_actual_continuation(&started["continuation"]);
    let observed = project.assert_case(
        "delivery-false",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-false.event.json",
        ],
        "2",
        "5",
    );
    assert_eq!(observed["continuation"], first_continuation);
    assert_eq!(observed["advanced"], false);
    assert!(observed["segment"].is_null());
    project.save_actual_continuation(&observed["continuation"]);
    let completed = project.assert_case(
        "delivery-true",
        &[
            "--resume",
            "actual-continuation.json",
            "--event",
            "delivery-true.event.json",
        ],
        "6",
        "11",
    );
    assert_eq!(
        completed["segment"]["execution"]["pre"],
        observed["continuation"]["boundary"]
    );
    assert!(completed["residual"].as_array().unwrap().is_empty());
    let closing_duty = &completed["segment"]["execution"]["post"]["liabilities"][0];
    assert_eq!(closing_duty["name"], "release");
    assert_eq!(closing_duty["amount"], "0");
}
===== END FILE source/formatting/AUTHOR-ORIGINAL.rs =====

===== BEGIN FILE source/formatting/RUSTFMT.json | bytes=247 | sha256=b2c5314a8bd014f6a42e9d9ffb7268fc77c5ab074f118dbb42a06c8453df5570 | terminalLF=True =====
{
  "exitCode": 0,
  "originalSha256": "9e4a373b37b874cfb8540cb23b939a429f7852ffde0a2da26b6fdf6bb6edca7d",
  "reviewedSha256": "c8a48dbccc77169822b57246b91d842573d274af31d54930d65a4d06012ddfbc",
  "identical": false,
  "diff": "FORMATTING.diff"
}
===== END FILE source/formatting/RUSTFMT.json =====

===== BEGIN FILE source/formatting/FORMATTING.diff | bytes=510 | sha256=a085049c96ea7bad960f5000158e89e187bc6cbd9f647a8ce72bc6f8d9a693db | terminalLF=True =====
--- author-original
+++ formatted-candidate
@@ -129,7 +129,12 @@
         let output = Command::new(env!("CARGO_BIN_EXE_mori"))
             .current_dir(&self.root)
             .args([
-                "--color", "never", "simulate", "escrow.mori", "--action", "deliver",
+                "--color",
+                "never",
+                "simulate",
+                "escrow.mori",
+                "--action",
+                "deliver",
             ])
             .args(args)
             .output()
===== END FILE source/formatting/FORMATTING.diff =====

===== BEGIN FILE source/independent-oracle/escrow.program.expected.json | bytes=1765 | sha256=595abd45404c509e3b9300c39ed6ef2c5a409315a0f88aa4771a33a6b8df47c1 | terminalLF=True =====
{
  "actions": [
    {
      "inputs": [
        {
          "name": "amount",
          "ty": "UInt"
        }
      ],
      "name": "deliver",
      "statements": [
        {
          "amount": 0,
          "asset": "PAY",
          "from": "alice",
          "kind": "Transfer",
          "to": "escrow"
        },
        {
          "amount": 1,
          "asset": "PAY",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "release"
        },
        {
          "binders": [
            {
              "name": "ready",
              "ty": "Bool"
            }
          ],
          "guard": 2,
          "kind": "Await",
          "label": "delivery"
        },
        {
          "amount": 3,
          "asset": "PAY",
          "from": "escrow",
          "kind": "Transfer",
          "to": "bob"
        },
        {
          "amount": 4,
          "kind": "Discharge",
          "name": "release"
        }
      ]
    }
  ],
  "assets": [
    {
      "identity": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "name": "PAY"
    }
  ],
  "expressions": [
    {
      "kind": "Name",
      "name": "amount"
    },
    {
      "kind": "Name",
      "name": "amount"
    },
    {
      "kind": "Name",
      "name": "ready"
    },
    {
      "kind": "Name",
      "name": "amount"
    },
    {
      "kind": "Name",
      "name": "amount"
    }
  ],
  "language": "moriarty/2",
  "name": "ContinuationEscrowOracle",
  "semantics": "moriarty-core/2",
  "states": [
    {
      "initial": {
        "type": "UInt",
        "value": "77"
      },
      "name": "marker",
      "ty": "UInt"
    }
  ]
}
===== END FILE source/independent-oracle/escrow.program.expected.json =====

===== BEGIN FILE source/independent-oracle/escrow.plan.expected.json | bytes=786 | sha256=fe8463054c617311e270b1ba0e904350a84899158e550260b39cef174e0305c6 | terminalLF=True =====
{
  "actions": [
    {
      "action": "deliver",
      "committed_before": [
        "0",
        "5"
      ],
      "plan_total": "11",
      "segment_cost": [
        "5",
        "6"
      ],
      "sites": [
        {
          "binders": [
            {
              "name": "ready",
              "ty": "Bool"
            }
          ],
          "captures": [
            {
              "name": "amount",
              "ordinal": 0,
              "origin": "Input",
              "ty": "UInt"
            }
          ],
          "label": "delivery",
          "ordinal": 2,
          "site": 1
        }
      ],
      "statement_count": 5
    }
  ],
  "format": "moriarty-stage-plan/1",
  "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e"
}
===== END FILE source/independent-oracle/escrow.plan.expected.json =====

===== BEGIN FILE source/independent-oracle/start.report.expected.json | bytes=11376 | sha256=affd4b4523dd4bcc865ae2cb97e80e8ee592f77a463fcef1a0a8d7d7cc50182c | terminalLF=True =====
{
  "advanced": true,
  "attempted": "5",
  "authentication": "none",
  "committed": "5",
  "consumption": "untracked",
  "continuation": {
    "action": "deliver",
    "boundary": {
      "balances": [
        {
          "account": "alice",
          "amount": "90",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "42",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "release"
        }
      ],
      "state": {
        "marker": {
          "type": "UInt",
          "value": "77"
        }
      }
    },
    "captures": [
      {
        "name": "amount",
        "value": {
          "type": "UInt",
          "value": "10"
        }
      }
    ],
    "format": "moriarty-continuation/1",
    "logical": {
      "cap": "11",
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25",
    "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "release"
    }
  ],
  "segment": {
    "action": "deliver",
    "execution": {
      "action": "deliver",
      "effects": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "from": "alice",
          "kind": "Transfer",
          "to": "escrow"
        },
        {
          "after": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "0",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "release"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "pre": {
        "balances": [
          {
            "account": "alice",
            "amount": "100",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "semantics": "moriarty-core/2",
      "steps": [
        {
          "effects": [
            {
              "amount": "10",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "from": "alice",
              "kind": "Transfer",
              "to": "escrow"
            }
          ],
          "index": 0,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 0,
            "asset": "PAY",
            "from": "alice",
            "kind": "Transfer",
            "to": "escrow"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        },
        {
          "effects": [
            {
              "after": "10",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "0",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "release"
            }
          ],
          "index": 1,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 1,
            "asset": "PAY",
            "creditor": "bob",
            "debtor": "escrow",
            "kind": "Liability",
            "name": "release"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ]
    },
    "frame_post": {
      "captures": [
        {
          "name": "amount",
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ],
      "site": 1
    },
    "frame_pre": {
      "captures": [],
      "site": 0
    }
  },
  "status": "waiting"
}
===== END FILE source/independent-oracle/start.report.expected.json =====

===== BEGIN FILE source/independent-oracle/start.stdout.expected.json | bytes=11376 | sha256=affd4b4523dd4bcc865ae2cb97e80e8ee592f77a463fcef1a0a8d7d7cc50182c | terminalLF=True =====
{
  "advanced": true,
  "attempted": "5",
  "authentication": "none",
  "committed": "5",
  "consumption": "untracked",
  "continuation": {
    "action": "deliver",
    "boundary": {
      "balances": [
        {
          "account": "alice",
          "amount": "90",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "42",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "release"
        }
      ],
      "state": {
        "marker": {
          "type": "UInt",
          "value": "77"
        }
      }
    },
    "captures": [
      {
        "name": "amount",
        "value": {
          "type": "UInt",
          "value": "10"
        }
      }
    ],
    "format": "moriarty-continuation/1",
    "logical": {
      "cap": "11",
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25",
    "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "release"
    }
  ],
  "segment": {
    "action": "deliver",
    "execution": {
      "action": "deliver",
      "effects": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "from": "alice",
          "kind": "Transfer",
          "to": "escrow"
        },
        {
          "after": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "0",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "release"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "pre": {
        "balances": [
          {
            "account": "alice",
            "amount": "100",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "semantics": "moriarty-core/2",
      "steps": [
        {
          "effects": [
            {
              "amount": "10",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "from": "alice",
              "kind": "Transfer",
              "to": "escrow"
            }
          ],
          "index": 0,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 0,
            "asset": "PAY",
            "from": "alice",
            "kind": "Transfer",
            "to": "escrow"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        },
        {
          "effects": [
            {
              "after": "10",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "0",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "release"
            }
          ],
          "index": 1,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 1,
            "asset": "PAY",
            "creditor": "bob",
            "debtor": "escrow",
            "kind": "Liability",
            "name": "release"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ]
    },
    "frame_post": {
      "captures": [
        {
          "name": "amount",
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ],
      "site": 1
    },
    "frame_pre": {
      "captures": [],
      "site": 0
    }
  },
  "status": "waiting"
}
===== END FILE source/independent-oracle/start.stdout.expected.json =====

===== BEGIN FILE source/independent-oracle/delivery-false.report.expected.json | bytes=2524 | sha256=15188506d4800b167b93c550b12630312e48dd80db52f4b2fe66b2101ccd8d3b | terminalLF=True =====
{
  "advanced": false,
  "attempted": "2",
  "authentication": "none",
  "committed": "5",
  "consumption": "untracked",
  "continuation": {
    "action": "deliver",
    "boundary": {
      "balances": [
        {
          "account": "alice",
          "amount": "90",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "42",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "release"
        }
      ],
      "state": {
        "marker": {
          "type": "UInt",
          "value": "77"
        }
      }
    },
    "captures": [
      {
        "name": "amount",
        "value": {
          "type": "UInt",
          "value": "10"
        }
      }
    ],
    "format": "moriarty-continuation/1",
    "logical": {
      "cap": "11",
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25",
    "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "release"
    }
  ],
  "segment": null,
  "status": "waiting"
}
===== END FILE source/independent-oracle/delivery-false.report.expected.json =====

===== BEGIN FILE source/independent-oracle/delivery-false.stdout.expected.json | bytes=2524 | sha256=15188506d4800b167b93c550b12630312e48dd80db52f4b2fe66b2101ccd8d3b | terminalLF=True =====
{
  "advanced": false,
  "attempted": "2",
  "authentication": "none",
  "committed": "5",
  "consumption": "untracked",
  "continuation": {
    "action": "deliver",
    "boundary": {
      "balances": [
        {
          "account": "alice",
          "amount": "90",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "bob",
          "amount": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "escrow",
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        },
        {
          "account": "observer",
          "amount": "42",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          }
        }
      ],
      "liabilities": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "creditor": "bob",
          "debtor": "escrow",
          "name": "release"
        }
      ],
      "state": {
        "marker": {
          "type": "UInt",
          "value": "77"
        }
      }
    },
    "captures": [
      {
        "name": "amount",
        "value": {
          "type": "UInt",
          "value": "10"
        }
      }
    ],
    "format": "moriarty-continuation/1",
    "logical": {
      "cap": "11",
      "committed": "5",
      "profile": "moriarty-reference-logical-cost/2"
    },
    "plan_hash": "0405200b4e9758ffdd0daede5db60ccb0698f504f7f17b1531450fc99c19ce25",
    "program_hash": "511bcac385cba4fc9b2452b71013a0cfdaa507661684c8f357a7dc647368465e",
    "site": 1
  },
  "qualification": "local-reference-stage",
  "residual": [
    {
      "amount": "10",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      },
      "creditor": "bob",
      "debtor": "escrow",
      "name": "release"
    }
  ],
  "segment": null,
  "status": "waiting"
}
===== END FILE source/independent-oracle/delivery-false.stdout.expected.json =====

===== BEGIN FILE source/independent-oracle/delivery-true.report.expected.json | bytes=11980 | sha256=72c8938500f666de59da2dbceff22a86c943614cb34dffd47be17ae1a3e943ad | terminalLF=True =====
{
  "attempted": "6",
  "authentication": "none",
  "committed": "11",
  "consumption": "untracked",
  "qualification": "local-reference-stage",
  "residual": [],
  "segment": {
    "action": "deliver",
    "execution": {
      "action": "deliver",
      "effects": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "from": "escrow",
          "kind": "Transfer",
          "to": "bob"
        },
        {
          "after": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "10",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "release"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "pre": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "semantics": "moriarty-core/2",
      "steps": [
        {
          "effects": [],
          "index": 2,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "binders": [
              {
                "name": "ready",
                "ty": "Bool"
              }
            ],
            "guard": 2,
            "kind": "Await",
            "label": "delivery"
          },
          "value": {
            "type": "Bool",
            "value": true
          }
        },
        {
          "effects": [
            {
              "amount": "10",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "from": "escrow",
              "kind": "Transfer",
              "to": "bob"
            }
          ],
          "index": 3,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 3,
            "asset": "PAY",
            "from": "escrow",
            "kind": "Transfer",
            "to": "bob"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        },
        {
          "effects": [
            {
              "after": "0",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "10",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "release"
            }
          ],
          "index": 4,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 4,
            "kind": "Discharge",
            "name": "release"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ]
    },
    "frame_post": {
      "captures": [],
      "site": 0
    },
    "frame_pre": {
      "captures": [
        {
          "name": "amount",
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ],
      "site": 1
    }
  },
  "status": "complete"
}
===== END FILE source/independent-oracle/delivery-true.report.expected.json =====

===== BEGIN FILE source/independent-oracle/delivery-true.stdout.expected.json | bytes=11980 | sha256=72c8938500f666de59da2dbceff22a86c943614cb34dffd47be17ae1a3e943ad | terminalLF=True =====
{
  "attempted": "6",
  "authentication": "none",
  "committed": "11",
  "consumption": "untracked",
  "qualification": "local-reference-stage",
  "residual": [],
  "segment": {
    "action": "deliver",
    "execution": {
      "action": "deliver",
      "effects": [
        {
          "amount": "10",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "from": "escrow",
          "kind": "Transfer",
          "to": "bob"
        },
        {
          "after": "0",
          "asset": {
            "issuer": "continuation-escrow-oracle",
            "network": "local",
            "reference": "PAY",
            "scale": 0
          },
          "before": "10",
          "creditor": "bob",
          "debtor": "escrow",
          "kind": "Liability",
          "name": "release"
        }
      ],
      "post": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "pre": {
        "balances": [
          {
            "account": "alice",
            "amount": "90",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "bob",
            "amount": "0",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "escrow",
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          },
          {
            "account": "observer",
            "amount": "42",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            }
          }
        ],
        "liabilities": [
          {
            "amount": "10",
            "asset": {
              "issuer": "continuation-escrow-oracle",
              "network": "local",
              "reference": "PAY",
              "scale": 0
            },
            "creditor": "bob",
            "debtor": "escrow",
            "name": "release"
          }
        ],
        "state": {
          "marker": {
            "type": "UInt",
            "value": "77"
          }
        }
      },
      "semantics": "moriarty-core/2",
      "steps": [
        {
          "effects": [],
          "index": 2,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "binders": [
              {
                "name": "ready",
                "ty": "Bool"
              }
            ],
            "guard": 2,
            "kind": "Await",
            "label": "delivery"
          },
          "value": {
            "type": "Bool",
            "value": true
          }
        },
        {
          "effects": [
            {
              "amount": "10",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "from": "escrow",
              "kind": "Transfer",
              "to": "bob"
            }
          ],
          "index": 3,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 3,
            "asset": "PAY",
            "from": "escrow",
            "kind": "Transfer",
            "to": "bob"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        },
        {
          "effects": [
            {
              "after": "0",
              "asset": {
                "issuer": "continuation-escrow-oracle",
                "network": "local",
                "reference": "PAY",
                "scale": 0
              },
              "before": "10",
              "creditor": "bob",
              "debtor": "escrow",
              "kind": "Liability",
              "name": "release"
            }
          ],
          "index": 4,
          "snapshot": {
            "balances": [
              {
                "account": "alice",
                "amount": "90",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "bob",
                "amount": "10",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "escrow",
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              },
              {
                "account": "observer",
                "amount": "42",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                }
              }
            ],
            "liabilities": [
              {
                "amount": "0",
                "asset": {
                  "issuer": "continuation-escrow-oracle",
                  "network": "local",
                  "reference": "PAY",
                  "scale": 0
                },
                "creditor": "bob",
                "debtor": "escrow",
                "name": "release"
              }
            ],
            "state": {
              "marker": {
                "type": "UInt",
                "value": "77"
              }
            }
          },
          "statement": {
            "amount": 4,
            "kind": "Discharge",
            "name": "release"
          },
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ]
    },
    "frame_post": {
      "captures": [],
      "site": 0
    },
    "frame_pre": {
      "captures": [
        {
          "name": "amount",
          "value": {
            "type": "UInt",
            "value": "10"
          }
        }
      ],
      "site": 1
    }
  },
  "status": "complete"
}
===== END FILE source/independent-oracle/delivery-true.stdout.expected.json =====

===== BEGIN FILE actual/attempt-1/START.json | bytes=1787 | sha256=3a1cca18f37d575ef7bd042434a55974ce3c3776bd4375d71b177075b4165ff5 | terminalLF=True =====
{
  "startedAt": "2026-10-18T09:13:43.975218+00:00",
  "sourceCommit": "718b1b020718c6b23a12846d32d5d2db9f14de7c",
  "sourceTree": "1e3fc1912705ce349353accaee7924644e684012",
  "sourceStatus": "",
  "sourcePinsVerified": 437,
  "toolPinsVerified": 4,
  "ancestorConfigsAbsent": 14,
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
    "--test",
    "core2_escrow_complete",
    "--",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-core2-complete-check-6k4n0364/cwd",
  "ownedTemporaryRoot": "/tmp/doctorwatson-core2-complete-check-6k4n0364",
  "childEnvironment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-core2-complete-check-6k4n0364/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-core2-complete-check-6k4n0364/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-core2-complete-check-6k4n0364/tmp"
  },
  "attempt": 1,
  "attemptLimit": 1,
  "nativeAuthority": 0,
  "scope": "3 genuine finite Core2 complete positive consumer predicates only"
}
===== END FILE actual/attempt-1/START.json =====

===== BEGIN FILE actual/attempt-1/CHILD.json | bytes=71 | sha256=d91023f26f4aaadd5536f1239ee5c76e95083550b87db176e61389718c227331 | terminalLF=True =====
{
  "pid": 518521,
  "startedAt": "2026-10-18T09:13:43.975718+00:00"
}
===== END FILE actual/attempt-1/CHILD.json =====

===== BEGIN FILE actual/attempt-1/RESULT.json | bytes=494 | sha256=cc1721925212daad6a229afef537946c3126b6b480b75bd44214b9365cfe5a2e | terminalLF=True =====
{
  "finishedAt": "2026-10-18T09:14:38.680122+00:00",
  "exitCode": 0,
  "wallSeconds": 54.704761924000195,
  "stdoutSha256": "9b0d5dd6a1ebd53335cc6c5c9585bf338c3a9f83a52e61b62438d13b2a45d811",
  "stderrSha256": "06b46407a9b3856d40a5e6b76dfc5aa0bc8f9a84589aa89b9ea199e4a84849c8",
  "sourceCommitAfter": "718b1b020718c6b23a12846d32d5d2db9f14de7c",
  "sourceTreeAfter": "1e3fc1912705ce349353accaee7924644e684012",
  "sourceStatusAfter": "",
  "allocationConsumed": true,
  "nativeAuthority": 0
}
===== END FILE actual/attempt-1/RESULT.json =====

===== BEGIN FILE actual/attempt-1/CLEANUP.json | bytes=178 | sha256=ba96bf10c67f699f89ecbc0fa8083a1257318df43a7cad5ecd79df933aeb4f62 | terminalLF=True =====
{
  "finishedAt": "2026-10-18T09:14:38.777578+00:00",
  "path": "/tmp/doctorwatson-core2-complete-check-6k4n0364",
  "existsAfter": false,
  "wallSeconds": 0.04397254499781411
}
===== END FILE actual/attempt-1/CLEANUP.json =====

===== BEGIN FILE actual/attempt-1/STDOUT.log | bytes=360 | sha256=9b0d5dd6a1ebd53335cc6c5c9585bf338c3a9f83a52e61b62438d13b2a45d811 | terminalLF=True =====

running 3 tests
test escrow_cli_start_false_observation_and_delivery_match_complete_public_bytes ... ok
test escrow_library_start_false_observation_and_delivery_match_complete_reports ... ok
test escrow_public_load_matches_complete_program_and_filled_plan ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

===== END FILE actual/attempt-1/STDOUT.log =====

===== BEGIN FILE actual/attempt-1/STDERR.log | bytes=5052 | sha256=06b46407a9b3856d40a5e6b76dfc5aa0bc8f9a84589aa89b9ea199e4a84849c8 | terminalLF=True =====
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
   Compiling litemap v0.8.3
   Compiling icu_locale_fallback_data v2.3.0
   Compiling rustix v1.1.5
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling subtle v2.6.1
   Compiling linux-raw-sys v0.12.1
   Compiling const-oid v0.9.6
   Compiling icu_segmenter_data v2.3.0
   Compiling bitflags v2.13.2
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
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling getrandom v0.2.17
   Compiling owo-colors v4.4.0
   Compiling supports-unicode v3.0.0
   Compiling unicode-width v0.1.14
   Compiling supports-hyperlinks v3.2.0
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
   Compiling base16ct v0.2.0
   Compiling crossbeam-deque v0.8.8
   Compiling serde_core v1.0.229
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling once_cell v1.21.4
   Compiling utf8parse v0.2.2
   Compiling zmij v1.0.23
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling serde v1.0.229
   Compiling anstyle-query v1.1.5
   Compiling const-oid v0.10.2
   Compiling cpufeatures v0.2.17
   Compiling thiserror v2.0.21
   Compiling getrandom v0.4.3
   Compiling serde_json v1.0.151
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstyle v1.0.14
   Compiling rayon-core v1.13.0
   Compiling regex-syntax v0.8.11
   Compiling colorchoice v1.0.5
   Compiling anstream v1.0.0
   Compiling regex-automata v0.4.18
   Compiling sha2 v0.10.9
   Compiling digest v0.11.3
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling thiserror-impl v2.0.21
   Compiling serde_derive v1.0.229
   Compiling similar v2.7.0
   Compiling same-file v1.0.6
   Compiling cpufeatures v0.3.1
   Compiling log v0.4.34
   Compiling strsim v0.11.1
   Compiling itoa v1.0.18
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling globset v0.4.20
   Compiling sha2 v0.11.0
   Compiling walkdir v2.5.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-parser)
   Compiling fastrand v2.5.0
   Compiling either v1.19.0
   Compiling rayon v1.12.0
   Compiling tempfile v3.27.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 54.66s
     Running tests/core2_escrow_complete.rs (/tmp/doctorwatson-core2-complete-check-6k4n0364/target/debug/deps/core2_escrow_complete-71cb8ac1e633176c)
===== END FILE actual/attempt-1/STDERR.log =====

END PRIMARY EVIDENCE. Judge the complete finite actual-result predicate independently. Preserve all limitations. Return the concise whole JSON now; no further commands or execution.
