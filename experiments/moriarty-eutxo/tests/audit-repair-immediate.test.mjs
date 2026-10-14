/**
 * Immediate-group controls for F2, F5, and B1.
 * Calls exported runNativeCaller, observePosix, and prepareGuardSibling.
 * No node latch. A stub CLI is not cardano-cli. ledgerExecution stays false.
 * F3 and F4 call the exported spend and head interval functions.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { chmodSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { CARDANO_CLI } from '../src/native-body.mjs';
import { boundSpendInterval, consumedHeadInterval, runNativeCaller } from '../src/native-caller.mjs';
import { observePosix, prepareGuardSibling } from '../src/native-consumer.mjs';
import { planWindows, productionParityGate } from '../src/custody-time.mjs';
import { planWindows as schemaWindows } from '../owned/runtime-schemas-71/schemas.mjs';

const IDS = ['11', '22', '33', '44', '55'].map((byte) => ({ txId: byte.repeat(32), index: 0 }));

function unboundEvidence(extra = {}) {
  return {
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
    measuredByThisProcess: true,
    ...extra,
  };
}

test('B1 unbound equal POSIX is the calibrateGuards refusal, not a digit measurement', () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-b1-'));
  const socket = join(dir, 'node.sock');
  writeFileSync(socket, '');
  const plain = observePosix(unboundEvidence());
  assert.equal(plain.code, 'CALIBRATED_SOURCE_NOT_THIS_PHASE');
  assert.equal(plain.siblingEmitted, false);
  assert.equal(plain.measured, false);
  assert.equal(plain.flagIgnored, true);
  assert.equal(plain.ledgerExecution, false);
  const queried = observePosix(unboundEvidence({
    query: { queried: true, unitOnly: false, socket, stdout: 'lucid 1000 slotLengthMs 1000' },
  }));
  assert.equal(queried.code, 'CALIBRATED_SOURCE_NOT_THIS_PHASE');
  assert.equal(queried.measured, false);
  assert.equal(queried.siblingEmitted, false);
  assert.notEqual(queried.code, 'PARITY_MATCHED');
  const sibling = prepareGuardSibling(unboundEvidence({
    query: { queried: true, unitOnly: false, socket, stdout: '1000' },
  }), {
    sourceText: 'x'.repeat(10000),
    previousProgramHash: 'cd'.repeat(28),
    compile(input) {
      assert.equal(typeof input.sourceText, 'string');
      return { binary: 'aiken', programHash: 'ab'.repeat(28), unitOnly: false };
    },
  });
  assert.equal(sibling.code, 'CALIBRATED_SOURCE_NOT_THIS_PHASE');
  assert.equal(sibling.siblingEmitted, false);
  const gate = productionParityGate({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
  });
  assert.equal(gate.code, 'CALIBRATED_SOURCE_NOT_THIS_PHASE');
  assert.equal(plain.code, gate.code);
  assert.equal(gate.ok, false);
});

test('F5 real cardano-cli unitOnly does not skip the parity block', async () => {
  const request = {
    financialBody: true,
    buildBody: true,
    cliPath: CARDANO_CLI,
    utxos: IDS,
  };
  const control = await runNativeCaller(request);
  assert.equal(control.stages.body.code, 'PARITY_NOT_ADMITTED');
  assert.equal(control.stages.body.cborHex, undefined);
  assert.equal(control.stages.spend.code, 'PARITY_NOT_ADMITTED');
  assert.equal(control.ledgerExecution, false);
  assert.equal(control.ledgerSubmitted, false);
  assert.equal(control.startedNode, false);
  const unit = await runNativeCaller({ ...request, unitOnly: true });
  assert.equal(unit.stages.body.code, 'PARITY_NOT_ADMITTED');
  assert.equal(unit.stages.body.cborHex, undefined);
  assert.equal(unit.stages.spend?.code, 'PARITY_NOT_ADMITTED');
  assert.equal(unit.ledgerExecution, false);
  assert.equal(unit.ledgerSubmitted, false);
  assert.equal(unit.startedNode, false);
  assert.equal(Boolean(unit.stages.body.cborHex), false);
});

function writeStub(dir) {
  const logPath = join(dir, 'cli.log');
  const tipPath = join(dir, 'tip.txt');
  const stub = join(dir, 'cli.sh');
  const refs = IDS.slice(0, 3).map((item) => `${item.txId}#${item.index}`).join(' ');
  writeFileSync(stub, `#!/bin/sh
printf '%s\\n' "$*" >> "$MORIARTY_BODY_LOG"
if [ "$1" = query ] && [ "$2" = tip ]; then
  slot=$(cat "$MORIARTY_TIP_FILE")
  printf '%s\\n' "{\\"slot\\":$slot,\\"hash\\":\\"abc\\",\\"block\\":1}"
  exit 0
fi
if [ "$1" = query ] && [ "$2" = era-history ]; then
  printf '%s\\n' '{"type":"EraHistory","description":"unit envelope","cborHex":"9fff"}'
  exit 0
fi
if [ "$1" = address ]; then
  printf '%s\\n' 'addr_test1qzunit'
  exit 0
fi
if [ "$1" = conway ] && [ "$3" = build-raw ]; then
  out=""
  prev=""
  for arg in "$@"; do
    if [ "$prev" = --out-file ]; then out="$arg"; fi
    prev="$arg"
  done
  printf '%s\\n' '{"type":"Tx ConwayEra","description":"unit","cborHex":"84a300"}' > "$out"
  exit 0
fi
if [ "$1" = conway ] && [ "$3" = txid ]; then
  printf '%s\\n' 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
  exit 0
fi
if [ "$1" = debug ]; then
  printf '%s\\n' "$MORIARTY_REFS"
  exit 0
fi
exit 0
`);
  chmodSync(stub, 0o755);
  writeFileSync(join(dir, 'shelley.json'), `${JSON.stringify({
    slotLength: 1,
    epochLength: 3600,
    systemStart: '2026-01-01T00:00:00Z',
  })}\n`);
  const source = join(dir, 'sibling.mori');
  writeFileSync(source, 'unit sibling source\n');
  return { logPath, tipPath, stub, refs, source, genesis: join(dir, 'shelley.json') };
}

test('F2 observed clock slots reach the unsigned body argv', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-f2-'));
  const bin = writeStub(dir);
  const expected = [
    [0, '30', '90'],
    [11, '41', '101'],
    [100, '130', '190'],
  ];
  process.env.MORIARTY_BODY_LOG = bin.logPath;
  process.env.MORIARTY_TIP_FILE = bin.tipPath;
  process.env.MORIARTY_REFS = bin.refs;
  try {
    for (const [tip, before, after] of expected) {
      writeFileSync(bin.logPath, '');
      writeFileSync(bin.tipPath, `${tip}\n`);
      const ran = await runNativeCaller({
        financialBody: true,
        buildBody: true,
        unitOnly: true,
        cliPath: bin.stub,
        utxos: IDS,
        networkMagic: 764,
        outputLovelace: 2000000,
        feeLovelace: 200000,
        time: {
          cliPath: bin.stub,
          socket: bin.tipPath,
          execute: true,
          magic: 764,
          genesisPath: bin.genesis,
        },
        sibling: { compilerBin: bin.stub, sourcePath: bin.source, previousProgramHash: '11'.repeat(28) },
      });
      assert.equal(ran.stages.time.code, 'SLOT_OBSERVED', `tip ${tip} clock`);
      assert.equal(String(ran.stages.time.windows.releaseSlot), before);
      assert.equal(String(ran.stages.time.windows.partitionSlot), after);
      const log = readFileSync(bin.logPath, 'utf8');
      assert.match(log, new RegExp(`--invalid-before ${before}\\b`));
      assert.match(log, new RegExp(`--invalid-hereafter ${after}\\b`));
      assert.doesNotMatch(log, /--invalid-before 40\b/);
      assert.equal(ran.stages.sibling?.code, 'SIBLING_NOT_COMPILED_IN_THIS_RUN');
      assert.equal(ran.stages.sibling?.siblingEmitted, false);
      assert.equal(ran.ledgerExecution, false);
      assert.equal(ran.startedNode, false);
    }
  } finally {
    delete process.env.MORIARTY_BODY_LOG;
    delete process.env.MORIARTY_TIP_FILE;
    delete process.env.MORIARTY_REFS;
  }
});

test('F3 unrepresentable POSIX pairs do not become CLI slot flags', () => {
  const release = boundSpendInterval('release', '1000', '1099');
  assert.equal(release.ok, false);
  assert.equal(release.code, 'FIXTURE_WINDOW_UNREPRESENTABLE');
  assert.equal(release.detail, 'ROUNDING');
  assert.equal(release.encodeSpendBodyCalled, false);
  assert.equal(release.cliIntervalEmitted, false);
  assert.equal(release.invalidBefore, null);
  const refund = boundSpendInterval('refund', '1100', '1100');
  assert.equal(refund.code, 'FIXTURE_WINDOW_UNREPRESENTABLE');
  assert.equal(refund.detail, 'ROUNDING');
  assert.equal(refund.encodeSpendBodyCalled, false);
  assert.equal(refund.invalidBefore, null);
  const exclusiveRefund = boundSpendInterval('refund', '1000', '2000');
  assert.equal(exclusiveRefund.code, 'FIXTURE_WINDOW_UNREPRESENTABLE');
  assert.equal(exclusiveRefund.detail, 'REFUND_UPPER_EXCLUSIVE');
  assert.equal(exclusiveRefund.encodeSpendBodyCalled, false);
  const cancel = boundSpendInterval('cancel', '1050', '1051');
  assert.equal(cancel.code, 'FIXTURE_WINDOW_UNREPRESENTABLE');
  assert.equal(cancel.detail, 'ROUNDING');
  assert.equal(cancel.cliIntervalEmitted, false);
  const aligned = boundSpendInterval('release', '40000', '99999');
  assert.equal(aligned.ok, true);
  assert.equal(aligned.code, 'SLOT_POSIX_SEPARATED');
  assert.equal(aligned.invalidBefore, '40');
  assert.equal(aligned.invalidHereafter, '100');
  assert.equal(aligned.redeemerLower, '40000');
  assert.equal(aligned.redeemerUpper, '99999');
  assert.equal(aligned.ledgerExecution, false);
});

test('F4 schema and custody clocks share half-open slot fields', () => {
  const start = '2026-01-01T00:00:00Z';
  const samples = [
    [0, '30', '90'],
    [11, '41', '101'],
    [100, '130', '190'],
  ];
  for (const [tip, before, after] of samples) {
    const interval = consumedHeadInterval(schemaWindows({
      systemStart: start,
      tipSlot: tip,
      slotLengthMs: 1000,
      epochLengthSlots: 3600,
    }));
    assert.equal(interval.ok, true, `schema tip ${tip}`);
    assert.equal(interval.invalidBefore, before);
    assert.equal(interval.invalidHereafter, after);
    assert.notEqual(interval.code, 'TIME_WINDOW_ABSENT');
  }
  const custody = planWindows({ zeroTime: '0', zeroSlot: '0', slotLengthMs: '1000', tipSlot: '10' });
  const head = consumedHeadInterval(custody);
  assert.equal(head.ok, true);
  assert.equal(head.invalidBefore, '40');
  assert.equal(head.invalidHereafter, '100');
  assert.equal(head.invalidBefore, custody.releaseSlots.startSlot);
  assert.notEqual(head.invalidBefore, custody.releaseArgs.lower);
  assert.notEqual(head.invalidHereafter, custody.releaseArgs.upper);
});
