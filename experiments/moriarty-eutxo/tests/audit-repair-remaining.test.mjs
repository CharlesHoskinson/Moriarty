/**
 * F6, B4, and F1 controls.
 * Attempt 04 ran the first two tests red, before the repair.
 * Later tests call exported functions with injected unit spawns.
 * This file does not set MORIARTY_ALLOW_REAL_NODE and does not query a live node.
 * A function spawn cannot admit production, including when realNodeSpawn is forced true.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { chmodSync, mkdtempSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { bindFreshInitialFunds, buildBootstrapPlan } from '../src/native-bootstrap.mjs';
import { ownedGenesisArgv } from '../src/native-producer.mjs';
import { runOwnedRace } from '../src/native-caller.mjs';
import { PROBE_SOURCE, observeQueriedNode } from '../src/posix-parity-harness.mjs';

const SUPPLY = '45000000000000001';
const UNSAFE = Number('9007199254740993');

function vkey(body) {
  return `5820${body}`;
}

function writeFunds(dir) {
  const shelleyWork = join(dir, 'shelley.json');
  const byronWork = join(dir, 'byron.json');
  const template = join(dir, 'template.json');
  writeFileSync(shelleyWork, `${JSON.stringify({
    slotLength: 1,
    epochLength: 3600,
    networkMagic: 764,
    systemStart: '2026-01-01T00:00:00Z',
    minFeeA: 44,
    minFeeB: 155381,
    genDelegs: { fresh: '01' },
    maxLovelaceSupply: 9000000,
  })}\n`);
  writeFileSync(byronWork, `${JSON.stringify({ byron: true })}\n`);
  writeFileSync(template, `${JSON.stringify({ genDelegs: {} })}\n`);
  const bodies = [
    '0123456789abcdeffedcba9876543210'.repeat(2),
    '00112233445566778899aabbccddeeff'.repeat(2),
    'ffeeddccbbaa99887766554433221100'.repeat(2),
  ];
  const utxoVkeys = bodies.map((body, index) => {
    const path = join(dir, `k${index}.vkey`);
    writeFileSync(path, `${vkey(body)}\n`);
    return path;
  });
  const addrOuts = ['a', 'b', 'c'].map((mark, index) => {
    const path = join(dir, `a${index}.addr`);
    writeFileSync(path, `addr_test1${mark}\n`);
    return path;
  });
  return { shelleyWork, byronWork, template, utxoVkeys, addrOuts };
}

test('F6 digit supply is preserved and an unsafe number is refused', () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-f6-'));
  const paths = writeFunds(dir);
  const bound = bindFreshInitialFunds(paths, {
    unitOnly: true,
    magic: 764,
    supplyLovelace: SUPPLY,
    templateShelley: paths.template,
  });
  assert.equal(bound.ok, true, bound.code);
  assert.equal(bound.numberUsed, false);
  assert.equal(bound.ledgerExecution, false);
  const written = JSON.parse(readFileSync(paths.shelleyWork, 'utf8'));
  const sum = Object.values(written.initialFunds).map((item) => BigInt(item)).reduce((total, item) => total + item, 0n);
  assert.equal(sum.toString(), SUPPLY);
  const unsafeDir = mkdtempSync(join(tmpdir(), 'moriarty-f6-unsafe-'));
  const unsafePaths = writeFunds(unsafeDir);
  const refused = bindFreshInitialFunds(unsafePaths, {
    unitOnly: true,
    magic: 764,
    supplyLovelace: UNSAFE,
    templateShelley: unsafePaths.template,
  });
  assert.equal(refused.code, 'UNSAFE_SUPPLY');
  assert.equal(refused.ok, false);
  const home = mkdtempSync(join(tmpdir(), 'moriarty-f6-argv-'));
  const genesisDir = join(home, 'genesis');
  mkdirSync(genesisDir);
  const templates = {};
  for (const name of ['byron', 'shelley', 'alonzo', 'conway']) {
    templates[`${name}Template`] = join(home, `${name}.json`);
    writeFileSync(templates[`${name}Template`], '{}\n');
  }
  const exact = ownedGenesisArgv({
    cli: 'cardano-cli',
    ownedHome: home,
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    slotLength: 1000,
    genUtxoKeys: 3,
    supplyLovelace: SUPPLY,
    genesisDir,
    ...templates,
  });
  assert.equal(exact.ok, true, exact.code);
  assert.equal(exact.executed, false);
  assert.ok(exact.argv.includes(SUPPLY));
  const unsafeArgv = ownedGenesisArgv({
    cli: 'cardano-cli',
    ownedHome: home,
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    slotLength: 1000,
    genUtxoKeys: 3,
    supplyLovelace: UNSAFE,
    genesisDir,
    ...templates,
  });
  assert.equal(unsafeArgv.code, 'UNSAFE_SUPPLY');
  const plan = buildBootstrapPlan({
    supplyLovelace: UNSAFE,
    ownedHome: home,
    runId: 'red',
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    cli: 'cardano-cli',
  }, {
    pinnedTemplates: { byron: templates.byronTemplate, shelley: templates.shelleyTemplate, alonzo: templates.alonzoTemplate, conway: templates.conwayTemplate, nodeConfig: templates.byronTemplate, topology: templates.byronTemplate },
    ownedGenesisArgv,
    // Attempt 05 threw on `for (const extra of outputPlan.steps)` because this
    // stub was `{}`. An empty step list lets the supply blocker stay visible.
    // UNSAFE_SUPPLY is still required. The empty object was not an economic assertion.
    planOutputMinimums: () => ({ steps: [] }),
    plutusCostArgv: () => ({ ok: false, code: 'SKIP' }),
  });
  assert.equal(plan.executed, false);
  assert.equal(plan.ledgerExecution, false);
  assert.ok(plan.blockers.includes('UNSAFE_SUPPLY'), JSON.stringify(plan.blockers));
  const safe = ownedGenesisArgv({
    cli: 'cardano-cli',
    ownedHome: home,
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    slotLength: 1000,
    genUtxoKeys: 3,
    supplyLovelace: 9000000,
    genesisDir,
    ...templates,
  });
  assert.equal(safe.ok, true, safe.code);
  assert.ok(safe.argv.includes('9000000'));
});

function writeExec(path, lines) {
  writeFileSync(path, `${lines.join('\n')}\n`);
  chmodSync(path, 0o755);
  return path;
}

test('B4 unit race does not treat query tip or submission order as the winner', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-b4-'));
  const logPath = join(dir, 'race.log');
  const appender = (name) => writeExec(join(dir, name), [
    '#!/bin/sh',
    'if [ -n "$MORIARTY_RACE_LOG" ]; then printf \'%s\\n\' "$*" >> "$MORIARTY_RACE_LOG"; fi',
    'exit 0',
  ]);
  const ids = ['11', '22', '33', '44', '55'].map((byte) => ({ txId: byte.repeat(32), index: 0 }));
  const leftBody = join(dir, 'release.body.json');
  const rightBody = join(dir, 'cancel.body.json');
  writeFileSync(leftBody, '{"type":"Tx ConwayEra","cborHex":"aa"}\n');
  writeFileSync(rightBody, '{"type":"Tx ConwayEra","cborHex":"bb"}\n');
  const signingKey = join(dir, 'payment.skey');
  writeFileSync(signingKey, 'unit-key-not-a-wallet\n', { mode: 0o600 });
  chmodSync(signingKey, 0o600);
  const raced = await runOwnedRace({
    signBin: appender('sign.sh'),
    submitBin: appender('submit.sh'),
    pollBin: appender('poll.sh'),
    logPath,
    head: ids[0],
    left: { fee: ids[1], collateral: ids[2], actor: 'release', resource: 'release-fee' },
    right: { fee: ids[3], collateral: ids[4], actor: 'cancel', resource: 'cancel-fee' },
    leftBodyFile: leftBody,
    rightBodyFile: rightBody,
    signingKeyPath: signingKey,
    networkMagic: 764,
  });
  const lines = readFileSync(logPath, 'utf8').trim().split('\n');
  assert.equal(lines.filter((line) => line.startsWith('query tip')).length, 0);
  assert.equal(raced.code, 'RACE_UNIT_OBSERVATION');
  assert.equal(raced.replay.executed, false);
  assert.equal(raced.ledgerExecution, false);
  assert.equal(raced.ledgerSubmitted, false);
  assert.equal(raced.submitted, false);
  assert.equal(raced.assumedSecondIsLoser, false);
  assert.equal(raced.poll, 'query-utxo-tx-in');
  const submits = lines.map((line, index) => (line.includes('transaction submit') ? index : -1)).filter((index) => index >= 0);
  const polls = lines.map((line, index) => (line.includes('query utxo') ? index : -1)).filter((index) => index >= 0);
  assert.equal(submits.length, 2);
  if (polls.length) assert.ok(submits[1] < polls[0]);
});

test('B4 the queried output names the winner and the other body is the loser', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-b4-winner-'));
  const logPath = join(dir, 'race.log');
  const leftTx = '11'.repeat(32);
  const rightTx = '22'.repeat(32);
  const script = writeExec(join(dir, 'race.sh'), [
    '#!/bin/sh',
    'if [ -n "$MORIARTY_RACE_LOG" ]; then printf \'%s\\n\' "$*" >> "$MORIARTY_RACE_LOG"; fi',
    'txfile=""',
    'txin=""',
    'prev=""',
    'for arg in "$@"; do',
    '  if [ "$prev" = "--tx-file" ]; then txfile=$arg; fi',
    '  if [ "$prev" = "--tx-in" ]; then txin=$arg; fi',
    '  prev=$arg',
    'done',
    `left=${leftTx}`,
    `right=${rightTx}`,
    'case "$*" in',
    '  *"transaction txid"*)',
    '    case "$txfile" in',
    '      *release*) printf \'%s\\n\' "$left" ;;',
    '      *) printf \'%s\\n\' "$right" ;;',
    '    esac',
    '    ;;',
    '  *"query utxo"*)',
    '    if [ "$txin" = "$right#0" ]; then printf \'%s\\n\' "$txin"; fi',
    '    ;;',
    '  *"transaction submit"*)',
    '    case "$txfile" in',
    '      *release*)',
    '        printf \'%s\\n\' BadInputsUTxO >&2',
    '        ;;',
    '    esac',
    '    ;;',
    'esac',
    'exit 0',
  ]);
  const ids = ['aa', 'bb', 'cc', 'dd', 'ee'].map((byte) => ({ txId: byte.repeat(32), index: 0 }));
  const leftBody = join(dir, 'release.body.json');
  const rightBody = join(dir, 'cancel.body.json');
  writeFileSync(leftBody, '{"type":"Tx ConwayEra","cborHex":"aa"}\n');
  writeFileSync(rightBody, '{"type":"Tx ConwayEra","cborHex":"bb"}\n');
  const signingKey = join(dir, 'payment.skey');
  writeFileSync(signingKey, 'unit-key-not-a-wallet\n', { mode: 0o600 });
  chmodSync(signingKey, 0o600);
  const raced = await runOwnedRace({
    signBin: script,
    submitBin: script,
    pollBin: script,
    logPath,
    head: ids[0],
    left: { fee: ids[1], collateral: ids[2], actor: 'release', resource: 'release-fee' },
    right: { fee: ids[3], collateral: ids[4], actor: 'cancel', resource: 'cancel-fee' },
    leftBodyFile: leftBody,
    rightBodyFile: rightBody,
    signingKeyPath: signingKey,
    networkMagic: 764,
  });
  const lines = readFileSync(logPath, 'utf8').trim().split('\n');
  const submitLines = lines.filter((line) => line.includes('transaction submit'));
  assert.equal(lines.filter((line) => line.startsWith('query tip')).length, 0);
  assert.equal(raced.code, 'RACE_UNIT_OBSERVATION');
  assert.equal(raced.productionAdmitted, undefined);
  assert.equal(raced.replay.executed, false);
  assert.equal(raced.replay.observed, true);
  assert.equal(raced.ledgerExecution, false);
  assert.equal(raced.submitted, false);
  assert.equal(raced.assumedSecondIsLoser, false);
  assert.equal(raced.poll, 'query-utxo-tx-in');
  assert.equal(raced.winnerTxid, rightTx);
  assert.equal(raced.winnerActor, 'cancel');
  assert.equal(raced.winnerOutRef, `${rightTx}#0`);
  assert.equal(raced.resubmitted, false);
  assert.equal(raced.replay.resubmitted, false);
  assert.equal(raced.failureClass, 'BadInputsUTxO');
  assert.equal(submitLines.length, 2);
  assert.equal(submitLines.filter((line) => line.includes('assembled-release')).length, 1);
  assert.equal(submitLines.filter((line) => line.includes('assembled-cancel')).length, 1);
  const polls = lines.map((line, index) => (line.includes('query utxo') ? index : -1)).filter((index) => index >= 0);
  const firstTwo = submitLines.slice(0, 2).map((line) => lines.indexOf(line));
  assert.ok(polls.length >= 1);
  assert.ok(firstTwo[1] < polls[0]);
});

const FIXTURE_SPEND = 'e25450233e4bedd00c8bda15c48c2d4018223bd88271e194052294c4e5be7d55';
const FIXTURE_COLLATERAL = '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef';

function nodePaths(dir) {
  const genesisPath = join(dir, 'shelley.json');
  writeFileSync(genesisPath, `${JSON.stringify({
    slotLength: 1,
    systemStart: '2026-01-01T00:00:00Z',
    networkMagic: 764,
    zeroSlot: 0,
  })}\n`);
  return {
    genesisPath,
    bodyPath: join(dir, 'probe.body'),
    artifactPath: join(dir, 'probe.ak'),
    readbackPath: join(dir, 'readback.json'),
    transcriptPath: join(dir, 'transcript.json'),
    cliPath: join(dir, 'cardano-cli'),
    socketPath: join(dir, 'node.sock'),
  };
}

test('F1 an incomplete queried utxo is refused without the offline fixture ids', () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-f1-incomplete-'));
  const paths = nodePaths(dir);
  const only = `${'ab'.repeat(32)}#0`;
  const result = observeQueriedNode({
    ...paths,
    magic: 764,
    compile() { throw new Error('compile is not reached for an incomplete utxo'); },
  }, () => ({
    status: 0,
    stdout: JSON.stringify({ [only]: { address: 'addr_test1only', value: { lovelace: 1 } } }),
  }), { realNodeSpawn: true });
  assert.equal(result.code, 'QUERY_UTXO_INCOMPLETE');
  assert.equal(result.productionAdmitted, false);
  assert.equal(result.evidenceClass, 'unit-command-sequence-not-a-node');
  assert.equal(result.ledgerExecution, false);
  assert.equal(result.nativeUnexecuted, true);
  assert.equal(result.syntheticOfflineUtxo, false);
  const packed = JSON.stringify(result);
  assert.equal(packed.includes(FIXTURE_SPEND), false);
  assert.equal(packed.includes(FIXTURE_COLLATERAL), false);
  assert.equal(readFileSync(paths.artifactPath, 'utf8'), PROBE_SOURCE);
  assert.equal(readFileSync(paths.artifactPath, 'utf8').includes('ms == ms'), false);
});

test('F1 a unit command sequence parses the hashed artifact and does not admit production', () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-f1-unit-'));
  const paths = nodePaths(dir);
  const spend = `${'ab'.repeat(32)}#0`;
  const collateral = `${'cd'.repeat(32)}#0`;
  const hash = 'ef'.repeat(28);
  const lower = String(Date.parse('2026-01-01T00:00:00Z') + 10 * 1000);
  const utxo = {
    [spend]: { address: 'addr_test1spend', value: { lovelace: 5000000 } },
    [collateral]: { address: 'addr_test1coll', value: { lovelace: 5000000 } },
  };
  const protocol = {
    utxoCostPerByte: 4310,
    maxTxExecutionUnits: { memory: 14000000, steps: 10000000000 },
    maxTxSize: 16384,
  };
  const era = { type: 'EraHistory', description: 'unit era', cborHex: '9fff' };
  const calls = [];
  const result = observeQueriedNode({
    ...paths,
    magic: 764,
    compile({ sourcePath, sourceSha256 }) {
      const text = readFileSync(sourcePath, 'utf8');
      assert.equal(text, PROBE_SOURCE);
      assert.equal(createHash('sha256').update(text).digest('hex'), sourceSha256);
      return { ok: true, hash, compiledCode: '590100', sourceSha256 };
    },
  }, (argv) => {
    calls.push(argv);
    const text = argv.join(' ');
    if (text.includes('query utxo')) return { status: 0, stdout: JSON.stringify(utxo) };
    if (text.includes('protocol-parameters')) {
      writeFileSync(argv[argv.indexOf('--out-file') + 1], JSON.stringify(protocol));
      return { status: 0, stdout: '' };
    }
    if (text.includes('query tip') || argv.includes('tip')) {
      return { status: 0, stdout: JSON.stringify({ slot: 10, hash: 'ab', block: 1 }) };
    }
    if (text.includes('era-history')) {
      writeFileSync(argv[argv.indexOf('--out-file') + 1], JSON.stringify(era));
      return { status: 0, stdout: '' };
    }
    if (text.includes('address') && text.includes('build')) return { status: 0, stdout: 'addr_test1unitprobe\n' };
    if (text.includes('build-raw')) {
      writeFileSync(argv[argv.indexOf('--out-file') + 1], '{"type":"Unwitnessed Tx ConwayEra"}\n');
      return { status: 0, stdout: '' };
    }
    if (text.includes('calculate-plutus-script-cost')) {
      return {
        status: 1,
        stdout: '',
        stderr: [
          'transaction calculate-plutus-script-cost',
          `MORIARTY_LOWER:${lower}`,
          `Valid range: [ POSIXTime ${lower}`,
          `with hash: Just "${hash}"`,
        ].join('\n'),
      };
    }
    return { status: 0, stdout: '' };
  }, { realNodeSpawn: true });
  assert.equal(result.code, 'UNIT_NODE_SEQUENCE_PARSED', result.code);
  assert.equal(result.ok, false);
  assert.equal(result.productionAdmitted, false);
  assert.equal(result.evidenceClass, 'unit-command-sequence-not-a-node');
  assert.equal(result.ledgerExecution, false);
  assert.equal(result.nativeUnexecuted, true);
  assert.equal(result.networkCommandsRun, false);
  assert.equal(result.syntheticOfflineUtxo, false);
  assert.equal(result.tipSource, 'query-tip');
  assert.equal(result.scriptContextLower, lower);
  assert.equal(result.cliPosix, lower);
  assert.equal(result.lucidPosix, lower);
  assert.equal(result.probeExecuted, true);
  const sha = (path) => createHash('sha256').update(readFileSync(path)).digest('hex');
  assert.equal(result.bindings.artifact.sha256, sha(paths.artifactPath));
  assert.equal(result.bindings.genesis.sha256, sha(paths.genesisPath));
  assert.equal(result.bindings.body.sha256, sha(paths.bodyPath));
  assert.equal(result.bindings.utxo.syntheticOfflineUtxo, false);
  assert.equal(result.command.includes('online'), true);
  const joined = calls.map((argv) => argv.join(' ')).join('\n');
  assert.equal(joined.includes(FIXTURE_SPEND), false);
  assert.equal(joined.includes(FIXTURE_COLLATERAL), false);
  assert.equal(joined.includes('calculate-plutus-script-cost'), true);
  assert.equal(joined.includes(' offline'), false);
  assert.ok(joined.includes(spend));
  assert.ok(joined.includes(collateral));
  const transcript = JSON.parse(readFileSync(paths.transcriptPath, 'utf8'));
  assert.equal(transcript.productionAdmitted, false);
  assert.equal(transcript.evidenceClass, 'unit-command-sequence-not-a-node');
  assert.equal(transcript.tipSource, 'query-tip');
});
