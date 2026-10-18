# Current capped remittance pipeline: ordinary source and actual-result review

Requested reviewer: fresh AGY Gemini 4 Argon High, HIGH effort, one 420s request. Review only ordinary finance/tooling/source/result correspondence. No security opinion, hostile test design, product execution, SDK/native/proof/ledger or merge. Root alone merges.

Exact source: base e4d7a2b5709caa18a94f5256d03a0302364b8235 -> head 5acfebe651af47e8edd6741bba298d7336a03b0e / tree abb61a07e502d02a2f6b25f8f20e276246a51c80. Full candidate has31paths: original5 Rust test/fixtures plus26 additive Node/oracle paths. Candidate/ holds whole31files; actual-source/ holds full actual build pins, direct TypeScript import closure, current Rust compiler and scope/normative sources. The whole modified Rust test file includes unchanged earlier tests; no security judgment is requested on those.

Actual current evidence: cold Rust build exit0 41.93541553099931s; Node2tests pass 0.20155944900034228s; all actual stdout/errors, complete AST/lowered/outcome objects retained before assertions. Both returned PreparedUnqualified: statuses are observed, NOT oracle-asserted. Independent complete Source6 ASTs,5effects and conditional balance/allowance/head/round/selected-replay projections match. Fee=0. Do not elevate this to whole Core post/Program/Target/Artifact/work-cost/authentication/ledger qualification. The23member independent freeze4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79 predates current actual checks; provenance discloses prior Source6 exposure. Full original source proposal and original freeze remain unmodified.

Historical Rusttext one-test pass/7filtered 53.494s belongs only to34257f3ef95e7a1bf29031417e7f6d33f708f06a treef529a1924885019aa884bc6447f8d47fff47526f. Those5files are unchanged at5acf, but that test was NOT rerun at5acf. Current cold build and Node results belong to5acf/abb. Older5path review runs separately: infer no judgment from it.

Review GuardA ONLY as INTEGRATION-GUARDS.json: exact main e4d plus exact candidate5acf/treeabb; no other base is allowed. Root withdrew prospectiveGuardB before this request. A prior5path review is ineligible and supplies no vote; review all31paths fresh from actual code and receipts. FACTUAL-SHEET.json records exact actual argv/temp paths, cleanup and new Rust assertions to prevent narrative drift. No LSP scope is included.

Read PRIMARY fully, every complete candidate file physically, and necessary full actual-source compiler/import/consumer/normative/scope files. Do not claim unread supplement coverage. PRIMARY stays under85000bytes; supplements are full physical sources, not excerpts hidden behind a summary. Report incomplete coverage as a limitation/abstention when material. No commands or code execution; file reads only. Preserve complete returned variants without splicing.


## WHOLE FILE candidate/crates/mori/tests/source6_expand.rs

```
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

## WHOLE FILE candidate/experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs

```
// Opt-in public Rust CLI -> actual provisional Source/6/Core/5 consumers.
// New independent oracle custody; old Source/6 transfer fixtures are untouched.
import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseAndLowerSource6 } from '../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified } from '../src/successor/mil4-s0-source-v6.ts';

const fixtures = fileURLToPath(new URL('./fixtures/source6-capped-remittance/', import.meta.url));
const binary = process.env.MORI_RUST_BIN;
const observations = process.env.MORI_SOURCE6_OBSERVATIONS;
assert.ok(binary && isAbsolute(binary), 'MORI_RUST_BIN must name the absolute reviewed Rust binary');
assert.ok(observations && isAbsolute(observations), 'MORI_SOURCE6_OBSERVATIONS must name a fresh absolute directory');
mkdirSync(observations);
const json = (name) => JSON.parse(readFileSync(join(fixtures, name), 'utf8'));
const pins = json('FIXTURE-PINS.json');
for (const [name, expected] of Object.entries(pins.files)) {
  const bytes = readFileSync(join(fixtures, name));
  assert.equal(createHash('sha256').update(bytes).digest('hex'), expected,
    `custodian fixture bytes changed: ${name}`);
}
const facts = json('oracle/EXPECTED-SEMANTICS.json');
assert.equal(facts.format, 'independent-capped-remittance-semantic-oracle/1');
assert.equal(facts.cases.length, 2);

// These helpers only serialize observations and select named financial fields.
// They never calculate amounts, simulate a transfer or construct product ASTs.
function captureJson(path, value) {
  const encoded = JSON.stringify(value, null, 2) + '\n';
  writeFileSync(path, encoded);
  return JSON.parse(encoded);
}
function errorRecord(error) {
  return {
    name: error?.name ?? typeof error,
    message: error?.message ?? String(error),
    code: error?.code ?? null,
    offset: error?.offset ?? null,
    stack: error?.stack ?? null,
  };
}
function financialFrame(state) {
  return {
    domain: state.domain, asset: state.asset,
    balances: state.balances, allowances: state.allowances,
  };
}

captureJson(join(observations, 'run.json'), {
  node: process.version, binary,
  binarySha256: createHash('sha256').update(readFileSync(binary)).digest('hex'),
  oracleFreezeSha256: pins.custodianFreezeSha256,
  oracleManifestSha256: pins.custodianManifestSha256,
  expectedSemanticsSha256: pins.expectedSemanticsSha256,
  unasserted: ['whole emitted wire bytes/whitespace', 'Core status/qualification/gates',
    'measured work/cost and post-work counters', 'complete Core post/global replay inventory',
    'whole Program/Target/Artifact', 'authentication', 'native/proof/ledger', 'parent acceptance'],
});
// Preserve exact run inputs and the oracle manifest alongside the observations.
for (const name of ['capped-remittance.mori', 'capped-remittance.scenario.json']) {
  writeFileSync(join(observations, name), readFileSync(join(fixtures, 'oracle/inputs', name)));
}
writeFileSync(join(observations, 'oracle-MANIFEST.json'), readFileSync(join(fixtures, 'oracle/MANIFEST.json')));
writeFileSync(join(observations, 'oracle-FREEZE.sha256'), readFileSync(join(fixtures, 'oracle/FREEZE.sha256')));

for (const expected of facts.cases) {
  test(`${expected.action}: actual public Source/6 bytes against independent semantic observations`, () => {
    const expectedAst = json(`oracle/${expected.source6AstFile}`);
    const scenario = json('oracle/inputs/capped-remittance.scenario.json');
    const post = expected.conditionalClosingFinancialProjection;
    const args = ['--quiet', '--color', 'never', 'expand',
      join(fixtures, 'oracle/inputs/capped-remittance.mori'), '--action', expected.action,
      '--scenario', join(fixtures, 'oracle/inputs/capped-remittance.scenario.json'),
      '--emit-source6-transfer'];
    const output = spawnSync(binary, args, {
      cwd: fixtures, stdio: ['ignore', 'pipe', 'pipe'], shell: false,
      timeout: 30_000, maxBuffer: 1024 * 1024,
    });
    const destination = join(observations, expected.action);
    mkdirSync(destination);
    const stdout = output.stdout ?? Buffer.alloc(0);
    const stderr = output.stderr ?? Buffer.alloc(0);
    // Every actual process byte is saved before any product result assertion.
    writeFileSync(join(destination, 'stdout.source6'), stdout);
    writeFileSync(join(destination, 'stderr.txt'), stderr);
    captureJson(join(destination, 'process.json'), {
      executable: binary, args, cwd: fixtures, status: output.status, signal: output.signal,
      error: output.error ? errorRecord(output.error) : null,
      stdoutSha256: createHash('sha256').update(stdout).digest('hex'),
      stderrSha256: createHash('sha256').update(stderr).digest('hex'),
    });

    const actual = {
      decodeError: null, loweringAttempted: false, loweringError: null, lowered: null,
      preparationAttempted: false, preparationError: null, outcome: null,
    };
    let source6;
    try { source6 = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(stdout); }
    catch (error) { actual.decodeError = errorRecord(error); }
    if (source6 !== undefined) {
      // Untouched decoded stdout goes directly to both actual APIs. No trim,
      // fixture Source/6 substitution, JSON unwrap, AST construction or replay.
      actual.loweringAttempted = true;
      try { actual.lowered = parseAndLowerSource6(source6); }
      catch (error) { actual.loweringError = errorRecord(error); }
      actual.preparationAttempted = true;
      try { actual.outcome = prepareSource6S0Unqualified(source6); }
      catch (error) { actual.preparationError = errorRecord(error); }
    }
    // Capture complete returned objects or thrown errors before checking them.
    // JSON is the observation representation: undefined optional members are
    // omitted by JSON serialization; no product field value is recalculated.
    const observed = captureJson(join(destination, 'actual-consumer.json'), actual);

    assert.ifError(output.error);
    assert.equal(output.signal, null);
    assert.equal(output.status, 0, stderr.toString('utf8'));
    assert.equal(stderr.length, 0, '--quiet must suppress the adopted public status line');
    assert.ok(stdout.length > 0);
    assert.equal(stdout.at(-1), 10, 'adopted public output includes its final LF');
    assert.notEqual(stdout.at(-2), 10, 'adopted public output has a single final LF');
    assert.equal(observed.decodeError, null);
    assert.equal(observed.loweringError, null);
    assert.equal(observed.preparationError, null);
    assert.ok(observed.loweringAttempted && observed.preparationAttempted);
    const { ast, state, intent } = observed.lowered;
    // Full independent Source/6 AST, including every ordered effect and all
    // source/intent/claim/input fields. This is not a Core Program oracle.
    assert.deepEqual(ast, expectedAst);
    assert.deepEqual(ast.submitted.effects, expected.source6OrderedEffects);
    assert.equal(ast.submitted.effects.length, expected.effectCount);
    assert.deepEqual(observed.outcome.ast, expectedAst);

    assert.deepEqual(financialFrame(state), expected.openingFinancialFrame);
    assert.equal(state.head, expectedAst.authenticated.head);
    assert.equal(state.round, expectedAst.authenticated.round);
    assert.equal(state.workRemaining, expected.source6InputWork.remaining);
    assert.equal(state.workSpent, expected.source6InputWork.spent);
    assert.equal(state.consumedReplay.includes(post.selectedReplay.encodedKey), false);
    assert.deepEqual({
      sourceProfile: intent.sourceProfile, programId: intent.programId,
      sourceHash: intent.sourceHash, policyDigest: intent.policyDigest, keyRef: intent.keyRef,
      domain: intent.domain, asset: intent.asset, signer: intent.signer, nonce: intent.nonce,
      preHead: intent.preHead, notBefore: intent.notBefore, notAfter: intent.notAfter,
      grossCap: intent.grossCap, feeCap: intent.feeCap, netFloor: intent.netFloor,
      kind: intent.kind, recipient: intent.recipient, feeRecipient: intent.feeRecipient,
      amount: intent.amount, fee: intent.fee,
    }, {
      sourceProfile: expectedAst.profile, programId: expectedAst.selected.actionId,
      sourceHash: expectedAst.selected.sourceHash, policyDigest: expectedAst.selected.policyDigest,
      keyRef: expectedAst.intent.keyRef, domain: expectedAst.domain, asset: expectedAst.settlement.asset,
      signer: expectedAst.intent.signer, nonce: expectedAst.intent.nonce,
      preHead: expectedAst.intent.preHead, notBefore: expectedAst.intent.notBefore,
      notAfter: expectedAst.intent.notAfter, grossCap: expected.grossCap,
      feeCap: expected.feeCap, netFloor: expected.netFloor,
      kind: expectedAst.intent.signedAction.kind, recipient: expectedAst.intent.signedAction.to,
      feeRecipient: expectedAst.intent.signedAction.feeTo, amount: expected.value, fee: expected.fee,
    });
    assert.deepEqual(observed.lowered.submittedEffects, expected.core5SemanticOrderedEffects);
    assert.equal(observed.lowered.proposedPostHead, post.head);

    // Require the actual financial projection for the positive comparison;
    // status, qualification, required-premise and work-cost strings are only
    // observed in actual-consumer.json, never asserted as an oracle verdict.
    const candidate = observed.outcome.candidate;
    assert.ok(candidate && candidate.candidatePost, 'actual consumer must expose the compared financial candidate');
    assert.equal(candidate.preHead, expectedAst.authenticated.head);
    assert.deepEqual(candidate.effects, expected.core5SemanticOrderedEffects);
    assert.equal(candidate.effects.length, expected.effectCount);
    assert.deepEqual(financialFrame(candidate.candidatePost), {
      domain: post.domain, asset: post.asset, balances: post.balances, allowances: post.allowances,
    });
    assert.equal(candidate.candidatePost.head, post.head);
    assert.equal(candidate.candidatePost.round, post.round);
    // The independent oracle fixes the selected key, not a global replay
    // inventory or a security/history judgment about consuming that key.
    const selectedKey = candidate.candidatePost.consumedReplay
      .find((key) => key === post.selectedReplay.encodedKey);
    assert.equal(selectedKey, post.selectedReplay.encodedKey);
    assert.deepEqual(JSON.parse(selectedKey), post.selectedReplay.tuple);
    // No complete Core post, residual-duty DTO, post-work or qualification
    // oracle is invented. Their actual fields remain in the captured result.
  });
}

```

## WHOLE FILE candidate/crates/mori/tests/fixtures/source6-expansion/capped-remittance.mori

```
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

## WHOLE FILE candidate/crates/mori/tests/fixtures/source6-expansion/capped-remittance.scenario.json

```
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

## WHOLE FILE candidate/crates/mori/tests/fixtures/source6-expansion/capped-tight.source6

```
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

## WHOLE FILE candidate/crates/mori/tests/fixtures/source6-expansion/capped-loose.source6

```
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

## WHOLE FILE candidate/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/EXPECTED-SEMANTICS.json

```
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

## WHOLE FILE candidate/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/pay_tight.source6.ast.expected.json

```
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

## WHOLE FILE candidate/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/pay_loose.source6.ast.expected.json

```
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

## WHOLE FILE candidate/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/DERIVATION.md

```
# Independent capped-remittance derivation

The only chosen case data read were capped-remittance.mori and capped-remittance.scenario.json. The new candidate test, capped-tight.source6, capped-loose.source6, INDEPENDENT-SOURCE-SHAPE and actual outputs were not opened, hashed or copied. Normative/reference documents and DTO declarations were pinned directly to74a909d69639df8823a527a4d608a7516ccd69a2 using explicitly authorized read-only Git queries. No producer, parser, checker, evaluator, emitter, compiler, SDK or runtime was called.

USD is a source alias; its economic asset is A in Midnight at scale2. Buyer/Seller/Treasury map to Owner/Recipient/Fee. Exact base10 conversion gives0.75USD=75atoms,2.10USD=210,2.50USD=250. Scalar3 produces75*3=225atoms without rounding. min(225,210)=210 chooses tightLimit(b); min(225,250)=225 chooses quoted(a). atoms(USD,0)=0. Both gross caps are amount+0, fee caps0, and net floors equal amount. All shown intermediates are nonnegative and below UInt128 and the S0 nominal2^127-1 bound; no floatingpoint or product numeric function is involved.

Both actions independently use the same opening frame: Owner1000,Recipient5,Fee7, allowance remaining1000/spent25. Tight debit210 gives Owner790/Recipient215/Fee7, allowance790/235. Loose debit225 gives Owner775/Recipient230/Fee7, allowance775/250. Provided balances total1012 in each arithmetic frame; allowance remaining+spent remains1025. These are arithmetic cross-checks of this finite conditional projection, not a security/conservation proof or observed acceptance. They are alternatives, not sequential transactions.

The Source6 contract mandates Debit(owner,value+fee), Credit(recipient,value), optional Credit(feeRecipient,fee) iff fee>0, UseAllowance(owner,gross), UseReplay(key), AdvanceHead(pre,post). Fee0 omits the optional credit entirely; it does not remove Fee's required third opening row or reset7 to0. Exactly five effects remain. Both signed_action and submit still explicitly say fee_to Fee and fee0. The text nonce is nC; the separately labeled Core5 semantic effect uses the encoded domain/signer/nonce tuple [Midnight,Owner,nC]. AdvanceHead uses currentheadc0->c1, not authenticated.predecessor prior. Round4 is retained, not incremented.

Every Source6 statement/block is frozen in each independent-source6 text and ordered-target-lines file. The typed Source6Ast expectation separately retains agreement CappedRemittance, scale2 and predecessor prior; selected.actionId is the normative profile tag TransferLiteralFee, not pay_tight/pay_loose. The input work12/2 is carried exactly in authenticated fields. Target indentation is independently chosen from the EBNF; exact stdout/whitespace is not asserted. No expected compiler Program/Artifact, hash identity, measured work, gate outcome, qualification, complete Core post or ledger result is supplied; see UNKNOWN.json.

Previous exposure is explicit: this author handled earlier Source6 owner packets with Rust Source6 CLI/library/emitter, lexer/checker source, old invoice/workshop inputs/tests and artifact copies. This is not global pre-implementation blindness. In this lane no producer emission or new authored oracle/test was used. Only DTO declaration ranges were used from the Source6/S0 TS modules; an earlier displayed DTO range incidentally included the adjacent Parser class header/fields and numeric-bound declarations, not parsing/lowering execution or chosen-case results. That incidental source exposure is recorded rather than hidden.

```

## WHOLE FILE candidate/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/UNKNOWN.json

```
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

## WHOLE FILE actual-current/RESULT.json

```
{
  "recordedAt": "2026-10-18T11:52:08.891985+00:00",
  "sourceCommit": "5acfebe651af47e8edd6741bba298d7336a03b0e",
  "sourceTree": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
  "steps": [
    {
      "step": "build-public-rust-binary",
      "finishedAt": "2026-10-18T11:52:08.418953+00:00",
      "exitCode": 0,
      "wallSeconds": 41.93541553099931,
      "sourceCommitAfter": "5acfebe651af47e8edd6741bba298d7336a03b0e",
      "sourceTreeAfter": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
      "sourceStatusAfter": "",
      "stdoutSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "stderrSha256": "f3abd6a67619f3cc6dd0ddf21514ad3dc628eedf23f2c6117a7a973fba7b9b9c",
      "attemptConsumed": true
    },
    {
      "step": "node-capped-public-consumer",
      "finishedAt": "2026-10-18T11:52:08.800222+00:00",
      "exitCode": 0,
      "wallSeconds": 0.20155944900034228,
      "sourceCommitAfter": "5acfebe651af47e8edd6741bba298d7336a03b0e",
      "sourceTreeAfter": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
      "sourceStatusAfter": "",
      "stdoutSha256": "c6b685aca70758df46854f95f2c324389db9a602ea76575284d0df77d45f4d91",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "attemptConsumed": true
    }
  ],
  "temporaryRootExistsAfter": false,
  "allStepsCompleted": true,
  "allExitZero": true,
  "scope": "Real Rust output for both min branches through actual experimental Source6 parser/lowerer and provisional S0 consumer; complete independent semantic AST/effect/financial projections only. No wholeCoreProgram/Artifact/measurement/authentication/native/proof/ledger qualification."
}

```

## WHOLE FILE actual-current/node-observations/run.json

```
{
  "node": "v24.21.0",
  "binary": "/tmp/doctorwatson-source6-min-node-b562prfk/target/debug/mori",
  "binarySha256": "15a8e785e80868c2f56243523c72b9012ed689e346a35382309832d4a310316d",
  "oracleFreezeSha256": "4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79",
  "oracleManifestSha256": "c911856c73f9970bc87187ad132412b1f142c4721c2891cb5dcfe1cb5758b5d1",
  "expectedSemanticsSha256": "da997f7f4f59300007cb0368b2d96b728049623315e6649e1bf9c6cccf6fd624",
  "unasserted": [
    "whole emitted wire bytes/whitespace",
    "Core status/qualification/gates",
    "measured work/cost and post-work counters",
    "complete Core post/global replay inventory",
    "whole Program/Target/Artifact",
    "authentication",
    "native/proof/ledger",
    "parent acceptance"
  ]
}

```

## WHOLE FILE actual-current/node-observations/pay_tight/actual-consumer.json

```
{
  "decodeError": null,
  "loweringAttempted": true,
  "loweringError": null,
  "lowered": {
    "ast": {
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
    },
    "state": {
      "core": "moriarty-core/5",
      "domain": "Midnight",
      "asset": "A",
      "head": "c0",
      "round": "4",
      "workRemaining": "12",
      "workSpent": "2",
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
      ],
      "obligations": [],
      "consumedReplay": []
    },
    "intent": {
      "version": "moriarty-intent/3",
      "core": "moriarty-core/5",
      "sourceProfile": "moriarty-financial-agreement-source/6",
      "programId": "TransferLiteralFee",
      "sourceHash": "srcC",
      "policyDigest": "policyC",
      "keyRef": "keyC",
      "domain": "Midnight",
      "asset": "A",
      "signer": "Owner",
      "nonce": "nC",
      "preHead": "c0",
      "notBefore": "2",
      "notAfter": "20",
      "grossCap": "210",
      "feeCap": "0",
      "netFloor": "210",
      "kind": "Transfer",
      "recipient": "Recipient",
      "feeRecipient": "Fee",
      "amount": "210",
      "fee": "0"
    },
    "submittedEffects": [
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
    "proposedPostHead": "c1"
  },
  "preparationAttempted": true,
  "preparationError": null,
  "outcome": {
    "status": "PreparedUnqualified",
    "ast": {
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
    },
    "candidate": {
      "status": "PreparedUnqualified",
      "core": "moriarty-core/5",
      "preHead": "c0",
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
          "key": "[\"Midnight\",\"Owner\",\"nC\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "c0",
          "successor": "c1"
        }
      ],
      "candidatePost": {
        "core": "moriarty-core/5",
        "domain": "Midnight",
        "asset": "A",
        "head": "c1",
        "round": "4",
        "workRemaining": "11",
        "workSpent": "3",
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
        "obligations": [],
        "consumedReplay": [
          "[\"Midnight\",\"Owner\",\"nC\"]"
        ]
      },
      "requiredPremises": [
        "canonical-intent-signature",
        "snapshot-to-head",
        "head-extension",
        "atomic-ledger-compare-and-consume"
      ]
    },
    "unverifiedBindings": [
      "agreement-id",
      "selected-program",
      "asset-scale",
      "authenticated-predecessor"
    ]
  }
}

```

## WHOLE FILE actual-current/node-observations/pay_loose/actual-consumer.json

```
{
  "decodeError": null,
  "loweringAttempted": true,
  "loweringError": null,
  "lowered": {
    "ast": {
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
    },
    "state": {
      "core": "moriarty-core/5",
      "domain": "Midnight",
      "asset": "A",
      "head": "c0",
      "round": "4",
      "workRemaining": "12",
      "workSpent": "2",
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
      ],
      "obligations": [],
      "consumedReplay": []
    },
    "intent": {
      "version": "moriarty-intent/3",
      "core": "moriarty-core/5",
      "sourceProfile": "moriarty-financial-agreement-source/6",
      "programId": "TransferLiteralFee",
      "sourceHash": "srcC",
      "policyDigest": "policyC",
      "keyRef": "keyC",
      "domain": "Midnight",
      "asset": "A",
      "signer": "Owner",
      "nonce": "nC",
      "preHead": "c0",
      "notBefore": "2",
      "notAfter": "20",
      "grossCap": "225",
      "feeCap": "0",
      "netFloor": "225",
      "kind": "Transfer",
      "recipient": "Recipient",
      "feeRecipient": "Fee",
      "amount": "225",
      "fee": "0"
    },
    "submittedEffects": [
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
    "proposedPostHead": "c1"
  },
  "preparationAttempted": true,
  "preparationError": null,
  "outcome": {
    "status": "PreparedUnqualified",
    "ast": {
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
    },
    "candidate": {
      "status": "PreparedUnqualified",
      "core": "moriarty-core/5",
      "preHead": "c0",
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
          "key": "[\"Midnight\",\"Owner\",\"nC\"]"
        },
        {
          "kind": "AdvanceHead",
          "predecessor": "c0",
          "successor": "c1"
        }
      ],
      "candidatePost": {
        "core": "moriarty-core/5",
        "domain": "Midnight",
        "asset": "A",
        "head": "c1",
        "round": "4",
        "workRemaining": "11",
        "workSpent": "3",
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
        "obligations": [],
        "consumedReplay": [
          "[\"Midnight\",\"Owner\",\"nC\"]"
        ]
      },
      "requiredPremises": [
        "canonical-intent-signature",
        "snapshot-to-head",
        "head-extension",
        "atomic-ledger-compare-and-consume"
      ]
    },
    "unverifiedBindings": [
      "agreement-id",
      "selected-program",
      "asset-scale",
      "authenticated-predecessor"
    ]
  }
}

```

## WHOLE FILE actual-current/node-observations/pay_tight/stdout.source6

```
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

## WHOLE FILE actual-current/node-observations/pay_loose/stdout.source6

```
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

## WHOLE FILE actual-current/node-observations/pay_tight/process.json

```
{
  "executable": "/tmp/doctorwatson-source6-min-node-b562prfk/target/debug/mori",
  "args": [
    "--quiet",
    "--color",
    "never",
    "expand",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/inputs/capped-remittance.mori",
    "--action",
    "pay_tight",
    "--scenario",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/inputs/capped-remittance.scenario.json",
    "--emit-source6-transfer"
  ],
  "cwd": "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/",
  "status": 0,
  "signal": null,
  "error": null,
  "stdoutSha256": "5616a8fe33a41947d34833d52764a72bf602bd659bc1017d80fed48a738e7e6e",
  "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
}

```

## WHOLE FILE actual-current/node-observations/pay_loose/process.json

```
{
  "executable": "/tmp/doctorwatson-source6-min-node-b562prfk/target/debug/mori",
  "args": [
    "--quiet",
    "--color",
    "never",
    "expand",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/inputs/capped-remittance.mori",
    "--action",
    "pay_loose",
    "--scenario",
    "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/oracle/inputs/capped-remittance.scenario.json",
    "--emit-source6-transfer"
  ],
  "cwd": "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-language/regressions/fixtures/source6-capped-remittance/",
  "status": 0,
  "signal": null,
  "error": null,
  "stdoutSha256": "21e1db1eba67e5bc1a446a65ad0754dea63dab3490e165f86369b14f11549bdf",
  "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
}

```

## WHOLE FILE actual-current/node-capped-public-consumer/STDOUT.log

```
✔ pay_tight: actual public Source/6 bytes against independent semantic observations (16.27107ms)
✔ pay_loose: actual public Source/6 bytes against independent semantic observations (6.378985ms)
ℹ tests 2
ℹ suites 0
ℹ pass 2
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 166.462404

```

## WHOLE FILE actual-current/node-capped-public-consumer/STDERR.log

```

```

## WHOLE FILE actual-current/build-public-rust-binary/RESULT.json

```
{
  "finishedAt": "2026-10-18T11:52:08.418953+00:00",
  "exitCode": 0,
  "wallSeconds": 41.93541553099931,
  "sourceCommitAfter": "5acfebe651af47e8edd6741bba298d7336a03b0e",
  "sourceTreeAfter": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
  "sourceStatusAfter": "",
  "stdoutSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "stderrSha256": "f3abd6a67619f3cc6dd0ddf21514ad3dc628eedf23f2c6117a7a973fba7b9b9c",
  "attemptConsumed": true
}

```

## WHOLE FILE actual-current/node-capped-public-consumer/RESULT.json

```
{
  "finishedAt": "2026-10-18T11:52:08.800222+00:00",
  "exitCode": 0,
  "wallSeconds": 0.20155944900034228,
  "sourceCommitAfter": "5acfebe651af47e8edd6741bba298d7336a03b0e",
  "sourceTreeAfter": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
  "sourceStatusAfter": "",
  "stdoutSha256": "c6b685aca70758df46854f95f2c324389db9a602ea76575284d0df77d45f4d91",
  "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "attemptConsumed": true
}

```

## WHOLE FILE INTEGRATION-GUARDS.json

```
{
  "guardA": {
    "allowedBaseExactly": "e4d7a2b5709caa18a94f5256d03a0302364b8235",
    "candidateExactly": "5acfebe651af47e8edd6741bba298d7336a03b0e",
    "candidateTreeExactly": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
    "diffPathCount": 31
  },
  "actualResultScopes": {
    "currentNodeBuildAndTwoCases": {
      "head": "5acfebe651af47e8edd6741bba298d7336a03b0e",
      "tree": "abb61a07e502d02a2f6b25f8f20e276246a51c80"
    },
    "historicalRustOneTestSevenFiltered": {
      "head": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
      "tree": "f529a1924885019aa884bc6447f8d47fff47526f",
      "notExecutedAtCurrentHead": true
    }
  },
  "rootAloneMerges": true,
  "otherBasesAllowed": false,
  "prospectiveGuardB": "Removed before dispatch under root update; not in current request."
}

```

## WHOLE FILE FACTUAL-SHEET.json

```
{
  "status": "Source and recorded experiment observations only; no review verdict inferred",
  "scope": "Fresh31path ordinary pipeline review at5acf/abb; noLSP changes",
  "current": {
    "buildStart": {
      "sourceCommit": "5acfebe651af47e8edd6741bba298d7336a03b0e",
      "sourceTree": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
      "argv": [
        "/usr/bin/timeout",
        "--signal=TERM",
        "--kill-after=5s",
        "90s",
        "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
        "build",
        "--manifest-path",
        "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/Cargo.toml",
        "--frozen",
        "-j1",
        "-p",
        "mori",
        "--no-default-features",
        "--features",
        "source6-expansion",
        "--bin",
        "mori"
      ],
      "cwd": "/tmp/doctorwatson-source6-min-node-b562prfk/cwd",
      "sourcePinsVerified": 594,
      "completeOriginalReceipt": "actual-current/build-public-rust-binary/START.json"
    },
    "nodeStart": {
      "sourceCommit": "5acfebe651af47e8edd6741bba298d7336a03b0e",
      "sourceTree": "abb61a07e502d02a2f6b25f8f20e276246a51c80",
      "argv": [
        "/usr/bin/timeout",
        "--signal=TERM",
        "--kill-after=5s",
        "90s",
        "/usr/local/bin/node",
        "--test",
        "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs"
      ],
      "cwd": "/tmp/doctorwatson-source6-min-node-b562prfk/cwd",
      "sourcePinsVerified": 594,
      "completeOriginalReceipt": "actual-current/node-capped-public-consumer/START.json"
    },
    "cleanup": {
      "recordedAt": "2026-10-18T11:52:08.891800+00:00",
      "ownedTemporaryRoot": "/tmp/doctorwatson-source6-min-node-b562prfk",
      "existsAfter": false,
      "scope": "Only owned temporary root removed, public cache symlinks not traversed."
    }
  },
  "historicalOnly": {
    "start": {
      "sourceCommit": "34257f3ef95e7a1bf29031417e7f6d33f708f06a",
      "sourceTree": "f529a1924885019aa884bc6447f8d47fff47526f",
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
      "sourcePinsVerified": 511,
      "completeOriginalReceipt": "actual-historical-rust/public-quantity-min/START.json"
    },
    "cleanup": {
      "recordedAt": "2026-10-18T11:45:15.722053+00:00",
      "ownedTemporaryRoot": "/tmp/doctorwatson-source6-min-50vxmzgd",
      "existsAfter": false,
      "scope": "Only owned temporary root removed, public cache symlinks not traversed."
    },
    "notRerunAt5acf": true
  },
  "exactNewRustTestAssertions": {
    "checkStatus": "accepted",
    "reportActionsArrayLength": 2,
    "actionNames": [
      "pay_tight",
      "pay_loose"
    ],
    "eachActionSupport": "local_s0",
    "expandedBytes": "Both raw whole expected source6 fixtures; stderr empty; exit0",
    "notAssertedByNewFn": [
      "report.valid",
      "report.agreement",
      "report.intents",
      "local-s0 spelling"
    ]
  },
  "cliHelpersPhysicallyIncluded": [
    "actual-source/crates/mori/src/json.rs",
    "actual-source/crates/mori/src/commands/core_io.rs",
    "actual-source/crates/mori/src/display.rs",
    "actual-source/crates/mori/src/error.rs"
  ],
  "oldReviewDisposition": "Root reports older5path review ineligible due material factual errors. Its whole variants remain preserved at original location; no old review object or verdict is copied or incorporated here. Audit current31paths fresh.",
  "identityDiscipline": "Requested AGY model/effort is not canonical model attestation. Only genuine provider-envelope identity may support returned identity; recursive quoted metadata is insufficient."
}

```
