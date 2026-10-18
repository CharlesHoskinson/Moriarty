## Complete file: REVIEW-SCOPE.md
```text
# Exact Source6 capped-remittance source/result review

One independent ordinary nonsecurity review context. Review all five candidate paths and actual scoped results against independently frozen expectations. Do not author code, execute tests or commands, inspect provider/private files, browse, delegate, install, run SDK/model/circuit/native/key/proof/wallet/ledger operations, or supply security judgments. File reads within this packet are allowed.

Strict candidate: base e4d7a2b5709caa18a94f5256d03a0302364b8235, head 34257f3ef95e7a1bf29031417e7f6d33f708f06a, complete tree f529a1924885019aa884bc6447f8d47fff47526f. The Git ref named main is stale; this full base commit is authoritative. Five changed paths only. No approval for hypothetical future bases or trees. Root alone integrates after checking actual base==e4d7a2b5709caa18a94f5256d03a0302364b8235, candidate head==34257f3ef95e7a1bf29031417e7f6d33f708f06a, exact composed tree==f529a1924885019aa884bc6447f8d47fff47526f, all affected inputs unchanged. If any guard differs, fresh review is required. Root-formatted test bytes equal original candidate bytes; FORMATTING.diff is empty.

The captured check is terminal exit0, one selected test PASS and seven filtered, wall53.494262769 seconds. This one regression actually invokes three public CLI commands: check plus pay_tight/pay_loose raw Source6 expansions. Native features OFF via --no-default-features --features source6-expansion. This observes Rust checked source and literal Source6 text only. The new actions' actual TypeScript parser/lowerer/consumer remain UNRUN. Existing older TypeScript results do not establish these new actions. Do not turn this pass into full Source6 grammar, Program, Target, Artifact, financial execution, work, hash, native, proof, ledger or target qualification. All original obligations remain open.

The separate preexecution independent oracle FREEZE.sha256 digest is4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79,23 verified members; frozen11:43:30 UTC before plan11:44:22 and execution. Its provenance discloses previous Source6 exposure. It independently derives normalized values, semantic statements, five ordered effects and conditional postframes for both min branches, zero fee and three opening cells. It is not a wire/hash/work/qualification golden, and raw presentation fixtures are separately hand-authored. Assess those limits and the financial arithmetic/statement order without inventing a security test or security verdict.

Physical coverage: candidate/ contains complete exact-current five files; actual-source/ has complete current checker (including min implementation), lexer/parser/AST, Source6 emitter, public CLI/check/expand dispatch and test harness, TS frontend and other available source. actual-check/ has entire captured source pins, complete STDOUT/STDERR, start/result, plan, cleanup and run_actual.py. independent-oracle/ contains entire unmodified oracle. original-source-proposal/ preserves whole original proposal/snapshots and its source-only UNRUN-era records as history, not current execution. PRIMARY.md inlines the full five candidate files, complete expected/provenance oracle root files, complete actual emitter+CLI expansion consumer, full logs/plan/cleanup and execution harness. Other source is supplied physically, not claimed inline. Read what is needed and enumerate actual read coverage honestly. Report unavailable files or required source explicitly rather than assuming inspection.

Requested model gemini-4-argon-high, HIGH effort through AGY. Actual model identity remains unattested unless the provider envelope independently identifies it; do not treat prompt names as attestation. Return exactly one complete object conforming to the supplied schema. Preserve findings/dissent and limitations; no truncated previews, partial findings, spliced votes or multiple answer variants. A single provider context counts once. You may approve-exact-source-result-and-guarded-merge only for the strict base/head/tree above; otherwise requires-fix or refuse-scope. Explicitly distinguish implementation, observed result and acceptance.

Physical missing requested source paths (do not claim read): ["crates/mori/src/commands/source.rs"].

```

## Complete file: candidate/crates/mori/tests/fixtures/source6-expansion/capped-loose.source6
```text
profile "moriarty-financial-agreement-source/6";
agreement CappedRemittance {
domain Midnight;
settlement A scale 2;
selected TransferLiteralFee source_hash "srcC" digest "policyC";
intent {
signer Owner key "keyC";
nonce "nC";
pre_head "c0";
valid 2..20;
gross_cap 225;
fee_cap 0;
net_floor 225;
failure success_only;
signed_action transfer from Owner to Recipient fee_to Fee value 225 fee 0;
observations empty;
disclosures empty;
retained_effects empty;
retained_duties empty;
delegation none;
recovery none;
}
authenticated {
head "c0";
predecessor "prior";
round 4;
balance Owner 1000;
balance Recipient 5;
balance Fee 7;
allowance Owner remaining 1000 spent 25;
replay unused;
work_remaining 12;
work_spent 2;
}
submit transfer from Owner to Recipient fee_to Fee value 225 fee 0;
effects {
debit Owner 225;
credit Recipient 225;
use_allowance Owner 225;
use_replay "nC";
advance_head "c0" "c1";
}
post_head "c1";
}

```

## Complete file: candidate/crates/mori/tests/fixtures/source6-expansion/capped-remittance.mori
```text
profile "moriarty-beta/1";
agreement CappedRemittance {
 domain Local = { id: "Midnight", chain: "midnight", network: "preview" };
 account Buyer = { domain: Local, id: "Owner" };
 account Seller = { domain: Local, id: "Recipient" };
 account Treasury = { domain: Local, id: "Fee" };
 asset USD = { domain: Local, id: "A", scale: 2, representation: "canonical" };
 const unitPrice: Qty<USD> = 0.75 USD;
 const itemCount: Scalar = 3;
 const quoted = unitPrice * itemCount;
 const tightLimit = 2.10 USD;
 const looseLimit = 2.50 USD;
 const tightAmount = min(a: quoted, b: tightLimit);
 const looseAmount = min(a: quoted, b: looseLimit);
 const noCharge = atoms(asset: USD, value: 0);
 intent TightPayment = {
  domain: Local, asset: USD, signer: Buyer, key: "keyC", nonce: "nC", pre_head: "c0",
  valid: rounds(domain: Local, from: 2, to: 20), gross_cap: tightAmount + noCharge, fee_cap: noCharge, net_floor: tightAmount,
  operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: tightAmount, fee: noCharge),
  source_hash: "srcC", policy_digest: "policyC", failure: SuccessOnly,
  observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None,
 };
 intent LoosePayment = {
  domain: Local, asset: USD, signer: Buyer, key: "keyC", nonce: "nC", pre_head: "c0",
  valid: rounds(domain: Local, from: 2, to: 20), gross_cap: looseAmount + noCharge, fee_cap: noCharge, net_floor: looseAmount,
  operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: looseAmount, fee: noCharge),
  source_hash: "srcC", policy_digest: "policyC", failure: SuccessOnly,
  observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None,
 };
 action pay_tight uses TightPayment;
 action pay_loose uses LoosePayment;
}

```

## Complete file: candidate/crates/mori/tests/fixtures/source6-expansion/capped-remittance.scenario.json
```text
{
  "profile": "moriarty-local-scenario/1",
  "kind": "local-stipulation",
  "domain": "Midnight",
  "asset": "A",
  "head": "c0",
  "predecessor": "prior",
  "round": "4",
  "balances": [
    { "account": "Owner", "amount": "1000" },
    { "account": "Recipient", "amount": "5" },
    { "account": "Fee", "amount": "7" }
  ],
  "allowance": { "owner": "Owner", "remaining": "1000", "spent": "25" },
  "replay": "unused",
  "work_remaining": "12",
  "work_spent": "2",
  "post_head": "c1"
}

```

## Complete file: candidate/crates/mori/tests/fixtures/source6-expansion/capped-tight.source6
```text
profile "moriarty-financial-agreement-source/6";
agreement CappedRemittance {
domain Midnight;
settlement A scale 2;
selected TransferLiteralFee source_hash "srcC" digest "policyC";
intent {
signer Owner key "keyC";
nonce "nC";
pre_head "c0";
valid 2..20;
gross_cap 210;
fee_cap 0;
net_floor 210;
failure success_only;
signed_action transfer from Owner to Recipient fee_to Fee value 210 fee 0;
observations empty;
disclosures empty;
retained_effects empty;
retained_duties empty;
delegation none;
recovery none;
}
authenticated {
head "c0";
predecessor "prior";
round 4;
balance Owner 1000;
balance Recipient 5;
balance Fee 7;
allowance Owner remaining 1000 spent 25;
replay unused;
work_remaining 12;
work_spent 2;
}
submit transfer from Owner to Recipient fee_to Fee value 210 fee 0;
effects {
debit Owner 210;
credit Recipient 210;
use_allowance Owner 210;
use_replay "nC";
advance_head "c0" "c1";
}
post_head "c1";
}

```

## Complete file: candidate/crates/mori/tests/source6_expand.rs
```text
#![cfg(feature = "source6-expansion")]

use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/source6-expansion/",
            $name
        ))
        .as_slice()
    };
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Files(PathBuf);

impl Files {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-source6-expand-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&root).unwrap();
        fs::write(root.join("invoice.mori"), fixture!("invoice.mori")).unwrap();
        fs::write(
            root.join("scenario.json"),
            fixture!("invoice.scenario.json"),
        )
        .unwrap();
        fs::write(root.join("core.mori"), fixture!("core.mori")).unwrap();
        fs::write(root.join("core.json"), fixture!("core.scenario.json")).unwrap();
        fs::write(root.join("repayment.mori"), fixture!("repayment.mori")).unwrap();
        fs::write(root.join("oracle.mori"), fixture!("oracle.mori")).unwrap();
        Self(root)
    }

    fn invoke(&self, arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .args(["--color", "never"])
            .args(arguments)
            .output()
            .unwrap()
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn source6_arguments() -> [&'static str; 7] {
    [
        "expand",
        "invoice.mori",
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
        "--emit-source6-transfer",
    ]
}

#[test]
fn checked_beta_reaches_real_expansion_library_and_raw_stdout() {
    let files = Files::new();
    let check = files.invoke(&["check", "invoice.mori", "--json"]);
    assert_eq!(check.status.code(), Some(0));
    assert!(check.stderr.is_empty());
    let report: Value = serde_json::from_slice(&check.stdout).unwrap();
    assert_eq!(report["status"], "accepted");
    assert_eq!(report["actions"][0]["name"], "pay");
    assert_eq!(report["actions"][0]["support"], "local_s0");

    let expansion = files.invoke(&source6_arguments());
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert_eq!(expansion.stderr.as_slice(), fixture!("source6.stderr.txt"));
}

#[test]
fn explicit_source6_quiet_output_is_raw_source6_text() {
    let files = Files::new();
    let mut arguments = vec!["--quiet"];
    arguments.extend(source6_arguments());
    let expansion = files.invoke(&arguments);
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert!(expansion.stderr.is_empty());
}

#[test]
fn core_default_retains_complete_program_bytes() {
    let files = Files::new();
    let output = files.invoke(&[
        "expand",
        "core.mori",
        "--action",
        "noop",
        "--scenario",
        "core.json",
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout.as_slice(), fixture!("core.stdout.json"));
    assert!(output.stderr.is_empty());
}

#[test]
fn beta_default_retains_core_parser_refusal_boundary() {
    let files = Files::new();
    let output = files.invoke(&[
        "expand",
        "invoice.mori",
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
    ]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("mori::core::syntax"));
}

#[test]
fn source_action_selection_precedes_scenario_file_read() {
    let files = Files::new();
    for (source, action, expected) in [
        (
            "invoice.mori",
            "absent_action",
            "mori::source6::unknown_action",
        ),
        (
            "repayment.mori",
            "repay_loan",
            "mori::source6::unsupported_operation",
        ),
        (
            "oracle.mori",
            "read_fixing",
            "mori::source6::unsupported_operation",
        ),
    ] {
        let output = files.invoke(&[
            "expand",
            source,
            "--action",
            action,
            "--scenario",
            "absent.json",
            "--emit-source6-transfer",
        ]);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(expected), "{stderr}");
        assert!(!stderr.contains("could not read"));
    }
}

#[test]
fn scenario_profile_and_subset_have_explicit_consumer_diagnostics() {
    let files = Files::new();
    let base: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    let foreign: Value = serde_json::from_slice(fixture!("core.scenario.json")).unwrap();
    let mut with_override = base.clone();
    with_override["candidate_effects"] = serde_json::json!([]);
    let mut with_obligation = base.clone();
    with_obligation["obligation"] = serde_json::json!({});
    let mut missing_cell = base;
    missing_cell["balances"].as_array_mut().unwrap().pop();
    for (scenario, expected) in [
        (foreign, "mori::command::source6-scenario-profile"),
        (with_override, "mori::command::source6-scenario-unsupported"),
        (
            with_obligation,
            "mori::command::source6-scenario-unsupported",
        ),
        (missing_cell, "mori::command::source6-scenario-cells"),
    ] {
        fs::write(
            files.0.join("scenario.json"),
            serde_json::to_vec(&scenario).unwrap(),
        )
        .unwrap();
        let output = files.invoke(&source6_arguments());
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains(expected));
    }
}

#[test]
fn diagnostic_presentation_escapes_filename_action_and_scenario_shape_errors() {
    let files = Files::new();
    let hostile_file = "invoice\u{1b}\u{202e}.mori";
    fs::write(files.0.join(hostile_file), fixture!("invoice.mori")).unwrap();

    let unknown = files.invoke(&[
        "expand",
        hostile_file,
        "--action",
        "pay\n\u{1b}\u{202e}",
        "--scenario",
        "absent.json",
        "--emit-source6-transfer",
    ]);
    assert_eq!(unknown.status.code(), Some(1));
    assert!(unknown.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(stderr.contains("mori::source6::unknown_action"), "{stderr}");
    assert!(stderr.contains(r"invoice\u{1b}\u{202e}.mori"), "{stderr}");
    assert!(
        stderr.contains(r"No checked action named `pay\n\u{1b}\u{202e}` was found."),
        "{stderr}"
    );
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");

    let mut bad_serde: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    bad_serde["replay"] = serde_json::json!("bad\u{1b}\u{202e}");
    fs::write(
        files.0.join("scenario.json"),
        serde_json::to_vec(&bad_serde).unwrap(),
    )
    .unwrap();
    let shape = files.invoke(&source6_arguments());
    assert_eq!(shape.status.code(), Some(1));
    assert!(shape.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&shape.stderr);
    assert!(
        stderr.contains("mori::command::source6-scenario-shape"),
        "{stderr}"
    );
    assert!(stderr.contains(r"bad\u{1b}\u{202e}"), "{stderr}");
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");

    let mut bad_domain: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    bad_domain["domain"] = serde_json::json!("Other");
    fs::write(
        files.0.join("scenario.json"),
        serde_json::to_vec(&bad_domain).unwrap(),
    )
    .unwrap();
    let mismatch = files.invoke(&[
        "expand",
        hostile_file,
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
        "--emit-source6-transfer",
    ]);
    assert_eq!(mismatch.status.code(), Some(1));
    assert!(mismatch.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&mismatch.stderr);
    assert!(stderr.contains("mori::source6::scenario_shape"), "{stderr}");
    assert!(stderr.contains(r"invoice\u{1b}\u{202e}.mori"), "{stderr}");
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");
}

// Public consumer coverage for already admitted nominal arithmetic. Expected
// bytes are hand-derived fixtures, not output from the checker or expander.
#[test]
fn quantity_min_selects_both_operands_through_public_source6_expansion() {
    let files = Files::new();
    fs::write(
        files.0.join("capped.mori"),
        fixture!("capped-remittance.mori"),
    )
    .unwrap();
    fs::write(
        files.0.join("capped.json"),
        fixture!("capped-remittance.scenario.json"),
    )
    .unwrap();

    let checked = files.invoke(&["check", "capped.mori", "--json"]);
    assert_eq!(checked.status.code(), Some(0));
    assert!(checked.stderr.is_empty());
    let report: Value = serde_json::from_slice(&checked.stdout).unwrap();
    assert_eq!(report["status"], "accepted");
    let actions = report["actions"].as_array().unwrap();
    assert_eq!(actions.len(), 2);
    for (action, name) in actions.iter().zip(["pay_tight", "pay_loose"]) {
        assert_eq!(action["name"], name);
        assert_eq!(action["support"], "local_s0");
    }

    // 75 * 3 = 225 atoms. The tight minimum chooses b=210; the loose minimum
    // chooses a=225. The atoms(...,0) fee retains its input cell but adds no
    // fee Credit proposal. Both are complete Source/6 text comparisons.
    for (action, expected) in [
        ("pay_tight", fixture!("capped-tight.source6")),
        ("pay_loose", fixture!("capped-loose.source6")),
    ] {
        let output = files.invoke(&[
            "--quiet",
            "expand",
            "capped.mori",
            "--action",
            action,
            "--scenario",
            "capped.json",
            "--emit-source6-transfer",
        ]);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        assert_eq!(output.stdout.as_slice(), expected);
    }
}

```

## Complete file: independent-oracle/DERIVATION.md
```text
# Independent capped-remittance derivation

The only chosen case data read were capped-remittance.mori and capped-remittance.scenario.json. The new candidate test, capped-tight.source6, capped-loose.source6, INDEPENDENT-SOURCE-SHAPE and actual outputs were not opened, hashed or copied. Normative/reference documents and DTO declarations were pinned directly to74a909d69639df8823a527a4d608a7516ccd69a2 using explicitly authorized read-only Git queries. No producer, parser, checker, evaluator, emitter, compiler, SDK or runtime was called.

USD is a source alias; its economic asset is A in Midnight at scale2. Buyer/Seller/Treasury map to Owner/Recipient/Fee. Exact base10 conversion gives0.75USD=75atoms,2.10USD=210,2.50USD=250. Scalar3 produces75*3=225atoms without rounding. min(225,210)=210 chooses tightLimit(b); min(225,250)=225 chooses quoted(a). atoms(USD,0)=0. Both gross caps are amount+0, fee caps0, and net floors equal amount. All shown intermediates are nonnegative and below UInt128 and the S0 nominal2^127-1 bound; no floatingpoint or product numeric function is involved.

Both actions independently use the same opening frame: Owner1000,Recipient5,Fee7, allowance remaining1000/spent25. Tight debit210 gives Owner790/Recipient215/Fee7, allowance790/235. Loose debit225 gives Owner775/Recipient230/Fee7, allowance775/250. Provided balances total1012 in each arithmetic frame; allowance remaining+spent remains1025. These are arithmetic cross-checks of this finite conditional projection, not a security/conservation proof or observed acceptance. They are alternatives, not sequential transactions.

The Source6 contract mandates Debit(owner,value+fee), Credit(recipient,value), optional Credit(feeRecipient,fee) iff fee>0, UseAllowance(owner,gross), UseReplay(key), AdvanceHead(pre,post). Fee0 omits the optional credit entirely; it does not remove Fee's required third opening row or reset7 to0. Exactly five effects remain. Both signed_action and submit still explicitly say fee_to Fee and fee0. The text nonce is nC; the separately labeled Core5 semantic effect uses the encoded domain/signer/nonce tuple [Midnight,Owner,nC]. AdvanceHead uses currentheadc0->c1, not authenticated.predecessor prior. Round4 is retained, not incremented.

Every Source6 statement/block is frozen in each independent-source6 text and ordered-target-lines file. The typed Source6Ast expectation separately retains agreement CappedRemittance, scale2 and predecessor prior; selected.actionId is the normative profile tag TransferLiteralFee, not pay_tight/pay_loose. The input work12/2 is carried exactly in authenticated fields. Target indentation is independently chosen from the EBNF; exact stdout/whitespace is not asserted. No expected compiler Program/Artifact, hash identity, measured work, gate outcome, qualification, complete Core post or ledger result is supplied; see UNKNOWN.json.

Previous exposure is explicit: this author handled earlier Source6 owner packets with Rust Source6 CLI/library/emitter, lexer/checker source, old invoice/workshop inputs/tests and artifact copies. This is not global pre-implementation blindness. In this lane no producer emission or new authored oracle/test was used. Only DTO declaration ranges were used from the Source6/S0 TS modules; an earlier displayed DTO range incidentally included the adjacent Parser class header/fields and numeric-bound declarations, not parsing/lowering execution or chosen-case results. That incidental source exposure is recorded rather than hidden.

```

## Complete file: independent-oracle/EXPECTED-SEMANTICS.json
```text
{
  "format": "independent-capped-remittance-semantic-oracle/1",
  "status": "FROZEN_BEFORE_PRODUCT_EXECUTION",
  "common": {
    "agreement": "CappedRemittance",
    "sourceProfile": "moriarty-beta/1",
    "source6Profile": "moriarty-financial-agreement-source/6",
    "aliases": {
      "domain": {
        "Local": "Midnight"
      },
      "accounts": {
        "Buyer": "Owner",
        "Seller": "Recipient",
        "Treasury": "Fee"
      },
      "asset": {
        "USD": {
          "id": "A",
          "scale": "2",
          "representation": "canonical",
          "domain": "Midnight"
        }
      }
    },
    "constants": [
      {
        "name": "unitPrice",
        "type": "Qty<USD>",
        "source": "0.75 USD",
        "atoms": "75"
      },
      {
        "name": "itemCount",
        "type": "Scalar",
        "source": "3",
        "value": "3"
      },
      {
        "name": "quoted",
        "type": "Qty<USD>",
        "source": "unitPrice * itemCount",
        "atoms": "225"
      },
      {
        "name": "tightLimit",
        "type": "Qty<USD>",
        "source": "2.10 USD",
        "atoms": "210"
      },
      {
        "name": "looseLimit",
        "type": "Qty<USD>",
        "source": "2.50 USD",
        "atoms": "250"
      },
      {
        "name": "tightAmount",
        "type": "Qty<USD>",
        "source": "min(a: quoted, b: tightLimit)",
        "atoms": "210",
        "selectedArgument": "b"
      },
      {
        "name": "looseAmount",
        "type": "Qty<USD>",
        "source": "min(a: quoted, b: looseLimit)",
        "atoms": "225",
        "selectedArgument": "a"
      },
      {
        "name": "noCharge",
        "type": "Qty<USD>",
        "source": "atoms(asset: USD, value: 0)",
        "atoms": "0"
      }
    ],
    "selectedSource6ActionId": "TransferLiteralFee",
    "sourceHashLiteral": "srcC",
    "policyDigestLiteral": "policyC",
    "literalDigestQualification": "Opaque source claims; neither is a computed hash or verified commitment.",
    "inputScenario": {
      "profile": "moriarty-local-scenario/1",
      "kind": "local-stipulation",
      "domain": "Midnight",
      "asset": "A",
      "head": "c0",
      "predecessor": "prior",
      "round": "4",
      "balances": [
        {
          "account": "Owner",
          "amount": "1000"
        },
        {
          "account": "Recipient",
          "amount": "5"
        },
        {
          "account": "Fee",
          "amount": "7"
        }
      ],
      "allowance": {
        "owner": "Owner",
        "remaining": "1000",
        "spent": "25"
      },
      "replay": "unused",
      "work_remaining": "12",
      "work_spent": "2",
      "post_head": "c1"
    },
    "actionRelationship": "Two independent alternative expansions use the same opening scenario; no sequential execution or shared-nonce acceptance is asserted."
  },
  "cases": [
    {
      "action": "pay_tight",
      "selectedIntent": "TightPayment",
      "selectedMinArgument": "b: tightLimit=210 (225 > 210)",
      "value": "210",
      "fee": "0",
      "gross": "210",
      "grossCap": "210",
      "feeCap": "0",
      "netFloor": "210",
      "openingFinancialFrame": {
        "domain": "Midnight",
        "asset": "A",
        "balances": [
          {
            "account": "Owner",
            "amount": "1000"
          },
          {
            "account": "Recipient",
            "amount": "5"
          },
          {
            "account": "Fee",
            "amount": "7"
          }
        ],
        "allowances": [
          {
            "owner": "Owner",
            "remaining": "1000",
            "spent": "25"
          }
        ]
      },
      "conditionalClosingFinancialProjection": {
        "domain": "Midnight",
        "asset": "A",
        "balances": [
          {
            "account": "Owner",
            "amount": "790"
          },
          {
            "account": "Recipient",
            "amount": "215"
          },
          {
            "account": "Fee",
            "amount": "7"
          }
        ],
        "allowances": [
          {
            "owner": "Owner",
            "remaining": "790",
            "spent": "235"
          }
        ],
        "head": "c1",
        "round": "4",
        "selectedReplay": {
          "tuple": [
            "Midnight",
            "Owner",
            "nC"
          ],
          "encodedKey": "[\"Midnight\",\"Owner\",\"nC\"]",
          "state": "consumed"
        }
      },
      "source6OrderedEffects": [
        {
          "kind": "Debit",
          "account": "Owner",
          "asset": "A",
          "amount": "210"
        },
        {
          "kind": "Credit",
          "account": "Recipient",
          "asset": "A",
          "amount": "210"
        },
        {
          "kind": "UseAllowance",
          "owner": "Owner",
          "amount": "210"
        },
        {
          "kind": "UseReplay",
          "key": "nC"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "c0",
          "successor": "c1"
        }
      ],
      "core5SemanticOrderedEffects": [
        {
          "kind": "Debit",
          "account": "Owner",
          "asset": "A",
          "amount": "210"
        },
        {
          "kind": "Credit",
          "account": "Recipient",
          "asset": "A",
          "amount": "210"
        },
        {
          "kind": "UseAllowance",
          "owner": "Owner",
          "amount": "210"
        },
        {
          "kind": "UseReplay",
          "key": "[\"Midnight\",\"Owner\",\"nC\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "c0",
          "successor": "c1"
        }
      ],
      "effectCount": 5,
      "feeCreditOmitted": true,
      "feeOpeningRowRequiredAndRetained": {
        "account": "Fee",
        "before": "7",
        "after": "7"
      },
      "source6InputWork": {
        "remaining": "12",
        "spent": "2"
      },
      "closingProjectionQualification": "Conditional arithmetic consequence of the normative transfer vector only; not a complete Core post, acceptance/PreparedUnqualified result, authenticated state or ledger observation. Work fields intentionally absent.",
      "source6AstFile": "pay_tight.source6.ast.expected.json",
      "orderedTargetLinesFile": "pay_tight.ordered-target-lines.expected.json",
      "independentSource6File": "pay_tight.independent-source6.txt"
    },
    {
      "action": "pay_loose",
      "selectedIntent": "LoosePayment",
      "selectedMinArgument": "a: quoted=225 (225 < 250)",
      "value": "225",
      "fee": "0",
      "gross": "225",
      "grossCap": "225",
      "feeCap": "0",
      "netFloor": "225",
      "openingFinancialFrame": {
        "domain": "Midnight",
        "asset": "A",
        "balances": [
          {
            "account": "Owner",
            "amount": "1000"
          },
          {
            "account": "Recipient",
            "amount": "5"
          },
          {
            "account": "Fee",
            "amount": "7"
          }
        ],
        "allowances": [
          {
            "owner": "Owner",
            "remaining": "1000",
            "spent": "25"
          }
        ]
      },
      "conditionalClosingFinancialProjection": {
        "domain": "Midnight",
        "asset": "A",
        "balances": [
          {
            "account": "Owner",
            "amount": "775"
          },
          {
            "account": "Recipient",
            "amount": "230"
          },
          {
            "account": "Fee",
            "amount": "7"
          }
        ],
        "allowances": [
          {
            "owner": "Owner",
            "remaining": "775",
            "spent": "250"
          }
        ],
        "head": "c1",
        "round": "4",
        "selectedReplay": {
          "tuple": [
            "Midnight",
            "Owner",
            "nC"
          ],
          "encodedKey": "[\"Midnight\",\"Owner\",\"nC\"]",
          "state": "consumed"
        }
      },
      "source6OrderedEffects": [
        {
          "kind": "Debit",
          "account": "Owner",
          "asset": "A",
          "amount": "225"
        },
        {
          "kind": "Credit",
          "account": "Recipient",
          "asset": "A",
          "amount": "225"
        },
        {
          "kind": "UseAllowance",
          "owner": "Owner",
          "amount": "225"
        },
        {
          "kind": "UseReplay",
          "key": "nC"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "c0",
          "successor": "c1"
        }
      ],
      "core5SemanticOrderedEffects": [
        {
          "kind": "Debit",
          "account": "Owner",
          "asset": "A",
          "amount": "225"
        },
        {
          "kind": "Credit",
          "account": "Recipient",
          "asset": "A",
          "amount": "225"
        },
        {
          "kind": "UseAllowance",
          "owner": "Owner",
          "amount": "225"
        },
        {
          "kind": "UseReplay",
          "key": "[\"Midnight\",\"Owner\",\"nC\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "c0",
          "successor": "c1"
        }
      ],
      "effectCount": 5,
      "feeCreditOmitted": true,
      "feeOpeningRowRequiredAndRetained": {
        "account": "Fee",
        "before": "7",
        "after": "7"
      },
      "source6InputWork": {
        "remaining": "12",
        "spent": "2"
      },
      "closingProjectionQualification": "Conditional arithmetic consequence of the normative transfer vector only; not a complete Core post, acceptance/PreparedUnqualified result, authenticated state or ledger observation. Work fields intentionally absent.",
      "source6AstFile": "pay_loose.source6.ast.expected.json",
      "orderedTargetLinesFile": "pay_loose.ordered-target-lines.expected.json",
      "independentSource6File": "pay_loose.independent-source6.txt"
    }
  ]
}

```

## Complete file: independent-oracle/FREEZE.sha256
```text
a2d8e27020ed2417db1c56bc170e9ecc6d01933f0817471993420f41c558be08  DERIVATION.md
da997f7f4f59300007cb0368b2d96b728049623315e6649e1bf9c6cccf6fd624  EXPECTED-SEMANTICS.json
c911856c73f9970bc87187ad132412b1f142c4721c2891cb5dcfe1cb5758b5d1  MANIFEST.json
857d414b460efab93b8f45361d5f9ad5932934f8e24b8bb73d9b6079128e076d  PROVENANCE.json
ee589b6e28b71154703ed26e00a845d96d52279f8663a0b13fd397a2c8cc736f  README.md
36eadfded90b0897b76f10da071df8e4d68b881e9fa3aca47809e54033469a74  STATIC-CHECKS.json
de53b22a57d0f27b2d07531375adef006309eb4a2d930933f1dfac532515a8a8  UNKNOWN.json
78b827813bd9d45c82f3ea48fb2752f2c05915df7cdce309cb38b49b68b2226f  basis/docs/BETA-SOURCE6-EXPANSION.md
b79c6c74d1ce2e2761ded415bf8a5f65db173e01570f3dd6129717445fbb08a0  basis/dto/financial-agreement-source-v6-frontend.ts.declarations.txt
c9de1e005f16744cd4715907c9e8f871f837d0edced3a6ee66992a004f03571c  basis/dto/mil4-s0-core-v5.ts.declarations.txt
5fb03d51d863e5c959e1cb80f67da9d261c668200837bfab4d748cda0403f72e  basis/experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf
725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec  basis/experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md
88fa3353d1b1a889f1d06f8b35868ae187800719c67ab8345ebb09515e8de639  basis/experiments/moriarty-language/spec/successor/lexical.md
7bab6fbc179626977f5836e1389685cf070c752cebcd003824e579c31ad9f706  basis/packages/moriarty-beta/GETTING-STARTED.md
a12bee5aef1b1de2559a5da85ef0937e822af72ee530275606b427944a8d9c4d  basis/packages/moriarty-beta/README.md
898ffbec44b4165f1afb298398fe36fccc7459e2c1ff92ca7be3ea61fb7c043c  inputs/capped-remittance.mori
6c8c1548cbbc52eca5584f16d4e6e873b245841e6697ad816d22afe73b361dfa  inputs/capped-remittance.scenario.json
3913534693475e0132d4c5a68b7eef31d1d55c0bb898618ef1c39ad1f38d42d8  pay_loose.independent-source6.txt
05008475f2cabbec049350114110f031782bf600544bec64c9b4219123d2f42c  pay_loose.ordered-target-lines.expected.json
d84b4675121518394d8e292aedfcbe417534d662b1fd72a4bca26eb9c620e684  pay_loose.source6.ast.expected.json
cc616f8ed536ef03871f572743b6741d22debc39752b1e288515d9a22e992cd2  pay_tight.independent-source6.txt
c0a88fe3a8d647bb46f95a37063f0ca13d138b4bcf830c9480c4fcb1bfe7793c  pay_tight.ordered-target-lines.expected.json
7e6a072c7bfc19eb1e631f84ff688b1197191f77328afb9cee6ecac2a2bc3835  pay_tight.source6.ast.expected.json

```

## Complete file: independent-oracle/MANIFEST.json
```text
{
  "format": "independent-preexecution-semantic-freeze/1",
  "frozenAtUtc": "2026-10-18T11:43:30.468747+00:00",
  "status": "FROZEN_BEFORE_EXECUTION",
  "scope": "CappedRemittance pay_tight/pay_loose source-derived Source6 statements/AST/effects/conditional financial projection only",
  "payloadFilesBeforeManifest": 22,
  "files": [
    {
      "path": "DERIVATION.md",
      "bytes": 3524,
      "sha256": "a2d8e27020ed2417db1c56bc170e9ecc6d01933f0817471993420f41c558be08"
    },
    {
      "path": "EXPECTED-SEMANTICS.json",
      "bytes": 10071,
      "sha256": "da997f7f4f59300007cb0368b2d96b728049623315e6649e1bf9c6cccf6fd624"
    },
    {
      "path": "PROVENANCE.json",
      "bytes": 6568,
      "sha256": "857d414b460efab93b8f45361d5f9ad5932934f8e24b8bb73d9b6079128e076d"
    },
    {
      "path": "README.md",
      "bytes": 1027,
      "sha256": "ee589b6e28b71154703ed26e00a845d96d52279f8663a0b13fd397a2c8cc736f"
    },
    {
      "path": "STATIC-CHECKS.json",
      "bytes": 365,
      "sha256": "36eadfded90b0897b76f10da071df8e4d68b881e9fa3aca47809e54033469a74"
    },
    {
      "path": "UNKNOWN.json",
      "bytes": 1743,
      "sha256": "de53b22a57d0f27b2d07531375adef006309eb4a2d930933f1dfac532515a8a8"
    },
    {
      "path": "basis/docs/BETA-SOURCE6-EXPANSION.md",
      "bytes": 4376,
      "sha256": "78b827813bd9d45c82f3ea48fb2752f2c05915df7cdce309cb38b49b68b2226f"
    },
    {
      "path": "basis/dto/financial-agreement-source-v6-frontend.ts.declarations.txt",
      "bytes": 1885,
      "sha256": "b79c6c74d1ce2e2761ded415bf8a5f65db173e01570f3dd6129717445fbb08a0"
    },
    {
      "path": "basis/dto/mil4-s0-core-v5.ts.declarations.txt",
      "bytes": 1944,
      "sha256": "c9de1e005f16744cd4715907c9e8f871f837d0edced3a6ee66992a004f03571c"
    },
    {
      "path": "basis/experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf",
      "bytes": 3753,
      "sha256": "5fb03d51d863e5c959e1cb80f67da9d261c668200837bfab4d748cda0403f72e"
    },
    {
      "path": "basis/experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md",
      "bytes": 11648,
      "sha256": "725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec"
    },
    {
      "path": "basis/experiments/moriarty-language/spec/successor/lexical.md",
      "bytes": 5589,
      "sha256": "88fa3353d1b1a889f1d06f8b35868ae187800719c67ab8345ebb09515e8de639"
    },
    {
      "path": "basis/packages/moriarty-beta/GETTING-STARTED.md",
      "bytes": 11939,
      "sha256": "7bab6fbc179626977f5836e1389685cf070c752cebcd003824e579c31ad9f706"
    },
    {
      "path": "basis/packages/moriarty-beta/README.md",
      "bytes": 4772,
      "sha256": "a12bee5aef1b1de2559a5da85ef0937e822af72ee530275606b427944a8d9c4d"
    },
    {
      "path": "inputs/capped-remittance.mori",
      "bytes": 1815,
      "sha256": "898ffbec44b4165f1afb298398fe36fccc7459e2c1ff92ca7be3ea61fb7c043c"
    },
    {
      "path": "inputs/capped-remittance.scenario.json",
      "bytes": 491,
      "sha256": "6c8c1548cbbc52eca5584f16d4e6e873b245841e6697ad816d22afe73b361dfa"
    },
    {
      "path": "pay_loose.independent-source6.txt",
      "bytes": 1065,
      "sha256": "3913534693475e0132d4c5a68b7eef31d1d55c0bb898618ef1c39ad1f38d42d8"
    },
    {
      "path": "pay_loose.ordered-target-lines.expected.json",
      "bytes": 1455,
      "sha256": "05008475f2cabbec049350114110f031782bf600544bec64c9b4219123d2f42c"
    },
    {
      "path": "pay_loose.source6.ast.expected.json",
      "bytes": 2084,
      "sha256": "d84b4675121518394d8e292aedfcbe417534d662b1fd72a4bca26eb9c620e684"
    },
    {
      "path": "pay_tight.independent-source6.txt",
      "bytes": 1065,
      "sha256": "cc616f8ed536ef03871f572743b6741d22debc39752b1e288515d9a22e992cd2"
    },
    {
      "path": "pay_tight.ordered-target-lines.expected.json",
      "bytes": 1455,
      "sha256": "c0a88fe3a8d647bb46f95a37063f0ca13d138b4bcf830c9480c4fcb1bfe7793c"
    },
    {
      "path": "pay_tight.source6.ast.expected.json",
      "bytes": 2084,
      "sha256": "7e6a072c7bfc19eb1e631f84ff688b1197191f77328afb9cee6ecac2a2bc3835"
    }
  ]
}

```

## Complete file: independent-oracle/PROVENANCE.json
```text
{
  "format": "independent-oracle-provenance/1",
  "frozenAtUtc": "2026-10-18T11:43:30.468747+00:00",
  "author": "GPT-6.1 Sol high ordinary independent semantic derivation; no security decision",
  "sourceInputs": [
    {
      "path": "/home/charl/moriarty-c2-opus55-restored-20261017/source6-rule-coverage-candidate-20261018/candidate/crates/mori/tests/fixtures/source6-expansion/capped-remittance.mori",
      "copy": "inputs/capped-remittance.mori",
      "bytes": 1815,
      "sha256": "898ffbec44b4165f1afb298398fe36fccc7459e2c1ff92ca7be3ea61fb7c043c"
    },
    {
      "path": "/home/charl/moriarty-c2-opus55-restored-20261017/source6-rule-coverage-candidate-20261018/candidate/crates/mori/tests/fixtures/source6-expansion/capped-remittance.scenario.json",
      "copy": "inputs/capped-remittance.scenario.json",
      "bytes": 491,
      "sha256": "6c8c1548cbbc52eca5584f16d4e6e873b245841e6697ad816d22afe73b361dfa"
    }
  ],
  "basis": [
    {
      "path": "packages/moriarty-beta/README.md",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "085320f568b226a2de63b879738299697f941dff",
      "copy": "basis/packages/moriarty-beta/README.md",
      "bytes": 4772,
      "sha256": "a12bee5aef1b1de2559a5da85ef0937e822af72ee530275606b427944a8d9c4d",
      "kind": "pinned normative/reference document"
    },
    {
      "path": "packages/moriarty-beta/GETTING-STARTED.md",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "458194f2848aeb689fd6a34947f5b194a2ef58d8",
      "copy": "basis/packages/moriarty-beta/GETTING-STARTED.md",
      "bytes": 11939,
      "sha256": "7bab6fbc179626977f5836e1389685cf070c752cebcd003824e579c31ad9f706",
      "kind": "pinned normative/reference document"
    },
    {
      "path": "docs/BETA-SOURCE6-EXPANSION.md",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "1851e6eb666cd2a0b50d1564cc2044fe14b823f5",
      "copy": "basis/docs/BETA-SOURCE6-EXPANSION.md",
      "bytes": 4376,
      "sha256": "78b827813bd9d45c82f3ea48fb2752f2c05915df7cdce309cb38b49b68b2226f",
      "kind": "pinned normative/reference document"
    },
    {
      "path": "experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "dca76f9a9146a86f4c06b27156580692e4480dbe",
      "copy": "basis/experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md",
      "bytes": 11648,
      "sha256": "725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec",
      "kind": "pinned normative/reference document"
    },
    {
      "path": "experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "6fc1b5882524781db5cac693c6d3ca1bd982ab42",
      "copy": "basis/experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf",
      "bytes": 3753,
      "sha256": "5fb03d51d863e5c959e1cb80f67da9d261c668200837bfab4d748cda0403f72e",
      "kind": "pinned normative/reference document"
    },
    {
      "path": "experiments/moriarty-language/spec/successor/lexical.md",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "2a57e60840b712d8426a1463342c0375ee26c8eb",
      "copy": "basis/experiments/moriarty-language/spec/successor/lexical.md",
      "bytes": 5589,
      "sha256": "88fa3353d1b1a889f1d06f8b35868ae187800719c67ab8345ebb09515e8de639",
      "kind": "pinned normative/reference document"
    },
    {
      "path": "experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "80cd1dc71c0ca705969a470edebc55e568c31e60",
      "ranges": [
        [
          112,
          138
        ],
        [
          318,
          321
        ]
      ],
      "copy": "basis/dto/financial-agreement-source-v6-frontend.ts.declarations.txt",
      "bytes": 1885,
      "sha256": "b79c6c74d1ce2e2761ded415bf8a5f65db173e01570f3dd6129717445fbb08a0",
      "kind": "DTO declaration excerpt; no parser/evaluator body used"
    },
    {
      "path": "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts",
      "commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
      "gitBlob": "0b2dfcf3309b6d0976cfd33ca152e5c8d1b19a2e",
      "ranges": [
        [
          10,
          47
        ]
      ],
      "copy": "basis/dto/mil4-s0-core-v5.ts.declarations.txt",
      "bytes": 1944,
      "sha256": "c9de1e005f16744cd4715907c9e8f871f837d0edced3a6ee66992a004f03571c",
      "kind": "DTO declaration excerpt; no parser/evaluator body used"
    }
  ],
  "normativeCommit": "74a909d69639df8823a527a4d608a7516ccd69a2",
  "rootReportedCandidateCommit": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
  "rootReportedCandidateTree": "f529a1924885019aa884bc6447f8d47fff47526f",
  "rootReportedCurrentBase": "e4d7a2b5709caa18a94f5256d03a0302364b8235",
  "currencyQualification": "Root states PR219 changes only42ACTUS test/fixture paths and all beta/Source6 norms unchanged vs74. Norms were independently fetched at exact74; no mutable I manifest or newcandidate expected/test read.",
  "priorSource6Exposure": [
    "Previous Rust Source6 CLI/library/emitter/lexer/checker source review preparation.",
    "Old invoice/workshop source/scenario/test/fixture packet exposure and copies; not globally blinded.",
    "This task read Source6 DTO ranges plus incidental adjacent Parser header/fields and bound constants before narrowing excerpt; no parser/lowerer algorithm or chosen-case result used."
  ],
  "currentExcludedUnread": [
    "candidate/crates/mori/tests/source6_expand.rs",
    "candidate/crates/mori/tests/fixtures/source6-expansion/capped-tight.source6",
    "candidate/crates/mori/tests/fixtures/source6-expansion/capped-loose.source6",
    "Any INDEPENDENT-SOURCE-SHAPE authored values",
    "Any actualoutput/result file"
  ],
  "currentDerivationMethod": "Manual source-semantic base10 arithmetic and normative grammar/DTO field mapping, encoded with standard Python JSON/text only. No Moriarty code imported or executed.",
  "productCompilerRuntimeSDKProvider": "UNRUN/NOT_CALLED",
  "git": "Only newly authorized read-only show/rev-parse at exact74 for allowed norm/DTO sources; no Git or shared writes.",
  "securityJudgment": false,
  "Q1ProviderLoop": "Remains STOPPED; this ordinary lane makes no provider call.",
  "source6ByteForByteStdoutOracle": false
}

```

## Complete file: independent-oracle/README.md
```text
# Frozen independent capped-remittance semantic oracle

Start with EXPECTED-SEMANTICS.json, then the two complete Source6Ast files and ordered-target-lines files. Each independent-source6 text contains all target statements in grammar order; indentation is independently chosen and is not an actual stdout byte oracle. ConditionalClosingFinancialProjection contains complete provided3balance/1allowance financial projection, not a complete accepted Core post. Source6 bare nonce and Core5 encoded replay-key effect vectors are separately labeled. UNKNOWN.json keeps Program/Artifact/hashes/work/gates/qualification unasserted.

Inputs and exact74norms/DTO excerpts are frozen alongside DERIVATION.md and PROVENANCE.json. Prior Source6 implementation exposure is disclosed; no new authored test/expected Source6/INDEPENDENT-SOURCE-SHAPE or actualoutput was read. No product/provider/runtime/SDK call occurred. FREEZE.sha256 is the immutable payload manifest; do not add execution results or edit expectations in this directory.

```

## Complete file: independent-oracle/STATIC-CHECKS.json
```text
{
  "status": "independent ordinary arithmetic/custody checks only",
  "quotedAtoms": 225,
  "tightMin": 210,
  "looseMin": 225,
  "feeAtoms": 0,
  "effectCounts": {
    "pay_tight": 5,
    "pay_loose": 5
  },
  "providedBalanceTotal": 1012,
  "allowanceTotal": 1025,
  "feeRowRetained": true,
  "actualProductTests": "UNRUN",
  "producerEmissionCompared": false
}

```

## Complete file: independent-oracle/UNKNOWN.json
```text
{
  "format": "independent-oracle-explicit-unknowns/1",
  "wholeProgram": "UNKNOWN: no Core1/Core2 fullProgram arena/ExprId/normalization oracle derived.",
  "artifact": "UNKNOWN: compiler/ZKIR/SDK/plan/circuit/ABI artifact output not derived.",
  "hashes": "Computed source/scenario/program/plan/policy/target/proof identities UNKNOWN; srcC/policyC are literal unverified source claims. FREEZE/input SHA256 values are custody hashes only.",
  "work": "Input Source6 workRemaining12/workSpent2 are known literal transport fields. Actual CLI/lowerer/compiler/logical/committed/resource work, limits and complete post-work observations UNKNOWN/unasserted; the beta guide example is not used as this run's cost oracle.",
  "gates": "Actual check/formation/Core gate status, refusal precedence, authentication/signature/head-extension/atomic-ledger premises and openGate arrays UNKNOWN/unasserted. Numeric premise4 within inclusive2..20 is arithmetically true only.",
  "qualification": "Actual result qualification/status/PreparedUnqualified or stderr/exit code UNKNOWN/unasserted. local-stipulation is the supplied scenario kind, not a qualified outcome.",
  "bytes": "Canonical whitespace, actual stdout/stderr bytes, byte spans/origins, token/node counts and result wrappers UNKNOWN/unasserted. Independent Source6 documents freeze all statements and order using arbitrary grammar-valid indentation.",
  "fullPost": "Conditional financial projection covers all three provided balance rows, one allowance and selected replay/head/round only; no complete Core post/work/global frame/unlisted replay inventory or target post claimed.",
  "scope": "No security judgment, proof/native/ledger result or general Source6 adoption/closure supplied."
}

```

## Complete file: independent-oracle/pay_loose.independent-source6.txt
```text
profile "moriarty-financial-agreement-source/6";
agreement CappedRemittance {
  domain Midnight;
  settlement A scale 2;
  selected TransferLiteralFee source_hash "srcC" digest "policyC";
  intent {
    signer Owner key "keyC";
    nonce "nC";
    pre_head "c0";
    valid 2 .. 20;
    gross_cap 225;
    fee_cap 0;
    net_floor 225;
    failure success_only;
    signed_action transfer from Owner to Recipient fee_to Fee value 225 fee 0;
    observations empty;
    disclosures empty;
    retained_effects empty;
    retained_duties empty;
    delegation none;
    recovery none;
  }
  authenticated {
    head "c0";
    predecessor "prior";
    round 4;
    balance Owner 1000;
    balance Recipient 5;
    balance Fee 7;
    allowance Owner remaining 1000 spent 25;
    replay unused;
    work_remaining 12;
    work_spent 2;
  }
  submit transfer from Owner to Recipient fee_to Fee value 225 fee 0;
  effects {
    debit Owner 225;
    credit Recipient 225;
    use_allowance Owner 225;
    use_replay "nC";
    advance_head "c0" "c1";
  }
  post_head "c1";
}

```

## Complete file: independent-oracle/pay_loose.ordered-target-lines.expected.json
```text
{
  "scope": "All ordered meaningful Source6 lines/blocks from pinned grammar; independently chosen indentation, not actual stdout byte oracle.",
  "byteForByteStdoutOracle": false,
  "lines": [
    "profile \"moriarty-financial-agreement-source/6\";",
    "agreement CappedRemittance {",
    "domain Midnight;",
    "settlement A scale 2;",
    "selected TransferLiteralFee source_hash \"srcC\" digest \"policyC\";",
    "intent {",
    "signer Owner key \"keyC\";",
    "nonce \"nC\";",
    "pre_head \"c0\";",
    "valid 2 .. 20;",
    "gross_cap 225;",
    "fee_cap 0;",
    "net_floor 225;",
    "failure success_only;",
    "signed_action transfer from Owner to Recipient fee_to Fee value 225 fee 0;",
    "observations empty;",
    "disclosures empty;",
    "retained_effects empty;",
    "retained_duties empty;",
    "delegation none;",
    "recovery none;",
    "}",
    "authenticated {",
    "head \"c0\";",
    "predecessor \"prior\";",
    "round 4;",
    "balance Owner 1000;",
    "balance Recipient 5;",
    "balance Fee 7;",
    "allowance Owner remaining 1000 spent 25;",
    "replay unused;",
    "work_remaining 12;",
    "work_spent 2;",
    "}",
    "submit transfer from Owner to Recipient fee_to Fee value 225 fee 0;",
    "effects {",
    "debit Owner 225;",
    "credit Recipient 225;",
    "use_allowance Owner 225;",
    "use_replay \"nC\";",
    "advance_head \"c0\" \"c1\";",
    "}",
    "post_head \"c1\";",
    "}"
  ]
}

```

## Complete file: independent-oracle/pay_loose.source6.ast.expected.json
```text
{
  "profile": "moriarty-financial-agreement-source/6",
  "programId": "CappedRemittance",
  "domain": "Midnight",
  "settlement": {
    "asset": "A",
    "scale": "2"
  },
  "selected": {
    "actionId": "TransferLiteralFee",
    "sourceHash": "srcC",
    "policyDigest": "policyC"
  },
  "intent": {
    "signer": "Owner",
    "keyRef": "keyC",
    "nonce": "nC",
    "preHead": "c0",
    "notBefore": "2",
    "notAfter": "20",
    "grossCap": "225",
    "feeCap": "0",
    "netFloor": "225",
    "signedAction": {
      "kind": "Transfer",
      "from": "Owner",
      "to": "Recipient",
      "feeTo": "Fee",
      "value": "225",
      "fee": "0"
    },
    "failure": "success_only",
    "observations": "empty",
    "disclosures": "empty",
    "retainedEffects": "empty",
    "retainedDuties": "empty",
    "delegation": "none",
    "recovery": "none"
  },
  "authenticated": {
    "head": "c0",
    "predecessor": "prior",
    "round": "4",
    "balances": [
      {
        "account": "Owner",
        "amount": "1000"
      },
      {
        "account": "Recipient",
        "amount": "5"
      },
      {
        "account": "Fee",
        "amount": "7"
      }
    ],
    "allowance": {
      "owner": "Owner",
      "remaining": "1000",
      "spent": "25"
    },
    "replay": "unused",
    "workRemaining": "12",
    "workSpent": "2"
  },
  "submitted": {
    "action": {
      "kind": "Transfer",
      "from": "Owner",
      "to": "Recipient",
      "feeTo": "Fee",
      "value": "225",
      "fee": "0"
    },
    "effects": [
      {
        "kind": "Debit",
        "account": "Owner",
        "asset": "A",
        "amount": "225"
      },
      {
        "kind": "Credit",
        "account": "Recipient",
        "asset": "A",
        "amount": "225"
      },
      {
        "kind": "UseAllowance",
        "owner": "Owner",
        "amount": "225"
      },
      {
        "kind": "UseReplay",
        "key": "nC"
      },
      {
        "kind": "AdvanceHead",
        "predecessor": "c0",
        "successor": "c1"
      }
    ],
    "postHead": "c1"
  }
}

```

## Complete file: independent-oracle/pay_tight.independent-source6.txt
```text
profile "moriarty-financial-agreement-source/6";
agreement CappedRemittance {
  domain Midnight;
  settlement A scale 2;
  selected TransferLiteralFee source_hash "srcC" digest "policyC";
  intent {
    signer Owner key "keyC";
    nonce "nC";
    pre_head "c0";
    valid 2 .. 20;
    gross_cap 210;
    fee_cap 0;
    net_floor 210;
    failure success_only;
    signed_action transfer from Owner to Recipient fee_to Fee value 210 fee 0;
    observations empty;
    disclosures empty;
    retained_effects empty;
    retained_duties empty;
    delegation none;
    recovery none;
  }
  authenticated {
    head "c0";
    predecessor "prior";
    round 4;
    balance Owner 1000;
    balance Recipient 5;
    balance Fee 7;
    allowance Owner remaining 1000 spent 25;
    replay unused;
    work_remaining 12;
    work_spent 2;
  }
  submit transfer from Owner to Recipient fee_to Fee value 210 fee 0;
  effects {
    debit Owner 210;
    credit Recipient 210;
    use_allowance Owner 210;
    use_replay "nC";
    advance_head "c0" "c1";
  }
  post_head "c1";
}

```

## Complete file: independent-oracle/pay_tight.ordered-target-lines.expected.json
```text
{
  "scope": "All ordered meaningful Source6 lines/blocks from pinned grammar; independently chosen indentation, not actual stdout byte oracle.",
  "byteForByteStdoutOracle": false,
  "lines": [
    "profile \"moriarty-financial-agreement-source/6\";",
    "agreement CappedRemittance {",
    "domain Midnight;",
    "settlement A scale 2;",
    "selected TransferLiteralFee source_hash \"srcC\" digest \"policyC\";",
    "intent {",
    "signer Owner key \"keyC\";",
    "nonce \"nC\";",
    "pre_head \"c0\";",
    "valid 2 .. 20;",
    "gross_cap 210;",
    "fee_cap 0;",
    "net_floor 210;",
    "failure success_only;",
    "signed_action transfer from Owner to Recipient fee_to Fee value 210 fee 0;",
    "observations empty;",
    "disclosures empty;",
    "retained_effects empty;",
    "retained_duties empty;",
    "delegation none;",
    "recovery none;",
    "}",
    "authenticated {",
    "head \"c0\";",
    "predecessor \"prior\";",
    "round 4;",
    "balance Owner 1000;",
    "balance Recipient 5;",
    "balance Fee 7;",
    "allowance Owner remaining 1000 spent 25;",
    "replay unused;",
    "work_remaining 12;",
    "work_spent 2;",
    "}",
    "submit transfer from Owner to Recipient fee_to Fee value 210 fee 0;",
    "effects {",
    "debit Owner 210;",
    "credit Recipient 210;",
    "use_allowance Owner 210;",
    "use_replay \"nC\";",
    "advance_head \"c0\" \"c1\";",
    "}",
    "post_head \"c1\";",
    "}"
  ]
}

```

## Complete file: independent-oracle/pay_tight.source6.ast.expected.json
```text
{
  "profile": "moriarty-financial-agreement-source/6",
  "programId": "CappedRemittance",
  "domain": "Midnight",
  "settlement": {
    "asset": "A",
    "scale": "2"
  },
  "selected": {
    "actionId": "TransferLiteralFee",
    "sourceHash": "srcC",
    "policyDigest": "policyC"
  },
  "intent": {
    "signer": "Owner",
    "keyRef": "keyC",
    "nonce": "nC",
    "preHead": "c0",
    "notBefore": "2",
    "notAfter": "20",
    "grossCap": "210",
    "feeCap": "0",
    "netFloor": "210",
    "signedAction": {
      "kind": "Transfer",
      "from": "Owner",
      "to": "Recipient",
      "feeTo": "Fee",
      "value": "210",
      "fee": "0"
    },
    "failure": "success_only",
    "observations": "empty",
    "disclosures": "empty",
    "retainedEffects": "empty",
    "retainedDuties": "empty",
    "delegation": "none",
    "recovery": "none"
  },
  "authenticated": {
    "head": "c0",
    "predecessor": "prior",
    "round": "4",
    "balances": [
      {
        "account": "Owner",
        "amount": "1000"
      },
      {
        "account": "Recipient",
        "amount": "5"
      },
      {
        "account": "Fee",
        "amount": "7"
      }
    ],
    "allowance": {
      "owner": "Owner",
      "remaining": "1000",
      "spent": "25"
    },
    "replay": "unused",
    "workRemaining": "12",
    "workSpent": "2"
  },
  "submitted": {
    "action": {
      "kind": "Transfer",
      "from": "Owner",
      "to": "Recipient",
      "feeTo": "Fee",
      "value": "210",
      "fee": "0"
    },
    "effects": [
      {
        "kind": "Debit",
        "account": "Owner",
        "asset": "A",
        "amount": "210"
      },
      {
        "kind": "Credit",
        "account": "Recipient",
        "asset": "A",
        "amount": "210"
      },
      {
        "kind": "UseAllowance",
        "owner": "Owner",
        "amount": "210"
      },
      {
        "kind": "UseReplay",
        "key": "nC"
      },
      {
        "kind": "AdvanceHead",
        "predecessor": "c0",
        "successor": "c1"
      }
    ],
    "postHead": "c1"
  }
}

```

## Complete file: actual-source/crates/mori-source6/src/lib.rs
```text
//! Local Source/6 presentation for the closed beta transfer slice.
//!
//! This crate emits a proposal. Its scenario cells and declared hashes remain
//! claims; it performs no Core/5 preparation, signing, proof or ledger action.
//! The generated text must be consumed by the existing Source/6 lowerer before
//! any claim of Source/6 formation or Core/5 preparation.

#[cfg(feature = "source6-transfer-slice")]
mod diagnostics;
#[cfg(feature = "source6-transfer-slice")]
mod transfer;

#[cfg(feature = "source6-transfer-slice")]
pub use transfer::{
    Allowance, Balance, LocalTransferScenario, Replay, SelectedTransfer, TransferExpansion,
    expand_transfer, select_transfer,
};

```

## Complete file: actual-source/crates/mori-source6/src/transfer.rs
```text
use mori_ast::NodeIdx;
use mori_checker::{Checked, Support, ValueIdx, ValueTag};
use mori_diagnostics::MoriDiagnostic;
use mori_lexer::{TokenIdx, TokenKind, decode_string};
use mori_span::Span;

use crate::diagnostics;

type Result<T> = std::result::Result<T, MoriDiagnostic>;

/// A balance cell supplied as an unverified local claim.
#[derive(Debug, Clone, Copy)]
pub struct Balance<'a> {
    pub account: &'a str,
    pub amount: u128,
}

/// Explicit opening counters; expansion never resets or consumes them.
#[derive(Debug, Clone, Copy)]
pub struct Allowance<'a> {
    pub owner: &'a str,
    pub remaining: u128,
    pub spent: u128,
}

#[derive(Debug, Clone, Copy)]
pub enum Replay {
    Unused,
    Consumed,
}

/// Typed local scenario for the default transfer proposal only.
///
/// Balance order is owner, recipient, fee recipient. All three cells are
/// supplied even for a zero fee. This is not a JSON wire format, authenticated
/// snapshot or candidate-effects override API.
#[derive(Debug, Clone, Copy)]
pub struct LocalTransferScenario<'a> {
    pub domain: &'a str,
    pub asset: &'a str,
    pub head: &'a str,
    pub predecessor: &'a str,
    pub round: u128,
    pub balances: [Balance<'a>; 3],
    pub allowance: Allowance<'a>,
    pub replay: Replay,
    pub work_remaining: u128,
    pub work_spent: u128,
    pub post_head: &'a str,
}

/// Generated source text, with no Core result or published post-state.
#[derive(Debug, PartialEq, Eq)]
pub struct TransferExpansion {
    pub source6: String,
    pub qualification: &'static str,
}

/// A source-selected transfer, produced only after the existing frontend checks.
/// The owned presentation terms permit scenario I/O to happen after selection.
pub struct SelectedTransfer {
    terms: Terms,
    span: Span,
}

impl SelectedTransfer {
    /// Shape explicit local cells and emit a proposal, without Core preparation.
    pub fn expand(
        &self,
        scenario: &LocalTransferScenario<'_>,
    ) -> std::result::Result<TransferExpansion, MoriDiagnostic> {
        validate_scenario(&self.terms, scenario, self.span)?;
        let source6 = emit(&self.terms, scenario, self.span)?;
        Ok(TransferExpansion {
            source6,
            qualification: "local-stipulation-only",
        })
    }
}

/// Parse, check and select a beta transfer before a caller reads its scenario.
/// Existing checker diagnostics and unsupported-action selection are retained.
pub fn select_transfer(
    source: &str,
    action: &str,
) -> std::result::Result<SelectedTransfer, Vec<MoriDiagnostic>> {
    let ast = mori_parser::parse(source).map_err(|error| vec![error])?;
    let result = mori_checker::check(&ast);
    if !result.is_accepted() {
        return Err(result.diagnostics);
    }
    let (terms, span) = extract(&result.checked, action).map_err(|error| vec![error])?;
    Ok(SelectedTransfer { terms, span })
}

/// Parse and check beta source, then emit its selected transfer proposal.
/// Quantities and claims come from the unchanged checked transfer mapping.
/// Call select_transfer first when scenario I/O must follow source selection.
pub fn expand_transfer(
    source: &str,
    action: &str,
    scenario: &LocalTransferScenario<'_>,
) -> std::result::Result<TransferExpansion, Vec<MoriDiagnostic>> {
    select_transfer(source, action)?
        .expand(scenario)
        .map_err(|error| vec![error])
}

struct Terms {
    agreement: String,
    domain: String,
    asset: String,
    scale: u128,
    signer: String,
    owner: String,
    recipient: String,
    fee_recipient: String,
    key: String,
    nonce: String,
    pre_head: String,
    valid_from: u128,
    valid_to: u128,
    gross_cap: u128,
    fee_cap: u128,
    net_floor: u128,
    value: u128,
    fee: u128,
    gross: u128,
    source_hash: String,
    policy_digest: String,
}

fn field(checked: &Checked<'_>, value: ValueIdx, name: &str) -> Result<ValueIdx> {
    checked
        .values
        .pairs(value)
        .find(|&(key, _)| checked.ast.token_text(TokenIdx::new(usize::from(key))) == name)
        .map(|(_, field)| field)
        .ok_or_else(|| {
            diagnostics::shape(
                checked.ast.span(checked.values.node(value)),
                &format!("the checked `{name}` field"),
            )
        })
}

fn string(checked: &Checked<'_>, value: ValueIdx) -> Result<String> {
    if checked.values.tag(value) != ValueTag::String {
        return Err(diagnostics::shape(
            checked.ast.span(checked.values.node(value)),
            "a checked string",
        ));
    }
    let token = TokenIdx::new(usize::from(checked.values.rows.a()[value.index()]));
    Ok(decode_string(checked.ast.token_text(token)))
}

fn number(checked: &Checked<'_>, value: ValueIdx, tag: ValueTag) -> Result<u128> {
    if checked.values.tag(value) != tag {
        return Err(diagnostics::shape(
            checked.ast.span(checked.values.node(value)),
            "a checked scalar or nominal quantity in its declared role",
        ));
    }
    Ok(checked.values.amount(value))
}

fn id(checked: &Checked<'_>, entity: ValueIdx) -> Result<String> {
    string(checked, field(checked, entity, "id")?)
}

fn call_name(checked: &Checked<'_>, value: ValueIdx) -> String {
    let mut token = checked.ast.main_token(checked.values.node(value));
    let mut name = checked.ast.token_text(token).to_owned();
    while checked.ast.tokens.kind(checked.ast.token_after(token, 1)) == TokenKind::Dot {
        token = checked.ast.token_after(token, 2);
        name.push('.');
        name.push_str(checked.ast.token_text(token));
    }
    name
}

fn extract(checked: &Checked<'_>, action: &str) -> Result<(Terms, Span)> {
    let ast = checked.ast;
    let index = checked
        .actions
        .node()
        .iter()
        .position(|&node| ast.token_text(ast.token_after(ast.main_token(node), 1)) == action);
    let index = index.ok_or_else(|| diagnostics::action(ast.span(NodeIdx::ROOT), action))?;
    let node = checked.actions.node()[index];
    let span = ast.span(node);
    if checked.actions.support()[index] != Some(Support::LocalS0) {
        return Err(diagnostics::unsupported(span));
    }
    let intent = checked.actions.intent()[index]
        .and_then(|decl| checked.value(decl))
        .ok_or_else(|| diagnostics::shape(span, "a resolved checked intent"))?;
    let operation = field(checked, intent, "operation")?;
    if call_name(checked, operation) != "transfer" {
        return Err(diagnostics::unsupported(span));
    }
    let asset = field(checked, intent, "asset")?;
    let validity = field(checked, intent, "valid")?;
    let quantity =
        |record, name| number(checked, field(checked, record, name)?, ValueTag::Quantity);
    let scalar = |record, name| number(checked, field(checked, record, name)?, ValueTag::Scalar);
    let text = |name| string(checked, field(checked, intent, name)?);
    let value = quantity(operation, "value")?;
    let fee = quantity(operation, "fee")?;
    let gross = value
        .checked_add(fee)
        .ok_or_else(|| diagnostics::shape(span, "a UInt128 transfer total"))?;
    Ok((
        Terms {
            agreement: ast.token_text(ast.main_token(NodeIdx::ROOT)).to_owned(),
            domain: id(checked, field(checked, intent, "domain")?)?,
            asset: id(checked, asset)?,
            scale: scalar(asset, "scale")?,
            signer: id(checked, field(checked, intent, "signer")?)?,
            owner: id(checked, field(checked, operation, "from")?)?,
            recipient: id(checked, field(checked, operation, "to")?)?,
            fee_recipient: id(checked, field(checked, operation, "fee_to")?)?,
            key: text("key")?,
            nonce: text("nonce")?,
            pre_head: text("pre_head")?,
            valid_from: scalar(validity, "from")?,
            valid_to: scalar(validity, "to")?,
            gross_cap: quantity(intent, "gross_cap")?,
            fee_cap: quantity(intent, "fee_cap")?,
            net_floor: quantity(intent, "net_floor")?,
            value,
            fee,
            gross,
            source_hash: text("source_hash")?,
            policy_digest: text("policy_digest")?,
        },
        span,
    ))
}

fn validate_scenario(terms: &Terms, s: &LocalTransferScenario<'_>, span: Span) -> Result<()> {
    for (field, actual, expected) in [
        ("domain", s.domain, terms.domain.as_str()),
        ("asset", s.asset, terms.asset.as_str()),
        ("allowance.owner", s.allowance.owner, terms.signer.as_str()),
    ] {
        if actual != expected {
            return Err(diagnostics::scenario(span, field, expected));
        }
    }
    for (index, (balance, expected)) in s
        .balances
        .iter()
        .zip([
            terms.owner.as_str(),
            terms.recipient.as_str(),
            terms.fee_recipient.as_str(),
        ])
        .enumerate()
    {
        if balance.account != expected {
            return Err(diagnostics::scenario(
                span,
                &format!("balances[{index}].account"),
                expected,
            ));
        }
    }
    for (field, text) in [
        ("head", s.head),
        ("predecessor", s.predecessor),
        ("post_head", s.post_head),
    ] {
        if text.is_empty() || text.len() > mori_lexer::MAX_STRING_BYTES {
            return Err(diagnostics::scenario(
                span,
                field,
                "nonempty text of at most 1024 UTF-8 bytes",
            ));
        }
    }
    if s.allowance
        .remaining
        .checked_add(s.allowance.spent)
        .is_none()
        || s.work_remaining.checked_add(s.work_spent).is_none()
    {
        return Err(diagnostics::scenario(
            span,
            "counters",
            "opening totals that fit UInt128",
        ));
    }
    Ok(())
}

fn quote(text: &str) -> String {
    serde_json::to_string(text).expect("serializing a UTF-8 string cannot fail")
}

fn append_line(out: &mut String, line: String, span: Span) -> Result<()> {
    if out.len() + line.len() + 1 > mori_lexer::MAX_SOURCE_BYTES {
        return Err(diagnostics::bound(span));
    }
    out.push_str(&line);
    out.push('\n');
    Ok(())
}

fn emit(t: &Terms, s: &LocalTransferScenario<'_>, span: Span) -> Result<String> {
    let mut out = String::new();
    macro_rules! line {
        ($($arg:tt)*) => {
            append_line(&mut out, format!($($arg)*), span)?
        };
    }
    let statement = format!(
        "transfer from {} to {} fee_to {} value {} fee {}",
        t.owner, t.recipient, t.fee_recipient, t.value, t.fee,
    );
    line!("profile \"moriarty-financial-agreement-source/6\";");
    line!("agreement {} {{", t.agreement);
    line!("domain {};", t.domain);
    line!("settlement {} scale {};", t.asset, t.scale);
    line!(
        "selected TransferLiteralFee source_hash {} digest {};",
        quote(&t.source_hash),
        quote(&t.policy_digest)
    );
    line!("intent {{");
    line!("signer {} key {};", t.signer, quote(&t.key));
    line!("nonce {};", quote(&t.nonce));
    line!("pre_head {};", quote(&t.pre_head));
    line!("valid {}..{};", t.valid_from, t.valid_to);
    line!("gross_cap {};", t.gross_cap);
    line!("fee_cap {};", t.fee_cap);
    line!("net_floor {};", t.net_floor);
    line!("failure success_only;");
    line!("signed_action {statement};");
    for field in [
        "observations",
        "disclosures",
        "retained_effects",
        "retained_duties",
    ] {
        line!("{field} empty;");
    }
    for field in ["delegation", "recovery"] {
        line!("{field} none;");
    }
    line!("}}");
    line!("authenticated {{");
    line!("head {};", quote(s.head));
    line!("predecessor {};", quote(s.predecessor));
    line!("round {};", s.round);
    for balance in &s.balances {
        line!("balance {} {};", balance.account, balance.amount);
    }
    line!(
        "allowance {} remaining {} spent {};",
        s.allowance.owner,
        s.allowance.remaining,
        s.allowance.spent
    );
    let replay = match s.replay {
        Replay::Unused => "unused",
        Replay::Consumed => "consumed",
    };
    line!("replay {replay};");
    line!("work_remaining {};", s.work_remaining);
    line!("work_spent {};", s.work_spent);
    line!("}}");
    line!("submit {statement};");
    line!("effects {{");
    line!("debit {} {};", t.owner, t.gross);
    line!("credit {} {};", t.recipient, t.value);
    if t.fee > 0 {
        line!("credit {} {};", t.fee_recipient, t.fee);
    }
    line!("use_allowance {} {};", t.signer, t.gross);
    line!("use_replay {};", quote(&t.nonce));
    line!(
        "advance_head {} {};",
        quote(&t.pre_head),
        quote(s.post_head)
    );
    line!("}}");
    line!("post_head {};", quote(s.post_head));
    line!("}}");
    Ok(out)
}

```

## Complete file: actual-source/crates/mori/src/commands/expand.rs
```text
use std::process::ExitCode;

use crate::cli::ActionInput;
use crate::shell::Shell;

#[cfg(feature = "source6-expansion")]
mod source6;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Emit a local Source/6 transfer proposal; requires source6-expansion
    #[arg(
        long,
        long_help = "Emit raw local Source/6 transfer proposal text. Requires the source6-expansion build feature; default Core expansion is unchanged."
    )]
    pub emit_source6_transfer: bool,
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    if args.emit_source6_transfer {
        return run_source6(args.input, shell);
    }
    let (_, program) = super::core_io::program(&args.input.file)?;
    // Validate the selected scenario/action without discarding the generic core.
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    mori_core::simulate(&program, &args.input.action, &scenario)
        .map_err(super::core_io::operation)?;
    super::core_io::print(&program)?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "source6-expansion")]
fn run_source6(input: ActionInput, shell: &mut Shell) -> super::Result<ExitCode> {
    source6::run(input, shell)
}

#[cfg(not(feature = "source6-expansion"))]
fn run_source6(_input: ActionInput, _shell: &mut Shell) -> super::Result<ExitCode> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::source6-unavailable".into(),
        message: "Source/6 transfer expansion requires the source6-expansion build feature.".into(),
    }
    .into())
}

```

## Complete file: actual-source/crates/mori/src/commands/expand/source6.rs
```text
//! Explicit local transfer presentation consumer of mori-source6.
use std::{io::Write, process::ExitCode};

use mori_core::UInt;
use mori_diagnostics::NamedSource;
use mori_source6::{Allowance, Balance, LocalTransferScenario, Replay, select_transfer};
use serde::Deserialize;

use crate::{cli::ActionInput, commands::core_io, shell::Shell};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TransferScenario {
    #[serde(rename = "profile")]
    _profile: String,
    #[serde(rename = "kind")]
    _kind: String,
    domain: String,
    asset: String,
    head: String,
    predecessor: String,
    round: UInt,
    balances: [BalanceCell; 3],
    allowance: AllowanceCell,
    replay: ReplayClaim,
    work_remaining: UInt,
    work_spent: UInt,
    post_head: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BalanceCell {
    account: String,
    amount: UInt,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AllowanceCell {
    owner: String,
    remaining: UInt,
    spent: UInt,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ReplayClaim {
    Unused,
    Consumed,
}

impl TransferScenario {
    fn local(&self) -> LocalTransferScenario<'_> {
        LocalTransferScenario {
            domain: &self.domain,
            asset: &self.asset,
            head: &self.head,
            predecessor: &self.predecessor,
            round: self.round.0,
            balances: self.balances.each_ref().map(|cell| Balance {
                account: &cell.account,
                amount: cell.amount.0,
            }),
            allowance: Allowance {
                owner: &self.allowance.owner,
                remaining: self.allowance.remaining.0,
                spent: self.allowance.spent.0,
            },
            replay: match self.replay {
                ReplayClaim::Unused => Replay::Unused,
                ReplayClaim::Consumed => Replay::Consumed,
            },
            work_remaining: self.work_remaining.0,
            work_spent: self.work_spent.0,
            post_head: &self.post_head,
        }
    }
}

fn scenario_error(code: &str, message: impl Into<String>) -> mori_diagnostics::Report {
    crate::error::CliError::Operation {
        code: format!("mori::command::{code}"),
        message: crate::display::text(&message.into()),
    }
    .into()
}

fn read_scenario(path: &std::path::Path) -> crate::commands::Result<TransferScenario> {
    let raw: serde_json::Value = core_io::read_json(path)?;
    let object = raw.as_object().ok_or_else(|| {
        scenario_error(
            "source6-scenario-profile",
            "Expected a local transfer scenario object.",
        )
    })?;
    if object.get("profile").and_then(serde_json::Value::as_str)
        != Some("moriarty-local-scenario/1")
        || object.get("kind").and_then(serde_json::Value::as_str) != Some("local-stipulation")
    {
        return Err(scenario_error(
            "source6-scenario-profile",
            "Source/6 transfer expansion requires moriarty-local-scenario/1 local-stipulation input.",
        ));
    }
    for field in ["obligation", "candidate_effects"] {
        if object.contains_key(field) {
            return Err(scenario_error(
                "source6-scenario-unsupported",
                format!("The --emit-source6-transfer subset does not support `{field}`."),
            ));
        }
    }
    if object
        .get("balances")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|balances| balances.len() != 3)
    {
        return Err(scenario_error(
            "source6-scenario-cells",
            "The transfer subset requires exactly three balance cells: owner, recipient, fee recipient.",
        ));
    }
    serde_json::from_value(raw).map_err(|error| {
        scenario_error(
            "source6-scenario-shape",
            format!("Local transfer scenario shape: {error}"),
        )
    })
}

pub(super) fn run(input: ActionInput, shell: &mut Shell) -> crate::commands::Result<ExitCode> {
    let source = core_io::read_source(&input.file)?;
    let name = crate::display::text(&input.file.display().to_string());
    let selected = match select_transfer(&source, &input.action) {
        Ok(selected) => selected,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                shell.report(&diagnostic.with_source_code(NamedSource::new(&name, source.clone())));
            }
            return Ok(ExitCode::FAILURE);
        }
    };
    let scenario = read_scenario(&input.scenario)?;
    let expansion = match selected.expand(&scenario.local()) {
        Ok(expansion) => expansion,
        Err(diagnostic) => {
            shell.report(&diagnostic.with_source_code(NamedSource::new(name, source)));
            return Ok(ExitCode::FAILURE);
        }
    };
    // The library includes its final newline. This writes its actual proposal.
    std::io::stdout()
        .lock()
        .write_all(expansion.source6.as_bytes())
        .map_err(core_io::operation)?;
    shell.status(
        "Expanded",
        format!("Source/6 transfer proposal ({})", expansion.qualification),
    );
    Ok(ExitCode::SUCCESS)
}

```

## Complete file: actual-check/CURRENT-SOURCE.json
```text
{
  "recordedAt": "2026-10-18T11:36:27.980158+00:00",
  "base": "e4d7a2b5709caa18a94f5256d03a0302364b8235",
  "head": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
  "tree": "f529a1924885019aa884bc6447f8d47fff47526f",
  "candidatePatchSha256": "7c267b075d251ab2f2bfa7a3512b79168f85ec89599f97384a53b044838d9d5a",
  "files": [
    {
      "path": "crates/mori/tests/fixtures/source6-expansion/capped-loose.source6",
      "sha256": "21e1db1eba67e5bc1a446a65ad0754dea63dab3490e165f86369b14f11549bdf"
    },
    {
      "path": "crates/mori/tests/fixtures/source6-expansion/capped-remittance.mori",
      "sha256": "898ffbec44b4165f1afb298398fe36fccc7459e2c1ff92ca7be3ea61fb7c043c"
    },
    {
      "path": "crates/mori/tests/fixtures/source6-expansion/capped-remittance.scenario.json",
      "sha256": "6c8c1548cbbc52eca5584f16d4e6e873b245841e6697ad816d22afe73b361dfa"
    },
    {
      "path": "crates/mori/tests/fixtures/source6-expansion/capped-tight.source6",
      "sha256": "5616a8fe33a41947d34833d52764a72bf602bd659bc1017d80fed48a738e7e6e"
    },
    {
      "path": "crates/mori/tests/source6_expand.rs",
      "sha256": "5dfaa249627db9c44526e5fedf212561a74ca1e24df8d29631bfb040fcbf0d16"
    }
  ],
  "rustfmtChanged": false,
  "sourceSelection": "Ordinary public beta check/Source6 expansion only; one new test, five paths. No production implementation modified. Independent oracle freeze and actual selected test remain pending.",
  "actualTests": "UNRUN",
  "allOriginalAcceptanceRetained": true
}

```

## Complete file: actual-check/CHECK-PLAN.json
```text
{
  "preparedAt": "2026-10-18T11:44:22.088575+00:00",
  "sourceCommit": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
  "sourceTree": "f529a1924885019aa884bc6447f8d47fff47526f",
  "authorization": "Existing ordinary source/tooling tests; no SDK/native/model/key/proof/ledger execution. No historical cap reset.",
  "sourceAudit": "Root inspected ordinary test/source/input and existing consumer; fresh AGY full source/result review before merge",
  "oracleFreezeSha256": "4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79",
  "oracleMembersVerified": 23,
  "independentScope": "Independent source values, complete semantic Source6 statement vectors and conditional financial frames for two min branches, fee0; prior Source6 exposure disclosed. Raw expected presentation authored separately; no whole Program/Artifact/hash/work/gate qualification oracle.",
  "ownedTemporaryRoot": "/tmp/doctorwatson-source6-min-50vxmzgd",
  "publicCacheSymlinks": [
    "registry",
    "git"
  ],
  "baseEnvironment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-source6-min-50vxmzgd/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-source6-min-50vxmzgd/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-source6-min-50vxmzgd/tmp"
  },
  "toolPins": [
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
      "resolved": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
      "sha256": "e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
      "resolved": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
      "sha256": "f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
      "resolved": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
      "sha256": "a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"
    },
    {
      "path": "/usr/bin/timeout",
      "resolved": "/usr/lib/cargo/bin/coreutils/timeout",
      "sha256": "f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"
    }
  ],
  "absentAncestorConfigs": [
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
    "/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/.cargo/config",
    "/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/.cargo/config.toml",
    "/tmp/doctorwatson-source6-min-50vxmzgd/.cargo/config",
    "/tmp/doctorwatson-source6-min-50vxmzgd/.cargo/config.toml",
    "/tmp/.cargo/config",
    "/tmp/.cargo/config.toml",
    "/tmp/doctorwatson-source6-min-50vxmzgd/cwd/.cargo/config",
    "/tmp/doctorwatson-source6-min-50vxmzgd/cwd/.cargo/config.toml"
  ],
  "attemptsPerStep": 1,
  "timeoutSecondsPerStep": 90,
  "killGraceSeconds": 5,
  "sharedColdTarget": "One newly selected quantity-min test on existing Source6 CLI target; no separate warmup/install/fetch.",
  "RAMDiskAggregateCaps": null,
  "originalExitsRemainOpen": true
}

```

## Complete file: actual-check/RESULT.json
```text
{
  "recordedAt": "2026-10-18T11:45:15.722239+00:00",
  "sourceCommit": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
  "sourceTree": "f529a1924885019aa884bc6447f8d47fff47526f",
  "steps": [
    {
      "step": "public-quantity-min",
      "finishedAt": "2026-10-18T11:45:15.630811+00:00",
      "exitCode": 0,
      "wallSeconds": 53.4942627690034,
      "sourceCommitAfter": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
      "sourceTreeAfter": "f529a1924885019aa884bc6447f8d47fff47526f",
      "sourceStatusAfter": "",
      "stdoutSha256": "aef1a780855988721551681e64144e3535d5f0c09b55db987c6f9e79d41ca2fc",
      "stderrSha256": "b6a4a2f598317a48cc513e52c37375fc15e68220eea06af84898fca062e4d842",
      "attemptConsumed": true
    }
  ],
  "temporaryRootExistsAfter": false,
  "allStepsCompleted": true,
  "allExitZero": true,
  "scope": "One public CLI regression: beta check plus both min-action raw Source6 text outputs, zero fee and three retained opening cells. Actual TypeScript lowerer/native/proof/ledger/full grammar/Program/Artifact remain unobserved by this command."
}

```

## Complete file: actual-check/CLEANUP.json
```text
{
  "recordedAt": "2026-10-18T11:45:15.722053+00:00",
  "ownedTemporaryRoot": "/tmp/doctorwatson-source6-min-50vxmzgd",
  "existsAfter": false,
  "scope": "Only owned temporary root removed, public cache symlinks not traversed."
}

```

## Complete file: actual-check/FORMATTING.diff
```text

```

## Complete file: actual-check/public-quantity-min/START.json
```text
{
  "startedAt": "2026-10-18T11:44:22.093292+00:00",
  "sourceCommit": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
  "sourceTree": "f529a1924885019aa884bc6447f8d47fff47526f",
  "sourceStatus": "",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--features",
    "source6-expansion",
    "--test",
    "source6_expand",
    "--",
    "quantity_min_selects_both_operands_through_public_source6_expansion",
    "--exact",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-source6-min-50vxmzgd/cwd",
  "environment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-source6-min-50vxmzgd/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-source6-min-50vxmzgd/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-source6-min-50vxmzgd/tmp"
  },
  "sourcePinsVerified": 511,
  "attempt": 1,
  "attemptLimit": 1
}

```

## Complete file: actual-check/public-quantity-min/CHILD.json
```text
{
  "pid": 661909,
  "session": 661909
}

```

## Complete file: actual-check/public-quantity-min/STDOUT.log
```text

running 1 test
test quantity_min_selects_both_operands_through_public_source6_expansion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.01s


```

## Complete file: actual-check/public-quantity-min/STDERR.log
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
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling typenum v1.20.1
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling writeable v0.6.4
   Compiling zeroize v1.9.1
   Compiling version_check v0.9.5
   Compiling libc v0.2.190
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling litemap v0.8.3
   Compiling rustix v1.1.5
   Compiling icu_locale_fallback_data v2.3.0
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling linux-raw-sys v0.12.1
   Compiling bitflags v2.13.2
   Compiling icu_segmenter_data v2.3.0
   Compiling icu_provider v2.3.1
   Compiling const-oid v0.9.6
   Compiling subtle v2.6.1
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
   Compiling supports-unicode v3.0.0
   Compiling unicode-width v0.1.14
   Compiling owo-colors v4.4.0
   Compiling supports-hyperlinks v3.2.0
   Compiling miette v7.6.0
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling getrandom v0.2.17
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-span)
   Compiling memchr v2.8.3
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling der v0.7.10
   Compiling crossbeam-utils v0.8.23
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-lexer)
   Compiling serde_core v1.0.229
   Compiling crossbeam-epoch v0.9.21
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling base16ct v0.2.0
   Compiling zmij v1.0.23
   Compiling crossbeam-deque v0.8.8
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-ast)
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling serde_json v1.0.151
   Compiling utf8parse v0.2.2
   Compiling once_cell v1.21.4
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling thiserror v2.0.21
   Compiling anstyle v1.0.14
   Compiling regex-syntax v0.8.11
   Compiling rayon-core v1.13.0
   Compiling itoa v1.0.18
   Compiling colorchoice v1.0.5
   Compiling serde v1.0.229
   Compiling anstyle-query v1.1.5
   Compiling getrandom v0.4.3
   Compiling const-oid v0.10.2
   Compiling is_terminal_polyfill v1.70.2
   Compiling cpufeatures v0.2.17
   Compiling sha2 v0.10.9
   Compiling anstream v1.0.0
   Compiling digest v0.11.3
   Compiling regex-automata v0.4.18
   Compiling ecdsa v0.16.9
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-parser)
   Compiling bstr v1.13.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling similar v2.7.0
   Compiling same-file v1.0.6
   Compiling strsim v0.11.1
   Compiling log v0.4.34
   Compiling cpufeatures v0.3.1
   Compiling sha2 v0.11.0
   Compiling globset v0.4.20
   Compiling clap_builder v4.6.7
   Compiling walkdir v2.5.0
   Compiling clap_derive v4.6.7
   Compiling k256 v0.13.4
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-checker)
   Compiling either v1.19.0
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling rayon v1.12.0
   Compiling mori-source6 v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-source6)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-core)
   Compiling clap v4.6.7
   Compiling ignore v0.4.33
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-fmt)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 53.46s
     Running tests/source6_expand.rs (/tmp/doctorwatson-source6-min-50vxmzgd/target/debug/deps/source6_expand-ccc8d149b77de772)

```

## Complete file: actual-check/public-quantity-min/RESULT.json
```text
{
  "finishedAt": "2026-10-18T11:45:15.630811+00:00",
  "exitCode": 0,
  "wallSeconds": 53.4942627690034,
  "sourceCommitAfter": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
  "sourceTreeAfter": "f529a1924885019aa884bc6447f8d47fff47526f",
  "sourceStatusAfter": "",
  "stdoutSha256": "aef1a780855988721551681e64144e3535d5f0c09b55db987c6f9e79d41ca2fc",
  "stderrSha256": "b6a4a2f598317a48cc513e52c37375fc15e68220eea06af84898fca062e4d842",
  "attemptConsumed": true
}

```

## Complete file: actual-check/public-quantity-min/PROCESS-CLEANUP.json
```text
{
  "ownedSession": 661909,
  "remainingSessionMembers": [],
  "scope": "Owned SID census only; no syscall/detached-process exclusion."
}

```

## Complete file: actual-check/run_actual.py
```text
from pathlib import Path
import subprocess,datetime,time,json,hashlib,tempfile,shutil,os,signal
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018');C=R/'source6-capped-remittance-check-cycle';O=R/'source6-capped-remittance-independent-oracle-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def git(*a):return subprocess.check_output(['git',*a],cwd=K).decode().strip()
source=json.loads((C/'CURRENT-SOURCE.json').read_text());head=source['head'];tree=source['tree'];assert git('rev-parse','HEAD')==head and not git('status','--porcelain=v1')
assert sha(O/'FREEZE.sha256')=='4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79'
lines=(O/'FREEZE.sha256').read_text().splitlines();assert len(lines)==23
for line in lines:
 h,name=line.split(maxsplit=1);name=name.lstrip('*');assert sha(O/name)==h,name
for row in source['files']:assert sha(K/row['path'])==row['sha256']
prior=json.loads((R/'core2-partial-delivery-check-cycle/SOURCE-PINS.json').read_text())['files']
paths={r['path'] for r in prior}|{r['path'] for r in source['files']}|set(json.loads((R/'source6-public-consumer-check-cycle/CURRENT-SOURCE.json').read_text())['changedPaths'])|{'crates/mori/src/commands/init.rs','crates/mori/tests/init.rs'}
pins={}
for path in sorted(paths):
 expected_bytes=subprocess.check_output(['git','show',head+':'+path],cwd=K)
 actual_bytes=(K/path).read_bytes();assert actual_bytes==expected_bytes,path
 pins[path]={'path':path,'sha256':hashlib.sha256(expected_bytes).hexdigest(),'bytes':len(expected_bytes),'gitBlob':git('rev-parse',head+':'+path)}
write(C/'SOURCE-PINS.json',{'sourceCommit':head,'sourceTree':tree,'files':list(pins.values())})
root=Path(tempfile.mkdtemp(prefix='doctorwatson-source6-min-'));(root/'cwd').mkdir();(root/'tmp').mkdir();(root/'cargo-home').mkdir()
for name in ['registry','git']:(root/'cargo-home'/name).symlink_to(Path('/home/charl/.cargo')/name,target_is_directory=True)
toolchain=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin');tools=[toolchain/'cargo',toolchain/'rustc',toolchain/'rustdoc',Path('/usr/bin/timeout')];toolpins=[{'path':str(p),'resolved':str(p.resolve()),'sha256':sha(p.resolve())} for p in tools]
configs=[]
for directory in list(K.parents)+[K]+list((root/'cwd').parents)+[root/'cwd']:
 for filename in ['config','config.toml']:
  p=directory/'.cargo'/filename
  if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
env={'PATH':str(toolchain)+':/usr/bin:/bin','RUSTC':str(toolchain/'rustc'),'RUSTDOC':str(toolchain/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
steps=[('public-quantity-min','mori','source6_expand')]
plan={'preparedAt':now(),'sourceCommit':head,'sourceTree':tree,'authorization':'Existing ordinary source/tooling tests; no SDK/native/model/key/proof/ledger execution. No historical cap reset.','sourceAudit':'Root inspected ordinary test/source/input and existing consumer; fresh AGY full source/result review before merge','oracleFreezeSha256':sha(O/'FREEZE.sha256'),'oracleMembersVerified':23,'independentScope':'Independent source values, complete semantic Source6 statement vectors and conditional financial frames for two min branches, fee0; prior Source6 exposure disclosed. Raw expected presentation authored separately; no whole Program/Artifact/hash/work/gate qualification oracle.','ownedTemporaryRoot':str(root),'publicCacheSymlinks':['registry','git'],'baseEnvironment':env,'toolPins':toolpins,'absentAncestorConfigs':configs,'attemptsPerStep':1,'timeoutSecondsPerStep':90,'killGraceSeconds':5,'sharedColdTarget':'One newly selected quantity-min test on existing Source6 CLI target; no separate warmup/install/fetch.','RAMDiskAggregateCaps':None,'originalExitsRemainOpen':True}
write(C/'CHECK-PLAN.json',plan);results=[]
try:
 for name,package,target in steps:
  d=C/name;d.mkdir();cmd=[str(toolchain/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p',package,'--no-default-features','--features','source6-expansion','--test',target,'--','quantity_min_selects_both_operands_through_public_source6_expansion','--exact','--test-threads=1'];argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s','90s',*cmd]
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
  write(d/'START.json',{'startedAt':now(),'sourceCommit':head,'sourceTree':tree,'sourceStatus':git('status','--porcelain=v1'),'argv':argv,'cwd':str(root/'cwd'),'environment':env,'sourcePinsVerified':len(pins),'attempt':1,'attemptLimit':1})
  start=time.monotonic()
  with (d/'STDOUT.log').open('wb') as out,(d/'STDERR.log').open('wb') as err:
   child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdout=out,stderr=err,start_new_session=True);sid=child.pid;write(d/'CHILD.json',{'pid':sid,'session':sid});code=child.wait()
  remaining=[]
  for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines():
   pid,s=line.split()
   if int(s)==sid:remaining.append(int(pid))
  write(d/'PROCESS-CLEANUP.json',{'ownedSession':sid,'remainingSessionMembers':remaining,'scope':'Owned SID census only; no syscall/detached-process exclusion.'});assert not remaining,remaining
  result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'sourceCommitAfter':git('rev-parse','HEAD'),'sourceTreeAfter':git('rev-parse','HEAD^{tree}'),'sourceStatusAfter':git('status','--porcelain=v1'),'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'attemptConsumed':True};write(d/'RESULT.json',result);results.append({'step':name,**result});print(json.dumps(results[-1]),flush=True)
  assert code==0 and result['sourceCommitAfter']==head and result['sourceTreeAfter']==tree and not result['sourceStatusAfter']
  for p,row in pins.items():assert sha(K/p)==row['sha256'],p
finally:
 shutil.rmtree(root);write(C/'CLEANUP.json',{'recordedAt':now(),'ownedTemporaryRoot':str(root),'existsAfter':root.exists(),'scope':'Only owned temporary root removed, public cache symlinks not traversed.'});write(C/'RESULT.json',{'recordedAt':now(),'sourceCommit':head,'sourceTree':tree,'steps':results,'temporaryRootExistsAfter':root.exists(),'allStepsCompleted':len(results)==1,'allExitZero':len(results)==1 and all(x['exitCode']==0 for x in results),'scope':'One public CLI regression: beta check plus both min-action raw Source6 text outputs, zero fee and three retained opening cells. Actual TypeScript lowerer/native/proof/ledger/full grammar/Program/Artifact remain unobserved by this command.'})

```
