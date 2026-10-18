/**
 * Layer N time controls. Outcomes are the revision03 time section,
 * the time211 fixture measurement, and the bootstrap183 1000 ms slot.
 * UNIT/SYNTHETIC results are not production parity and not ledger acceptance.
 */
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { slotToBeginUnixTime, unixTimeToEnclosingSlot } from '/home/charl/moriarty-eutxo-validation-2026-10-12/dependency203-verified-freeze206/packet199/snapshot/node_modules/@lucid-evolution/plutus/dist/index.js';
import { admitCustody } from '../src/admit-custody.mjs';
import { bindCapacity } from '../src/custody-capacity.mjs';
import { bindOracleSpends } from '../src/custody-oracle-bind.mjs';
import {
  actionTimeGuards,
  bindSpendTime,
  calibrateGuards,
  planWindows,
  posixWindowToSlots,
  slotsToRedeemer,
  substituteTimeGuards,
} from '../src/custody-time.mjs';
import { redeemerFromArgs } from '../src/native-financial.mjs';
import { executeParityCapture } from '../src/posix-parity-harness.mjs';
import { runNativeCaller } from '../src/native-caller.mjs';
import { CARDANO_CLI } from '../src/native-runtime.mjs';

const SOURCE = '/home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time/escrow-custody-v5.mori';
const SEED = '/home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time/financial-seed-custody-v5.json';
const MANIFEST = new URL('../examples/custody-unit.manifest.json', import.meta.url);
const HARNESS = new URL('../src/posix-parity-harness.mjs', import.meta.url);
const BOUNDS = 'b3d02ace7a7ac7daec45318f2814beff4d27ad26f91d45405942c30d04fa846d';
const START = '1791979230000';
const ZERO = { zeroTime: '0', zeroSlot: '0', slotLengthMs: '1000' };

function sha(text) {
  return createHash('sha256').update(text).digest('hex');
}

test('design window keeps POSIX redeemer bounds and half-open slot bounds apart', () => {
  const planned = planWindows({ ...ZERO, tipSlot: '1700000000' });
  assert.equal(planned.ok, true);
  assert.equal(planned.releaseArgs.lower, '1700000030000');
  assert.equal(planned.releaseArgs.upper, '1700000089999');
  assert.equal(planned.refundArgs.lower, '1700000090000');
  assert.equal(planned.refundArgs.upper, '1700000149999');
  assert.equal(planned.releaseSlots.startSlot, '1700000030');
  assert.equal(planned.releaseSlots.endSlot, '1700000090');
  assert.equal(planned.refundSlots.startSlot, '1700000090');
  assert.equal(planned.refundSlots.endSlot, '1700000150');
  assert.deepEqual(planned.cancelSlots, planned.releaseSlots);
  assert.notEqual(planned.releaseSlots.startSlot, planned.releaseArgs.lower);
  assert.equal(planned.ledgerExecution, false);
  assert.equal(planned.sourceEmitted, false);
});

test('a 1000 ms slot converts and 999 or 1001 do not', () => {
  const passed = slotsToRedeemer({ ...ZERO, startSlot: '1', endSlot: '2' });
  assert.equal(passed.ok, true);
  assert.equal(passed.lower, '1000');
  assert.equal(passed.upper, '1999');
  assert.equal(passed.invalidBefore, '1');
  assert.equal(passed.invalidHereafter, '2');
  const lucid = slotToBeginUnixTime(1, { zeroTime: 0, zeroSlot: 0, slotLength: 1000 });
  assert.equal(String(lucid), passed.lower);
  assert.equal(unixTimeToEnclosingSlot(1000, { zeroTime: 0, zeroSlot: 0, slotLength: 1000 }), 1);
  assert.equal(slotsToRedeemer({ ...ZERO, slotLengthMs: '999', startSlot: '1', endSlot: '2' }).code, 'SLOT_LENGTH_REFUSED');
  assert.equal(slotsToRedeemer({ ...ZERO, slotLengthMs: '1001', startSlot: '1', endSlot: '2' }).code, 'SLOT_LENGTH_REFUSED');
});

test('overflow, negative, rounding, empty, and refund upper-exclusive intervals fail', () => {
  assert.equal(planWindows({ ...ZERO, tipSlot: '340282366920938463463374607431768211455' }).code, 'OVERFLOW');
  assert.equal(planWindows({ ...ZERO, zeroTime: '-1', tipSlot: '1' }).code, 'NEGATIVE_POSIX');
  assert.equal(posixWindowToSlots({ ...ZERO, kind: 'release', lower: '1001', upper: '1999' }).code, 'ROUNDING');
  assert.equal(slotsToRedeemer({ ...ZERO, startSlot: '5', endSlot: '5' }).code, 'EMPTY_INTERVAL');
  assert.equal(slotsToRedeemer({ ...ZERO, startSlot: '5', endSlot: '4' }).code, 'EMPTY_INTERVAL');
  assert.equal(posixWindowToSlots({ ...ZERO, kind: 'refund', lower: '2000', upper: '3000' }).code, 'REFUND_UPPER_EXCLUSIVE');
});

test('measured fixture windows stay unrepresentable on the approved clock', () => {
  const clock = { zeroTime: START, zeroSlot: '0', slotLengthMs: '1000' };
  for (const row of [
    ['release', '1000', '1099'],
    ['cancel', '1050', '1051'],
    ['refund', '1100', '1100'],
  ]) {
    const bound = bindSpendTime({ kind: row[0], lower: row[1], upper: row[2], ...clock });
    assert.equal(bound.ok, false, row[0]);
    assert.equal(bound.code, 'FIXTURE_WINDOW_UNREPRESENTABLE');
    assert.equal(bound.cliIntervalEmitted, false);
    assert.equal(bound.redeemerLower, row[1]);
    assert.equal(bound.redeemerUpper, row[2]);
  }
  const zeroRelease = bindSpendTime({ kind: 'release', lower: '1000', upper: '1099', ...ZERO });
  assert.equal(zeroRelease.ok, false);
  assert.equal(zeroRelease.detail, 'ROUNDING');
  const planned = planWindows({ ...clock, tipSlot: '0' });
  assert.equal(planned.ok, true);
  const release = bindSpendTime({
    kind: 'release',
    lower: planned.releaseArgs.lower,
    upper: planned.releaseArgs.upper,
    ...clock,
  });
  assert.equal(release.ok, true);
  assert.equal(release.invalidBefore, planned.releaseSlots.startSlot);
  assert.equal(release.invalidHereafter, planned.releaseSlots.endSlot);
  const refund = bindSpendTime({
    kind: 'refund',
    lower: planned.refundArgs.lower,
    upper: planned.refundArgs.upper,
    ...clock,
  });
  assert.equal(refund.ok, true);
  assert.equal(refund.invalidBefore, planned.refundSlots.startSlot);
  assert.equal(refund.invalidHereafter, planned.refundSlots.endSlot);
  assert.equal(release.invalidBefore, '30');
  assert.equal(release.invalidHereafter, '90');
});

test('redeemer fields stay POSIX while a supplied clock reports slot bounds', () => {
  const planned = planWindows({ ...ZERO, tipSlot: '1700000000' });
  const legacy = redeemerFromArgs('release', {
    actor: 'Lender', recipient: 'Lender', transferId: 'Release1', lower: '1000', upper: '1099',
  });
  assert.equal(legacy.ok, true);
  assert.equal(legacy.lower, 1000);
  assert.equal(legacy.upper, 1099);
  assert.equal(legacy.intervalClass, 'legacy-unseparated');
  const separated = redeemerFromArgs('release', {
    actor: 'Lender',
    recipient: 'Lender',
    transferId: 'Release1',
    lower: planned.releaseArgs.lower,
    upper: planned.releaseArgs.upper,
  }, { ...ZERO, tipSlot: '1700000000' });
  assert.equal(separated.ok, true);
  assert.equal(separated.intervalClass, 'slot-posix-separated');
  assert.equal(separated.plutus.fields[3].int, Number(planned.releaseArgs.lower));
  assert.equal(separated.plutus.fields[4].int, Number(planned.releaseArgs.upper));
  assert.equal(separated.invalidBefore, Number(planned.releaseSlots.startSlot));
  assert.equal(separated.invalidHereafter, Number(planned.releaseSlots.endSlot));
  assert.notEqual(separated.invalidBefore, separated.lower);
});

test('guard substitution touches only the time predicates', () => {
  const source = readFileSync(SOURCE, 'utf8');
  const before = sha(source);
  const decoy = `${source}\n// non-time literal 1000 stays\n`;
  const sub = substituteTimeGuards(decoy, {
    releaseLower: '1700000030000',
    partition: '1700000090000',
    refundInclusive: '1700000149999',
  });
  assert.equal(sub.ok, true);
  assert.equal(sub.evidenceClass, 'UNIT/SYNTHETIC');
  assert.equal(sub.productionArtifact, false);
  assert.equal(sha(source), before);
  assert.match(sub.text, /requires lower >= 1700000030000;/);
  assert.match(sub.text, /requires upper < 1700000090000;/);
  assert.match(sub.text, /requires lower >= 1700000090000;/);
  assert.match(sub.text, /requires upper <= 1700000149999;/);
  assert.equal(sub.text.includes('requires lower >= 1000;'), false);
  assert.equal(sub.text.includes('requires upper < 1100;'), false);
  assert.equal(sub.text.includes('requires lower >= 1100;'), false);
  assert.equal(sub.text.includes('requires upper <= 1200;'), false);
  assert.match(sub.text, /amount<Cash>\(100\)/);
  assert.match(sub.text, /non-time literal 1000 stays/);
  assert.match(sub.text, /pre\.locked == 1/);
  assert.match(sub.text, /pre\.revision \+ 1/);
});

test('synthetic sibling changes source and program hashes and keeps bounds', async () => {
  const source = readFileSync(SOURCE, 'utf8');
  const seedText = readFileSync(SEED, 'utf8');
  const manifest = JSON.parse(readFileSync(MANIFEST, 'utf8'));
  const sub = substituteTimeGuards(source, {
    releaseLower: '1700000030000',
    partition: '1700000090000',
    refundInclusive: '1700000149999',
  });
  const before = admitCustody({ sourceText: source, seedText, manifest });
  const after = admitCustody({ sourceText: sub.text, seedText, manifest });
  assert.equal(before.ok, true);
  assert.equal(after.ok, true);
  assert.equal(before.hashes.boundsHash, BOUNDS);
  assert.equal(after.hashes.boundsHash, BOUNDS);
  assert.notEqual(after.hashes.sourceHash, before.hashes.sourceHash);
  assert.notEqual(after.hashes.programHash, before.hashes.programHash);
  assert.equal(after.actions.every((row) => row.staticWorkBound === '48' && row.debit === 49), true);
  const { createFinancialAgreementSourceV5 } = await import('../../moriarty-language/src/successor/financial-agreement-source-v5.ts');
  const evaluator = createFinancialAgreementSourceV5();
  const ordinary = JSON.stringify({
    Args: {
      actor: 'Lender',
      lower: '1700000030000',
      recipient: 'Lender',
      transferId: 'Release1',
      upper: '1700000089999',
    },
    Obs: {},
    Pre: { locked: '1', revision: '0' },
    workInitial: '1024',
  });
  const evaluated = evaluator.evaluate(sub.text, 'release', ordinary, seedText);
  assert.equal(evaluated.status, 'FundedExpressionPrepared');
  assert.equal(evaluated.financialPost.balances.find((row) => row.party === 'Lender').amount, '100');
  const stale = evaluator.evaluate(sub.text, 'release', JSON.stringify({
    Args: { actor: 'Lender', lower: '1000', recipient: 'Lender', transferId: 'Release1', upper: '1099' },
    Obs: {},
    Pre: { locked: '1', revision: '0' },
    workInitial: '1024',
  }), seedText);
  assert.notEqual(stale.status, 'FundedExpressionPrepared');
});

test('missing, mismatched, forged, and unit-only parity refuse production', () => {
  assert.equal(calibrateGuards(undefined).code, 'PARITY_ABSENT');
  assert.equal(calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
  }).code, 'CALIBRATED_SOURCE_NOT_THIS_PHASE');
  assert.equal(calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'UNIT/SYNTHETIC',
    unitOnly: true,
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
    bindings: { present: true },
  }).code, 'UNIT_PARITY_REFUSED');
  assert.equal(calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
    bindings: { forged: true, genesis: {}, artifact: {}, body: {}, readback: { present: true } },
  }).code, 'FORGED_PARITY');
  assert.equal(calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '5',
    cliPosix: '5',
    scriptContextLower: '6',
    bindings: { genesis: { sha256: 'aa' }, artifact: { sha256: 'bb' }, body: { sha256: 'cc' }, readback: { present: true, sha256: 'dd' } },
  }).code, 'IMPOSSIBLE_BOUNDARY');
  const wrongGenesis = calibrateGuards({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    unitOnly: false,
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
    bindings: {
      genesis: { sha256: '11', systemStart: '2026-10-14T12:00:30Z', zeroSlot: '0', slotLengthMs: '1000' },
      artifact: { sha256: '22' },
      body: { sha256: '33', invalidBefore: '1', invalidHereafter: '2' },
      readback: { present: false },
    },
  });
  assert.equal(wrongGenesis.ok, false);
  assert.equal(wrongGenesis.sourceEmitted, false);
  assert.equal(wrongGenesis.code, 'READBACK_BINDING_ABSENT');
});

test('unarmed parity harness does not invent a V3 lower bound', () => {
  const ran = spawnSync(process.execPath, [HARNESS.pathname], { encoding: 'utf8' });
  assert.notEqual(ran.status, 0);
  const transcript = JSON.parse(ran.stdout);
  assert.equal(transcript.productionAdmitted, false);
  assert.equal(transcript.scriptContextLower, null);
  assert.equal(transcript.ledgerExecution, false);
  assert.equal(transcript.networkCommandsRun, false);
  assert.equal(transcript.code, 'PARITY_NOT_EXECUTED');
});

test('capacity oracle derives the release window from the selected action', () => {
  const source = readFileSync(SOURCE, 'utf8');
  const seed = JSON.parse(readFileSync(SEED, 'utf8'));
  const manifest = JSON.parse(readFileSync(MANIFEST, 'utf8'));
  const layerWindow = actionTimeGuards(source, 'release');
  assert.equal(layerWindow.lower, '1000');
  assert.equal(layerWindow.upper, '1099');
  assert.equal(layerWindow.exclusiveUpper, '1100');
  const layer = bindCapacity({ sourceText: source, seed, manifest });
  assert.equal(layer.ok, true, layer.detail);
  assert.equal(layer.capacity, 128);
  const lender = layer.ids127.post.balances.find((row) => row.party === 'Lender' && row.asset === 'Cash');
  assert.equal(lender.amount, '100');
  assert.equal(layer.ids127.post.work.spent, '49');
  const bounds = {
    releaseLower: '1700000030000',
    partition: '1700000090000',
    refundInclusive: '1700000149999',
  };
  const substituted = substituteTimeGuards(source, bounds);
  assert.equal(substituted.ok, true);
  const sibling = bindCapacity({
    sourceText: substituted.text,
    seed,
    manifest,
    timeBinding: { evidenceClass: 'UNIT/SYNTHETIC', productionArtifact: false, bounds },
  });
  assert.equal(sibling.ok, true, sibling.detail);
  const siblingLender = sibling.ids127.post.balances.find((row) => row.party === 'Lender' && row.asset === 'Cash');
  assert.equal(siblingLender.amount, '100');
  assert.equal(sibling.ids127.post.work.spent, '49');
  assert.equal(sibling.ids127.post.work.remaining, '975');
  const staleBinding = bindCapacity({
    sourceText: substituted.text,
    seed,
    manifest,
    timeBinding: {
      evidenceClass: 'UNIT/SYNTHETIC',
      productionArtifact: false,
      bounds: { releaseLower: '1000', partition: '1100', refundInclusive: '1200' },
    },
  });
  assert.equal(staleBinding.ok, false);
  assert.equal(staleBinding.code, 'ORACLE_DIVERGENCE');
  const forgedProduction = bindCapacity({
    sourceText: source,
    seed,
    manifest,
    timeBinding: { evidenceClass: 'UNIT/SYNTHETIC', productionArtifact: true, bounds },
  });
  assert.equal(forgedProduction.code, 'UNIT_PARITY_REFUSED');
});

test('parity capture stays fail-closed and a unit stub cannot admit production', () => {
  const root = '/home/charl/moriarty-eutxo-validation-2026-10-12/t230/c';
  mkdirSync(root, { recursive: true });
  const cli = `${root}/cli`;
  const socket = `${root}/node.sock`;
  const genesis = `${root}/shelley.json`;
  writeFileSync(cli, 'unit-cli\n');
  writeFileSync(socket, '');
  writeFileSync(genesis, JSON.stringify({ slotLength: 1, systemStart: '2026-01-01T00:00:00Z', networkMagic: 764 }));
  writeFileSync(`${root}/bad.json`, JSON.stringify({ slotLength: 2, systemStart: '2026-01-01T00:00:00Z' }));
  writeFileSync(`${root}/review-a.json`, '{"evidenceClass":"reviewed-implementation"}\n');
  writeFileSync(`${root}/review-b.json`, '{"evidenceClass":"reviewed-implementation"}\n');
  let calls = 0;
  const touch = () => { calls += 1; return { status: 0, stdout: '' }; };
  assert.equal(executeParityCapture({}).code, 'PARITY_NOT_EXECUTED');
  assert.equal(executeParityCapture({}).networkCommandsRun, false);
  assert.equal(executeParityCapture({ armed: true, slotLengthMs: '999', cliPath: cli, socketPath: socket, spawn: touch }).code, 'SLOT_LENGTH_REFUSED');
  assert.equal(executeParityCapture({ armed: true, forged: true, cliPath: cli, socketPath: socket, spawn: touch }).code, 'FORGED_PARITY');
  assert.equal(executeParityCapture({ armed: true, productionAdmitted: true, cliPath: cli, socketPath: socket, spawn: touch }).code, 'FORGED_PARITY');
  assert.equal(executeParityCapture({ armed: true, cliPath: cli, socketPath: socket, spawn: touch }).code, 'REVIEW_ABSENT');
  assert.equal(executeParityCapture({
    armed: true,
    unitOnly: true,
    cliPath: cli,
    socketPath: socket,
    genesisPath: `${root}/bad.json`,
    magic: 764,
    spawn: touch,
  }).code, 'SLOT_LENGTH_REFUSED');
  assert.equal(calls, 0);
  const start = Date.parse('2026-01-01T00:00:00Z');
  const lower = String(start + 10 * 1000);
  const unit = executeParityCapture({
    armed: true,
    unitOnly: true,
    evidenceClass: 'UNIT',
    cliPath: cli,
    socketPath: socket,
    genesisPath: genesis,
    magic: 764,
    tipSlot: 10,
    spawn(argv) {
      calls += 1;
      if (argv[2] === 'tip') return { status: 0, stdout: JSON.stringify({ slot: 10, hash: 'ab', block: 1 }) };
      if (argv[1] === 'conway') return { status: 0, stdout: `scriptContextLower=${lower}\n` };
      return { status: 0, stdout: '{}\n' };
    },
  });
  assert.equal(calls >= 3, true);
  assert.equal(unit.calls[0][2], 'tip');
  assert.equal(unit.calls[1][2], 'era-history');
  assert.equal(unit.calls[2][1], 'conway');
  assert.equal(unit.calls[2][2], 'transaction');
  assert.equal(unit.calls[2][3], 'build-raw');
  assert.equal(unit.calls[2][4], '--invalid-before');
  assert.equal(unit.calls[2][5], '10');
  assert.equal(unit.calls[2][6], '--invalid-hereafter');
  assert.equal(unit.calls[2][7], '11');
  assert.equal(unit.productionAdmitted, false);
  assert.equal(unit.networkCommandsRun, false);
  assert.equal(unit.ledgerExecution, false);
  assert.equal(unit.siblingEmitted, false);
  assert.equal(unit.cliPosix, lower);
  assert.equal(unit.lucidPosix, lower);
  assert.equal(unit.scriptContextLower, lower);
  assert.equal(unit.code, 'UNIT_PARITY_OBSERVED');
  const blocked = executeParityCapture({
    armed: true,
    cliPath: cli,
    socketPath: socket,
    reviewA: `${root}/review-a.json`,
    reviewB: `${root}/review-b.json`,
    genesisPath: genesis,
    magic: 764,
    spawn: touch,
  });
  assert.equal(blocked.code, 'REAL_QUERY_NOT_ARMED');
  assert.equal(blocked.networkCommandsRun, false);
});

test('synthetic oracle cases derive the action window and keep economic failures', () => {
  const packet = JSON.parse(readFileSync('/home/charl/moriarty-eutxo-validation-2026-10-12/t1-independent-oracles-19/RESULTS.json', 'utf8'));
  const bounds = {
    releaseLower: '1700000030000',
    partition: '1700000090000',
    refundInclusive: '1700000149999',
  };
  const historical = bindOracleSpends(packet);
  assert.match(historical.source, /test oracle_e13_release_control\(\) \{/);
  assert.match(historical.source, /transfer_id: "Release1", lower: 1000, upper: 1099 \}/);
  assert.equal(historical.source.includes('1700000030000'), false);
  const bound = bindOracleSpends(packet, { evidenceClass: 'UNIT/SYNTHETIC', productionArtifact: false, bounds });
  const body = bound.source;
  assert.match(body, /test oracle_e13_release_control\(\) \{[\s\S]*lower: 1700000030000, upper: 1700000089999 \}, "Lender", "Lender", 100, 0, 100, 1700000030000, 1700000090000\)/);
  assert.match(body, /test oracle_e13_wrong_recipient\(\) fail \{[\s\S]*recipient: "Attacker", transfer_id: "Release1", lower: 1700000030000, upper: 1700000089999 \}/);
  assert.match(body, /test oracle_e13_upper_1100\(\) fail \{[\s\S]*lower: 1700000030000, upper: 1700000090000 \}/);
  assert.match(body, /test oracle_e13_lower_gt_upper\(\) fail \{[\s\S]*lower: 1700000089999, upper: 1700000030000 \}/);
  assert.match(body, /test oracle_e11_early\(\) fail \{[\s\S]*lower: 1700000089999, upper: 1700000089999 \}/);
  assert.match(body, /test oracle_e11_past_end\(\) fail \{[\s\S]*lower: 1700000149999, upper: 1700000150000 \}/);
  assert.match(body, /test oracle_refund_full_window\(\) \{[\s\S]*lower: 1700000090000, upper: 1700000149999 \}/);
  assert.match(body, /test oracle_release_point_1000\(\) \{[\s\S]*lower: 1700000030000, upper: 1700000030000 \}/);
  assert.match(body, /test oracle_release_point_1099\(\) \{[\s\S]*lower: 1700000089999, upper: 1700000089999 \}/);
  assert.match(body, /test oracle_refund_point_1200\(\) \{[\s\S]*lower: 1700000149999, upper: 1700000149999 \}/);
  assert.equal(body.includes('lower: 1000, upper: 1099'), false);
  assert.equal(body.includes(', 100, 0, 100,'), true);
  assert.throws(
    () => bindOracleSpends(packet, { evidenceClass: 'UNIT/SYNTHETIC', productionArtifact: true, bounds }),
    (error) => error.code === 'UNIT_PARITY_REFUSED',
  );
});

test('real cli financial body refuses before encode without admitted parity', async () => {
  assert.equal(existsSync(CARDANO_CLI), true);
  const ids = ['11', '22', '33', '44', '55'].map((byte, index) => ({ txId: byte.repeat(32), index }));
  const missing = await runNativeCaller({
    financialBody: true,
    buildBody: true,
    cliPath: CARDANO_CLI,
    utxos: ids,
  });
  assert.equal(missing.stages.inputs.ok, true);
  assert.equal(missing.stages.inputs.code, 'INPUTS_SELECTED');
  assert.equal(missing.stages.body.code, 'PARITY_NOT_ADMITTED');
  assert.equal(missing.stages.body.bodyBuilt, false);
  assert.equal(missing.stages.body.cliIntervalEmitted, false);
  assert.equal(missing.stages.body.cborHex, undefined);
  assert.equal(missing.stages.spend.code, 'PARITY_NOT_ADMITTED');
  assert.equal(missing.ledgerExecution, false);
  assert.equal(missing.nativePins, false);
  assert.equal(missing.startedNode, false);
  assert.equal(missing.ledgerSubmitted, false);
  const root = '/home/charl/moriarty-eutxo-validation-2026-10-12/t230/c';
  mkdirSync(root, { recursive: true });
  writeFileSync(`${root}/node.sock`, '');
  const unarmed = await runNativeCaller({
    financialBody: true,
    buildBody: true,
    cliPath: CARDANO_CLI,
    utxos: ids,
    time: { cliPath: CARDANO_CLI, socket: `${root}/node.sock`, execute: true, magic: 764 },
  });
  assert.equal(unarmed.stages.time.code, 'REAL_QUERY_NOT_ARMED');
  assert.equal(unarmed.stages.time.spawned, false);
  assert.equal(unarmed.stages.productionTime.code === 'PARITY_ADMITTED', false);
  assert.equal(unarmed.stages.body.code, 'PARITY_NOT_ADMITTED');
  assert.equal(unarmed.stages.body.cborHex, undefined);
  assert.equal(unarmed.ledgerExecution, false);
  assert.equal(unarmed.startedNode, false);
});

test('armed observation binds queried tip and readback files and does not admit production', () => {
  const root = '/home/charl/moriarty-eutxo-validation-2026-10-12/t230/p';
  mkdirSync(root, { recursive: true });
  const cli = `${root}/cli`;
  const socket = `${root}/node.sock`;
  const genesis = `${root}/shelley.json`;
  writeFileSync(cli, 'unit-cli\n');
  writeFileSync(socket, '');
  writeFileSync(genesis, JSON.stringify({ slotLength: 1, systemStart: '2026-01-01T00:00:00Z', networkMagic: 764 }));
  writeFileSync(`${root}/review-a.json`, '{"evidenceClass":"reviewed-implementation"}\n');
  writeFileSync(`${root}/review-b.json`, '{"evidenceClass":"reviewed-implementation"}\n');
  writeFileSync(`${root}/audit-a.json`, '{"audit":"full-a"}\n');
  writeFileSync(`${root}/audit-b.json`, '{"audit":"full-b"}\n');
  const bodyPath = `${root}/probe.body`;
  const artifactPath = `${root}/probe.ak`;
  const readbackPath = `${root}/readback.json`;
  const transcriptPath = `${root}/transcript.json`;
  const start = Date.parse('2026-01-01T00:00:00Z');
  const lower = String(start + 10 * 1000);
  const common = {
    armed: true,
    cliPath: cli,
    socketPath: socket,
    genesisPath: genesis,
    magic: 764,
    reviewA: `${root}/review-a.json`,
    reviewB: `${root}/review-b.json`,
    auditA: `${root}/audit-a.json`,
    auditB: `${root}/audit-b.json`,
    bodyPath,
    artifactPath,
    readbackPath,
    transcriptPath,
    tipSlot: 99,
    scriptContextLower: lower,
  };
  const spawn = (readbackBody) => (argv) => {
    if (argv[2] === 'tip') return { status: 0, stdout: JSON.stringify({ slot: 10, hash: 'ab', block: 1 }) };
    if (argv.includes('build-raw')) {
      writeFileSync(argv[argv.indexOf('--out-file') + 1], 'probe-body\n');
      return { status: 0, stdout: '' };
    }
    if (argv[0] === 'probe-context' && readbackBody != null) writeFileSync(argv[1], readbackBody);
    return { status: 0, stdout: '{}\n' };
  };
  const prev = process.env.MORIARTY_ALLOW_REAL_NODE;
  process.env.MORIARTY_ALLOW_REAL_NODE = '1';
  try {
    const fenced = executeParityCapture({
      ...common,
      bodyPath: '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-native-caller-50/checks/probe.body',
      spawn() { throw new Error('historical path spawned'); },
    });
    assert.equal(fenced.code, 'OUTPUT_PATH_FORBIDDEN');
    const missing = executeParityCapture({ ...common, readbackPath: `${root}/no-readback.json`, spawn: spawn(null) });
    assert.equal(missing.code, 'READBACK_BINDING_ABSENT');
    assert.equal(missing.scriptContextLower, null);
    assert.equal(missing.tipSlot, '10');
    assert.equal(missing.cliPosix, lower);
    assert.equal(missing.productionAdmitted, false);
    assert.equal(missing.networkCommandsRun, false);
    assert.equal(missing.evaluateCode, 'NO_CLI_EVALUATE_COMMAND');
    assert.equal(missing.calls[2][4], '--invalid-before');
    assert.equal(missing.calls[2][5], '10');
    writeFileSync(readbackPath, JSON.stringify({ scriptContextLower: lower }));
    const stale = executeParityCapture({ ...common, spawn: spawn(null) });
    assert.equal(stale.code, 'STALE_READBACK');
    assert.equal(stale.scriptContextLower, null);
    assert.equal(stale.productionAdmitted, false);
    const wrong = executeParityCapture({ ...common, spawn: spawn(JSON.stringify({ scriptContextLower: '1' })) });
    assert.equal(wrong.code, 'IMPOSSIBLE_BOUNDARY');
    assert.equal(wrong.scriptContextLower, '1');
    assert.equal(wrong.cliPosix, lower);
    assert.equal(wrong.productionAdmitted, false);
    const promoted = executeParityCapture({
      ...common,
      spawn: spawn(JSON.stringify({ scriptContextLower: lower, productionAdmitted: true })),
    });
    assert.equal(promoted.code, 'FORGED_PARITY');
    assert.equal(promoted.scriptContextLower, null);
    assert.equal(promoted.productionAdmitted, false);
    const observed = executeParityCapture({ ...common, spawn: spawn(JSON.stringify({ scriptContextLower: lower })) });
    assert.equal(observed.code, 'INJECTED_CONTEXT_PARSED');
    assert.equal(observed.ok, false);
    assert.equal(observed.productionAdmitted, false);
    assert.equal(observed.networkCommandsRun, false);
    assert.equal(observed.ledgerExecution, false);
    assert.equal(observed.siblingEmitted, false);
    assert.equal(observed.sourceEmitted, false);
    assert.equal(observed.evidenceClass, 'injected-observation');
    assert.equal(observed.cliPosix, lower);
    assert.equal(observed.lucidPosix, lower);
    assert.equal(observed.scriptContextLower, lower);
    assert.equal(observed.tipSlot, '10');
    assert.equal(observed.ignoredCallerTip, '99');
    assert.equal(observed.ignoredCallerLower, lower);
    assert.equal(observed.probeExecuted, false);
    const sha = (path) => createHash('sha256').update(readFileSync(path)).digest('hex');
    assert.equal(observed.bindings.genesis.sha256, sha(genesis));
    assert.equal(observed.bindings.body.sha256, sha(bodyPath));
    assert.equal(observed.bindings.artifact.sha256, sha(artifactPath));
    assert.equal(observed.bindings.readback.sha256, sha(readbackPath));
    assert.equal(readFileSync(artifactPath, 'utf8').includes('validator parity_probe'), true);
    const transcript = JSON.parse(readFileSync(transcriptPath, 'utf8'));
    assert.equal(transcript.productionAdmitted, false);
    assert.equal(transcript.networkCommandsRun, false);
    assert.equal(transcript.evidenceClass, 'injected-observation');
    assert.equal(transcript.cliPosix, lower);
    assert.equal(transcript.scriptContextLower, lower);
    assert.equal(calibrateGuards({
      probeExecuted: true,
      evidenceClass: observed.evidenceClass,
      unitOnly: false,
      slotLengthMs: '1000',
      lucidPosix: observed.lucidPosix,
      cliPosix: observed.cliPosix,
      scriptContextLower: observed.scriptContextLower,
      bindings: observed.bindings,
      transcriptPath,
    }).code, 'PARITY_NOT_NATIVE');
  } finally {
    if (prev === undefined) delete process.env.MORIARTY_ALLOW_REAL_NODE;
    else process.env.MORIARTY_ALLOW_REAL_NODE = prev;
  }
  assert.equal(process.env.MORIARTY_ALLOW_REAL_NODE, prev);
});
