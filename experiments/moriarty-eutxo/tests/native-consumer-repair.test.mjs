import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { buildApprovedCashPolicy } from '../src/apply-head.mjs';
import {
  bindHelperReceipt,
  collectHeadReadback,
  measureSlotLength,
  prepareEnterpriseKey,
  runOwnedRace,
  selectRunInputs,
} from '../src/native-caller.mjs';
import { compareFinancialFrame, loadSourceTerminals } from '../src/native-financial.mjs';
import {
  CARDANO_CLI,
  acquireBodyEvaluation,
  assessCashPolicy,
  loadAuthority,
  loadRuntimeInputs,
  produceOwnedRuntime,
} from '../src/native-runtime.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const HELPER = 'f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18';
const RECEIPTS = join(TASK, 'tooling/transaction-receipts/revision02');
const CASH = 'validator cash_policy {\n  mint(redeemer, policy, transaction) { void }\n}\n';
const FAIL = 'validator cash {\n  mint(redeemer, policy, transaction) { fail }\n}\nThis policy cannot mint\n';
const SOURCE = join(TASK, 'designers/02-escrow-time/escrow-custody-v5.mori');

function writeExec(path, lines) {
  writeFileSync(path, `${lines.join('\n')}\n`, { mode: 0o755 });
  chmodSync(path, 0o755);
  return path;
}

function sha(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function nativeVotes(dir, sourceText, verdict = 'APPROVE_SCOPED_T1_NATIVE_IMPLEMENTATION') {
  const sourcePath = join(dir, 'source.mori');
  writeFileSync(sourcePath, sourceText);
  const sourceHash = sha(sourceText);
  const manifestPath = join(dir, 'manifest.json');
  writeFileSync(manifestPath, `${JSON.stringify({
    sourceHash,
    reviewClass: 'native-implementation',
    files: [{ path: sourcePath, sha256: sourceHash, bytes: Buffer.byteLength(sourceText) }],
  })}\n`);
  const manifestHash = sha(readFileSync(manifestPath));
  const reviewAuthority = join(dir, 'review-authority.json');
  writeFileSync(reviewAuthority, JSON.stringify({ manifestSha256: manifestHash }));
  const base = {
    verdict,
    candidate: manifestHash,
    evidenceClass: 'reviewed-implementation',
    reviewClass: 'native-implementation',
    sourceHash,
  };
  const reviewA = join(dir, 'a.json');
  const reviewB = join(dir, 'b.json');
  writeFileSync(reviewA, JSON.stringify({ ...base, reviewer: 'reviewer-a' }));
  writeFileSync(reviewB, JSON.stringify({ ...base, reviewer: 'reviewer-b' }));
  return { reviewA, reviewB, candidate: manifestPath, manifest: manifestPath, reviewAuthority, manifestHash, sourceHash };
}

function txidCli(dir, txid) {
  return writeExec(join(dir, 'txid.sh'), [
    '#!/bin/sh',
    'if [ "$1" = "--version" ]; then echo stub-cli; exit 0; fi',
    `printf '%s\\n' '{"txid":"${txid}"}'`,
    'exit 0',
  ]);
}

test('current manifest authority is external and is not a cash or native50 pin', () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-manifest-'));
  try {
    const votes = nativeVotes(dir, CASH);
    const agreed = loadAuthority(votes);
    assert.equal(agreed.code, 'AUTHORITY_AGREED');
    assert.equal(agreed.requiredManifestDigest, votes.manifestHash);
    assert.notEqual(agreed.requiredManifestDigest, 'c7f97ef913d912226bf80b054a538deb70d8133ebcf8c307ac11c3eb5b743007');
    assert.notEqual(agreed.requiredManifestDigest, '4198846d047168a1c7b3f66eb8774757914cc22b98544bbc690dfb3fd635e32d');
    assert.equal(agreed.selfApproval, false);
    assert.equal(agreed.nativePins, false);
    mkdirSync(join(dir, 'generic'), { recursive: true });
    assert.equal(loadAuthority(nativeVotes(join(dir, 'generic'), CASH, 'APPROVE_GENERIC')).code, 'AUTHORITY_NOT_APPROVED');
    mkdirSync(join(dir, 'cash'), { recursive: true });
    assert.equal(loadAuthority(nativeVotes(join(dir, 'cash'), CASH, 'APPROVE_SCOPED_T1_CASH_FIXTURE_DESIGN')).code, 'AUTHORITY_NOT_APPROVED');
    const same = nativeVotes(dir, CASH);
    const row = JSON.parse(readFileSync(same.reviewB, 'utf8'));
    row.reviewer = 'reviewer-a';
    writeFileSync(same.reviewB, JSON.stringify(row));
    assert.equal(loadAuthority(same).code, 'AUTHORITY_REVIEWERS_NOT_DISTINCT');
    mkdirSync(join(dir, 'self'), { recursive: true });
    const selfVotes = nativeVotes(join(dir, 'self'), CASH);
    const selfRow = JSON.parse(readFileSync(selfVotes.reviewA, 'utf8'));
    selfRow.selfApproval = true;
    writeFileSync(selfVotes.reviewA, JSON.stringify(selfRow));
    assert.equal(loadAuthority(selfVotes).code, 'AUTHORITY_SELF_APPROVAL');
    mkdirSync(join(dir, 'mirror'), { recursive: true });
    const mirrorVotes = nativeVotes(join(dir, 'mirror'), CASH);
    const manifest = JSON.parse(readFileSync(mirrorVotes.manifest, 'utf8'));
    manifest.manifestSha256 = mirrorVotes.manifestHash;
    writeFileSync(mirrorVotes.manifest, JSON.stringify(manifest));
    const rewritten = sha(readFileSync(mirrorVotes.manifest));
    for (const file of [mirrorVotes.reviewA, mirrorVotes.reviewB]) {
      const review = JSON.parse(readFileSync(file, 'utf8'));
      review.candidate = rewritten;
      writeFileSync(file, JSON.stringify(review));
    }
    writeFileSync(mirrorVotes.reviewAuthority, JSON.stringify({ manifestSha256: rewritten }));
    const mirrorCandidate = join(dir, 'mirror-candidate.json');
    writeFileSync(mirrorCandidate, readFileSync(mirrorVotes.manifest));
    assert.equal(loadAuthority({ ...mirrorVotes, candidate: mirrorCandidate }).code, 'AUTHORITY_SELF_REFERENCE');
    const historical = join(TASK, 't1-native-caller-50/CANDIDATE-SHA256.json');
    const historicalHash = sha(readFileSync(historical));
    const oldA = join(dir, 'old-a.json');
    const oldB = join(dir, 'old-b.json');
    const old = {
      verdict: 'APPROVE_SCOPED_T1_NATIVE_IMPLEMENTATION',
      candidate: historicalHash,
      evidenceClass: 'reviewed-implementation',
      reviewClass: 'native-implementation',
      sourceHash: '11'.repeat(32),
    };
    writeFileSync(oldA, JSON.stringify({ ...old, reviewer: 'old-a' }));
    writeFileSync(oldB, JSON.stringify({ ...old, reviewer: 'old-b' }));
    const uncovered = loadAuthority({ reviewA: oldA, reviewB: oldB, candidate: historical });
    assert.equal(uncovered.code, 'AUTHORITY_MANIFEST_ABSENT');
    assert.equal(uncovered.requiredManifestDigest, null);
    assert.equal(agreed.reviewClass, 'native-implementation');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('approved cash source is not the fail-closed mint and does not fund', () => {
  const source = readFileSync(join(TASK, 't1-cash-repair-53/prototype/validators/cash_fixture.ak'), 'utf8');
  const open = assessCashPolicy(source, null);
  assert.notEqual(open.code, 'CASH_POLICY_FAIL_CLOSED');
  assert.equal(open.cashCanMint, false);
  assert.equal(assessCashPolicy(FAIL, null).code, 'CASH_POLICY_FAIL_CLOSED');
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-cash-cache-'));
  try {
    const seed = `${'a'.repeat(63)}1`;
    const frozen = join(TASK, `t1-native-repair-59/checks/cash-policy/${seed}-1/cash-policy.json`);
    const work = join(dir, 'work');
    mkdirSync(work, { recursive: true });
    writeFileSync(join(work, 'cash-policy.json'), readFileSync(frozen));
    const compiled = buildApprovedCashPolicy({
      sourcePath: join(TASK, 't1-cash-repair-53/prototype/validators/cash_fixture.ak'),
      seedTxId: seed,
      seedIndex: 1,
      workDir: work,
    });
    assert.equal(compiled.cacheHit, true);
    assert.equal(compiled.cashCanMint, false);
    assert.equal(compiled.ledgerFunding, false);
    assert.equal(compiled.nativePins, false);
    assert.notEqual(compiled.code, 'CASH_POLICY_ACCEPTED');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('receipt helper keeps callback failures and ignores unit confirmation', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-receipt-'));
  const txid = 'ab'.repeat(32);
  const cli = txidCli(dir, txid);
  const label = `t1-native-64-collision-${createHash('sha256').update(String(Date.now())).digest('hex').slice(0, 8)}`;
  try {
    const calls = [];
    const confirmed = await bindHelperReceipt('aa', {
      cliPath: cli,
      label: `${label}-ok`,
      keepReceipt: false,
      confirmationBudgetMs: 200,
      localTxid: txid,
      submit: (hex) => {
        calls.push(`submit:${hex}`);
        return txid;
      },
      pollConfirmation: () => {
        calls.push('poll');
        return { status: 'confirmed' };
      },
    });
    assert.equal(confirmed.helperSha256, HELPER);
    assert.equal(confirmed.callbacksSupplied, true);
    assert.deepEqual(calls, ['submit:aa', 'poll']);
    assert.equal(confirmed.helperConfirmed, true);
    assert.equal(confirmed.confirmed, false);
    assert.equal(confirmed.unitConfirmationIgnored, true);
    assert.equal(confirmed.code, 'RECEIPT_UNIT_CONFIRMATION_IGNORED');
    assert.notEqual(confirmed.code, 'RECEIPT_HELPER_INVOKED');
    assert.equal(confirmed.nativePins, false);
    const timed = await bindHelperReceipt('bb', {
      cliPath: cli,
      confirmationBudgetMs: 30,
      submit: () => txid,
      pollConfirmation: () => ({ status: 'unconfirmed' }),
    });
    assert.equal(timed.failureClass, 'confirmation-timeout');
    assert.equal(timed.confirmed, false);
    assert.notEqual(timed.code, 'RECEIPT_HELPER_INVOKED');
    const late = await bindHelperReceipt('cc', {
      cliPath: cli,
      confirmationBudgetMs: 20,
      submit: () => txid,
      pollConfirmation: () => new Promise((resolve) => setTimeout(() => resolve({ status: 'confirmed' }), 150)),
    });
    assert.equal(late.failureClass, 'confirmation-timeout');
    const hung = await bindHelperReceipt('dd', {
      cliPath: cli,
      confirmationBudgetMs: 30,
      submit: () => txid,
      pollConfirmation: () => new Promise(() => {}),
    });
    assert.equal(hung.failureClass, 'confirmation-timeout');
    const mismatched = await bindHelperReceipt('ee', {
      cliPath: cli,
      confirmationBudgetMs: 50,
      submit: () => 'cd'.repeat(32),
    });
    assert.equal(mismatched.failureClass, 'transport-binding-failure');
    assert.equal(mismatched.confirmed, false);
    const first = await bindHelperReceipt('11', { cliPath: cli, label, keepReceipt: true, submit: () => txid });
    assert.notEqual(first.failureClass, 'label-collision');
    const second = await bindHelperReceipt('22', { cliPath: cli, label, keepReceipt: true, submit: () => txid });
    assert.equal(second.failureClass, 'label-collision');
    assert.equal(existsSync(join(RECEIPTS, label, 'rejected-attempts')), true);
    const copied = join(TASK, 't1-consumer-repair-119/owned-output/checks/receipts', label, 'rejected-attempts');
    assert.equal(existsSync(copied), true);
  } finally {
    rmSync(dir, { recursive: true, force: true });
    rmSync(join(RECEIPTS, label), { recursive: true, force: true });
  }
});

test('readback compares financial fields and rejects stale or reused state', () => {
  const txid = 'ef'.repeat(32);
  const headPolicy = '12'.repeat(28);
  const cashPolicy = '34'.repeat(28);
  const expected = {
    balances: { Escrow: 0, Borrower: 0, Lender: 100 },
    allowances: {
      Escrow: { remaining: 0, spent: 100 },
      Borrower: { remaining: 0, spent: 0 },
      Lender: { remaining: 0, spent: 0 },
    },
    obligations: [],
    work: { remaining: 975, spent: 49, closureReserve: 16 },
    usedTransferIds: ['Release1'],
    transport_remaining: 0,
    ordinary_locked: 0,
    sourceHash: 'ab'.repeat(32),
    instanceId: 'cd'.repeat(32),
    headQuantity: 1,
    continuingCash: 0,
    payoutCash: 100,
  };
  assert.equal(compareFinancialFrame(expected, expected).code, 'FINANCIAL_FRAME_MATCHED');
  assert.equal(compareFinancialFrame({ ...expected, payoutCash: 90 }, expected).mismatches.includes('payoutCash'), true);
  assert.equal(compareFinancialFrame({
    ...expected,
    allowances: { ...expected.allowances, Escrow: { remaining: 100, spent: 0 } },
  }, expected).mismatches.includes('allowances'), true);
  assert.equal(compareFinancialFrame({ ...expected, obligations: [{ id: 'keep' }] }, expected).mismatches.includes('obligations'), true);
  assert.equal(compareFinancialFrame({ ...expected, runId: 'other' }, expected, { runId: 'current' }).code, 'READBACK_STALE_RUN');
  assert.equal(compareFinancialFrame({ ...expected, bodySha256: 'bb' }, expected, { bodySha256: 'aa' }).code, 'READBACK_BODY_CHANGED');
  assert.equal(compareFinancialFrame({
    ...expected,
    outputRef: `${txid}#0`,
  }, expected, { outputRef: `${txid}#0`, previousOutputRef: `${txid}#0` }).code, 'READBACK_OUTPUT_REUSED');
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-readback-'));
  try {
    const logical = {
      program_hash: '11'.repeat(32),
      bounds_hash: '22'.repeat(32),
      source_hash: expected.sourceHash,
      schema_hash: '44'.repeat(32),
      seed_hash: '55'.repeat(32),
      domain_network: 'yaci-isolated-t1',
      domain_deployment: 't1-custody-revision03',
      instance_id: expected.instanceId,
      transport_revision: 1,
      transport_remaining: 0,
      ordinary_locked: 0,
      ordinary_revision: 1,
      lifecycle: {
        balances: [
          { party: 'Escrow', asset: 'Cash', amount: 0 },
          { party: 'Borrower', asset: 'Cash', amount: 0 },
          { party: 'Lender', asset: 'Cash', amount: 100 },
        ],
        allowances: [
          { party: 'Escrow', asset: 'Cash', remaining: 0, spent: 100 },
          { party: 'Borrower', asset: 'Cash', remaining: 0, spent: 0 },
          { party: 'Lender', asset: 'Cash', remaining: 0, spent: 0 },
        ],
        obligations: [],
        usedTransferIds: ['Release1'],
        work: { remaining: 975, spent: 49, closure_reserve: 16 },
      },
    };
    writeFileSync(join(dir, 'out0.json'), JSON.stringify({
      [`${txid}#0`]: {
        value: { lovelace: 2000000, [headPolicy]: { '48656164': 1 } },
        inlineDatum: logical,
        runId: 'current',
      },
    }));
    writeFileSync(join(dir, 'out1.json'), JSON.stringify({
      [`${txid}#1`]: { value: { lovelace: 2000000, [cashPolicy]: { '43617368': 100 } } },
    }));
    writeFileSync(join(dir, 'bad1.json'), JSON.stringify({
      [`${txid}#1`]: { value: { lovelace: 2000000, [cashPolicy]: { '43617368': 90 } } },
    }));
    const log = join(dir, 'argv.txt');
    const cli = writeExec(join(dir, 'read.sh'), [
      '#!/bin/sh',
      'printf \'%s\\n\' "$*" >> ' + log,
      'ref=""',
      'while [ $# -gt 0 ]; do',
      '  if [ "$1" = "--tx-in" ]; then ref="$2"; shift 2; else shift; fi',
      'done',
      'case "$ref" in',
      `  *#1) cat ${join(dir, 'out1.json')};;`,
      `  *) cat ${join(dir, 'out0.json')};;`,
      'esac',
    ]);
    const socket = join(dir, 'node.socket');
    writeFileSync(socket, '');
    const matched = collectHeadReadback({
      cliPath: cli,
      socket,
      networkMagic: 764,
      expectedTxid: txid,
      outputIndex: 0,
      payoutIndex: 1,
      expectedFrame: expected,
    });
    assert.equal(matched.code, 'HEAD_READBACK');
    assert.equal(matched.unitOnly, true);
    assert.equal(matched.financialMatch, true);
    assert.equal(matched.confirmed, false);
    assert.match(readFileSync(log, 'utf8'), new RegExp(`--tx-in ${txid}#0`));
    assert.match(readFileSync(log, 'utf8'), new RegExp(`--tx-in ${txid}#1`));
    assert.match(readFileSync(log, 'utf8'), /--output-json/);
    assert.match(readFileSync(log, 'utf8'), /--testnet-magic 764/);
    const badCli = writeExec(join(dir, 'bad.sh'), [
      '#!/bin/sh',
      'ref=""',
      'while [ $# -gt 0 ]; do',
      '  if [ "$1" = "--tx-in" ]; then ref="$2"; shift 2; else shift; fi',
      'done',
      'case "$ref" in',
      `  *#1) cat ${join(dir, 'bad1.json')};;`,
      `  *) cat ${join(dir, 'out0.json')};;`,
      'esac',
    ]);
    const wrong = collectHeadReadback({
      cliPath: badCli,
      socket,
      networkMagic: 764,
      expectedTxid: txid,
      outputIndex: 0,
      payoutIndex: 1,
      expectedFrame: expected,
    });
    assert.equal(wrong.financialMatch, false);
    assert.equal(wrong.financialCode, 'READBACK_FINANCIAL_MISMATCH');
    assert.equal(wrong.mismatches.includes('payoutCash'), true);
    const stale = collectHeadReadback({
      cliPath: cli,
      socket,
      networkMagic: 764,
      expectedTxid: txid,
      outputIndex: 0,
      payoutIndex: 1,
      expectedFrame: expected,
      link: { runId: 'other-run' },
    });
    assert.equal(stale.financialCode, 'READBACK_STALE_RUN');
    assert.equal(stale.financialMatch, false);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('incomplete inputs, address failure, absent runtime, and unmeasured evaluation fail closed', async () => {
  const ids = [1, 2, 3, 4, 5, 6, 7, 8].map((index) => ({ txId: index.toString(16).padStart(64, 'a'), index }));
  assert.equal(selectRunInputs(ids).code, 'INPUTS_INCOMPLETE');
  const full = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map((index) => ({ txId: index.toString(16).padStart(64, 'b'), index }));
  const selected = selectRunInputs(full);
  assert.equal(selected.code, 'INPUTS_SELECTED');
  assert.notEqual(`${selected.headFee.txId}#${selected.headFee.index}`, `${selected.left.fee.txId}#${selected.left.fee.index}`);
  assert.notEqual(`${selected.left.fee.txId}#${selected.left.fee.index}`, `${selected.right.fee.txId}#${selected.right.fee.index}`);
  const absent = await runOwnedRace({
    signBin: CARDANO_CLI,
    submitBin: CARDANO_CLI,
    pollBin: CARDANO_CLI,
    networkMagic: 764,
    head: full[0],
    left: { fee: full[6], collateral: full[7], actor: 'release', resource: 'release-fee' },
    right: { fee: full[8], collateral: full[9], actor: 'refund', resource: 'refund-fee' },
  });
  assert.equal(absent.code, 'RACE_BODY_ABSENT');
  assert.equal(absent.replay.executed, false);
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-gates-'));
  try {
    const keyCli = writeExec(join(dir, 'key.sh'), [
      '#!/bin/sh',
      'key=""',
      'while [ $# -gt 0 ]; do',
      '  if [ "$1" = "--signing-key-file" ]; then key="$2"; shift 2; else shift; fi',
      'done',
      'if [ -n "$key" ]; then printf \'unit\\n\' > "$key"; fi',
      'exit 0',
    ]);
    const authority = nativeVotes(dir, CASH);
    const failed = prepareEnterpriseKey({ cliPath: keyCli, runDir: dir, authority, magic: 764 });
    assert.equal(failed.code, 'ENTERPRISE_ADDRESS_FAILED');
    assert.equal(failed.ok, false);
    assert.equal(failed.addressBuilt, false);
    const genesis = join(dir, 'shelley.json');
    writeFileSync(genesis, JSON.stringify({
      slotLength: 1,
      epochLength: 3600,
      networkMagic: 764,
      systemStart: '2026-01-01T00:00:00Z',
      evidenceClass: 'unit-fixture',
    }));
    const socket = join(dir, 'node.socket');
    writeFileSync(socket, '');
    const timeCli = writeExec(join(dir, 'time.sh'), [
      '#!/bin/sh',
      'if [ "$2" = "tip" ]; then printf \'%s\\n\' \'{"slot":10,"hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","block":1}\'; exit 0; fi',
      'if [ "$2" = "era-history" ]; then printf \'%s\\n\' \'{"type":"EraHistory","description":"EraHistory","cborHex":"9fff"}\'; exit 0; fi',
      'echo slotLengthMs=500',
      'exit 0',
    ]);
    const timed = measureSlotLength({ cliPath: timeCli, socket, genesisPath: genesis, execute: true, magic: 764 });
    assert.equal(timed.code, 'SLOT_OBSERVED');
    assert.equal(timed.observed, '1000');
    assert.equal(timed.slotLengthSource, 'owned-genesis');
    assert.equal(timed.eraHistory, 'json');
    assert.equal(timed.unitOnly, true);
    const runtime = loadRuntimeInputs();
    assert.equal(runtime.code, 'RUNTIME_COMMANDS_READY');
    assert.equal(runtime.adopted, true);
    assert.equal(runtime.liveHome, false);
    assert.equal(runtime.spawned, false);
    assert.equal(runtime.executeNow, false);
    assert.equal(runtime.ledgerExecution, false);
    assert.deepEqual(runtime.faucetInputs, []);
    assert.equal(runtime.producer.code, 'RUNTIME_PRODUCER_HELD');
    assert.equal(runtime.producer.spawned, false);
    assert.equal(runtime.producer.createNode.code, 'CREATE_NODE_ARGV');
    assert.equal(runtime.producer.createNode.argv[runtime.producer.createNode.argv.indexOf('--slot-length') + 1], '1');
    assert.equal(runtime.producer.createNode.argv[runtime.producer.createNode.argv.indexOf('--epoch-length') + 1], '3600');
    assert.equal(runtime.producer.createNode.argv.includes('600'), false);
    assert.equal(runtime.producer.futureStart.code, 'FUTURE_START_AFTER_AUDITS');
    assert.equal(runtime.producer.futureStart.executeNow, false);
    assert.equal(runtime.producer.emptyGenesis.code, 'EMPTY_GENESIS');
    assert.equal(runtime.producer.proofTxt.code, 'PROOF_TXT_NOT_READINESS');
    assert.equal(runtime.producer.wrapper.code, 'YACI_WRAPPER_REFUSED');
    assert.equal(runtime.producer.faucetBuild.code, 'NO_TX_IN');
    assert.equal(runtime.producer.keygen.code, 'REAL_KEYGEN_NOT_ARMED');
    assert.equal(runtime.producer.tip.code, 'MAGIC_SOURCE_REFUSED');
    const held = produceOwnedRuntime({ executeNow: true });
    assert.equal(held.code, 'REAL_NODE_NOT_ARMED');
    assert.equal(held.spawned, false);
    const absent = loadRuntimeInputs(join(TASK, 't1-runtime-inputs-62'));
    assert.equal(absent.code, 'RUNTIME_INPUTS_ABSENT');
    assert.equal(absent.adopted, false);
    assert.equal(absent.partialLogsIgnored, true);
    const unevaluated = acquireBodyEvaluation({ cliPath: CARDANO_CLI, bodyPath: SOURCE, socket, execute: true });
    assert.equal(unevaluated.code, 'REAL_QUERY_NOT_ARMED');
    assert.equal(unevaluated.spawned, false);
    assert.equal(unevaluated.nativeBudgetMeasured, false);
    const missing = acquireBodyEvaluation({ cliPath: timeCli, bodyPath: '', socket, execute: true });
    assert.equal(missing.code, 'EVALUATION_UNAVAILABLE');
    assert.equal(missing.spawned, false);
    const realTime = measureSlotLength({ cliPath: CARDANO_CLI, socket, execute: true });
    assert.equal(realTime.code, 'REAL_QUERY_NOT_ARMED');
    assert.equal(realTime.spawned, false);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('source terminals keep the nonempty obligation fence', () => {
  const posts = loadSourceTerminals(readFileSync(SOURCE, 'utf8'));
  assert.equal(posts.code, 'SOURCE_POSTS_EVALUATED', posts.detail || posts.code);
  assert.equal(posts.residual.bodyBuilt, false);
  assert.equal(posts.residual.fenceBypass, false);
  assert.equal(posts.residual.code, 'NONEMPTY_OBLIGATIONS');
  assert.equal(posts.residual.obligationsPreserved, true);
  assert.equal(posts.terminals.release.financialPost.work.spent, '49');
  assert.equal(posts.terminals.release.continuingCash, 0);
  assert.equal(posts.terminals.release.payoutCash, 100);
  assert.equal(posts.terminals.cancel.redeemer.transferId, 'Cancel1');
});
