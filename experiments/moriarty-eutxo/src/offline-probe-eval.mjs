/**
 * Offline V3 lower probe.
 * cardano-cli calculate-plutus-script-cost success JSON is execution units and drops logs.
 * The probe fails on purpose. The accepted lower is the MORIARTY_LOWER trace in that failure.
 * Lucid's slotToBeginUnixTime is the clock. This module does not query a node.
 */
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { cpSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { slotToBeginUnixTime } from '../owned/node_modules/@lucid-evolution/plutus/dist/index.js';
import { slotLengthMsFromShelley } from '../owned/runtime-schemas-71/schemas.mjs';
import { genesisMagicText } from './native-body.mjs';
import { plutusCostArgv } from './native-producer.mjs';

const PROJECT = fileURLToPath(new URL('../probe-parity', import.meta.url));
const VALIDATOR = join(PROJECT, 'validators', 'parity_probe.ak');

const SPEND_TX = 'e25450233e4bedd00c8bda15c48c2d4018223bd88271e194052294c4e5be7d55';
const COLLATERAL_TX = '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef';
const COLLATERAL_ADDRESS = 'addr_test1vqfxq2s8yce3tuhjq9ulu2awuk623hzvtft9z8fh6qelzts49vuqw';

const HISTORICAL_SEGMENTS = [
  't1-capacity-repair-25',
  't1-native-implementation-36',
  't1-native-feasibility-23',
  't1-native-implementation-28',
  'source25-replay-42',
  'evidence-commit-38',
  'independent-review-a',
  'independent-review-b',
  'frozen-candidate44',
  'frozen-candidate50',
  'review41-freeze-46',
  't1-native-repair-46',
  't1-native-caller-50',
  't1-native-diagnosis-52',
  'transaction-receipts-10',
];

function closed(code, extra = {}) {
  return {
    ok: false,
    code,
    productionAdmitted: false,
    scriptContextLower: null,
    cliPosix: null,
    lucidPosix: null,
    evaluatorRangeLower: null,
    tipSlot: null,
    ledgerExecution: false,
    networkCommandsRun: false,
    siblingEmitted: false,
    sourceEmitted: false,
    productionArtifact: false,
    probeExecuted: false,
    intendedFailure: false,
    evidenceClass: 'offline-cli-v3-probe',
    nativeUnexecuted: true,
    ...extra,
    productionAdmitted: false,
  };
}

function sha256File(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

function sha256Text(text) {
  return createHash('sha256').update(text).digest('hex');
}

function bindingPath(path) {
  if (typeof path !== 'string' || path[0] !== '/') return closed('BINDING_PATH_ABSENT');
  const hit = HISTORICAL_SEGMENTS.find((name) => path.split('/').includes(name));
  if (hit) return closed('OUTPUT_PATH_FORBIDDEN', { path, historical: hit });
  return { ok: true, path };
}

function readClock(path) {
  if (!path || !existsSync(path)) return closed('TIME_QUERY_ABSENT', { reason: 'genesis-absent' });
  let body;
  try {
    body = JSON.parse(readFileSync(path, 'utf8'));
  } catch {
    return closed('MALFORMED_DATA', { reason: 'genesis-json' });
  }
  const slot = slotLengthMsFromShelley(body.slotLength);
  if (!slot.ok || slot.slotLengthMs !== 1000) {
    return closed('SLOT_LENGTH_REFUSED', { observed: slot.ok ? String(slot.slotLengthMs) : slot.code });
  }
  if (typeof body.systemStart !== 'string' || Number.isNaN(Date.parse(body.systemStart))) {
    return closed('PARITY_NOT_MEASURED', { reason: 'system-start' });
  }
  return {
    ok: true,
    body,
    systemStart: body.systemStart,
    systemStartMs: Date.parse(body.systemStart),
    zeroSlot: body.zeroSlot == null ? 0 : Number(body.zeroSlot),
    sha256: sha256File(path),
  };
}

function unique(values) {
  return [...new Set(values)];
}

export function parseProbeEvaluation(input = {}) {
  const text = `${input.stdout || ''}\n${input.stderr || ''}`;
  const status = input.status;
  const traces = [...text.matchAll(/MORIARTY_LOWER:([0-9]+)(?![0-9])/g)].map((match) => match[1]);
  const ranges = [...text.matchAll(/Valid range:\s*\[\s*POSIXTime\s+([0-9]+)(?![0-9])/g)].map((match) => match[1]);
  const hashes = [...text.matchAll(/with hash:\s*Just\s*"([0-9a-f]+)"/g)].map((match) => match[1]);
  const commandNamed = text.includes('transaction calculate-plutus-script-cost');
  const base = {
    traces: unique(traces),
    traceCount: traces.length,
    ranges: unique(ranges),
    hashes: unique(hashes),
    commandNamed,
    status: status == null ? null : status,
  };
  if (status == null) return closed('CLI_SPAWN_FAILED', base);
  if (traces.length > 1 || ranges.length > 1 || hashes.length > 1) {
    return closed('FORGED_TRACE', { ...base, probeExecuted: true, intendedFailure: status !== 0 });
  }
  if (status === 0) {
    return closed('COST_OUTPUT_HAS_NO_LOWER', {
      ...base,
      probeExecuted: true,
      intendedFailure: false,
      reason: 'Success JSON reports execution units and drops script logs. A zero exit is not a lower.',
    });
  }
  if (!commandNamed) return closed('COMMAND_IDENTITY_MISMATCH', { ...base, probeExecuted: true });
  if (traces.length !== 1 || ranges.length !== 1) {
    return closed('COST_OUTPUT_HAS_NO_LOWER', { ...base, probeExecuted: true, intendedFailure: true });
  }
  if (hashes.length !== 1) return closed('WRONG_SCRIPT', { ...base, probeExecuted: true, intendedFailure: true });
  if (input.compiledHash && hashes[0] !== input.compiledHash) {
    return closed('WRONG_SCRIPT', { ...base, probeExecuted: true, intendedFailure: true, observedHash: hashes[0] });
  }
  if (traces[0] !== ranges[0]) {
    return closed('IMPOSSIBLE_BOUNDARY', {
      ...base,
      probeExecuted: true,
      intendedFailure: true,
      scriptContextLower: traces[0],
      evaluatorRangeLower: ranges[0],
      reason: 'Probe trace and evaluator validity lower differ.',
    });
  }
  if (input.lucidPosix != null && traces[0] !== String(input.lucidPosix)) {
    return closed('IMPOSSIBLE_BOUNDARY', {
      ...base,
      probeExecuted: true,
      intendedFailure: true,
      scriptContextLower: traces[0],
      evaluatorRangeLower: ranges[0],
      lucidPosix: String(input.lucidPosix),
    });
  }
  return {
    ok: true,
    code: 'TRACE_MATCH',
    productionAdmitted: false,
    scriptContextLower: traces[0],
    evaluatorRangeLower: ranges[0],
    scriptHash: hashes[0],
    lucidPosix: input.lucidPosix == null ? null : String(input.lucidPosix),
    probeExecuted: true,
    intendedFailure: true,
    evidenceClass: 'offline-cli-v3-probe',
    ledgerExecution: false,
    networkCommandsRun: false,
    nativeUnexecuted: true,
    ...base,
  };
}

function ensureCompiled(workRoot, sourceSha) {
  const copyDir = join(workRoot, 'probe');
  const stamp = join(copyDir, '.source-sha');
  const blueprint = join(copyDir, 'plutus.json');
  if (!(existsSync(blueprint) && existsSync(stamp) && readFileSync(stamp, 'utf8') === sourceSha)) {
    mkdirSync(join(copyDir, 'build'), { recursive: true });
    cpSync(join(PROJECT, 'aiken.toml'), join(copyDir, 'aiken.toml'));
    cpSync(join(PROJECT, 'aiken.lock'), join(copyDir, 'aiken.lock'));
    cpSync(join(PROJECT, 'validators'), join(copyDir, 'validators'), { recursive: true });
    cpSync(join(PROJECT, 'build', 'packages'), join(copyDir, 'build', 'packages'), { recursive: true });
    const aiken = spawnSync('aiken', ['build', '-t', 'verbose'], { cwd: copyDir, encoding: 'utf8' });
    writeFileSync(join(workRoot, 'aiken.stdout'), aiken.stdout || '');
    writeFileSync(join(workRoot, 'aiken.stderr'), aiken.stderr || '');
    writeFileSync(join(workRoot, 'aiken.exit'), `${aiken.status == null ? 'null' : aiken.status}\n`);
    if (aiken.status !== 0 || !existsSync(blueprint)) {
      return closed('AIKEN_BUILD_FAILED', { aikenStatus: aiken.status, stderrSha256: sha256Text(aiken.stderr || '') });
    }
    writeFileSync(stamp, sourceSha);
  }
  const blueprintBody = JSON.parse(readFileSync(blueprint, 'utf8'));
  const validator = (blueprintBody.validators || []).find((item) => item.title === 'parity_probe.parity_probe.spend');
  if (!validator || typeof validator.compiledCode !== 'string' || typeof validator.hash !== 'string') {
    return closed('AIKEN_BUILD_FAILED', { reason: 'spend-validator-absent' });
  }
  return { ok: true, compiledCode: validator.compiledCode, compiledHash: validator.hash, copyDir, blueprint };
}

export function executeOfflineProbe(input = {}) {
  if (input == null || typeof input !== 'object') return closed('MALFORMED_DATA');
  if (input.unitOnly === true || input.evidenceClass === 'UNIT' || input.evidenceClass === 'UNIT/SYNTHETIC') {
    return closed('UNIT_PARITY_REFUSED');
  }
  if (input.forged === true || input.productionAdmitted === true) return closed('FORGED_PARITY');
  if (!Number.isInteger(input.tipSlot) || input.tipSlot < 0 || input.tipSlot > Number.MAX_SAFE_INTEGER) {
    return closed('TIP_SLOT_REQUIRED');
  }
  const work = bindingPath(input.workRoot);
  if (!work.ok) return work;
  if (!existsSync(work.path)) return closed('BINDING_PATH_ABSENT', { reason: 'work-root-absent' });
  for (const key of ['genesisPath', 'eraHistoryFile', 'protocolParamsFile', 'cliPath']) {
    if (typeof input[key] !== 'string' || !existsSync(input[key])) return closed('TIME_QUERY_ABSENT', { field: key });
  }
  const clock = readClock(input.genesisPath);
  if (!clock.ok) return clock;
  const magic = genesisMagicText(input.magic ?? input.networkMagic ?? clock.body.networkMagic);
  if (!magic.ok) return closed(magic.code);
  let lucidPosix;
  try {
    lucidPosix = String(slotToBeginUnixTime(input.tipSlot, {
      zeroTime: clock.systemStartMs,
      zeroSlot: clock.zeroSlot,
      slotLength: 1000,
    }));
  } catch {
    return closed('IMPOSSIBLE_BOUNDARY', { reason: 'lucid-threw' });
  }
  if (!/^[0-9]+$/.test(lucidPosix)) return closed('IMPOSSIBLE_BOUNDARY', { lucidPosix, reason: 'lucid-non-decimal' });
  if (!existsSync(VALIDATOR)) return closed('ARTIFACT_BINDING_MISMATCH', { reason: 'probe-source-absent' });
  const sourceText = readFileSync(VALIDATOR, 'utf8');
  if (!sourceText.includes('validator parity_probe') || !sourceText.includes('MORIARTY_LOWER:')) {
    return closed('ARTIFACT_BINDING_MISMATCH', { reason: 'probe-source-shape' });
  }
  const sourceSha = sha256Text(sourceText);
  const compiled = ensureCompiled(work.path, sourceSha);
  if (!compiled.ok) return compiled;
  const scriptPath = join(work.path, 'probe-script.json');
  const scriptEnvelope = {
    type: 'PlutusScriptV3',
    description: 'offline parity probe, deliberate failure, not a native spend',
    cborHex: compiled.compiledCode,
  };
  writeFileSync(scriptPath, JSON.stringify(scriptEnvelope));
  const scriptSha = sha256File(scriptPath);
  const usedScript = input.wrongScriptPath || scriptPath;
  if (input.wrongScriptPath && !existsSync(input.wrongScriptPath)) return closed('WRONG_SCRIPT', { reason: 'substitution-absent' });
  const address = spawnSync(input.cliPath, [
    'address', 'build', '--payment-script-file', usedScript, '--testnet-magic', magic.text,
  ], { encoding: 'utf8' });
  writeFileSync(join(work.path, 'address.stdout'), address.stdout || '');
  writeFileSync(join(work.path, 'address.stderr'), address.stderr || '');
  const scriptAddress = (address.stdout || '').trim();
  if (address.status !== 0 || !scriptAddress.startsWith('addr')) {
    return closed('CLI_ADDRESS_FAILED', { addressStatus: address.status });
  }
  const utxoPath = join(work.path, 'utxo.json');
  const utxo = {
    [`${SPEND_TX}#0`]: {
      address: scriptAddress,
      datum: null,
      datumhash: null,
      inlineDatum: null,
      referenceScript: null,
      value: { lovelace: 5000000 },
    },
    [`${COLLATERAL_TX}#0`]: {
      address: COLLATERAL_ADDRESS,
      datum: null,
      datumhash: null,
      inlineDatum: null,
      referenceScript: null,
      value: { lovelace: 5000000 },
    },
  };
  writeFileSync(utxoPath, JSON.stringify(utxo));
  const bodyPath = join(work.path, 'probe.tx');
  const hereafter = (BigInt(input.tipSlot) + 1n).toString();
  const build = spawnSync(input.cliPath, [
    'conway', 'transaction', 'build-raw',
    '--tx-in', `${SPEND_TX}#0`,
    '--tx-in-script-file', usedScript,
    '--tx-in-redeemer-value', '42',
    '--tx-in-execution-units', '(14000000,10000000000)',
    '--tx-in-collateral', `${COLLATERAL_TX}#0`,
    '--tx-out', `${scriptAddress}+3000000`,
    '--fee', '200000',
    '--invalid-before', String(input.tipSlot),
    '--invalid-hereafter', hereafter,
    '--protocol-params-file', input.protocolParamsFile,
    '--out-file', bodyPath,
  ], { encoding: 'utf8' });
  writeFileSync(join(work.path, 'build-raw.stdout'), build.stdout || '');
  writeFileSync(join(work.path, 'build-raw.stderr'), build.stderr || '');
  if (build.status !== 0 || !existsSync(bodyPath)) {
    return closed(input.wrongScriptPath ? 'WRONG_SCRIPT' : 'BODY_BINDING_ABSENT', { buildStatus: build.status });
  }
  const bodySha = sha256File(bodyPath);
  if (input.foreignBodyPath) {
    if (!existsSync(input.foreignBodyPath)) return closed('FOREIGN_BODY', { bodySha256: bodySha, reason: 'substitution-absent' });
    const foreignSha = sha256File(input.foreignBodyPath);
    if (foreignSha !== bodySha) {
      return closed('FOREIGN_BODY', { bodySha256: bodySha, foreignBodySha256: foreignSha, probeExecuted: false });
    }
  }
  const costOut = join(work.path, 'cost.json');
  const costArgv = plutusCostArgv({
    cli: input.cliPath,
    mode: 'offline',
    txFile: bodyPath,
    startTime: clock.systemStart,
    eraHistoryFile: input.eraHistoryFile,
    utxoFile: utxoPath,
    protocolParamsFile: input.protocolParamsFile,
    outFile: costOut,
  });
  if (!costArgv.ok) return closed(costArgv.code, { evaluateCode: costArgv.code });
  if (costArgv.argv.includes('evaluate') || !costArgv.argv.includes('calculate-plutus-script-cost') || !costArgv.argv.includes('offline')) {
    return closed('COMMAND_IDENTITY_MISMATCH', { argv: costArgv.argv });
  }
  const cost = spawnSync(costArgv.argv[0], costArgv.argv.slice(1), { encoding: 'utf8' });
  writeFileSync(join(work.path, 'cost.stdout'), cost.stdout || '');
  writeFileSync(join(work.path, 'cost.stderr'), cost.stderr || '');
  writeFileSync(join(work.path, 'cost.exit'), `${cost.status == null ? 'null' : cost.status}\n`);
  const parsed = parseProbeEvaluation({
    stdout: cost.stdout,
    stderr: cost.stderr,
    status: cost.status,
    compiledHash: compiled.compiledHash,
    lucidPosix,
  });
  // Later controls in the same work directory rewrite cost.* and probe.tx.
  // The first evaluator trace is kept under trace-match.* and is not rewritten.
  if (!input.wrongScriptPath && parsed.code === 'TRACE_MATCH' && !existsSync(join(work.path, 'trace-match.stderr'))) {
    writeFileSync(join(work.path, 'trace-match.stderr'), cost.stderr || '');
    writeFileSync(join(work.path, 'trace-match.stdout'), cost.stdout || '');
    writeFileSync(join(work.path, 'trace-match.exit'), `${cost.status == null ? 'null' : cost.status}\n`);
    writeFileSync(join(work.path, 'trace-match.tx'), readFileSync(bodyPath));
    writeFileSync(join(work.path, 'trace-match.utxo.json'), readFileSync(utxoPath));
  }
  const bindings = {
    genesis: { sha256: clock.sha256, path: input.genesisPath, systemStart: clock.systemStart, zeroSlot: String(clock.zeroSlot), slotLengthMs: '1000' },
    artifact: { sha256: sourceSha, path: VALIDATOR, present: true },
    script: { sha256: scriptSha, path: scriptPath, compiledHash: compiled.compiledHash, present: true },
    body: { sha256: bodySha, path: bodyPath, present: true },
    era: { sha256: sha256File(input.eraHistoryFile), path: input.eraHistoryFile },
    protocol: { sha256: sha256File(input.protocolParamsFile), path: input.protocolParamsFile },
    utxo: { sha256: sha256File(utxoPath), path: utxoPath, syntheticOfflineUtxo: true },
    stderr: { sha256: sha256Text(cost.stderr || ''), path: join(work.path, 'cost.stderr') },
  };
  const common = {
    ...parsed,
    tipSlot: String(input.tipSlot),
    tipSource: 'offline-fixture-slot',
    lucidPosix,
    cliPosix: parsed.evaluatorRangeLower,
    evaluateCode: costArgv.code,
    command: costArgv.command,
    argv: costArgv.argv,
    costStatus: cost.status,
    bindings,
    sourceSha256: sourceSha,
    bodySha256: bodySha,
    productionAdmitted: false,
    ledgerExecution: false,
    networkCommandsRun: false,
    nativeUnexecuted: true,
    evidenceClass: 'offline-cli-v3-probe',
    syntheticOfflineUtxo: true,
  };
  if (input.wrongScriptPath) {
    return closed('WRONG_SCRIPT', { ...common, ok: false, scriptContextLower: null, code: 'WRONG_SCRIPT' });
  }
  if (!parsed.ok) return { ...common, ok: false };
  const readbackPath = join(work.path, 'readback.json');
  const transcriptPath = join(work.path, 'transcript.json');
  const readbackBefore = existsSync(readbackPath) ? sha256File(readbackPath) : null;
  const readback = {
    scriptContextLower: parsed.scriptContextLower,
    evaluatorRangeLower: parsed.evaluatorRangeLower,
    evidenceClass: 'offline-cli-v3-probe',
    productionAdmitted: false,
    forged: false,
    unitOnly: false,
    stderrSha256: bindings.stderr.sha256,
    bodySha256: bodySha,
    sourceSha256: sourceSha,
    compiledHash: compiled.compiledHash,
  };
  writeFileSync(readbackPath, `${JSON.stringify(readback)}\n`);
  const readbackSha = sha256File(readbackPath);
  if (readbackBefore != null && readbackBefore === readbackSha) {
    return closed('STALE_READBACK', { ...common, ok: false, scriptContextLower: null, code: 'STALE_READBACK', refusedTrace: parsed.scriptContextLower });
  }
  const transcript = {
    productionAdmitted: false,
    networkCommandsRun: false,
    unitOnly: false,
    evidenceClass: 'offline-cli-v3-probe',
    lucidPosix,
    evaluatorRangeLower: parsed.evaluatorRangeLower,
    scriptContextLower: parsed.scriptContextLower,
    tipSlot: String(input.tipSlot),
    tipSource: 'offline-fixture-slot',
    slotLengthMs: '1000',
    genesisPath: input.genesisPath,
    artifactPath: VALIDATOR,
    bodyPath,
    readbackPath,
    ledgerExecution: false,
    command: costArgv.command,
    costStatus: cost.status,
    intendedFailure: true,
    nativeUnexecuted: true,
  };
  writeFileSync(transcriptPath, `${JSON.stringify(transcript)}\n`);
  bindings.readback = { sha256: readbackSha, path: readbackPath, present: true };
  return {
    ...common,
    ok: true,
    code: 'OFFLINE_LOWER_OBSERVED',
    scriptContextLower: parsed.scriptContextLower,
    probeExecuted: true,
    intendedFailure: true,
    transcriptPath,
    transcriptSha256: sha256File(transcriptPath),
    bindings,
  };
}
