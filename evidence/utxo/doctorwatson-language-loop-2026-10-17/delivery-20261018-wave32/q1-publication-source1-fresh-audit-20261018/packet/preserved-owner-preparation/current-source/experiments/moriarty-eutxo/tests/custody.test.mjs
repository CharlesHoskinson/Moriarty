/**
 * T1 script-custody compiler tests.
 * Source: designers/02-escrow-time/escrow-custody-v5.mori (profile /5).
 * Representation: elaborated Core, not a hand-built success envelope.
 * Ledger context in the Aiken tests is a synthetic Transaction passed to the
 * compiled custody.spend validator. It is source/UPLC-unit, not a node spend.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { admitCustody, fenceLifecycle, scanCore } from '../src/admit-custody.mjs';
import { calibrateGuards, planWindows } from '../src/custody-time.mjs';
import { compileCustody } from '../src/compile-custody.mjs';
import { evaluateOuter } from '../src/outer-eval.mjs';
import { bindCapacity } from '../src/custody-capacity.mjs';
import { lowerCustody } from '../src/lower-custody.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const CHECKS = mkdtempSync(join(process.env.TMPDIR || '/tmp', 'cu-'));
const DESIGNER = join(TASK, 'designers/02-escrow-time');
const ROOT = join(import.meta.dirname, '..');
const SOURCE = join(DESIGNER, 'escrow-custody-v5.mori');
const SEED = join(DESIGNER, 'financial-seed-custody-v5.json');
const RESIDUAL = join(DESIGNER, 'financial-residual-custody-v5.json');
const CASES = join(DESIGNER, 'cases.json');
const E13 = join(DESIGNER, 'E13-release-real-custody.expected.json');
const E11 = join(DESIGNER, 'E11-refund-real-custody.expected.json');
const INTERVAL = join(DESIGNER, 'interval-boundary.mori');
const MANIFEST_PATH = join(ROOT, 'examples/custody-unit.manifest.json');
const ATOMIC = join(ROOT, 'eutxo-public/validators/atomic.ak');
const UINT128_MAX = '340282366920938463463374607431768211455';
const SHAPE = {
  threadPolicyId: '24ba87f0a8b4f617d991e86e7a84fc3d4d90855dae7b2ce94e451a79',
  cashPolicyId: '1d038a1069d649adf83745f02ff5c84575e32839194593263b0a2855',
  lenderVkh: 'b39aa2a57e9c3730b8234ada7592743d512edca5c23f60ba03c9dd4b',
  borrowerVkh: '72f72192639eb173fce5342d2df730b73a0d838c48d2422ebf91e48c',
  scriptHash: '64b8d588ca2e8207014edd458014a7b49007f8c01fab78ff809925fe',
  instanceId: 'f36bb067a92c4c1c82d65e518bf2a5a633a95602b0937042be82d5cdf11924d4',
};
const PINS = {
  sourceHash: 'facb73a98d78c4a8839e6aa05af30a7702e15798d3d7ebb15e4eb1a4b78bc736',
  programHash: '93ed829602f6a554d3f47879ee220c7382140045250bf53b21470ee6091297b0',
  schemaHash: '2254f3ccbe5494a83bef188de808473f64b220b4f2f1178976916baa7d335b4c',
  seedHash: 'fc93bf4a17f0fb0a7dd70761e36a3b903bf66843d7beea29a210329d5d1c327f',
  boundsHash: 'b3d02ace7a7ac7daec45318f2814beff4d27ad26f91d45405942c30d04fa846d',
};

function repeat(byte, n) {
  return byte.repeat(n);
}

export function unitManifest(overrides = {}) {
  return {
    schema: 'moriarty-t1-custody-manifest/1',
    nativePins: false,
    unitFixture: true,
    production: false,
    domainNetwork: 'yaci-isolated-t1',
    domainDeployment: 't1-custody-revision03',
    threadPolicyId: repeat('e5', 28),
    cashPolicyId: repeat('b2', 28),
    lenderVkh: repeat('c3', 28),
    borrowerVkh: repeat('d4', 28),
    scriptHash: repeat('e5', 28),
    threadAssetNameHex: '48656164',
    cashAssetNameHex: '43617368',
    quantum: '1',
    ...overrides,
  };
}

const sourceText = readFileSync(SOURCE, 'utf8');
const seedText = readFileSync(SEED, 'utf8');
const atomicBefore = createHash('sha256').update(readFileSync(ATOMIC)).digest('hex');

test('custody modules load', () => {
  assert.equal(typeof admitCustody, 'function');
  assert.equal(typeof fenceLifecycle, 'function');
  assert.equal(typeof scanCore, 'function');
  assert.equal(typeof planWindows, 'function');
  assert.equal(typeof calibrateGuards, 'function');
  assert.equal(typeof compileCustody, 'function');
});

function admitted() {
  return admitCustody({ sourceText, seedText, manifest: unitManifest() });
}

test('source/5 admit pins the frozen program, bounds, source, schema, and seed hashes', () => {
  const result = admitted();
  assert.equal(result.ok, true, JSON.stringify(result));
  assert.equal(result.hashes.sourceHash, PINS.sourceHash);
  assert.equal(result.hashes.programHash, PINS.programHash);
  assert.equal(result.hashes.schemaHash, PINS.schemaHash);
  assert.equal(result.hashes.seedHash, PINS.seedHash);
  assert.equal(result.hashes.boundsHash, PINS.boundsHash);
  assert.notEqual(result.instanceId, SHAPE.instanceId);
  assert.equal(result.nativePins, false);
  assert.equal(result.ledgerExecution, false);
  assert.equal(result.layer, 'source-uplc-unit');
  for (const action of result.actions) {
    assert.equal(action.staticWorkBound, '48');
    assert.equal(action.constructorCount, 48);
    assert.equal(action.debit, 49);
  }
});

test('unit manifest file is the same fixture the tests admit', () => {
  const file = JSON.parse(readFileSync(MANIFEST_PATH, 'utf8'));
  assert.deepEqual(file, unitManifest());
});

test('nativePins true is refused before any script is compiled', () => {
  const result = admitCustody({ sourceText, seedText, manifest: unitManifest({ nativePins: true }) });
  assert.equal(result.ok, false);
  assert.equal(result.code, 'NATIVE_PINS_ABSENT');
});

test('same-script thread policy is required and other role collisions stay refused', () => {
  const distinct = admitCustody({
    sourceText,
    seedText,
    manifest: unitManifest({ threadPolicyId: repeat('a1', 28), scriptHash: repeat('e5', 28) }),
  });
  assert.equal(distinct.ok, false);
  assert.equal(distinct.code, 'THREAD_POLICY_MUST_EQUAL_SCRIPT_HASH');
  const collided = admitCustody({
    sourceText,
    seedText,
    manifest: unitManifest({ lenderVkh: repeat('b2', 28) }),
  });
  assert.equal(collided.ok, false);
  assert.equal(collided.code, 'POLICY_NOT_DISTINCT');
  const admittedNow = admitted();
  assert.equal(admittedNow.ok, true, JSON.stringify(admittedNow));
  assert.equal(admittedNow.manifest.threadPolicyId, admittedNow.manifest.scriptHash);
});

test('shape-sample policy ids and the all-zero id are refused', () => {
  const shape = admitCustody({
    sourceText,
    seedText,
    manifest: unitManifest({ threadPolicyId: SHAPE.threadPolicyId }),
  });
  assert.equal(shape.ok, false);
  assert.equal(shape.code, 'SHAPE_SAMPLE_FORBIDDEN');
  const zero = admitCustody({
    sourceText,
    seedText,
    manifest: unitManifest({ cashPolicyId: '00'.repeat(28) }),
  });
  assert.equal(zero.ok, false);
  assert.equal(zero.code, 'ALL_ZERO_FORBIDDEN');
});

test('quantum other than string 1 is refused before the bounds hash', () => {
  const text = admitCustody({ sourceText, seedText, manifest: unitManifest({ quantum: '2' }) });
  assert.equal(text.ok, false);
  assert.equal(text.code, 'QUANTUM');
  const numeric = admitCustody({ sourceText, seedText, manifest: unitManifest({ quantum: 1 }) });
  assert.equal(numeric.ok, false);
  assert.equal(numeric.code, 'NON_CANONICAL_VALUE');
});

test('profile /6 interval-boundary source is not compiled as custody', () => {
  const result = admitCustody({
    sourceText: readFileSync(INTERVAL, 'utf8'),
    seedText,
    manifest: unitManifest(),
  });
  assert.equal(result.ok, false);
  assert.ok(result.code);
  assert.notEqual(result.code, 'NATIVE_PINS_ABSENT');
});

test('FloorDiv, Let, Text, Bool, and a non-128 literal are refused on the elaborated core', () => {
  const ok = admitted();
  assert.equal(ok.ok, true, JSON.stringify(ok));
  const floor = structuredClone(ok.elaborated);
  floor.actions[0].core.statements.push({
    constructor: 'Require',
    operands: { condition: { constructor: 'FloorDiv', operands: { left: { constructor: 'LitUInt', operands: { width: '128', value: '1' } }, right: { constructor: 'LitUInt', operands: { width: '128', value: '1' } } } } },
    span: { kind: 'synthetic', start: '0', end: '0' },
  });
  assert.equal(scanCore(floor.actions[0].core).code, 'FLOOR_DIV');
  const letNode = structuredClone(ok.elaborated);
  letNode.actions[0].core.statements[0] = {
    constructor: 'Let',
    operands: { name: 'x', value: { constructor: 'LitUInt', operands: { width: '128', value: '1' } } },
    span: { kind: 'synthetic', start: '0', end: '0' },
  };
  assert.equal(scanCore(letNode.actions[0].core).code, 'LET');
  const text = structuredClone(ok.elaborated);
  text.actions[0].schema.fields.note = { type: ['Text'], writeClass: 'ordinary' };
  assert.equal(admitCustody({ sourceText, seedText, manifest: unitManifest(), elaborated: text }).code, 'ORDINARY_TYPE');
  const bool = structuredClone(ok.elaborated);
  bool.actions[0].schema.fields.flag = { type: ['Bool'], writeClass: 'ordinary' };
  assert.equal(admitCustody({ sourceText, seedText, manifest: unitManifest(), elaborated: bool }).code, 'ORDINARY_TYPE');
  const wide = structuredClone(ok.elaborated);
  wide.actions[0].core.statements[0].operands.condition.operands.right.operands.width = '64';
  assert.equal(scanCore(wide.actions[0].core).code, 'UINT_WIDTH');
});

test('malformed seed and a lone surrogate are refused', () => {
  const bad = admitCustody({ sourceText, seedText: '{', manifest: unitManifest() });
  assert.equal(bad.ok, false);
  assert.equal(bad.code, 'MALFORMED_DATA');
  const utf = admitCustody({ sourceText: '\uD800', seedText, manifest: unitManifest() });
  assert.equal(utf.ok, false);
  assert.ok(utf.code);
});

test('E12 residual state is fenced even when the source evaluator prepares it', () => {
  const result = admitCustody({
    sourceText,
    seedText: readFileSync(RESIDUAL, 'utf8'),
    manifest: unitManifest(),
  });
  assert.equal(result.ok, false);
  assert.equal(result.code, 'NONEMPTY_OBLIGATIONS');
  const residual = JSON.parse(readFileSync(RESIDUAL, 'utf8'));
  assert.equal(fenceLifecycle(residual).code, 'NONEMPTY_OBLIGATIONS');
});

test('slot length other than 1000 is refused and overflow keeps the codec code OVERFLOW', () => {
  assert.equal(planWindows({ zeroTime: '0', zeroSlot: '0', slotLengthMs: '999', tipSlot: '10' }).code, 'SLOT_LENGTH_REFUSED');
  assert.equal(planWindows({ zeroTime: '0', zeroSlot: '0', slotLengthMs: '1001', tipSlot: '10' }).code, 'SLOT_LENGTH_REFUSED');
  assert.equal(planWindows({ zeroTime: '-1', zeroSlot: '0', slotLengthMs: '1000', tipSlot: '1' }).code, 'NEGATIVE_POSIX');
  const overflow = planWindows({ zeroTime: '0', zeroSlot: '0', slotLengthMs: '1000', tipSlot: UINT128_MAX });
  assert.equal(overflow.ok, false);
  assert.equal(overflow.code, 'OVERFLOW');
  const planned = planWindows({ zeroTime: '0', zeroSlot: '0', slotLengthMs: '1000', tipSlot: '1700000000' });
  assert.equal(planned.ok, true);
  assert.equal(planned.evidenceClass, 'offline-integer-math-not-native');
  assert.equal(planned.ledgerExecution, false);
  assert.equal(planned.tipPosix, '1700000000000');
  assert.equal(planned.releaseStart, '1700000030000');
  assert.equal(planned.partition, '1700000090000');
  assert.equal(planned.refundExcl, '1700000150000');
  assert.equal(planned.releaseArgs.upper, '1700000089999');
  assert.equal(planned.refundArgs.upper, '1700000149999');
});

test('guard calibration refuses without a native parity probe and does not emit source', () => {
  assert.equal(calibrateGuards(undefined).code, 'PARITY_ABSENT');
  assert.equal(calibrateGuards({ probeExecuted: true, evidenceClass: 'synthetic-wallclock' }).code, 'PARITY_NOT_NATIVE');
  assert.equal(calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '999',
    lucidPosix: '1',
    cliPosix: '1',
    scriptContextLower: '1',
  }).code, 'SLOT_LENGTH_REFUSED');
  assert.equal(calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1',
    cliPosix: '2',
    scriptContextLower: '1',
  }).code, 'IMPOSSIBLE_BOUNDARY');
  assert.equal(calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '-5',
    cliPosix: '-5',
    scriptContextLower: '-5',
  }).code, 'NEGATIVE_POSIX');
  const claimed = calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
  });
  assert.equal(claimed.ok, false);
  assert.equal(claimed.code, 'CALIBRATED_SOURCE_NOT_THIS_PHASE');
  assert.equal(claimed.sourceEmitted, false);
});

test('evaluate envelopes match the designer expected files before a validator is built', async () => {
  const { createFinancialAgreementSourceV5 } = await import('../../moriarty-language/src/successor/financial-agreement-source-v5.ts');
  const { canonicalEncode } = await import('../../moriarty-language/src/codec.ts');
  const v5 = createFinancialAgreementSourceV5();
  const cases = JSON.parse(readFileSync(CASES, 'utf8'));
  const pick = (id) => cases.cases.find((item) => item.id === id);
  const e13 = v5.evaluate(sourceText, 'release', pick('E13-release-real-custody').ordinaryInputCanonicalJSON, seedText);
  const e11 = v5.evaluate(sourceText, 'refund', pick('E11-refund-real-custody').ordinaryInputCanonicalJSON, seedText);
  const cancelInput = canonicalEncode({
    Args: { actor: 'Borrower', lower: '1050', recipient: 'Borrower', transferId: 'Cancel1', upper: '1051' },
    Obs: {},
    Pre: { locked: '1', revision: '0' },
    workInitial: '1024',
  });
  const cancel = v5.evaluate(sourceText, 'cancel', cancelInput, seedText);
  const e12 = v5.evaluate(sourceText, 'cancel', pick('E12-cancel-preserves-duty').ordinaryInputCanonicalJSON, readFileSync(RESIDUAL, 'utf8'));
  const expected13 = JSON.parse(readFileSync(E13, 'utf8'));
  const expected11 = JSON.parse(readFileSync(E11, 'utf8'));
  assert.equal(e13.status, expected13.status);
  assert.deepEqual(e13.financialPost, expected13.financialPost);
  assert.deepEqual(e13.post, expected13.post);
  assert.deepEqual(e13.effects, expected13.effects);
  assert.equal(expected13.constructorWorkCount, 49);
  assert.equal(e13.financialPost.work.spent, '49');
  assert.equal(e13.financialPost.work.remaining, '975');
  assert.equal(e11.status, expected11.status);
  assert.deepEqual(e11.financialPost, expected11.financialPost);
  assert.deepEqual(e11.effects, expected11.effects);
  assert.equal(cancel.status, 'FundedExpressionPrepared');
  assert.deepEqual(cancel.post, { locked: '0', revision: '1' });
  assert.deepEqual(cancel.financialPost.balances, expected11.financialPost.balances);
  assert.deepEqual(cancel.financialPost.usedTransferIds, ['Cancel1']);
  assert.deepEqual(cancel.financialPost.obligations, []);
  assert.equal(cancel.financialPost.work.remaining, '975');
  assert.equal(cancel.financialPost.work.spent, '49');
  assert.equal(e12.financialPost.obligations[0].outstanding, '40');
  assert.equal(fenceLifecycle(e12.financialPost).code, 'NONEMPTY_OBLIGATIONS');
});

test('lifecycle capacity envelopes keep 127-to-128 growth and refuse 128-to-129', () => {
  const seed = JSON.parse(readFileSync(SEED, 'utf8'));
  const bound = bindCapacity({ sourceText: readFileSync(SOURCE, 'utf8'), seed, manifest: unitManifest() });
  assert.equal(bound.ok, true, bound.detail || bound.code);
  assert.equal(bound.capacity, 128);
  assert.equal(bound.ids127.post.usedTransferIds.length, 128);
  assert.equal(bound.ids127.post.usedTransferIds.at(-1), 'CapRelease');
  assert.equal(bound.balances127.post.balances.length, 128);
  assert.equal(bound.balances127.post.balances.at(-1).party, 'Lender');
  assert.equal(bound.balances127.post.balances.at(-1).amount, '100');
  assert.equal(bound.balances128Update.post.balances.length, 128);
  assert.equal(bound.balances128Update.post.work.spent, '49');
  assert.equal(bound.ids128.candidate.usedTransferIds.length, 129);
  assert.equal(bound.balances128.candidate.balances.length, 129);
  assert.equal(bound.reachability.ids128Seed, 'USED_ID_FENCE');
  assert.equal(bound.reachability.balances128MissingLender, 'MISSING_BALANCE');
  assert.equal(bound.reachability.balances128UpdateAdmitted, true);
});

test('compiled custody validator spends the three source actions and rejects the adversarial contexts', { timeout: 900000 }, () => {
  const outDir = join(CHECKS, 'custody-compile');
  const result = compileCustody({
    sourcePath: SOURCE,
    seedPath: SEED,
    casesPath: CASES,
    expectedReleasePath: E13,
    expectedRefundPath: E11,
    residualPath: RESIDUAL,
    manifest: unitManifest(),
    outDir,
  });
  assert.equal(result.ok, true, `${result.code || ''} check=${result.check?.status} build=${result.build?.status}\n${result.check?.stderr || ''}\n${result.build?.stderr || ''}`);
  assert.equal(result.ledgerExecution, false);
  assert.equal(result.nativePins, false);
  assert.equal(result.layer, 'source-uplc-unit');
  assert.ok(result.blueprintPath);
  assert.equal(result.check.status, 0);
  const titles = new Set((result.check.tests || []).map((item) => item.title));
  for (const name of [
    'source_uplc_unit_release',
    'source_uplc_unit_refund',
    'source_uplc_unit_cancel_empty',
    'nonempty_obligations',
    'n01_refund_args_wider_interval',
    'missing_signatory',
    'wrong_recipient_payout',
    'shared_payout',
    'duplicate_transfer_id',
    'mutated_work',
    'stale_bounds_hash',
    'schema_version',
    'seed_cash_utxo_mismatch',
    'wrong_actor_redeemer',
    'mint_nonzero',
    'second_script_input',
    'replay_used_id',
    'wrong_transport',
    'sponsorship_moved',
    'decode_lucid_definite_e13_pre',
    'decode_lucid_definite_e13_post',
    'decode_lucid_definite_balance',
    'decode_lucid_definite_release',
    'frozen_shape_sample_instance_differs',
    'frozen_shape_sample_head_not_spent',
    'release_missing_lender_row',
    'generic_absent_party_and_asset_appends',
    'hostile_duplicate_lender_rows_do_not_inflate',
    'oracle_noninjective_escrow_rows',
    'hostile_other_asset_dropped',
    'oracle_foreign_asset_on_continuation',
    'oracle_foreign_asset_on_payout',
    'oracle_foreign_mint',
    'unadmitted_name_on_admitted_cash_policy',
    'hostile_change_lovelace_merged_into_payout',
    'work_shortfall_reaches_validator',
    'balance_shortfall_reaches_validator',
    'allowance_shortfall_reaches_validator',
    'missing_allowance_reaches_validator',
    'oracle_release_missing_lender_row',
    'oracle_release_revision_4',
    'oracle_release_work_exact_debit',
    'oracle_release_witness_balance',
    'cap_ids_127_source_post_spends',
    'cap_ids_128_candidate_post_spends',
    'cap_balances_127_append_spends',
    'cap_balances_128_candidate_append_spends',
    'cap_balances_128_update_spends',
    'mint_seed_head_unit',
    'mint_forged_instance',
  ]) {
    assert.ok(titles.has(name), `missing aiken test ${name}; saw ${[...titles].join(',')}`);
  }
  const failed = (result.check.tests || []).filter((item) => item.status !== 'success');
  assert.deepEqual(failed, []);
  const source = readFileSync(result.validatorPath, 'utf8');
  assert.match(source, /escrow-custody-v5\.mori/);
  assert.match(source, /source\/UPLC-unit/);
  assert.match(source, /ledgerExecution false/);
  assert.doesNotMatch(source, /submit-with-receipt/);
  assert.match(source, /const collection_capacity: Int = 128/);
  assert.match(source, /expect list\.length\(life\.balances\) < collection_capacity/);
  assert.match(source, /expect list\.length\(life\.used_transfer_ids\) < collection_capacity/);
  assert.doesNotMatch(source, /COLLECTION_CAPACITY/);
  assert.equal(existsSync(join(ROOT, 'eutxo-public/validators/custody.ak')), false);
  assert.equal(result.applied.cashCanMint, false);
  assert.equal(result.applied.manifestMatchesCompiledCash, false);
  assert.equal(result.applied.appliedCashPolicyId, result.applied.cashHash);
  assert.notEqual(result.applied.appliedCashPolicyId, 'b2'.repeat(28));
  assert.notEqual(result.applied.appliedHash, 'b87b493284c84a4eef169922f3f6376ecccba384f15078237cefd36e');
  assert.equal(result.applied.literals.ownHash, false);
  assert.equal(result.applied.literals.instanceDigest, false);
  assert.equal(result.applied.literals.unitScriptHash, false);
  const outer = evaluateOuter({
    custodySourcePath: result.validatorPath,
    appliedBlueprintPath: join(outDir, 'plutus.applied.json'),
    outDir: join(CHECKS, 'outer-eval'),
  });
  assert.equal(outer.prototypePolicyReused, false);
  assert.equal(outer.ledgerExecution, false);
  assert.equal(outer.phase2, false);
  assert.equal(outer.nativeBudgetMeasured, false);
  assert.equal(outer.cases.MINT_CTX?.unit, true, JSON.stringify(outer.cases.MINT_CTX));
  assert.equal(outer.cases.SPEND_CTX?.unit, true, JSON.stringify(outer.cases.SPEND_CTX));
  assert.equal(outer.cases.WITHDRAW_CTX?.unit, false, JSON.stringify(outer.cases.WITHDRAW_CTX));
  assert.equal(outer.cases.FORGED_MINT_CTX?.unit, false, JSON.stringify(outer.cases.FORGED_MINT_CTX));
  assert.equal(outer.cases.FORGED_SPEND_CTX?.unit, false, JSON.stringify(outer.cases.FORGED_SPEND_CTX));
  assert.equal(outer.cases.FOREIGN_MISMATCH_CTX?.unit, false, JSON.stringify(outer.cases.FOREIGN_MISMATCH_CTX));
  assert.equal(outer.cases.FOREIGN_MATCH_CTX?.unit, true, JSON.stringify(outer.cases.FOREIGN_MATCH_CTX));
  assert.equal(typeof outer.cases.MINT_CTX?.evaluatorCpu, 'number');
  assert.notEqual(outer.cases.MINT_CTX.evaluatorCpu, outer.cases.MINT_CTX.nativeBudget);
});

test('atomic validator bytes stay on the public wallet-transfer path', () => {
  const after = createHash('sha256').update(readFileSync(ATOMIC)).digest('hex');
  assert.equal(after, atomicBefore);
});

test('compile-custody rejects a historical output path before it writes', () => {
  const historical = join(TASK, 't1-capacity-repair-25/checks/cli-custody');
  const marker = join(historical, 'plutus.json');
  const before = existsSync(marker) ? statSync(marker).mtimeMs : null;
  const run = spawnSync(process.execPath, ['src/cli.mjs', 'compile-custody', '--out', historical], {
    cwd: ROOT,
    encoding: 'utf8',
    timeout: 30000,
  });
  assert.equal(run.status, 1);
  assert.match(`${run.stdout}${run.stderr}`, /OUTPUT_PATH_FORBIDDEN/);
  const after = existsSync(marker) ? statSync(marker).mtimeMs : null;
  assert.equal(after, before);
});

test('cli compile-custody is a separate command and unknown commands still exit 2', () => {
  const missing = spawnSync(process.execPath, ['src/cli.mjs'], { cwd: ROOT, encoding: 'utf8' });
  assert.equal(missing.status, 2);
  assert.match(missing.stderr, /compile-custody/);
  const run = spawnSync(process.execPath, [
    'src/cli.mjs', 'compile-custody',
    '--source', SOURCE,
    '--seed', SEED,
    '--manifest', MANIFEST_PATH,
    '--out', join(CHECKS, 'cli-custody'),
  ], { cwd: ROOT, encoding: 'utf8', timeout: 600000 });
  assert.equal(run.status, 0, run.stderr + run.stdout);
  const parsed = JSON.parse(run.stdout);
  assert.equal(parsed.ok, true);
  assert.equal(parsed.ledgerExecution, false);
  assert.equal(parsed.nativePins, false);
});

test('lowerCustody recounts core and refuses a forged admission debit before render', () => {
  const admitted = admitCustody({
    sourceText: readFileSync(SOURCE, 'utf8'),
    seedText: readFileSync(SEED, 'utf8'),
    manifest: unitManifest(),
  });
  assert.equal(admitted.ok, true);
  admitted.actions[0].debit = 50;
  assert.throws(
    () => lowerCustody({ admitted, fixtures: null }),
    (error) => error.code === 'DEBIT_MISMATCH',
  );
});

test('an admission refusal removes a stale plutus.json before it returns', () => {
  const outDir = join(CHECKS, 'stale-admission');
  rmSync(outDir, { recursive: true, force: true });
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, 'plutus.json'), 'STALE\n');
  const result = compileCustody({
    sourcePath: INTERVAL,
    seedPath: SEED,
    casesPath: CASES,
    expectedReleasePath: E13,
    expectedRefundPath: E11,
    residualPath: RESIDUAL,
    manifest: unitManifest(),
    outDir,
  });
  assert.equal(result.ok, false);
  assert.equal(result.code, 'PROFILE_MISMATCH');
  assert.equal(result.blueprintPath, null);
  assert.equal(existsSync(join(outDir, 'plutus.json')), false);
  const admission = JSON.parse(readFileSync(join(outDir, 'admission.json'), 'utf8'));
  assert.equal(admission.ok, false);
  assert.equal(admission.ok, result.ok);
});

test('lowerCustody refuses a mutated core before render', () => {
  const admitted = admitCustody({
    sourceText: readFileSync(SOURCE, 'utf8'),
    seedText: readFileSync(SEED, 'utf8'),
    manifest: unitManifest(),
  });
  const extra = admitted.elaborated.actions[0].core.statements.find((item) => item.constructor === 'Require');
  admitted.elaborated.actions[0].core.statements.push(structuredClone(extra));
  assert.equal(admitted.actions[0].debit, 49);
  assert.throws(
    () => lowerCustody({ admitted, fixtures: null }),
    (error) => error.code === 'WORK_BOUND',
  );
});

test('aiken status 7 does not publish admission ok true or a stale blueprint', () => {
  const outDir = join(CHECKS, 'aiken-exit-7');
  rmSync(outDir, { recursive: true, force: true });
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, 'plutus.json'), 'STALE\n');
  const result = compileCustody({
    sourcePath: SOURCE,
    seedPath: SEED,
    casesPath: CASES,
    expectedReleasePath: E13,
    expectedRefundPath: E11,
    residualPath: RESIDUAL,
    manifest: unitManifest(),
    outDir,
    aikenRun() {
      return { status: 7, stdout: '', stderr: 'forced-exit-7' };
    },
  });
  assert.equal(result.ok, false);
  assert.equal(result.code, 'AIKEN_CHECK');
  assert.equal(result.check.status, 7);
  assert.equal(result.blueprintPath, null);
  assert.equal(existsSync(join(outDir, 'plutus.json')), false);
  const admission = JSON.parse(readFileSync(join(outDir, 'admission.json'), 'utf8'));
  assert.equal(admission.ok, false);
  assert.equal(admission.ok, result.ok);
  assert.equal(admission.checkStatus, 7);
});

test('fee and collateral are a body expectation and not a script-context collateral pin', async () => {
  const { expectCustodyBody, buildCustodyBody } = await import('../src/custody-body.mjs');
  const head = { transactionId: 'head', outputIndex: 0 };
  const fee = { transactionId: 'fee', outputIndex: 0 };
  const collateral = { transactionId: 'collateral', outputIndex: 0 };
  const distinct = expectCustodyBody({ headRef: head, feeRef: fee, collateralRef: collateral });
  assert.equal(distinct.ok, true);
  assert.equal(distinct.nativeBuilder, 'encodeUnsignedBody');
  assert.equal(distinct.builderInvoked, false);
  assert.equal(distinct.code, 'BODY_SHAPE_OK');
  assert.equal(distinct.ledgerExecution, false);
  const built = buildCustodyBody({
    head: { txId: '11'.repeat(32), index: 0 },
    fee: { txId: '22'.repeat(32), index: 1 },
    collateral: { txId: '33'.repeat(32), index: 2 },
    outputLovelace: '2000000',
    feeLovelace: '200000',
    invalidBefore: 40,
    invalidHereafter: 100,
    magic: 764,
  });
  assert.equal(built.builderInvoked, true);
  assert.equal(built.ok, true, JSON.stringify({ code: built.code, detail: built.detail }));
  assert.equal(built.ledgerExecution, false);
  assert.equal(built.signed, false);
  assert.equal(distinct.scriptContextCollateral, false);
  assert.equal(expectCustodyBody({ headRef: head, feeRef: fee, collateralRef: fee }).code, 'FEE_COLLATERAL_ALIAS');
  assert.equal(expectCustodyBody({ headRef: head, feeRef: head, collateralRef: collateral }).code, 'HEAD_IS_NOT_FEE_OR_COLLATERAL');
  assert.equal(expectCustodyBody({ headRef: head, feeRef: fee }).code, 'BODY_INPUTS_REQUIRED');
  const source = readFileSync(join(ROOT, 'src/lower-custody.mjs'), 'utf8');
  assert.equal(source.includes('tx.collateral'), false);
  assert.equal(source.includes('list.length(self.inputs) == 3'), false);
});

test('dropped source fields and E12 stay on their own layers', async () => {
  const { createFinancialAgreementSourceV5 } = await import('../../moriarty-language/src/successor/financial-agreement-source-v5.ts');
  const { canonicalEncode } = await import('../../moriarty-language/src/codec.ts');
  const v5 = createFinancialAgreementSourceV5();
  const oracles = JSON.parse(readFileSync(join(TASK, 't1-independent-oracles-19/ORACLES.json'), 'utf8'));
  const row = (id) => oracles.sourceBindings.find((item) => item.caseId === id);
  const financialOf = (args) => (
    typeof args.financialState === 'string'
      ? readFileSync(join(DESIGNER, args.financialState), 'utf8')
      : JSON.stringify(args.financialState)
  );
  const ordinary = (args) => canonicalEncode({
    Args: {
      actor: args.args.actor,
      lower: args.args.lower,
      recipient: args.args.recipient,
      transferId: args.args.transferId,
      upper: args.args.upper,
    },
    Obs: args.obs || {},
    Pre: args.pre,
    workInitial: args.workInitial,
  });
  const sourceText = readFileSync(SOURCE, 'utf8');
  const mismatch = row('release-work-initial-mismatch');
  const mismatchEval = v5.evaluate(sourceText, 'release', ordinary(mismatch.sourceArguments), financialOf(mismatch.sourceArguments));
  assert.equal(mismatchEval.code, 'WORK_MISMATCH');
  const extra = row('release-extra-domain-field');
  const extraEval = v5.evaluate(sourceText, 'release', ordinary(extra.sourceArguments), financialOf(extra.sourceArguments));
  assert.equal(extraEval.code, 'UNKNOWN_FIELD');
  const obs = row('release-nonempty-obs');
  const obsEval = v5.evaluate(sourceText, 'release', ordinary(obs.sourceArguments), financialOf(obs.sourceArguments));
  assert.equal(obsEval.code, 'INPUT_SCHEMA');
  const residual = v5.evaluate(sourceText, 'cancel', JSON.parse(readFileSync(CASES, 'utf8')).cases.find((item) => item.id === 'E12-cancel-preserves-duty').ordinaryInputCanonicalJSON, readFileSync(RESIDUAL, 'utf8'));
  assert.equal(residual.financialPost.obligations[0].outstanding, '40');
  assert.equal(fenceLifecycle(residual.financialPost).code, 'NONEMPTY_OBLIGATIONS');
});
