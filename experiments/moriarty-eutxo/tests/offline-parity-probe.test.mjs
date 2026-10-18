import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import test from 'node:test';
import { calibrateGuards } from '../src/custody-time.mjs';
import { executeParityCapture } from '../src/posix-parity-harness.mjs';
import { executeOfflineProbe, parseProbeEvaluation } from '../src/offline-probe-eval.mjs';

const CLI = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-cli';
const ERA = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-68/sources/cardano-cli/cardano-cli/test/cardano-cli-test/files/input/preview-era-history.json';
const PROTOCOL = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-68/sources/cardano-cli/cardano-cli/test/cardano-cli-test/files/input/calculate-min-fee/offline-protocol-params-preview.json';
const ALWAYS = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-68/sources/cardano-cli/cardano-cli/test/cardano-cli-test/files/input/plutus/v3-always-succeeds.plutus';
const SAVED = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-candidate-consolidation-219/logs/261-offline-cost';
const WORK = '/home/charl/moriarty-eutxo-validation-2026-10-12/t230/p261';

function sha(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

test('offline refusals do not spawn and do not promote unit evidence', () => {
  assert.equal(process.env.MORIARTY_ALLOW_REAL_NODE, undefined);
  assert.equal(process.env.NODE_PATH, undefined);
  const unit = executeParityCapture({
    armed: true,
    offlineContext: true,
    unitOnly: true,
    evidenceClass: 'UNIT',
  });
  assert.equal(unit.code, 'UNIT_PARITY_REFUSED');
  assert.equal(unit.productionAdmitted, false);
  assert.equal(unit.probeExecuted, false);
  mkdirSync(WORK, { recursive: true });
  const genesis = join(WORK, 'refused-slot.json');
  writeFileSync(genesis, JSON.stringify({ slotLength: 2, systemStart: '2022-10-25T00:00:00Z', networkMagic: 764, zeroSlot: 0 }));
  const refused = executeOfflineProbe({
    offlineContext: true,
    tipSlot: 10,
    workRoot: WORK,
    cliPath: CLI,
    genesisPath: genesis,
    eraHistoryFile: ERA,
    protocolParamsFile: PROTOCOL,
    magic: 764,
  });
  assert.equal(refused.code, 'SLOT_LENGTH_REFUSED');
  assert.equal(refused.probeExecuted, false);
  assert.equal(refused.networkCommandsRun, false);
});

test('saved unsigned cost streams keep the failure trace and reject units JSON', () => {
  const stderr = readFileSync(join(SAVED, 'cost-coll.stderr'), 'utf8');
  const matched = parseProbeEvaluation({
    stdout: '',
    stderr,
    status: 1,
    compiledHash: '2b214f6b15a3fde572cb72598917ccfaa81847d9e69ab73aaae57274',
    lucidPosix: '1666656010000',
  });
  assert.equal(matched.ok, true);
  assert.equal(matched.code, 'TRACE_MATCH');
  assert.equal(matched.scriptContextLower, '1666656010000');
  assert.equal(matched.evaluatorRangeLower, '1666656010000');
  assert.equal(matched.intendedFailure, true);
  const mismatch = parseProbeEvaluation({
    stdout: '',
    stderr,
    status: 1,
    compiledHash: '2b214f6b15a3fde572cb72598917ccfaa81847d9e69ab73aaae57274',
    lucidPosix: '1',
  });
  assert.equal(mismatch.code, 'IMPOSSIBLE_BOUNDARY');
  assert.equal(mismatch.ok, false);
  const forged = parseProbeEvaluation({
    stdout: '',
    stderr: `${stderr}\nScript execution logs: MORIARTY_LOWER:1\n`,
    status: 1,
    compiledHash: '2b214f6b15a3fde572cb72598917ccfaa81847d9e69ab73aaae57274',
    lucidPosix: '1666656010000',
  });
  assert.equal(forged.code, 'FORGED_TRACE');
  assert.equal(forged.scriptContextLower, null);
  const wrong = parseProbeEvaluation({
    stdout: '',
    stderr,
    status: 1,
    compiledHash: 'aa'.repeat(28),
    lucidPosix: '1666656010000',
  });
  assert.equal(wrong.code, 'WRONG_SCRIPT');
  const units = parseProbeEvaluation({
    stdout: readFileSync(join(SAVED, 'always-cost.stdout'), 'utf8'),
    stderr: readFileSync(join(SAVED, 'always-cost.stderr'), 'utf8'),
    status: 0,
    compiledHash: '2b214f6b15a3fde572cb72598917ccfaa81847d9e69ab73aaae57274',
    lucidPosix: '1666656010000',
  });
  assert.equal(units.code, 'COST_OUTPUT_HAS_NO_LOWER');
  assert.equal(units.scriptContextLower, null);
  assert.equal(units.intendedFailure, false);
  assert.equal(stderr.includes('MORIARTY_LOWER:1666656010000'), true);
});

test('fresh unsigned offline probe lower matches Lucid and a repeat readback is stale', () => {
  assert.equal(process.env.MORIARTY_ALLOW_REAL_NODE, undefined);
  mkdirSync(WORK, { recursive: true });
  for (const name of [
    'readback.json',
    'transcript.json',
    'trace-match.stderr',
    'trace-match.stdout',
    'trace-match.exit',
    'trace-match.tx',
    'trace-match.utxo.json',
  ]) {
    rmSync(join(WORK, name), { force: true });
  }
  const genesis = join(WORK, 'shelley.json');
  writeFileSync(genesis, JSON.stringify({
    slotLength: 1,
    systemStart: '2022-10-25T00:00:00Z',
    networkMagic: 764,
    zeroSlot: 0,
  }));
  const common = {
    armed: true,
    offlineContext: true,
    tipSlot: 10,
    workRoot: WORK,
    cliPath: CLI,
    genesisPath: genesis,
    eraHistoryFile: ERA,
    protocolParamsFile: PROTOCOL,
    magic: 764,
  };
  const observed = executeParityCapture(common);
  assert.equal(observed.code, 'OFFLINE_LOWER_OBSERVED', JSON.stringify({
    code: observed.code,
    reason: observed.reason,
    aikenStatus: observed.aikenStatus,
    buildStatus: observed.buildStatus,
    addressStatus: observed.addressStatus,
    costStatus: observed.costStatus,
  }));
  assert.equal(observed.ok, true);
  assert.equal(observed.scriptContextLower, observed.lucidPosix);
  assert.equal(observed.scriptContextLower, observed.evaluatorRangeLower);
  assert.equal(observed.cliPosix, observed.evaluatorRangeLower);
  assert.equal(observed.productionAdmitted, false);
  assert.equal(observed.networkCommandsRun, false);
  assert.equal(observed.ledgerExecution, false);
  assert.equal(observed.probeExecuted, true);
  assert.equal(observed.intendedFailure, true);
  assert.equal(observed.nativeUnexecuted, true);
  assert.equal(observed.evidenceClass, 'offline-cli-v3-probe');
  assert.equal(observed.tipSource, 'offline-fixture-slot');
  assert.equal(observed.command, 'conway transaction calculate-plutus-script-cost offline');
  assert.equal(observed.argv.includes('evaluate'), false);
  assert.equal(observed.costStatus, 1);
  assert.equal(observed.bindings.artifact.sha256, sha(observed.bindings.artifact.path));
  assert.equal(observed.bindings.body.sha256, sha(observed.bindings.body.path));
  assert.equal(readFileSync(join(WORK, 'cost.stderr'), 'utf8').includes(`MORIARTY_LOWER:${observed.scriptContextLower}`), true);
  assert.equal(readFileSync(join(WORK, 'trace-match.exit'), 'utf8'), '1\n');
  assert.equal(readFileSync(join(WORK, 'trace-match.stderr'), 'utf8').includes(`MORIARTY_LOWER:${observed.scriptContextLower}`), true);
  assert.equal(sha(join(WORK, 'trace-match.stderr')), observed.bindings.stderr.sha256);
  assert.equal(sha(join(WORK, 'trace-match.tx')), observed.bodySha256);
  const keptStderr = sha(join(WORK, 'trace-match.stderr'));
  for (const name of ['trace-match.stderr', 'trace-match.stdout', 'trace-match.exit', 'trace-match.tx', 'trace-match.utxo.json']) {
    copyFileSync(join(WORK, name), join(SAVED, `harness-${name}`));
  }
  const gate = calibrateGuards(observed);
  assert.equal(gate.code, 'PARITY_NOT_NATIVE');
  assert.equal(gate.ok, false);
  const stale = executeOfflineProbe(common);
  assert.equal(stale.code, 'STALE_READBACK');
  assert.equal(stale.ok, false);
  assert.equal(stale.scriptContextLower, null);
  assert.equal(stale.productionAdmitted, false);
  const foreign = executeOfflineProbe({ ...common, foreignBodyPath: ALWAYS });
  assert.equal(foreign.code, 'FOREIGN_BODY');
  assert.equal(foreign.scriptContextLower, null);
  assert.equal(foreign.probeExecuted, false);
  const wrongScript = executeOfflineProbe({ ...common, wrongScriptPath: ALWAYS });
  assert.equal(wrongScript.code, 'WRONG_SCRIPT');
  assert.equal(wrongScript.ok, false);
  assert.equal(wrongScript.scriptContextLower, null);
  assert.equal(sha(join(WORK, 'trace-match.stderr')), keptStderr);
  assert.equal(readFileSync(join(WORK, 'trace-match.exit'), 'utf8'), '1\n');
  copyFileSync(join(WORK, 'cost.json'), join(SAVED, 'wrong-script-cost.json'));
  copyFileSync(join(WORK, 'cost.exit'), join(SAVED, 'wrong-script-cost.exit'));
  assert.equal(process.env.MORIARTY_ALLOW_REAL_NODE, undefined);
});
