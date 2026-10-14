/**
 * V3 ScriptContext POSIX parity capture.
 * Unarmed calls prepare the approved query and cost argv and do not spawn.
 * A unit stub may run that argv through an injected spawn. It cannot admit production.
 * Real spawn requires the existing node latch plus two review files. This process
 * does not set the latch.
 */
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, isAbsolute } from 'node:path';
import { pathToFileURL } from 'node:url';
import { slotToBeginUnixTime } from '/home/charl/moriarty-eutxo-validation-2026-10-12/dependency203-verified-freeze206/packet199/snapshot/node_modules/@lucid-evolution/plutus/dist/index.js';
import { parseTip, slotLengthMsFromShelley } from '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-71/own71/schemas.mjs';
import { genesisMagicText } from './native-body.mjs';
import { plutusCostArgv } from './native-producer.mjs';

export const PROBE_SOURCE = `use aiken/interval.{lower_inclusive}
use cardano/transaction.{Transaction}

validator parity_probe {
  spend(_d: Option<Data>, _r: Data, _o: OutputReference, self: Transaction) {
    expect Some(ms) = lower_inclusive(self.validity_range)
    ms == ms
  }
}
`;

function closed(code, extra = {}) {
  return {
    ok: false,
    code,
    productionAdmitted: false,
    scriptContextLower: null,
    cliPosix: null,
    lucidPosix: null,
    tipSlot: null,
    ledgerExecution: false,
    networkCommandsRun: false,
    siblingEmitted: false,
    sourceEmitted: false,
    productionArtifact: false,
    armed: extra.armed === true,
    ...extra,
    productionAdmitted: false,
  };
}

function fileSha(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

export function prepareParityCommands(input = {}) {
  const magic = genesisMagicText(input.magic ?? input.networkMagic ?? null);
  if (!magic.ok) return { ...closed(magic.code), magicBound: false };
  const cli = input.cliPath;
  const socket = input.socketPath;
  if (typeof cli !== 'string' || cli.length === 0 || typeof socket !== 'string' || socket.length === 0) {
    return closed('TIME_QUERY_ABSENT');
  }
  const queryTip = [cli, 'query', 'tip', '--socket-path', socket, '--testnet-magic', magic.text, '--output-json'];
  const queryEra = [cli, 'query', 'era-history', '--socket-path', socket, '--testnet-magic', magic.text];
  const tipSlot = input.tipSlot == null ? null : String(input.tipSlot);
  const hereafter = tipSlot == null ? null : String(BigInt(tipSlot) + 1n);
  const bodyPath = input.bodyPath || '/tmp/m228/probe.body';
  const cost = tipSlot == null ? null : plutusCostArgv({
    cli,
    command: 'cost',
    mode: 'offline',
    txFile: bodyPath,
    startTime: input.systemStart || null,
    eraHistoryFile: input.eraHistoryFile || '/tmp/m228/era.json',
    utxoFile: input.utxoFile || '/tmp/m228/utxo.json',
    protocolParamsFile: input.protocolParamsFile || '/tmp/m228/protocol.json',
    outFile: input.costOut || '/tmp/m228/cost.json',
  });
  return {
    ok: true,
    code: 'PARITY_COMMANDS_PREPARED',
    networkCommandsRun: false,
    ledgerExecution: false,
    productionAdmitted: false,
    magic: magic.text,
    commands: {
      queryTip,
      queryEra,
      probeBody: [
        cli, 'conway', 'transaction', 'build-raw',
        '--invalid-before', tipSlot || 'TIP_SLOT',
        '--invalid-hereafter', hereafter || 'TIP_SLOT_PLUS_ONE',
        '--fee', '0',
        '--out-file', bodyPath,
      ],
      plutusCost: cost && cost.ok ? cost.argv : null,
      plutusCostCode: cost && !cost.ok ? cost.code : null,
    },
    probeSource: PROBE_SOURCE,
    probeNote: 'Aiken probe equality is offline UPLC correspondence. It is not ledger acceptance.',
  };
}

function readGenesis(path) {
  if (!path || !existsSync(path)) return closed('TIME_QUERY_ABSENT', { reason: 'genesis-absent' });
  let body;
  try {
    body = JSON.parse(readFileSync(path, 'utf8'));
  } catch {
    return closed('MALFORMED_DATA', { reason: 'genesis-json' });
  }
  const slot = slotLengthMsFromShelley(body.slotLength);
  if (!slot.ok) return closed(slot.code || 'SLOT_LENGTH_REFUSED');
  if (slot.slotLengthMs !== 1000) return closed('SLOT_LENGTH_REFUSED', { observed: String(slot.slotLengthMs) });
  if (typeof body.systemStart !== 'string' || Number.isNaN(Date.parse(body.systemStart))) {
    return closed('PARITY_NOT_MEASURED', { reason: 'system-start' });
  }
  return {
    ok: true,
    body,
    systemStart: body.systemStart,
    systemStartMs: Date.parse(body.systemStart),
    zeroSlot: body.zeroSlot == null ? 0 : Number(body.zeroSlot),
    sha256: fileSha(path),
  };
}

function runUnitSequence(input) {
  const genesis = readGenesis(input.genesisPath);
  if (!genesis.ok) return { ...genesis, armed: true, evidenceClass: 'UNIT' };
  if (typeof input.spawn !== 'function') return closed('TIME_QUERY_ABSENT', { armed: true, reason: 'unit-spawn-absent', evidenceClass: 'UNIT' });
  const prepared = prepareParityCommands({
    ...input,
    systemStart: genesis.systemStart,
    tipSlot: input.tipSlot,
  });
  if (!prepared.ok) return { ...prepared, armed: true, evidenceClass: 'UNIT' };
  const calls = [];
  const spawn = (argv) => {
    calls.push(argv);
    return input.spawn(argv);
  };
  const tipRun = spawn(prepared.commands.queryTip);
  const eraRun = spawn(prepared.commands.queryEra);
  let tipJson;
  try {
    tipJson = JSON.parse(tipRun?.stdout || '');
  } catch {
    return closed('TIP_OBJECT_REQUIRED', { armed: true, evidenceClass: 'UNIT', preparedCommands: prepared.commands, calls });
  }
  const tip = parseTip(tipJson);
  if (!tip.ok || tip.slot == null) {
    return closed(tip.ok ? 'GENESIS_TIP_HAS_NO_SLOT' : tip.code, { armed: true, evidenceClass: 'UNIT', calls });
  }
  const cliPosix = String(genesis.systemStartMs + (tip.slot - genesis.zeroSlot) * 1000);
  const lucidPosix = String(slotToBeginUnixTime(tip.slot, {
    zeroTime: genesis.systemStartMs,
    zeroSlot: genesis.zeroSlot,
    slotLength: 1000,
  }));
  const probe = spawn(prepared.commands.probeBody);
  const text = `${probe?.stdout || ''}\n${tipRun?.stdout || ''}`;
  const found = text.match(/scriptContextLower["']?\s*[:=]\s*"?(\d+)/);
  const scriptContextLower = found ? found[1] : null;
  const matched = scriptContextLower != null && scriptContextLower === cliPosix && cliPosix === lucidPosix;
  return {
    ok: matched,
    code: matched ? 'UNIT_PARITY_OBSERVED' : 'PARITY_NOT_MEASURED',
    armed: true,
    evidenceClass: 'UNIT',
    unitOnly: true,
    productionAdmitted: false,
    siblingEmitted: false,
    sourceEmitted: false,
    productionArtifact: false,
    ledgerExecution: false,
    networkCommandsRun: false,
    cliPosix,
    lucidPosix,
    scriptContextLower,
    tipSlot: String(tip.slot),
    preparedCommands: prepared.commands,
    calls,
    eraStatus: eraRun?.status ?? null,
    bindings: {
      genesis: { sha256: genesis.sha256, systemStart: genesis.systemStart, zeroSlot: String(genesis.zeroSlot), slotLengthMs: '1000' },
    },
    probeSource: PROBE_SOURCE,
  };
}

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

function bindingPath(path) {
  if (typeof path !== 'string' || !isAbsolute(path)) return closed('BINDING_PATH_ABSENT');
  const hit = HISTORICAL_SEGMENTS.find((name) => path.split('/').includes(name));
  if (hit) return closed('OUTPUT_PATH_FORBIDDEN', { path, historical: hit });
  return { ok: true, path };
}

function decimalString(value) {
  return typeof value === 'string' && /^[0-9]+$/.test(value);
}

function runNodeObservation(input, spawnImpl, injected) {
  const bound = {};
  for (const key of ['bodyPath', 'artifactPath', 'readbackPath', 'transcriptPath']) {
    const checked = bindingPath(input[key]);
    if (!checked.ok) return { ...checked, armed: true, networkCommandsRun: false, evidenceClass: 'observation-refused' };
    if (!existsSync(dirname(checked.path))) return closed('BINDING_PATH_ABSENT', { armed: true, reason: 'parent-absent', field: key });
    bound[key] = checked.path;
  }
  const genesis = readGenesis(input.genesisPath);
  if (!genesis.ok) return { ...genesis, armed: true, networkCommandsRun: false };
  let artifactSha;
  try {
    writeFileSync(bound.artifactPath, PROBE_SOURCE);
    artifactSha = fileSha(bound.artifactPath);
  } catch {
    return closed('ARTIFACT_BINDING_MISMATCH', { armed: true, networkCommandsRun: false });
  }
  const prepared = prepareParityCommands({
    cliPath: input.cliPath,
    socketPath: input.socketPath,
    magic: input.magic ?? input.networkMagic,
    systemStart: genesis.systemStart,
  });
  if (!prepared.ok) return { ...prepared, armed: true, networkCommandsRun: false };
  const calls = [];
  const spawn = (argv) => {
    calls.push(argv);
    return spawnImpl(argv);
  };
  const tipRun = spawn(prepared.commands.queryTip);
  let tipJson;
  try {
    tipJson = JSON.parse(tipRun?.stdout || '');
  } catch {
    return closed('TIP_OBJECT_REQUIRED', { armed: true, calls, networkCommandsRun: false });
  }
  const tip = parseTip(tipJson);
  if (!tip.ok || tip.slot == null) {
    return closed(tip.ok ? 'GENESIS_TIP_HAS_NO_SLOT' : tip.code, { armed: true, calls, networkCommandsRun: false });
  }
  let cliPosix;
  let lucidPosix;
  try {
    cliPosix = (BigInt(genesis.systemStartMs) + (BigInt(tip.slot) - BigInt(genesis.zeroSlot)) * 1000n).toString();
    lucidPosix = String(slotToBeginUnixTime(tip.slot, {
      zeroTime: genesis.systemStartMs,
      zeroSlot: genesis.zeroSlot,
      slotLength: 1000,
    }));
  } catch {
    return closed('IMPOSSIBLE_BOUNDARY', { armed: true, calls, reason: 'posix-math' });
  }
  if (!decimalString(cliPosix) || !decimalString(lucidPosix)) {
    return closed('IMPOSSIBLE_BOUNDARY', { armed: true, calls, cliPosix, lucidPosix, reason: 'non-decimal' });
  }
  spawn(prepared.commands.queryEra);
  spawn([
    input.cliPath, 'conway', 'transaction', 'build-raw',
    '--invalid-before', String(tip.slot),
    '--invalid-hereafter', (BigInt(tip.slot) + 1n).toString(),
    '--fee', '0',
    '--out-file', bound.bodyPath,
  ]);
  const readbackBefore = existsSync(bound.readbackPath) ? fileSha(bound.readbackPath) : null;
  const evaluate = plutusCostArgv({ command: 'evaluate', cli: input.cliPath, txFile: bound.bodyPath });
  const base = {
    armed: true,
    unitOnly: false,
    productionAdmitted: false,
    siblingEmitted: false,
    sourceEmitted: false,
    productionArtifact: false,
    ledgerExecution: false,
    networkCommandsRun: injected !== true,
    evidenceClass: injected === true ? 'injected-observation' : 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    cliPosix,
    lucidPosix,
    tipSlot: String(tip.slot),
    calls,
    probeExecuted: false,
    scriptContextLower: null,
    ignoredCallerTip: input.tipSlot == null ? null : String(input.tipSlot),
    ignoredCallerLower: input.scriptContextLower == null ? null : String(input.scriptContextLower),
    evaluateCode: evaluate?.code || 'NO_CLI_EVALUATE_COMMAND',
    bindings: {
      genesis: { sha256: genesis.sha256, systemStart: genesis.systemStart, zeroSlot: String(genesis.zeroSlot), slotLengthMs: '1000', path: input.genesisPath },
      artifact: { sha256: artifactSha, path: bound.artifactPath, present: true },
    },
  };
  if (!existsSync(bound.bodyPath)) return { ...base, ok: false, code: 'BODY_BINDING_ABSENT' };
  base.bindings.body = { sha256: fileSha(bound.bodyPath), path: bound.bodyPath, present: true };
  if (injected !== true) {
    return {
      ...base,
      ok: false,
      code: 'NO_CLI_EVALUATE_COMMAND',
      scriptContextLower: null,
      probeExecuted: false,
      reason: 'cardano-cli has no evaluate command. A file or a caller field is not a V3 ScriptContext lower.',
    };
  }
  spawn(['probe-context', bound.readbackPath]);
  if (!existsSync(bound.readbackPath)) return { ...base, ok: false, code: 'READBACK_BINDING_ABSENT' };
  const readbackSha = fileSha(bound.readbackPath);
  if (readbackBefore != null && readbackBefore === readbackSha) {
    return { ...base, ok: false, code: 'STALE_READBACK', scriptContextLower: null };
  }
  let readback;
  try {
    readback = JSON.parse(readFileSync(bound.readbackPath, 'utf8'));
  } catch {
    return { ...base, ok: false, code: 'READBACK_BINDING_MISMATCH' };
  }
  if (readback?.productionAdmitted === true || readback?.forged === true || readback?.unitOnly === true) {
    return { ...base, ok: false, code: 'FORGED_PARITY', scriptContextLower: null };
  }
  const scriptContextLower = readback?.scriptContextLower;
  if (!decimalString(scriptContextLower)) return { ...base, ok: false, code: 'READBACK_BINDING_MISMATCH', scriptContextLower: null };
  base.scriptContextLower = scriptContextLower;
  base.bindings.readback = { sha256: readbackSha, path: bound.readbackPath, present: true };
  base.probeExecuted = false;
  if (scriptContextLower !== cliPosix || cliPosix !== lucidPosix) {
    return { ...base, ok: false, code: 'IMPOSSIBLE_BOUNDARY' };
  }
  const transcript = {
    productionAdmitted: false,
    networkCommandsRun: base.networkCommandsRun,
    unitOnly: false,
    evidenceClass: base.evidenceClass,
    cliPosix,
    lucidPosix,
    scriptContextLower,
    tipSlot: String(tip.slot),
    slotLengthMs: '1000',
    genesisPath: input.genesisPath,
    artifactPath: bound.artifactPath,
    bodyPath: bound.bodyPath,
    readbackPath: bound.readbackPath,
    ledgerExecution: false,
    siblingEmitted: false,
    sourceEmitted: false,
    productionArtifact: false,
  };
  try {
    writeFileSync(bound.transcriptPath, `${JSON.stringify(transcript)}\n`);
  } catch {
    return { ...base, ok: false, code: 'PARITY_TRANSCRIPT_ABSENT' };
  }
  return {
    ...base,
    ok: false,
    code: 'INJECTED_CONTEXT_PARSED',
    evidenceClass: 'injected-observation',
    networkCommandsRun: false,
    productionAdmitted: false,
    probeExecuted: false,
    transcriptPath: bound.transcriptPath,
    transcriptSha256: fileSha(bound.transcriptPath),
    reason: 'Injected probe-context bytes are a parser control. They are not a node V3 ScriptContext.',
  };
}

export function executeParityCapture(input = {}) {
  if (input == null || typeof input !== 'object') return closed('MALFORMED_DATA');
  if (input.armed !== true) {
    const prepared = prepareParityCommands({
      cliPath: input.cliPath || 'cardano-cli',
      socketPath: input.socketPath || '/tmp/m228/node.sock',
      magic: input.magic ?? input.networkMagic ?? 764,
      tipSlot: input.tipSlot,
      systemStart: input.systemStart,
    });
    return closed('PARITY_NOT_EXECUTED', {
      armed: false,
      evidenceClass: 'harness-refused-not-a-probe',
      preparedCommands: prepared.ok ? prepared.commands : null,
      prepareCode: prepared.ok ? prepared.code : prepared.code,
      reason: 'Unarmed capture does not query a node and does not read a ScriptContext.',
    });
  }
  if (input.forged === true || input.productionAdmitted === true) return closed('FORGED_PARITY', { armed: true });
  if (input.slotLengthMs != null && String(input.slotLengthMs) !== '1000') {
    return closed('SLOT_LENGTH_REFUSED', { armed: true, observed: String(input.slotLengthMs) });
  }
  const unit = input.unitOnly === true || input.evidenceClass === 'UNIT' || input.evidenceClass === 'UNIT/SYNTHETIC';
  if (!unit) {
    if (!input.reviewA || !input.reviewB || !existsSync(input.reviewA) || !existsSync(input.reviewB)) {
      return closed('REVIEW_ABSENT', { armed: true });
    }
  }
  if (!input.cliPath || !existsSync(input.cliPath) || !input.socketPath || !existsSync(input.socketPath)) {
    return closed('TIME_QUERY_ABSENT', { armed: true });
  }
  if (unit) return runUnitSequence(input);
  if (process.env.MORIARTY_ALLOW_REAL_NODE !== '1') return closed('REAL_QUERY_NOT_ARMED', { armed: true });
  if (!input.auditA || !input.auditB || !existsSync(input.auditA) || !existsSync(input.auditB)) {
    return closed('REAL_QUERY_NOT_ARMED', { armed: true, reason: 'two FULL audit files are not attached' });
  }
  const injected = typeof input.spawn === 'function';
  const spawnImpl = injected
    ? input.spawn
    : (argv) => spawnSync(argv[0], argv.slice(1), { encoding: 'utf8' });
  return runNodeObservation(input, spawnImpl, injected);
}

function runCli(argv) {
  if (!argv.includes('--armed')) {
    const prepared = prepareParityCommands({
      cliPath: 'cardano-cli',
      socketPath: '/tmp/m228/node.sock',
      magic: 764,
    });
    return {
      code: 'PARITY_NOT_EXECUTED',
      productionAdmitted: false,
      scriptContextLower: null,
      cliPosix: null,
      lucidPosix: null,
      tipSlot: null,
      ledgerExecution: false,
      networkCommandsRun: false,
      evidenceClass: 'harness-refused-not-a-probe',
      unitOnly: false,
      synthetic: false,
      sourceEmitted: false,
      siblingEmitted: false,
      productionArtifact: false,
      armed: false,
      genesis: { systemStart: null, zeroSlot: null, slotLengthMs: null, path: null, sha256: null },
      artifact: null,
      body: null,
      readback: null,
      proposedRunRoot: '/tmp/m228',
      preparedCommands: prepared.ok ? prepared.commands : null,
      proposedCommands: prepared.ok ? [
        prepared.commands.queryTip.join(' '),
        prepared.commands.queryEra.join(' '),
        prepared.commands.probeBody.join(' '),
      ] : [],
      reason: 'No genesis file, tip query, or V3 ScriptContext was read. MORIARTY_ALLOW_REAL_NODE is not a completed probe.',
    };
  }
  return executeParityCapture({ armed: true, cliPath: argv[argv.indexOf('--cli') + 1] || '' });
}

const entry = process.argv[1] ? pathToFileURL(process.argv[1]).href : '';
if (import.meta.url === entry) {
  const transcript = runCli(process.argv.slice(2));
  process.stdout.write(`${JSON.stringify(transcript)}\n`);
  process.exitCode = transcript.code === 'PARITY_NOT_EXECUTED' && transcript.armed !== true ? 2 : 1;
}
