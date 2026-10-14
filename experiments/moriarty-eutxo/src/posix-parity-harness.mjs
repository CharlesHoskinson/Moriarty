/**
 * V3 ScriptContext POSIX parity capture.
 * Unarmed calls prepare the approved query and cost argv and do not spawn.
 * A unit stub may run that argv through an injected spawn. It cannot admit production.
 * Real spawn requires the existing node latch plus two review files. This process
 * does not set the latch.
 */
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { cpSync, existsSync, mkdirSync, readFileSync, realpathSync, writeFileSync } from 'node:fs';
import { dirname, isAbsolute, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { slotToBeginUnixTime } from '../owned/node_modules/@lucid-evolution/plutus/dist/index.js';
import { parseEraHistoryEnvelope, parseProtocolParams, parseTip, slotLengthMsFromShelley } from '../owned/runtime-schemas-71/schemas.mjs';
import { executeOfflineProbe, parseProbeEvaluation } from './offline-probe-eval.mjs';
import { CARDANO_CLI, genesisMagicText } from './native-body.mjs';
import { plutusCostArgv } from './native-producer.mjs';

// Only this module can issue an in-process origin receipt after real execution.
const authenticCaptures = new WeakMap();
export function isAuthenticParityCapture(value) {
  return value != null && authenticCaptures.get(value) === JSON.stringify(value);
}

export const PROBE_SOURCE = `use aiken/interval.{IntervalBoundType}
use aiken/primitive/string
use cardano/transaction.{OutputReference, Transaction}

validator parity_probe {
  spend(_d: Option<Data>, _r: Data, _o: OutputReference, self: Transaction) {
    expect IntervalBoundType.Finite(ms) = self.validity_range.lower_bound.bound_type
    let inclusive = if self.validity_range.lower_bound.is_inclusive { ms } else { ms + 1 }
    fail string.concat(@"MORIARTY_LOWER:", string.from_int(inclusive))
  }
}
`;

const OFFLINE_FIXTURE_TX = new Set([
  'e25450233e4bedd00c8bda15c48c2d4018223bd88271e194052294c4e5be7d55',
  '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef',
]);

function nodeSpawn(argv) {
  return spawnSync(argv[0], argv.slice(1), { encoding: 'utf8' });
}

function realCli(cliPath) {
  try {
    return Boolean(cliPath) && realpathSync(cliPath) === realpathSync(CARDANO_CLI);
  } catch {
    return false;
  }
}

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
    probeNote: 'The probe fails on purpose and prints MORIARTY_LOWER. Equality of the three clocks is not ledger acceptance.',
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

function outRefs(body) {
  if (!body || typeof body !== 'object' || Array.isArray(body)) return [];
  return Object.keys(body).filter((key) => /^[0-9a-f]{64}#[0-9]+$/.test(key));
}

function compileWrittenArtifact(artifactPath, artifactSha) {
  const aiken = '/home/charl/.aiken/bin/aiken';
  if (!existsSync(aiken)) return { ok: false, code: 'COMPILE_FAILED', reason: 'aiken-absent' };
  const project = fileURLToPath(new URL('../probe-parity', import.meta.url));
  const work = join(dirname(artifactPath), 'compiled-artifact');
  try {
    mkdirSync(join(work, 'validators'), { recursive: true });
    mkdirSync(join(work, 'build'), { recursive: true });
    cpSync(join(project, 'aiken.toml'), join(work, 'aiken.toml'));
    cpSync(join(project, 'aiken.lock'), join(work, 'aiken.lock'));
    if (existsSync(join(project, 'build', 'packages'))) {
      cpSync(join(project, 'build', 'packages'), join(work, 'build', 'packages'), { recursive: true });
    }
    const validatorPath = join(work, 'validators', 'parity_probe.ak');
    writeFileSync(validatorPath, readFileSync(artifactPath));
    if (fileSha(validatorPath) !== artifactSha) return { ok: false, code: 'ARTIFACT_BINDING_MISMATCH' };
  } catch {
    return { ok: false, code: 'COMPILE_FAILED', reason: 'project-copy' };
  }
  const run = spawnSync(aiken, ['build', '-t', 'verbose'], { cwd: work, encoding: 'utf8' });
  if (run.status !== 0) return { ok: false, code: 'COMPILE_FAILED', reason: 'aiken-status', aikenStatus: run.status };
  const blueprintPath = join(work, 'plutus.json');
  if (!existsSync(blueprintPath)) return { ok: false, code: 'COMPILE_FAILED', reason: 'blueprint-absent' };
  let blueprint;
  try {
    blueprint = JSON.parse(readFileSync(blueprintPath, 'utf8'));
  } catch {
    return { ok: false, code: 'COMPILE_FAILED', reason: 'blueprint-json' };
  }
  const validator = (blueprint.validators || []).find((item) => item.title === 'parity_probe.parity_probe.spend');
  if (!validator || typeof validator.compiledCode !== 'string' || typeof validator.hash !== 'string') {
    return { ok: false, code: 'COMPILE_FAILED', reason: 'spend-validator-absent' };
  }
  return { ok: true, hash: validator.hash, compiledCode: validator.compiledCode, compiledBy: 'aiken-written-artifact' };
}

function acceptCompile(compiled, artifactSha) {
  if (!compiled || compiled.ok === false) return { ok: false, code: compiled?.code || 'COMPILE_FAILED' };
  if (typeof compiled.hash !== 'string' || !/^[0-9a-f]{56}$/.test(compiled.hash)) {
    return { ok: false, code: 'COMPILE_FAILED' };
  }
  if (compiled.sourceSha256 && compiled.sourceSha256 !== artifactSha) {
    return { ok: false, code: 'ARTIFACT_BINDING_MISMATCH' };
  }
  if (compiled.compiledCode != null) {
    const code = compiled.compiledCode;
    if (typeof code !== 'string' || code.length === 0 || code.length % 2 !== 0 || !/^[0-9a-fA-F]+$/.test(code)) {
      return { ok: false, code: 'COMPILE_FAILED' };
    }
  }
  return { ok: true, hash: compiled.hash, compiledCode: compiled.compiledCode || null };
}

export function observeQueriedNode(input, spawnImpl, { realNodeSpawn = false } = {}) {
  if (input == null || typeof input !== 'object') return closed('MALFORMED_DATA');
  if (typeof spawnImpl !== 'function') return closed('TIME_QUERY_ABSENT', { reason: 'spawn-absent' });
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
  const realSpawn = realNodeSpawn === true && spawnImpl === nodeSpawn && realCli(input.cliPath);
  const evidenceClass = realSpawn ? 'node-query-refused' : 'unit-command-sequence-not-a-node';
  const flags = {
    armed: true,
    evidenceClass,
    productionAdmitted: false,
    ledgerExecution: false,
    networkCommandsRun: false,
    nativeUnexecuted: true,
    syntheticOfflineUtxo: false,
    tipSource: 'query-tip',
    slotLengthMs: '1000',
    artifactSha256: artifactSha,
  };
  const magic = genesisMagicText(input.magic ?? input.networkMagic ?? genesis.body.networkMagic);
  if (!magic.ok) return closed(magic.code, flags);
  const cli = input.cliPath;
  const socket = input.socketPath;
  if (typeof cli !== 'string' || cli.length === 0 || typeof socket !== 'string' || socket.length === 0) {
    return closed('TIME_QUERY_ABSENT', flags);
  }
  const work = dirname(bound.artifactPath);
  const utxoPath = join(work, 'queried-utxo.json');
  const protocolPath = join(work, 'queried-protocol.json');
  const eraPath = join(work, 'queried-era.json');
  const tipPath = join(work, 'queried-tip.json');
  const calls = [];
  const spawn = (argv) => {
    calls.push(argv);
    return spawnImpl(argv) || {};
  };
  const utxoRun = spawn([cli, 'query', 'utxo', '--socket-path', socket, '--testnet-magic', magic.text, '--output-json']);
  const utxoStdout = utxoRun.stdout || '';
  if ([...OFFLINE_FIXTURE_TX].some((id) => utxoStdout.includes(id))) {
    return closed('OFFLINE_FIXTURE_UTXO_REFUSED', { ...flags, calls });
  }
  let utxoJson = null;
  try {
    utxoJson = JSON.parse(utxoStdout);
  } catch {
    utxoJson = null;
  }
  const refs = outRefs(utxoJson);
  if (refs.length < 2) {
    return closed('QUERY_UTXO_INCOMPLETE', { ...flags, calls, keyCount: refs.length });
  }
  writeFileSync(utxoPath, utxoStdout);
  const protocolRun = spawn([
    cli, 'query', 'protocol-parameters', '--socket-path', socket, '--testnet-magic', magic.text, '--out-file', protocolPath,
  ]);
  if (!existsSync(protocolPath) && protocolRun.stdout) writeFileSync(protocolPath, protocolRun.stdout);
  if (!existsSync(protocolPath)) return closed('QUERY_PROTOCOL_ABSENT', { ...flags, calls });
  let protocolJson;
  try {
    protocolJson = JSON.parse(readFileSync(protocolPath, 'utf8'));
  } catch {
    return closed('QUERY_PROTOCOL_ABSENT', { ...flags, calls });
  }
  const protocol = parseProtocolParams(protocolJson);
  if (!protocol.ok) return closed(protocol.code || 'QUERY_PROTOCOL_ABSENT', { ...flags, calls });
  const tipRun = spawn([cli, 'query', 'tip', '--socket-path', socket, '--testnet-magic', magic.text, '--output-json']);
  writeFileSync(tipPath, tipRun.stdout || '');
  let tipJson;
  try {
    tipJson = JSON.parse(tipRun.stdout || '');
  } catch {
    return closed('TIP_OBJECT_REQUIRED', { ...flags, calls });
  }
  const tip = parseTip(tipJson);
  if (!tip.ok || tip.slot == null) return closed(tip.ok ? 'GENESIS_TIP_HAS_NO_SLOT' : tip.code, { ...flags, calls });
  const eraRun = spawn([
    cli, 'query', 'era-history', '--socket-path', socket, '--testnet-magic', magic.text, '--out-file', eraPath,
  ]);
  if (!existsSync(eraPath) && eraRun.stdout) writeFileSync(eraPath, eraRun.stdout);
  if (!existsSync(eraPath)) return closed('QUERY_ERA_ABSENT', { ...flags, calls });
  let eraJson;
  try {
    eraJson = JSON.parse(readFileSync(eraPath, 'utf8'));
  } catch {
    return closed('QUERY_ERA_ABSENT', { ...flags, calls });
  }
  const era = parseEraHistoryEnvelope(eraJson);
  if (!era.ok) return closed(era.code || 'QUERY_ERA_ABSENT', { ...flags, calls });
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
    return closed('IMPOSSIBLE_BOUNDARY', { ...flags, calls, reason: 'posix-math' });
  }
  if (!decimalString(cliPosix) || !decimalString(lucidPosix)) {
    return closed('IMPOSSIBLE_BOUNDARY', { ...flags, calls, cliPosix, lucidPosix, reason: 'non-decimal' });
  }
  const compiledInput = typeof input.compile === 'function'
    ? input.compile({ sourcePath: bound.artifactPath, sourceSha256: artifactSha })
    : compileWrittenArtifact(bound.artifactPath, artifactSha);
  const compiled = acceptCompile(compiledInput, artifactSha);
  if (!compiled.ok) return closed(compiled.code, { ...flags, calls, cliPosix, lucidPosix });
  let scriptPath = null;
  if (compiled.compiledCode) {
    scriptPath = join(work, 'probe-script.json');
    writeFileSync(scriptPath, `${JSON.stringify({
      type: 'PlutusScriptV3',
      description: 'queried-node parity probe',
      cborHex: compiled.compiledCode,
    })}\n`);
  }
  let scriptAddress = null;
  if (scriptPath) {
    const addressRun = spawn([cli, 'address', 'build', '--payment-script-file', scriptPath, '--testnet-magic', magic.text]);
    scriptAddress = String(addressRun.stdout || '').trim();
    if (addressRun.status !== 0 || !scriptAddress.startsWith('addr')) {
      return closed('CLI_ADDRESS_FAILED', { ...flags, calls, cliPosix, lucidPosix });
    }
  }
  const buildArgv = [cli, 'conway', 'transaction', 'build-raw', '--tx-in', refs[0]];
  if (scriptPath) {
    buildArgv.push(
      '--tx-in-script-file', scriptPath,
      '--tx-in-redeemer-value', '42',
      '--tx-in-execution-units', '(14000000,10000000000)',
    );
  }
  buildArgv.push('--tx-in-collateral', refs[1]);
  if (scriptAddress) buildArgv.push('--tx-out', `${scriptAddress}+3000000`);
  buildArgv.push(
    '--fee', scriptAddress ? '200000' : '0',
    '--invalid-before', String(tip.slot),
    '--invalid-hereafter', (BigInt(tip.slot) + 1n).toString(),
    '--protocol-params-file', protocolPath,
    '--out-file', bound.bodyPath,
  );
  const buildRun = spawn(buildArgv);
  if (!existsSync(bound.bodyPath) && buildRun.stdout) writeFileSync(bound.bodyPath, buildRun.stdout);
  if (!existsSync(bound.bodyPath)) return closed('BODY_BINDING_ABSENT', { ...flags, calls, cliPosix, lucidPosix });
  const costArgv = plutusCostArgv({
    cli,
    mode: 'online',
    txFile: bound.bodyPath,
    magic: Number(magic.text),
    magicSource: 'shelley-networkMagic',
    socket,
  });
  if (!costArgv.ok) return closed(costArgv.code, { ...flags, calls, cliPosix, lucidPosix, evaluateCode: costArgv.code });
  if (costArgv.argv.includes('offline') || costArgv.argv.includes('evaluate') || !costArgv.argv.includes('online')) {
    return closed('COMMAND_IDENTITY_MISMATCH', { ...flags, calls, cliPosix, lucidPosix });
  }
  const costRun = spawn(costArgv.argv);
  const parsed = parseProbeEvaluation({
    stdout: costRun.stdout,
    stderr: costRun.stderr,
    status: costRun.status,
    compiledHash: compiled.hash,
    lucidPosix,
  });
  const clocksEqual = parsed.ok === true && parsed.scriptContextLower === cliPosix && cliPosix === lucidPosix;
  const bodySha = fileSha(bound.bodyPath);
  const bindings = {
    genesis: { sha256: genesis.sha256, path: input.genesisPath, systemStart: genesis.systemStart, zeroSlot: String(genesis.zeroSlot), slotLengthMs: '1000' },
    artifact: { sha256: artifactSha, path: bound.artifactPath, present: true },
    compiled: { hash: compiled.hash, sourceSha256: artifactSha, scriptPath },
    body: { sha256: bodySha, path: bound.bodyPath, present: true },
    utxo: { sha256: fileSha(utxoPath), path: utxoPath, syntheticOfflineUtxo: false, keyCount: refs.length },
    protocol: { sha256: fileSha(protocolPath), path: protocolPath },
    era: { sha256: fileSha(eraPath), path: eraPath },
    tip: { sha256: fileSha(tipPath), path: tipPath, slot: String(tip.slot) },
  };
  const admitted = realSpawn && clocksEqual && parsed.probeExecuted === true;
  const readback = {
    scriptContextLower: parsed.scriptContextLower || null,
    productionAdmitted: false,
    forged: false,
    unitOnly: !realSpawn,
    compiledHash: compiled.hash,
    artifactSha256: artifactSha,
  };
  try {
    writeFileSync(bound.readbackPath, `${JSON.stringify(readback)}\n`);
    bindings.readback = { sha256: fileSha(bound.readbackPath), path: bound.readbackPath, present: true };
    const commandLogPath = join(work, 'probe-command-log.txt');
    const commandLog = [`genesis ${input.genesisPath}`, ...calls.map((argv) => argv.join(' '))].join('\n');
    writeFileSync(commandLogPath, `${commandLog}\n`);
    const queriedRef = String(refs[0] || '');
    const queriedParts = queriedRef.split('#');
    const queriedSeed = /^[0-9a-f]{64}$/.test(queriedParts[0]) && Number.isInteger(Number(queriedParts[1]))
      ? { txId: queriedParts[0], index: Number(queriedParts[1]) }
      : null;
    const transcript = {
      productionAdmitted: admitted,
      evidenceClass: admitted ? 'node-genesis-v3-probe' : evidenceClass,
      cliPosix,
      lucidPosix,
      scriptContextLower: parsed.scriptContextLower || null,
      tipSlot: String(tip.slot),
      tipSource: 'query-tip',
      slotLengthMs: '1000',
      syntheticOfflineUtxo: false,
      ledgerExecution: false,
      networkCommandsRun: admitted,
      nativeUnexecuted: !admitted,
      genesisPath: input.genesisPath,
      artifactPath: bound.artifactPath,
      bodyPath: bound.bodyPath,
      readbackPath: bound.readbackPath,
      originTool: admitted ? 'executeParityCapture' : null,
      commandLogPath: admitted ? commandLogPath : null,
      commandLogSha256: admitted ? fileSha(commandLogPath) : null,
      queriedSeed: admitted ? queriedSeed : null,
      bindings,
    };
    writeFileSync(bound.transcriptPath, `${JSON.stringify(transcript)}\n`);
  } catch {
    return closed('PARITY_TRANSCRIPT_ABSENT', { ...flags, calls, cliPosix, lucidPosix });
  }
  const transcriptSha256 = fileSha(bound.transcriptPath);
  const common = {
    ...flags,
    calls,
    cliPosix,
    lucidPosix,
    tipSlot: String(tip.slot),
    scriptContextLower: parsed.scriptContextLower || null,
    probeExecuted: parsed.probeExecuted === true,
    evaluateCode: costArgv.code,
    command: costArgv.command,
    bindings,
    transcriptPath: bound.transcriptPath,
    transcriptSha256,
    parsedCode: parsed.code,
  };
  if (admitted) {
    return {
      ...common,
      ok: true,
      code: 'NODE_QUERY_ADMITTED',
      productionAdmitted: true,
      evidenceClass: 'node-genesis-v3-probe',
      networkCommandsRun: true,
      nativeUnexecuted: false,
      origin: 'executeParityCapture',
      queriedSeed: JSON.parse(readFileSync(bound.transcriptPath, 'utf8')).queriedSeed,
      zeroTime: String(genesis.systemStartMs),
      zeroSlot: String(genesis.zeroSlot),
    };
  }
  return closed(parsed.ok ? 'UNIT_NODE_SEQUENCE_PARSED' : parsed.code, common);
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
  if (injected !== true) {
    return observeQueriedNode(input, spawnImpl, { realNodeSpawn: spawnImpl === nodeSpawn });
  }
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
  const evaluate = injected === true
    ? plutusCostArgv({ command: 'evaluate', cli: input.cliPath, txFile: bound.bodyPath })
    : null;
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
    evaluateCode: injected === true ? (evaluate?.code || 'NO_CLI_EVALUATE_COMMAND') : 'EVALUATION_CONTEXT_ABSENT',
    bindings: {
      genesis: { sha256: genesis.sha256, systemStart: genesis.systemStart, zeroSlot: String(genesis.zeroSlot), slotLengthMs: '1000', path: input.genesisPath },
      artifact: { sha256: artifactSha, path: bound.artifactPath, present: true },
    },
  };
  if (!existsSync(bound.bodyPath)) return { ...base, ok: false, code: 'BODY_BINDING_ABSENT' };
  base.bindings.body = { sha256: fileSha(bound.bodyPath), path: bound.bodyPath, present: true };
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
  if (input.offlineContext === true) {
    if (input.unitOnly === true || input.evidenceClass === 'UNIT' || input.evidenceClass === 'UNIT/SYNTHETIC') {
      return closed('UNIT_PARITY_REFUSED', { armed: true, probeExecuted: false });
    }
    return executeOfflineProbe(input);
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
  if (typeof input.compile === 'function') {
    return closed('COMPILE_CALLBACK_NOT_PRODUCTION', { armed: true, reason: 'real capture requires the pinned written-artifact compiler' });
  }
  if (process.env.MORIARTY_ALLOW_REAL_NODE !== '1') return closed('REAL_QUERY_NOT_ARMED', { armed: true });
  if (!input.auditA || !input.auditB || !existsSync(input.auditA) || !existsSync(input.auditB)) {
    return closed('REAL_QUERY_NOT_ARMED', { armed: true, reason: 'two FULL audit files are not attached' });
  }
  const injected = typeof input.spawn === 'function';
  const spawnImpl = injected ? input.spawn : nodeSpawn;
  const result = runNodeObservation(input, spawnImpl, injected);
  if (!injected && spawnImpl === nodeSpawn && realCli(input.cliPath)
      && result.productionAdmitted === true && result.probeExecuted === true) {
    authenticCaptures.set(result, JSON.stringify(result));
  }
  return result;
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
