import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawn, spawnSync } from 'node:child_process';
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { homedir, tmpdir } from 'node:os';
import { join } from 'node:path';
import net from 'node:net';
import test from 'node:test';
import { scriptHashV3 } from '../src/apply-head.mjs';
import {
  collectHeadReadback,
  compileGuardSibling,
  decideConfirmation,
  measureSlotLength,
  prepareEnterpriseKey,
  runNativeCaller,
  runOwnedRace,
} from '../src/native-caller.mjs';
import {
  CARDANO_CLI,
  CARDANO_NODE,
  assessCashPolicy,
  certifyRuntimePins,
  launchOwnedRuntime,
  loadAuthority,
  loadCashDesignAuthority,
  parseCliUtxoJson,
  queryOwnedUtxos,
  resolveOutputPath,
  stopOwnedGroup,
} from '../src/native-runtime.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const ROOT = '/home/charl/Moriarty-wt-eutxo-validation-20261012/experiments/moriarty-eutxo';
const SOURCE = join(TASK, 'designers/02-escrow-time/escrow-custody-v5.mori');
const BLUEPRINT = join(TASK, 't1-native-repair-46/checks/custody-compile/plutus.applied.json');
const CHECKS = join(TASK, 't1-consumer-repair-119/checks/native-caller');
const CASH = 'validator cash_policy {\n  mint(redeemer, policy, transaction) { void }\n}\n';
const FAIL = 'validator cash {\n  mint(redeemer, policy, transaction) { fail }\n}\n';

function writeExec(path, lines) {
  writeFileSync(path, `${lines.join('\n')}\n`, { mode: 0o755 });
  chmodSync(path, 0o755);
  return path;
}

function writeAuthority(dir, evidenceClass, policyText) {
  const policySourceSha256 = createHash('sha256').update(policyText).digest('hex');
  const reviewA = join(dir, `review-a-${evidenceClass}.json`);
  const reviewB = join(dir, `review-b-${evidenceClass}.json`);
  if (evidenceClass === 'reviewed-implementation') {
    const sourcePath = join(dir, 'reviewed-source.mori');
    writeFileSync(sourcePath, policyText);
    const manifest = {
      sourceHash: policySourceSha256,
      reviewClass: 'native-implementation',
      files: [{ path: sourcePath, sha256: policySourceSha256, bytes: Buffer.byteLength(policyText) }],
    };
    const manifestPath = join(dir, 'manifest.json');
    writeFileSync(manifestPath, `${JSON.stringify(manifest)}\n`);
    const manifestHash = createHash('sha256').update(readFileSync(manifestPath)).digest('hex');
    const reviewAuthority = join(dir, 'review-authority.json');
    writeFileSync(reviewAuthority, JSON.stringify({ manifestSha256: manifestHash, sourceHash: policySourceSha256 }));
    const base = {
      verdict: 'APPROVE_SCOPED_T1_NATIVE_IMPLEMENTATION',
      candidate: manifestHash,
      evidenceClass,
      reviewClass: 'native-implementation',
      sourceHash: policySourceSha256,
      policySourceSha256,
    };
    writeFileSync(reviewA, JSON.stringify({ ...base, reviewer: 'fixture-reviewer-a' }));
    writeFileSync(reviewB, JSON.stringify({ ...base, reviewer: 'fixture-reviewer-b' }));
    return { reviewA, reviewB, candidate: manifestPath, manifest: manifestPath, reviewAuthority };
  }
  const candidate = join(dir, `candidate-${evidenceClass}.bin`);
  const body = `candidate ${evidenceClass}\n`;
  writeFileSync(candidate, body);
  const digest = createHash('sha256').update(body).digest('hex');
  const review = {
    verdict: 'APPROVE_SCOPED_T1_NATIVE_CALLER',
    candidate: digest,
    evidenceClass,
    policySourceSha256,
  };
  writeFileSync(reviewA, JSON.stringify(review));
  writeFileSync(reviewB, JSON.stringify(review));
  return { reviewA, reviewB, candidate };
}

function stubNode(dir) {
  return writeExec(join(dir, 'stub-node.sh'), [
    '#!/bin/sh',
    'if [ "$1" = "--version" ]; then',
    '  echo stub-node',
    '  exit 0',
    'fi',
    'tmp="$HOME/proof.txt.tmp"',
    'printf \'pid=%s\\nhome=%s\\nsocket=%s\\nargs=%s\\n\' "$$" "$HOME" "$CARDANO_NODE_SOCKET_PATH" "$*" > "$tmp"',
    'mv "$tmp" "$HOME/proof.txt"',
    'sleep 30',
  ]);
}

function keyStub(dir) {
  return writeExec(join(dir, 'key.sh'), [
    '#!/bin/sh',
    'key=""',
    'out=""',
    'while [ $# -gt 0 ]; do',
    '  if [ "$1" = "--signing-key-file" ]; then key="$2"; shift 2;',
    '  elif [ "$1" = "--out-file" ]; then out="$2"; shift 2;',
    '  else shift; fi',
    'done',
    'if [ -n "$key" ]; then printf \'unit-fixture-not-a-wallet\\n\' > "$key"; fi',
    'if [ -n "$out" ]; then printf \'addr_test1unitenterprise\\n\' > "$out"; fi',
    'exit 0',
  ]);
}

function appender(dir, name) {
  return writeExec(join(dir, name), [
    '#!/bin/sh',
    'if [ -n "$MORIARTY_RACE_LOG" ]; then',
    '  printf \'%s\\n\' "$*" >> "$MORIARTY_RACE_LOG"',
    '  exit 0',
    'fi',
    'log="$1"',
    'shift',
    'printf \'%s\\n\' "$*" >> "$log"',
    'exit 0',
  ]);
}

function pgidOf(pid) {
  const stat = readFileSync(`/proc/${pid}/stat`, 'utf8');
  return Number(stat.slice(stat.lastIndexOf(')') + 2).split(' ')[2]);
}

function ownPgid() {
  return pgidOf(process.pid);
}

async function waitGone(pid) {
  const deadline = Date.now() + 1500;
  while (Date.now() < deadline) {
    if (!existsSync(`/proc/${pid}`)) return true;
    await new Promise((resolve) => setTimeout(resolve, 20));
  }
  return !existsSync(`/proc/${pid}`);
}

function killRecorded(proof) {
  if (!existsSync(proof)) return;
  const text = readFileSync(proof, 'utf8');
  const found = text.match(/^pid=(\d+)$/m);
  if (!found) return;
  const pid = Number(found[1]);
  try {
    process.kill(-pgidOf(pid), 'SIGKILL');
  } catch {
    try { process.kill(pid, 'SIGKILL'); } catch { /* already gone */ }
  }
}

function listen(port) {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.once('error', reject);
    server.listen({ port, host: '127.0.0.1', exclusive: true }, () => resolve(server));
  });
}

test('authority, cash votes, and historical paths fail closed', () => {
  assert.equal(loadAuthority({ approved: true }).code, 'EXECUTION_AUTHORITY_ABSENT');
  assert.equal(assessCashPolicy('short', null).code, 'CASH_SOURCE_ABSENT');
  assert.equal(assessCashPolicy(FAIL, { approved: true }).code, 'CASH_POLICY_FAIL_CLOSED');
  const voted = assessCashPolicy(CASH, { approved: true });
  assert.equal(voted.code, 'CASH_POLICY_VOTES_ABSENT');
  assert.equal(voted.booleanIgnored, true);
  assert.equal(voted.cashCanMint, false);
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-auth-'));
  try {
    const unit = writeAuthority(dir, 'unit-fixture', CASH);
    const accepted = assessCashPolicy(CASH, unit);
    assert.equal(accepted.code, 'CASH_POLICY_UNIT_ACCEPTED');
    assert.equal(accepted.cashCanMint, false);
    assert.equal(accepted.ledgerFunding, false);
    assert.equal(accepted.nativePins, false);
    assert.equal(accepted.financialValidatorProof, false);
    assert.equal(assessCashPolicy(FAIL, unit).code, 'CASH_POLICY_FAIL_CLOSED');
    assert.equal(assessCashPolicy(`${CASH}extra`, unit).code, 'CASH_POLICY_SOURCE_MISMATCH');
    const reviewed = writeAuthority(dir, 'reviewed-implementation', CASH);
    const seam = assessCashPolicy(CASH, reviewed);
    assert.equal(seam.code, 'CASH_REVIEW_CLASS_MISMATCH');
    assert.equal(seam.cashCanMint, false);
    assert.equal(seam.ledgerFunding, false);
    assert.equal(seam.financialValidatorProof, false);
    assert.equal(seam.nativePins, false);
    const diagCandidate = join(dir, 'diagnostic-candidate.bin');
    writeFileSync(diagCandidate, 'not-the-manifest\n');
    const diagHash = createHash('sha256').update(readFileSync(diagCandidate)).digest('hex');
    const diag = {
      verdict: 'APPROVE_DIAGNOSTIC_FIXTURE_NOT_A_REVIEW',
      candidate: diagHash,
      evidenceClass: 'reviewed-implementation',
    };
    const diagA = join(dir, 'diag-a.json');
    const diagB = join(dir, 'diag-b.json');
    writeFileSync(diagA, JSON.stringify(diag));
    writeFileSync(diagB, JSON.stringify(diag));
    const diagGate = loadAuthority({ reviewA: diagA, reviewB: diagB, candidate: diagCandidate });
    assert.notEqual(diagGate.code, 'AUTHORITY_AGREED');
    assert.equal(diagGate.requiredManifestDigest, null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
  const historical = resolveOutputPath(join(TASK, 't1-native-repair-46/checks/cli-custody'));
  assert.equal(historical.code, 'OUTPUT_PATH_FORBIDDEN');
  assert.equal(historical.historical, 't1-native-repair-46');
});

test('owned stub spawns in its own group and cleanup leaves a bystander', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-launch-'));
  const byDir = mkdtempSync(join(tmpdir(), 'moriarty-bystander-'));
  const nodeBin = stubNode(dir);
  const byBin = stubNode(byDir);
  const authority = writeAuthority(dir, 'unit-fixture', CASH);
  let byPid = null;
  const byChild = spawn('setsid', [byBin], {
    detached: true,
    stdio: 'ignore',
    env: { PATH: process.env.PATH || '', HOME: byDir, CARDANO_NODE_SOCKET_PATH: join(byDir, 'b.socket') },
  });
  byChild.unref();
  const byProof = join(byDir, 'proof.txt');
  const deadline = Date.now() + 2000;
  while (!existsSync(byProof) && Date.now() < deadline) {
    await new Promise((resolve) => setTimeout(resolve, 20));
  }
  try {
    byPid = Number(readFileSync(byProof, 'utf8').match(/^pid=(\d+)$/m)[1]);
    const launched = await launchOwnedRuntime({ nodeBin, runDir: dir, authority });
    assert.equal(launched.code, 'OWNED_NODE_SPAWNED', launched.code);
    assert.equal(launched.started, true);
    assert.equal(launched.spawnAttempted, true);
    assert.notEqual(launched.pgid, ownPgid());
    assert.notEqual(launched.pgid, pgidOf(byPid));
    const proof = readFileSync(join(dir, 'proof.txt'), 'utf8');
    assert.match(proof, new RegExp(`home=${dir.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}`));
    assert.equal(proof.includes(homedir()), false);
    assert.match(proof, /--port /);
    assert.match(proof, new RegExp(`--socket-path ${dir.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}/node.socket`));
    const config = JSON.parse(readFileSync(launched.configPath, 'utf8'));
    assert.equal(config.Protocol, 'Cardano');
    assert.equal(config.home, undefined);
    assert.equal(config.ShelleyGenesisFile, 'genesis/shelley-genesis.json');
    assert.equal((statSync(launched.configPath).mode & 0o777), 0o600);
    assert.match(proof, /\brun\b/);
    assert.match(proof, /--topology /);
    assert.match(proof, /--database-path /);
    const genesis = JSON.parse(readFileSync(launched.shelleyGenesisPath, 'utf8'));
    assert.equal(genesis.slotLength, 1);
    assert.equal(genesis.epochLength, 3600);
    const stopped = stopOwnedGroup(launched.pgid);
    assert.equal(stopped.code, 'OWNED_GROUP_STOPPED');
    assert.equal(await waitGone(launched.pid), true);
    assert.equal(existsSync(`/proc/${byPid}`), true);
    assert.equal(existsSync(`/proc/${process.pid}`), true);
    assert.equal(stopOwnedGroup(ownPgid()).code, 'CLEANUP_REFUSED');
    assert.equal(stopOwnedGroup(1).code, 'CLEANUP_REFUSED');
    writeFileSync(join(dir, 'proof.txt'), 'pid=1\nhome=/stale\n');
    const again = await launchOwnedRuntime({ nodeBin, runDir: dir, authority });
    assert.equal(again.code, 'OWNED_NODE_SPAWNED');
    assert.notEqual(again.pid, 1);
    assert.notEqual(again.pgid, 1);
    const second = readFileSync(join(dir, 'proof.txt'), 'utf8');
    assert.equal(second.includes('home=/stale'), false);
    assert.equal(stopOwnedGroup(again.pgid).code, 'OWNED_GROUP_STOPPED');
    assert.equal(await waitGone(again.pid), true);
  } finally {
    killRecorded(join(dir, 'proof.txt'));
    killRecorded(byProof);
    if (byPid) await waitGone(byPid);
    rmSync(dir, { recursive: true, force: true });
    rmSync(byDir, { recursive: true, force: true });
  }
});

test('missing authority, offline inspect, real node, and bad paths do not exec', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-refuse-'));
  const nodeBin = stubNode(dir);
  const unit = writeAuthority(dir, 'unit-fixture', CASH);
  const reviewed = writeAuthority(dir, 'reviewed-implementation', CASH);
  const before = spawnSync('ps', ['-C', 'cardano-node', '-o', 'pid='], { encoding: 'utf8' }).stdout;
  try {
    const missing = await launchOwnedRuntime({ nodeBin, runDir: dir, authority: { approved: true } });
    assert.equal(missing.code, 'EXECUTION_AUTHORITY_ABSENT');
    assert.equal(missing.spawnAttempted, false);
    assert.equal(existsSync(join(dir, 'proof.txt')), false);
    const offline = await runNativeCaller({
      authority: unit,
      launch: { startNode: false, nodeBin, runDir: dir },
    });
    assert.equal(offline.startedNode, false);
    assert.equal(offline.stages.launcher.code, 'LAUNCHER_CHECKED');
    assert.equal(offline.stages.launcher.started, false);
    assert.equal(existsSync(join(dir, 'proof.txt')), false);
    const home = await launchOwnedRuntime({ nodeBin, runDir: homedir(), authority: unit });
    assert.equal(home.code, 'OWNED_HOME_REQUIRED');
    assert.equal(home.spawnAttempted, false);
    const socket = await launchOwnedRuntime({
      nodeBin, runDir: dir, authority: unit, socketPath: '/tmp/not-owned.socket',
    });
    assert.equal(socket.code, 'SOCKET_NOT_OWNED');
    const wrapped = await launchOwnedRuntime({
      nodeBin, runDir: dir, authority: unit, args: ['pkill', '-f', 'cardano-node'],
    });
    assert.equal(wrapped.code, 'YACI_WRAPPER_REFUSED');
    assert.equal(existsSync(join(dir, 'proof.txt')), false);
    const unitReal = await launchOwnedRuntime({ nodeBin: CARDANO_NODE, runDir: dir, authority: unit });
    assert.equal(unitReal.code, 'UNIT_FIXTURE_CANNOT_START_REAL_NODE');
    assert.equal(unitReal.spawnAttempted, false);
    const reviewedReal = await launchOwnedRuntime({ nodeBin: CARDANO_NODE, runDir: dir, authority: reviewed });
    assert.equal(reviewedReal.code, 'REAL_NODE_NOT_ARMED');
    assert.equal(reviewedReal.spawnAttempted, false);
    const after = spawnSync('ps', ['-C', 'cardano-node', '-o', 'pid='], { encoding: 'utf8' }).stdout;
    assert.equal(after, before);
    const server = await listen(0);
    const busyPort = server.address().port;
    const busy = await launchOwnedRuntime({ nodeBin, runDir: dir, authority: unit, port: busyPort });
    server.close();
    assert.equal(busy.code, 'PORT_BUSY');
    assert.equal(busy.spawnAttempted, false);
    const reviewedStub = await launchOwnedRuntime({ nodeBin, runDir: join(dir, 'reviewed-home'), authority: reviewed });
    assert.equal(reviewedStub.code, 'OWNED_NODE_SPAWNED');
    assert.equal(reviewedStub.nativePins, false);
    assert.equal(reviewedStub.ledgerExecution, false);
    assert.equal(reviewedStub.unitOnly, true);
    const stopped = stopOwnedGroup(reviewedStub.pgid);
    assert.equal(stopped.code, 'OWNED_GROUP_STOPPED');
    assert.equal(await waitGone(reviewedStub.pid), true);
  } finally {
    killRecorded(join(dir, 'proof.txt'));
    killRecorded(join(dir, 'reviewed-home/proof.txt'));
    rmSync(dir, { recursive: true, force: true });
  }
});

test('time, protocol, sibling, readback, and race stubs stay unit-only', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-pipe-'));
  try {
    const socket = join(dir, 'node.socket');
    writeFileSync(socket, '');
    const timeOf = (name, text) => writeExec(join(dir, name), ['#!/bin/sh', `cat <<'EOF'`, text, 'EOF']);
    const absent = measureSlotLength({ cliPath: timeOf('absent.sh', 'no-measurement'), socket, magic: 764 });
    assert.equal(absent.code, 'PARITY_NOT_MEASURED');
    assert.equal(absent.spawned, true);
    const quoted = measureSlotLength({ cliPath: timeOf('quoted.sh', 'slotLengthMs="1000"'), socket, magic: 764 });
    assert.equal(quoted.code, 'SLOT_LENGTH_QUOTED');
    const refused = measureSlotLength({
      cliPath: timeOf('short.sh', 'slotLengthMs=500\nlucidPosix=500\ncliPosix=500\nscriptContextLower=500\ntipSlot=10'),
      socket,
      magic: 764,
    });
    assert.equal(refused.code, 'SLOT_LENGTH_REFUSED');
    assert.equal(refused.observed, '500');
    const mismatch = measureSlotLength({
      cliPath: timeOf('mismatch.sh', 'slotLengthMs=1000\nlucidPosix=1\ncliPosix=2\nscriptContextLower=1\ntipSlot=10'),
      socket,
      magic: 764,
    });
    assert.equal(mismatch.code, 'IMPOSSIBLE_BOUNDARY');
    const held = measureSlotLength({ cliPath: timeOf('held.sh', 'slotLengthMs=1000'), socket, execute: false });
    assert.equal(held.spawned, false);
    const goodCli = timeOf('good.sh', [
      'slotLengthMs=1000',
      'lucidPosix=40000',
      'cliPosix=40000',
      'scriptContextLower=40000',
      'tipSlot=10',
      'evidenceClass=unit-fixture',
    ].join('\n'));
    const good = measureSlotLength({ cliPath: goodCli, socket, magic: 764 });
    assert.equal(good.code, 'SLOT_OBSERVED');
    assert.equal(good.observed, '1000');
    assert.equal(good.unitOnly, true);
    assert.equal(good.windows.ledgerExecution, false);
    assert.equal(good.windows.slotLengthMs, '1000');
    const siblingSource = join(dir, 'guard.mori');
    writeFileSync(siblingSource, `${CASH}guard-only sibling fixture\n`);
    const marker = join(dir, 'compiler-ran');
    const compiler = writeExec(join(dir, 'compiler.sh'), [
      '#!/bin/sh',
      `touch ${marker}`,
      'echo programHash=' + 'cd'.repeat(32),
      'echo evidenceClass=unit-fixture',
    ]);
    const sibling = compileGuardSibling({
      compilerBin: compiler,
      sourcePath: siblingSource,
      previousProgramHash: 'ab'.repeat(32),
    });
    assert.equal(sibling.code, 'HASH_LINE_NOT_A_COMPILE');
    assert.equal(sibling.siblingEmitted, false);
    assert.equal(sibling.unitOnly, true);
    assert.equal(sibling.programHashIgnored, true);
    assert.equal(sibling.compilerIdentity, 'path-is-not-aiken');
    assert.equal(existsSync(marker), true);
    rmSync(marker);
    const blocked = await runNativeCaller({
      authority: writeAuthority(dir, 'unit-fixture', CASH),
      time: { cliPath: timeOf('bad-time.sh', 'slotLengthMs=500'), socket, execute: true, magic: 764 },
      sibling: { compilerBin: compiler, sourcePath: siblingSource, previousProgramHash: 'ab'.repeat(32) },
    });
    assert.equal(blocked.stages.time.code, 'SLOT_LENGTH_REFUSED');
    assert.equal(blocked.stages.sibling.code, 'SIBLING_NOT_COMPILED_IN_THIS_RUN');
    assert.equal(blocked.stages.sibling.spawned, false);
    assert.equal(existsSync(marker), false);
    const protocol = writeExec(join(dir, 'protocol.sh'), [
      '#!/bin/sh',
      'out=""',
      'while [ $# -gt 0 ]; do',
      '  if [ "$1" = "--out-file" ]; then out="$2"; shift 2; else shift; fi',
      'done',
      `cp ${join(dir, 'params.json')} "$out"`,
    ]);
    writeFileSync(join(dir, 'params.json'), JSON.stringify({
      evidenceClass: 'unit-fixture',
      maxTxExecutionUnits: { memory: '16000000', steps: '10000000000' },
      maxTxSize: 16384,
    }));
    const { queryProtocolParameters, compareBudget } = await import('../src/native-runtime.mjs');
    const queried = queryProtocolParameters({ socket, cliPath: protocol, execute: true, magic: 764 });
    assert.equal(queried.code, 'PROTOCOL_PARAMS_QUERIED');
    assert.equal(queried.spawned, true);
    assert.equal(queried.unitOnly, true);
    assert.equal(queried.nativeBudgetMeasured, false);
    assert.equal(queried.source, 'unit-cli-spawn');
    const within = compareBudget({
      queried: true,
      source: queried.source,
      unitOnly: true,
      maxTxExecutionUnits: queried.maxTxExecutionUnits,
      maxTxSize: queried.maxTxSize,
      cost: { memory: '10', steps: '10' },
      txBytes: 100,
    });
    assert.equal(within.ok, true);
    assert.equal(within.nativeBudgetMeasured, false);
    assert.equal(within.quantities.protocolMinAda, 'unmeasured');
    assert.equal(within.quantities.kernelCash, '100');
    assert.equal(within.quantities.unitPayout, '2000000');
    const suppliedUnits = compareBudget({
      queried: true,
      source: 'cardano-cli-query',
      unitOnly: false,
      maxTxExecutionUnits: { memory: 16500000, steps: 10000000000 },
      maxTxSize: 16384,
      utxoCostPerByte: 4310,
      coinsPerUTxOByte: null,
      cost: { memory: 10, steps: 10 },
      txBytes: 100,
      executionSource: 'caller-flag',
    });
    assert.equal(suppliedUnits.minAdaMeasured, false);
    assert.equal(suppliedUnits.quantities.protocolMinAda, 'unmeasured');
    assert.equal(suppliedUnits.nativeBudgetMeasured, false);
    const namedAiken = writeExec(join(dir, 'aiken'), [
      '#!/bin/sh',
      'echo programHash=' + 'cd'.repeat(32),
    ]);
    const suffix = compileGuardSibling({
      compilerBin: namedAiken,
      sourcePath: siblingSource,
      previousProgramHash: 'ab'.repeat(32),
    });
    assert.equal(suffix.unitOnly, true);
    assert.equal(suffix.compilerIdentity, 'path-is-not-aiken');
    const presentOnly = writeExec(join(dir, 'present.sh'), [
      '#!/bin/sh',
      'echo \'{"present":true}\'',
    ]);
    const present = collectHeadReadback({ cliPath: presentOnly, socket, networkMagic: 764 });
    assert.equal(present.code, 'READBACK_FIELDS_ABSENT');
    assert.equal(present.booleanIgnored, true);
    assert.equal(present.confirmed, false);
    const full = {
      program_hash: '11'.repeat(32),
      bounds_hash: '22'.repeat(32),
      source_hash: '33'.repeat(32),
      schema_hash: '44'.repeat(32),
      seed_hash: '55'.repeat(32),
      domain_network: 'yaci-isolated-t1',
      domain_deployment: 't1-custody-revision03',
      instance_id: '66'.repeat(32),
      transport_revision: 0,
      transport_remaining: 128,
      ordinary_locked: 49,
      ordinary_revision: 1,
      lifecycle: { work: { spent: 49 }, balances: [] },
      evidenceClass: 'unit-fixture',
    };
    const readTx = 'ab'.repeat(32);
    const readCli = writeExec(join(dir, 'readback.sh'), ['#!/bin/sh', `cat <<'EOF'`, JSON.stringify({
      [`${readTx}#3`]: {
        address: 'addr_test1fixture',
        value: { lovelace: 2000000 },
        datum: null,
        inlineDatum: full,
      },
    }), 'EOF']);
    const readback = collectHeadReadback({
      cliPath: readCli,
      socket,
      networkMagic: 764,
      expectedTxid: readTx,
      outputIndex: 3,
    });
    assert.equal(readback.code, 'HEAD_READBACK');
    assert.equal(readback.fieldCount, 13);
    assert.equal(readback.unitOnly, true);
    assert.equal(readback.confirmed, false);
    assert.equal(readback.workSpent, 49);
    assert.equal(readback.txidLinked, true);
    assert.equal(readback.selector, '--tx-in');
    const forced = decideConfirmation(readback, 'confirmed');
    assert.equal(forced.code, 'READBACK_UNIT_ONLY');
    assert.equal(forced.confirmed, false);
    const predicate = decideConfirmation({ ...readback, unitOnly: false }, 'confirmed');
    assert.equal(predicate.code, 'READBACK_UNPROVEN');
    const unlinked = decideConfirmation({ ...readback, unitOnly: false, txidLinked: false }, 'confirmed');
    assert.equal(unlinked.code, 'READBACK_TXID_UNLINKED');
    assert.equal(predicate.confirmed, false);
    assert.equal(predicate.ledgerExecution, false);
    assert.equal(predicate.nativePins, false);
    const echoed = decideConfirmation({
      ...readback,
      unitOnly: false,
      queryProvenance: true,
      txidLinked: false,
      helperConfirmed: false,
    }, 'confirmed');
    assert.notEqual(echoed.code, 'FINANCIAL_READBACK_CONFIRMED');
    assert.equal(echoed.confirmed, false);
    const ids = ['11', '22', '33', '44', '55'].map((byte, index) => ({ txId: byte.repeat(32), index }));
    const logPath = join(dir, 'race.log');
    const leftBody = join(dir, 'release.body.json');
    const rightBody = join(dir, 'refund.body.json');
    writeFileSync(leftBody, '{"type":"Tx ConwayEra","cborHex":"aa"}\n');
    writeFileSync(rightBody, '{"type":"Tx ConwayEra","cborHex":"bb"}\n');
    const signingKey = join(dir, 'payment.skey');
    writeFileSync(signingKey, 'unit-key-not-a-wallet\n', { mode: 0o600 });
    chmodSync(signingKey, 0o600);
    const raced = await runOwnedRace({
      signBin: appender(dir, 'sign.sh'),
      submitBin: appender(dir, 'submit.sh'),
      pollBin: appender(dir, 'poll.sh'),
      logPath,
      head: ids[0],
      left: { fee: ids[1], collateral: ids[2], actor: 'release', resource: 'release-fee' },
      right: { fee: ids[3], collateral: ids[4], actor: 'refund', resource: 'refund-fee' },
      leftBodyFile: leftBody,
      rightBodyFile: rightBody,
      signingKeyPath: signingKey,
      networkMagic: 764,
    });
    assert.equal(raced.code, 'RACE_SUBMITTED_BEFORE_POLL');
    assert.equal(raced.submitsBeforeConfirms, true);
    assert.equal(raced.ledgerSubmitted, false);
    assert.equal(raced.confirmed, false);
    assert.equal(raced.submitted, false);
    assert.equal(raced.replay.executed, false);
    assert.equal(raced.replay.observed, false);
    assert.equal(raced.replay.ledgerSubmitted, false);
    assert.equal(raced.replay.expectedClass, 'BadInputsUTxO');
    assert.deepEqual(raced.actors, ['release', 'refund']);
    const lines = readFileSync(logPath, 'utf8').trim().split('\n');
    const text = lines.join('\n');
    assert.equal(lines.filter((line) => line.includes('transaction witness')).length, 2);
    assert.ok(lines.filter((line) => line.includes('transaction submit')).length >= 3);
    assert.equal(lines.filter((line) => line.startsWith('query tip')).length, 2);
    assert.equal(text.includes('unit-body.json'), false);
    assert.equal(text.includes('f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18'), false);
    assert.ok(text.includes(leftBody));
    assert.ok(text.includes(rightBody));
    const firstPoll = lines.findIndex((line) => line.includes('query tip'));
    const submits = lines.map((line, index) => (line.includes('transaction submit') ? index : -1)).filter((index) => index >= 0);
    assert.ok(submits[0] < firstPoll);
    assert.ok(submits[1] < firstPoll);
    assert.ok(text.indexOf('transaction witness') < text.indexOf('transaction submit'));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('queried utxo and key files do not fabricate native pins', () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-pins-'));
  try {
    const blueprint = JSON.parse(readFileSync(BLUEPRINT, 'utf8'));
    const minted = blueprint.validators.find((item) => item.title.includes('financial_head') && item.title.endsWith('.mint'));
    const cash = blueprint.validators.find((item) => item.title.includes('cash') && item.title.endsWith('.mint'));
    const scriptHash = scriptHashV3(minted.compiledCode);
    const cashPolicyId = scriptHashV3(cash.compiledCode);
    assert.equal(scriptHash, minted.hash);
    assert.equal(cashPolicyId, cash.hash);
    const loose = join(dir, 'loose.skey');
    const owned = join(dir, 'owned.skey');
    writeFileSync(loose, 'not-a-key\n');
    chmodSync(loose, 0o644);
    writeFileSync(owned, 'not-a-key\n');
    chmodSync(owned, 0o600);
    const base = {
      scriptHash,
      cashPolicyId,
      appliedBlueprintPath: BLUEPRINT,
      cashSourceText: CASH,
      nativePins: true,
      utxoOwnedByRun: true,
    };
    assert.equal(certifyRuntimePins({ ...base, keyPath: '/tmp/missing-caller50.skey' }).code, 'WITNESS_KEYS_ABSENT');
    assert.equal(certifyRuntimePins({ ...base, keyPath: loose }).code, 'KEY_MODE_REFUSED');
    const reviewed = writeAuthority(dir, 'reviewed-implementation', CASH);
    const socket = join(dir, 'node.socket');
    writeFileSync(socket, '');
    const utxoCli = writeExec(join(dir, 'utxo.sh'), [
      '#!/bin/sh',
      'nonce=""',
      'out=""',
      'while [ $# -gt 0 ]; do',
      '  case "$1" in',
      '    --nonce) nonce="$2"; shift 2;;',
      '    --out-file) out="$2"; shift 2;;',
      '    *) shift;;',
      '  esac',
      'done',
      'printf \'nonce=%s\\nevidenceClass=unit-fixture\\nutxos=%s\\n\' "$nonce" \'[{"txId":"' + 'ab'.repeat(32) + '","index":0}]\' > "$out"',
    ]);
    const queried = queryOwnedUtxos({ cliPath: utxoCli, socket, runDir: dir, execute: true, magic: 764 });
    assert.equal(queried.code, 'UTXO_QUERIED');
    assert.equal(queried.spawned, true);
    assert.equal(queried.unitOnly, true);
    assert.equal(queried.nativePins, false);
    const pinned = certifyRuntimePins({
      ...base,
      keyPath: owned,
      cashVotes: reviewed,
      utxoQuery: queried,
    });
    assert.equal(pinned.relationsOk, true);
    assert.equal(pinned.ledgerFunding, false);
    assert.equal(pinned.code, 'CASH_REVIEW_CLASS_MISMATCH');
    assert.equal(pinned.nativePins, false);
    assert.equal(pinned.runtimePinsCreated, false);
    assert.equal(pinned.flagIgnored, true);
    const forged = certifyRuntimePins({
      ...base,
      keyPath: owned,
      cashVotes: reviewed,
      utxoQuery: { ...queried, unitOnly: false, runId: 'ff'.repeat(16), spawned: true, queried: true },
    });
    assert.equal(forged.nativePins, false);
    assert.equal(forged.runtimePinsCreated, false);
    const realQuery = queryOwnedUtxos({ cliPath: CARDANO_CLI, socket, runDir: dir, execute: true });
    assert.equal(realQuery.code, 'REAL_QUERY_NOT_ARMED');
    assert.equal(realQuery.spawned, false);
    const parsed = parseCliUtxoJson(JSON.stringify({
      [`${'ab'.repeat(32)}#1`]: { value: { lovelace: 2000000 } },
    }));
    assert.equal(parsed[0].index, 1);
    assert.equal(parsed[0].txId, 'ab'.repeat(32));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('integrated caller reaches owned branches and blocks an over-limit submit', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-integrated-'));
  try {
    const authority = writeAuthority(dir, 'unit-fixture', CASH);
    const nodeBin = stubNode(dir);
    const socket = join(dir, 'query.socket');
    writeFileSync(socket, '');
    const timeCli = writeExec(join(dir, 'time.sh'), [
      '#!/bin/sh',
      'cat <<\'EOF\'',
      'slotLengthMs=1000',
      'lucidPosix=40000',
      'cliPosix=40000',
      'scriptContextLower=40000',
      'tipSlot=10',
      'evidenceClass=unit-fixture',
      'EOF',
    ]);
    const siblingSource = join(dir, 'guard.mori');
    writeFileSync(siblingSource, `${CASH}sibling\n`);
    const compiler = writeExec(join(dir, 'compiler.sh'), [
      '#!/bin/sh',
      'echo programHash=' + 'cd'.repeat(32),
      'echo evidenceClass=unit-fixture',
    ]);
    const protocol = writeExec(join(dir, 'protocol.sh'), [
      '#!/bin/sh',
      'out=""',
      'while [ $# -gt 0 ]; do',
      '  if [ "$1" = "--out-file" ]; then out="$2"; shift 2; else shift; fi',
      'done',
      `cp ${join(dir, 'params.json')} "$out"`,
    ]);
    writeFileSync(join(dir, 'params.json'), JSON.stringify({
      evidenceClass: 'unit-fixture',
      maxTxExecutionUnits: { memory: '1000', steps: '1000' },
      maxTxSize: 100,
    }));
    const keyCli = keyStub(dir);
    const ids = ['11', '22', '33', '44', '55'].map((byte, index) => ({ txId: byte.repeat(32), index }));
    const utxoCli = writeExec(join(dir, 'utxo.sh'), [
      '#!/bin/sh',
      'nonce=""',
      'out=""',
      'while [ $# -gt 0 ]; do',
      '  case "$1" in',
      '    --nonce) nonce="$2"; shift 2;;',
      '    --out-file) out="$2"; shift 2;;',
      '    *) shift;;',
      '  esac',
      'done',
      `printf 'nonce=%s\\nevidenceClass=unit-fixture\\nutxos=%s\\n' "$nonce" '${JSON.stringify(ids)}' > "$out"`,
    ]);
    const logPath = join(dir, 'race.log');
    const faucetLog = join(dir, 'faucet.log');
    const over = await runNativeCaller({
      authority,
      sourcePath: SOURCE,
      launch: { startNode: true, nodeBin, runDir: dir },
      cashSource: CASH,
      cashVotes: authority,
      protocol: { socket, cliPath: protocol, execute: true, magic: 764 },
      cost: { memory: '111620000', steps: '32820000000' },
      txBytes: 500,
      time: { cliPath: timeCli, socket, execute: true, magic: 764 },
      sibling: { compilerBin: compiler, sourcePath: siblingSource, previousProgramHash: 'ab'.repeat(32) },
      keys: { cliPath: keyCli, runDir: dir, magic: 764 },
      utxoQuery: { cliPath: utxoCli, socket, runDir: dir, execute: true, magic: 764 },
      race: {
        signBin: appender(dir, 'sign.sh'),
        submitBin: appender(dir, 'submit.sh'),
        pollBin: appender(dir, 'poll.sh'),
        logPath,
        leftActor: 'release',
        rightActor: 'cancel',
        leftResource: 'release-fee',
        rightResource: 'cancel-fee',
      },
      faucet: { bin: appender(dir, 'faucet.sh'), logPath: faucetLog },
    });
    assert.equal(over.code, 'BUDGET_EXCEEDED');
    assert.equal(over.nativePins, false);
    assert.equal(over.ledgerExecution, false);
    assert.equal(over.startedNode, true);
    assert.equal(over.stages.budget.submitBlocked, true);
    assert.equal(over.stages.race.code, 'BUDGET_EXCEEDED');
    assert.equal(existsSync(logPath), false);
    assert.equal(over.stages.time.code, 'SLOT_OBSERVED');
    assert.equal(over.stages.sibling.code, 'HASH_LINE_NOT_A_COMPILE');
    assert.equal(over.stages.sibling.siblingEmitted, false);
    assert.equal(over.stages.sibling.unitOnly, true);
    assert.equal(over.stages.keys.code, 'ENTERPRISE_KEY_FILE');
    assert.equal(over.stages.keys.wallet, false);
    assert.equal((statSync(over.stages.keys.keyPath).mode & 0o777), 0o600);
    assert.equal(over.stages.utxo.unitOnly, true);
    assert.equal(over.stages.source.code, 'SOURCE_COMPILED');
    assert.equal(over.stages.source.compiled, true);
    assert.equal(over.stages.source.sha256, createHash('sha256').update(readFileSync(SOURCE)).digest('hex'));
    assert.equal(await waitGone(over.stages.launcher.pid), true);
    const realKey = prepareEnterpriseKey({ cliPath: CARDANO_CLI, runDir: dir, authority });
    assert.equal(realKey.code, 'REAL_KEYGEN_NOT_ARMED');
    assert.equal(realKey.spawnAttempted, false);
  } finally {
    killRecorded(join(dir, 'proof.txt'));
    rmSync(dir, { recursive: true, force: true });
  }
});

test('caller builds a real unsigned body, rejects a garbage witness, and the cli reaches that caller', async () => {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-cli-'));
  const out = join(CHECKS, 'native-caller-cli');
  try {
    const authority = writeAuthority(dir, 'unit-fixture', CASH);
    const nodeBin = stubNode(dir);
    const socket = join(dir, 'query.socket');
    writeFileSync(socket, '');
    const cashPath = join(dir, 'cash.mori');
    writeFileSync(cashPath, CASH);
    const timeCli = writeExec(join(dir, 'time.sh'), [
      '#!/bin/sh',
      'cat <<\'EOF\'',
      'slotLengthMs=1000',
      'lucidPosix=40000',
      'cliPosix=40000',
      'scriptContextLower=40000',
      'tipSlot=10',
      'evidenceClass=unit-fixture',
      'EOF',
    ]);
    const siblingSource = join(dir, 'guard.mori');
    writeFileSync(siblingSource, `${CASH}sibling\n`);
    const compiler = writeExec(join(dir, 'compiler.sh'), [
      '#!/bin/sh',
      'echo programHash=' + 'cd'.repeat(32),
      'echo evidenceClass=unit-fixture',
    ]);
    const protocol = writeExec(join(dir, 'protocol.sh'), [
      '#!/bin/sh',
      'out=""',
      'while [ $# -gt 0 ]; do',
      '  if [ "$1" = "--out-file" ]; then out="$2"; shift 2; else shift; fi',
      'done',
      `cp ${join(dir, 'params.json')} "$out"`,
    ]);
    writeFileSync(join(dir, 'params.json'), JSON.stringify({
      evidenceClass: 'unit-fixture',
      maxTxExecutionUnits: { memory: '16000000', steps: '10000000000' },
      maxTxSize: 16384,
    }));
    const keyCli = keyStub(dir);
    const ids = ['11', '22', '33', '44', '55'].map((byte, index) => ({ txId: byte.repeat(32), index }));
    const utxoCli = writeExec(join(dir, 'utxo.sh'), [
      '#!/bin/sh',
      'nonce=""',
      'out=""',
      'while [ $# -gt 0 ]; do',
      '  case "$1" in',
      '    --nonce) nonce="$2"; shift 2;;',
      '    --out-file) out="$2"; shift 2;;',
      '    *) shift;;',
      '  esac',
      'done',
      `printf 'nonce=%s\\nevidenceClass=unit-fixture\\nutxos=%s\\n' "$nonce" '${JSON.stringify(ids)}' > "$out"`,
    ]);
    const readCli = writeExec(join(dir, 'readback.sh'), [
      '#!/bin/sh',
      'cat <<\'EOF\'',
      JSON.stringify({
        program_hash: '11'.repeat(32),
        bounds_hash: '22'.repeat(32),
        source_hash: '33'.repeat(32),
        schema_hash: '44'.repeat(32),
        seed_hash: '55'.repeat(32),
        domain_network: 'yaci-isolated-t1',
        domain_deployment: 't1-custody-revision03',
        instance_id: '66'.repeat(32),
        transport_revision: 0,
        transport_remaining: 128,
        ordinary_locked: 49,
        ordinary_revision: 1,
        lifecycle: { work: { spent: 49 }, balances: [{ asset: 'ada', quantity: '2000000' }] },
        evidenceClass: 'unit-fixture',
      }),
      'EOF',
    ]);
    const logPath = join(dir, 'race.log');
    const faucetLog = join(dir, 'faucet.log');
    const built = await runNativeCaller({
      authority,
      launch: { startNode: false, nodeBin, runDir: dir },
      keys: { cliPath: keyCli, runDir: dir, magic: 764 },
      utxos: ids,
      buildBody: true,
      time: { cliPath: timeCli, socket, execute: true, magic: 764 },
    });
    assert.equal(built.stages.body.ok, true, built.stages.body.code);
    assert.match(built.stages.body.txid, /^[0-9a-f]{64}$/);
    assert.equal(built.stages.body.type, 'Tx ConwayEra');
    assert.equal(built.stages.body.signed, false);
    assert.equal(built.stages.witness.code, 'WITNESS_CONSTRUCTOR_REJECTED');
    assert.equal(built.stages.witness.constructorReached, true);
    assert.equal(built.stages.witness.signed, false);
    assert.equal(built.nativePins, false);
    assert.equal(built.ledgerExecution, false);
    const argv = [
      '--experimental-strip-types', 'src/cli.mjs', 'native',
      '--out', out,
      '--source', SOURCE,
      '--run-dir', dir,
      '--authority-a', authority.reviewA,
      '--authority-b', authority.reviewB,
      '--candidate', authority.candidate,
      '--execute', 'true',
      '--node-bin', nodeBin,
      '--cash-source', cashPath,
      '--cash-vote-a', authority.reviewA,
      '--cash-vote-b', authority.reviewB,
      '--cash-candidate', authority.candidate,
      '--protocol-cli', protocol,
      '--protocol-socket', socket,
      '--tx-cost-memory', '10',
      '--tx-cost-steps', '10',
      '--tx-bytes', '100',
      '--time-cli', timeCli,
      '--time-socket', socket,
      '--sibling-compiler', compiler,
      '--sibling-source', siblingSource,
      '--previous-program-hash', 'ab'.repeat(32),
      '--key-cli', keyCli,
      '--utxo-cli', utxoCli,
      '--utxo-socket', socket,
      '--submit-bin', appender(dir, 'submit.sh'),
      '--poll-bin', appender(dir, 'poll.sh'),
      '--sign-bin', appender(dir, 'sign.sh'),
      '--race-log', logPath,
      '--left-actor', 'release',
      '--right-actor', 'cancel',
      '--left-resource', 'release-fee',
      '--right-resource', 'cancel-fee',
      '--readback-cli', readCli,
      '--readback-socket', socket,
      '--receipt-status', 'confirmed',
      '--faucet-bin', appender(dir, 'faucet.sh'),
      '--faucet-log', faucetLog,
    ];
    const ran = spawnSync(process.execPath, argv, { cwd: ROOT, encoding: 'utf8' });
    assert.equal(ran.status, 0, ran.stderr);
    const report = JSON.parse(ran.stdout);
    assert.equal(report.code, 'EVALUATION_UNAVAILABLE');
    assert.equal(report.nativePins, false);
    assert.equal(report.ledgerExecution, false);
    assert.equal(report.confirmed, false);
    assert.equal(report.submitted, false);
    assert.equal(report.shellExitMeansLedger, false);
    assert.equal(report.startedNode, true);
    assert.equal(report.stages.launcher, 'OWNED_NODE_SPAWNED');
    assert.equal(report.stages.cash, 'CASH_POLICY_UNIT_ACCEPTED');
    assert.equal(report.stages.protocol, 'PROTOCOL_PARAMS_QUERIED');
    assert.equal(report.stages.budget, 'EVALUATION_UNAVAILABLE');
    assert.equal(report.stages.time, 'SLOT_OBSERVED');
    assert.equal(report.stages.sibling, 'HASH_LINE_NOT_A_COMPILE');
    assert.equal(report.stages.keys, 'ENTERPRISE_KEY_FILE');
    assert.equal(report.stages.utxo, 'UTXO_QUERIED');
    assert.equal(report.stages.inputs, 'INPUTS_SELECTED');
    assert.equal(report.stages.race, 'EVALUATION_UNAVAILABLE');
    assert.equal(report.stages.readback, 'HEAD_READBACK');
    assert.equal(report.stages.confirmation, 'READBACK_UNIT_ONLY');
    assert.equal(report.stages.faucet, 'FAUCET_INPUTS_ABSENT');
    assert.equal(report.stages.source, 'SOURCE_COMPILED');
    assert.equal(report.source.sha256, createHash('sha256').update(readFileSync(SOURCE)).digest('hex'));
    assert.equal(report.source.compiled, true);
    assert.equal(report.ceilingRemoved, false);
    assert.equal(report.nativePins, false);
    assert.equal(report.ledgerSubmitted, false);
    assert.equal(report.unitOnly, true);
    assert.equal(report.submitted, false);
    const proof = readFileSync(join(dir, 'proof.txt'), 'utf8');
    assert.match(proof, new RegExp(`^home=${dir}$`, 'm'));
    assert.equal(await waitGone(Number(proof.match(/^pid=(\d+)$/m)[1])), true);
    assert.equal(existsSync(logPath), false);
    assert.equal(existsSync(faucetLog), false);
    const forbidden = spawnSync(process.execPath, [
      '--experimental-strip-types', 'src/cli.mjs', 'native',
      '--out', join(TASK, 't1-native-repair-46/checks/not-allowed'),
      '--source', SOURCE,
    ], { cwd: ROOT, encoding: 'utf8' });
    assert.equal(forbidden.status, 1);
    assert.match(`${forbidden.stdout}${forbidden.stderr}`, /OUTPUT_PATH_FORBIDDEN/);
    mkdirSync(CHECKS, { recursive: true });
    writeFileSync(join(CHECKS, 'caller-cli.argv.txt'), `${process.execPath} ${argv.join(' ')}\n`);
  } finally {
    killRecorded(join(dir, 'proof.txt'));
    rmSync(dir, { recursive: true, force: true });
  }
});

test('cash design votes are a separate class and do not fund', () => {
  const source = join(TASK, 't1-cash-repair-53/prototype/validators/cash_fixture.ak');
  const candidate = join(TASK, 't1-cash-repair-53/CANDIDATE-SHA256.json');
  const adopted = loadCashDesignAuthority({
    reviewA: join(TASK, 'cash54-approved56/a/RECEIPT.json'),
    reviewB: join(TASK, 'cash54-approved56/b/RECEIPT.json'),
    candidate,
    sourcePath: source,
  });
  assert.equal(adopted.code, 'CASH_DESIGN_ADOPTED');
  assert.equal(adopted.designAdopted, true);
  assert.equal(adopted.cashCanMint, false);
  assert.equal(adopted.ledgerFunding, false);
  assert.equal(adopted.nativePins, false);
  assert.equal(adopted.reviewClass, 'cash-fixture-design');
  assert.notEqual(adopted.reviewers[0], adopted.reviewers[1]);
  const diagnostic = mkdtempSync(join(tmpdir(), 'moriarty-cash-diag-'));
  const diag = join(diagnostic, 'vote.json');
  writeFileSync(diag, JSON.stringify({
    reviewer: 'same',
    verdict: 'APPROVE_DIAGNOSTIC_FIXTURE_NOT_A_REVIEW',
    candidateSha256: createHash('sha256').update(readFileSync(candidate)).digest('hex'),
    reviewClass: 'native-implementation',
  }));
  const rejected = loadCashDesignAuthority({ reviewA: diag, reviewB: diag, candidate, sourcePath: source });
  assert.equal(rejected.cashCanMint, false);
  assert.equal(rejected.ledgerFunding, false);
  assert.notEqual(rejected.code, 'CASH_DESIGN_ADOPTED');
  rmSync(diagnostic, { recursive: true, force: true });
});
