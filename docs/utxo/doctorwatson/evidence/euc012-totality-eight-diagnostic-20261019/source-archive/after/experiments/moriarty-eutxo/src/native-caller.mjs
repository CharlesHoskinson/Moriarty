/**
 * Integrated T1 native caller.
 * Offline inspect does not exec a node server.
 * Authority files that hash-agree can spawn an owned process group.
 * A unit fixture stays unit-only and does not set nativePins.
 * The real node and the real cardano-cli query/key-gen paths stay unarmed
 * unless MORIARTY_ALLOW_REAL_NODE=1. That latch is not evidence of a run.
 * Flags come from produced artifacts. A stub cannot set ledgerSubmitted.
 */
import { spawnSync } from 'node:child_process';
import { createHash, randomBytes } from 'node:crypto';
import { chmodSync, cpSync, existsSync, mkdirSync, readFileSync, realpathSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { admitCustody } from './admit-custody.mjs';
import { compileCustody, verifyCustodyPublication } from './compile-custody.mjs';
import { applyHeadToCashPolicy, buildApprovedCashPolicy, STDLIB_TREE, STDLIB_TREE_SHA256, stdlibTreeSha256 } from './apply-head.mjs';
import { bindSpendTime, isFixedUtxoProbe, isAuthenticProductionAdmission, planWindows, productionParityGate } from './custody-time.mjs';
import { executeParityCapture } from './posix-parity-harness.mjs';
import { bindWitness, encodeSpendBody, encodeUnsignedBody, genesisMagicText, paymentKeyHashOfVkeyCbor } from './native-body.mjs';
import { bindReceipt, buildConcurrentPair, RECEIPT_HELPER_PATH } from './native-consumer.mjs';
import { compareFinancialFrame, datumFromPost, expectedFrame, judgeEconomicAction, loadSourceTerminals, readKernelQuantity } from './native-financial.mjs';
import {
  CARDANO_CLI,
  TASK_ROOT,
  acquireBodyEvaluation,
  assessCashPolicy,
  compareBudget,
  inspectLauncher,
  loadAuthority,
  loadCashDesignAuthority,
  loadRuntimeInputs,
  produceOwnedRuntime,
  queryOwnedUtxos,
  queryProtocolParameters,
  stopOwnedGroup,
} from './native-runtime.mjs';
import { parseJsonExact, readWord64Coin } from './native-bootstrap.mjs';
import { consumePlutusCost, dispatchOwnedProducer, observeOwnedClock, plutusCostArgv, realProducerSpawn, selectOwnedFunder } from './native-producer.mjs';
import { bindSignedWitnessCount, compareClockParity, recomputeScriptDataHash, redeemerPointerAccepts } from './native-integrate.mjs';
import { OutputFeeError, calculateBodyMinimumFee, calculateOutputMinimum } from './output-fee.mjs';

const HERE = dirname(fileURLToPath(import.meta.url));
const DESIGNER = join(TASK_ROOT, 'designers/02-escrow-time');
const REPAIR_ROOT = join(TASK_ROOT, 't1-consumer-repair-119/owned-output');
const SOURCE_CACHE = join(REPAIR_ROOT, 'checks/source-cache');
const FROZEN_SOURCE_CACHE = join(TASK_ROOT, 't1-native-repair-59/checks/source-cache');
const CASH_WORK = join(REPAIR_ROOT, 'checks/cash-policy');
const FROZEN_CASH_WORK = join(TASK_ROOT, 't1-native-repair-59/checks/cash-policy');
const RECEIPT_ROOT = join(TASK_ROOT, 'tooling/transaction-receipts/revision02');
// Offline build input so cardano-cli can hash script data. Not a measured budget.
const OFFLINE_PROTOCOL_PARAMS = join(TASK_ROOT, 'tooling/runtime/actual-protocol-parameters.json');
export { compareClockParity };
const MANIFEST_PATH = join(HERE, '../examples/custody-unit.manifest.json');
const AIKEN_BIN = '/home/charl/.aiken/bin/aiken';
const UNIT_CASH_POLICY = 'aa'.repeat(28);
const UNIT_HEAD_POLICY = 'bb'.repeat(28);

const HEAD_FIELDS = [
  'program_hash', 'bounds_hash', 'source_hash', 'schema_hash', 'seed_hash',
  'domain_network', 'domain_deployment', 'instance_id',
  'transport_revision', 'transport_remaining', 'ordinary_locked', 'ordinary_revision', 'lifecycle',
];

function closed(code, extra = {}) {
  return { ok: false, code, ledgerExecution: false, nativePins: false, confirmed: false, submitted: false, ...extra };
}

function realCli(cliPath) {
  try {
    return Boolean(cliPath) && realpathSync(cliPath) === realpathSync(CARDANO_CLI);
  } catch {
    return false;
  }
}

function realAiken(compilerBin) {
  try {
    return Boolean(compilerBin) && realpathSync(compilerBin) === realpathSync(AIKEN_BIN);
  } catch {
    return false;
  }
}

function aikenIdentity() {
  try {
    const real = realpathSync(AIKEN_BIN);
    const version = spawnSync(real, ['--version'], { encoding: 'utf8' });
    if (version.status !== 0) return null;
    return { real, version: String(version.stdout || version.stderr || '').trim() };
  } catch {
    return null;
  }
}

function fileHash(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

function readGenesisSlot(paths) {
  for (const path of paths) {
    if (!path || !existsSync(path)) continue;
    try {
      const genesis = JSON.parse(readFileSync(path, 'utf8'));
      if (genesis.slotLength === 1) return { path, slotLengthMs: '1000' };
    } catch {
      /* unreadable genesis is not a measurement */
    }
  }
  return null;
}

function parseTipSlot(text) {
  try {
    const parsed = JSON.parse(String(text || ''));
    const slot = parsed?.slot ?? parsed?.slotNo ?? null;
    if (slot == null || !/^(0|[1-9][0-9]*)$/.test(String(slot))) return null;
    return String(slot);
  } catch {
    return null;
  }
}

function parseEraHistory(text) {
  try {
    const parsed = JSON.parse(String(text || ''));
    if (!parsed || typeof parsed !== 'object') return null;
    return parsed;
  } catch {
    return null;
  }
}

export function measureSlotLength({ cliPath, socket = '', execute = true, genesisPath = '', runDir = '', magic = null } = {}) {
  if (execute !== true) return closed('PARITY_NOT_MEASURED', { spawned: false, siblingEmitted: false });
  if (!cliPath || !existsSync(cliPath) || !socket || !existsSync(socket)) {
    return closed('TIME_QUERY_ABSENT', { spawned: false, siblingEmitted: false });
  }
  if (realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_QUERY_NOT_ARMED', { spawned: false, siblingEmitted: false });
  }
  const boundMagic = genesisMagicText(magic);
  if (!boundMagic.ok) return closed(boundMagic.code, { spawned: false, siblingEmitted: false, genesisMagicBound: false });
  const query = (name) => spawnSync(cliPath, [
    'query', name, '--socket-path', socket, '--testnet-magic', boundMagic.text,
  ], { encoding: 'utf8' });
  const tip = query('tip');
  const era = query('era-history');
  const stdout = `${tip.stdout || ''}\n${era.stdout || ''}`;
  const spawned = true;
  const tipJson = parseEraHistory(tip.stdout);
  const eraJson = parseEraHistory(era.stdout);
  if (tipJson && eraJson) {
    const genesisFile = [
      genesisPath,
      runDir ? join(runDir, 'config/genesis/shelley-genesis.json') : '',
      socket ? join(dirname(socket), 'config/genesis/shelley-genesis.json') : '',
    ].find((path) => path && existsSync(path)) || '';
    const clock = observeOwnedClock({
      tip: tipJson,
      era: eraJson,
      genesisPath: genesisFile,
      unitOnly: !realCli(cliPath),
    });
    return { ...clock, spawned, queries: ['tip', 'era-history'] };
  }
  if (/slotLengthMs\s*=\s*["']/.test(stdout)) return closed('SLOT_LENGTH_QUOTED', { spawned, unitOnly: true, siblingEmitted: false });
  const found = stdout.match(/slotLengthMs=(\d+)/);
  if (found && found[1] !== '1000') {
    return closed('SLOT_LENGTH_REFUSED', { spawned, observed: found[1], unitOnly: true, siblingEmitted: false });
  }
  const genesis = readGenesisSlot([
    genesisPath,
    runDir ? join(runDir, 'config/genesis/shelley-genesis.json') : '',
    socket ? join(dirname(socket), 'config/genesis/shelley-genesis.json') : '',
  ]);
  const observed = (found && found[1] === '1000') || genesis ? '1000' : null;
  if (!observed) return closed('PARITY_NOT_MEASURED', { spawned, unitOnly: true, siblingEmitted: false });
  const field = (name) => {
    const match = stdout.match(new RegExp(`^${name}=([0-9]+)$`, 'm'));
    return match ? match[1] : null;
  };
  const lucidPosix = field('lucidPosix');
  const cliPosix = field('cliPosix');
  const scriptContextLower = field('scriptContextLower');
  const tipSlot = field('tipSlot');
  if (!lucidPosix || !cliPosix || !scriptContextLower || !tipSlot) {
    return closed('PARITY_NOT_MEASURED', { spawned, observed, unitOnly: true, siblingEmitted: false });
  }
  if (lucidPosix !== cliPosix || lucidPosix !== scriptContextLower) {
    return closed('IMPOSSIBLE_BOUNDARY', { spawned, observed, unitOnly: true, siblingEmitted: false });
  }
  const windows = planWindows({ zeroTime: '0', zeroSlot: '0', slotLengthMs: '1000', tipSlot });
  if (!windows.ok) return { ...windows, spawned, observed, unitOnly: true, siblingEmitted: false, ledgerExecution: false, nativePins: false };
  const marked = stdout.includes('evidenceClass=unit-fixture') || !realCli(cliPath);
  return {
    ok: true,
    code: 'SLOT_OBSERVED',
    observed,
    slotLengthSource: genesis ? 'owned-genesis' : 'unit-stdout',
    genesisPath: genesis?.path || null,
    spawned,
    unitOnly: marked,
    lucidPosix,
    cliPosix,
    scriptContextLower,
    tipSlot,
    windows,
    queries: ['tip', 'era-history'],
    ledgerExecution: false,
    nativePins: false,
    siblingEmitted: false,
  };
}

function sourceCachePath(sha256) {
  const fresh = join(SOURCE_CACHE, `${sha256}.json`);
  if (existsSync(fresh)) return fresh;
  const frozen = join(FROZEN_SOURCE_CACHE, `${sha256}.json`);
  if (existsSync(frozen)) return frozen;
  return null;
}

export function compileGuardSibling({ compilerBin, sourcePath, previousProgramHash, evidenceRoot = null, production = null } = {}) {
  if (!compilerBin || !existsSync(compilerBin) || !sourcePath || !existsSync(sourcePath)) {
    return closed('SIBLING_NOT_COMPILED_IN_THIS_RUN', { siblingEmitted: false, spawned: false });
  }
  if (!realAiken(compilerBin)) {
    const result = spawnSync(compilerBin, ['check', sourcePath], { encoding: 'utf8' });
    const hashLine = /programHash=([0-9a-f]{56,64})/.test(String(result.stdout || ''));
    return closed('HASH_LINE_NOT_A_COMPILE', {
      siblingEmitted: false,
      spawned: true,
      unitOnly: true,
      programHashIgnored: hashLine,
      compilerIdentity: 'path-is-not-aiken',
      cliStatus: result.status,
    });
  }
  const pin = stdlibTreeSha256(STDLIB_TREE);
  if (!pin.ok || pin.sha256 !== STDLIB_TREE_SHA256 || pin.files !== 65) {
    return closed('STDLIB_PIN_MISMATCH', { siblingEmitted: false, spawned: false, pin });
  }
  const sourceText = readFileSync(sourcePath, 'utf8');
  const sourceHash = createHash('sha256').update(sourceText).digest('hex');
  const manifest = JSON.parse(readFileSync(MANIFEST_PATH, 'utf8'));
  if (production) {
    const bounds = production.measuredBounds;
    const seed = production.queriedSeed;
    const authentic = isAuthenticProductionAdmission(production) && production.code === 'PARITY_ADMITTED'
      && production.nativeProbe === true
      && production.captureOrigin === 'executeParityCapture'
      && bounds?.releaseLower && bounds?.partition && bounds?.refundInclusive
      && /^[0-9a-f]{64}$/.test(String(production.genesisSha256 || ''))
      && seed && !isFixedUtxoProbe(seed);
    if (!authentic) {
      return closed('CALIBRATED_SOURCE_NOT_THIS_PHASE', {
        siblingEmitted: false,
        productionAdmitted: false,
        calibrated: false,
        nativeProbe: false,
      });
    }
    const outDir = join(evidenceRoot || join(tmpdir(), 'moriarty-sibling'), 'production-compile');
    const compiled = compileCustody({
      sourcePath,
      seedPath: join(DESIGNER, 'financial-seed-custody-v5.json'),
      casesPath: join(DESIGNER, 'cases.json'),
      expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
      expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
      residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
      manifest,
      outDir,
      syntheticTime: {
        evidenceClass: 'node-genesis-v3-probe',
        productionArtifact: true,
        unitFixture: false,
        probeExecuted: true,
        genesisSha256: production.genesisSha256,
        queriedSeed: seed,
        bounds,
        productionAdmission: production,
      },
    });
    const programHash = compiled.hashes?.programHash || null;
    if (!compiled.ok || !programHash || programHash === previousProgramHash || compiled.hashes?.sourceHash === sourceHash) {
      return closed(compiled.code || 'SIBLING_COMPILE_REJECTED', {
        siblingEmitted: false,
        productionAdmitted: false,
        calibrated: false,
        nativeProbe: false,
        detail: compiled.detail || null,
      });
    }
    return {
      ok: true,
      code: 'SIBLING_COMPILED',
      siblingEmitted: true,
      calibrated: true,
      productionAdmitted: true,
      productionArtifact: true,
      nativeProbe: true,
      spawned: true,
      compilerInvoked: true,
      cacheHit: false,
      programHash,
      boundsHash: compiled.hashes.boundsHash,
      sourceHash: compiled.hashes.sourceHash,
      schemaHash: compiled.hashes.schemaHash,
      seedHash: compiled.hashes.seedHash,
      blueprintSha256: compiled.blueprintPath ? fileHash(compiled.blueprintPath) : null,
      blueprintPath: compiled.blueprintPath,
      sourcePath: join(outDir, 'sibling.mori'),
      applied: compiled.applied,
      productionAdmission: production,
      zeroTime: production.zeroTime,
      zeroSlot: production.zeroSlot,
      measuredBounds: bounds,
      genesisSha256: production.genesisSha256,
      queriedSeed: seed,
      siblingBudgetBound: false,
      minAdaMeasured: false,
      protocolMinAda: 'unmeasured',
      stdlibSha256: STDLIB_TREE_SHA256,
      stdlibFiles: 65,
      previousProgramHash,
      unitOnly: false,
      compilerIdentity: 'aiken-realpath',
      ledgerExecution: false,
      nativePins: false,
    };
  }
  const cachePath = sourceCachePath(sourceHash);
  const cached = cachePath ? JSON.parse(readFileSync(cachePath, 'utf8')) : null;
  const cacheReady = Boolean(cached?.blueprintPath && existsSync(cached.blueprintPath) && fileHash(cached.blueprintPath) === cached.blueprintSha256);
  if (!cacheReady) {
    const outDir = join(evidenceRoot || join(tmpdir(), 'moriarty-sibling'), 'compile');
    const compiled = compileCustody({
      sourcePath,
      seedPath: join(DESIGNER, 'financial-seed-custody-v5.json'),
      casesPath: join(DESIGNER, 'cases.json'),
      expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
      expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
      residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
      manifest,
      outDir,
    });
    if (!compiled.ok || !compiled.blueprintPath || !existsSync(compiled.blueprintPath)) {
      return closed(compiled.code || 'SIBLING_NOT_COMPILED_IN_THIS_RUN', {
        siblingEmitted: false,
        spawned: true,
        compilerInvoked: true,
        cacheHit: false,
      });
    }
    const seedTextEarly = readFileSync(join(DESIGNER, 'financial-seed-custody-v5.json'), 'utf8');
    const admittedEarly = admitCustody({ sourceText, seedText: seedTextEarly, manifest });
    if (!admittedEarly.ok || !admittedEarly.hashes?.programHash || admittedEarly.hashes.programHash === previousProgramHash) {
      return closed('SIBLING_COMPILE_REJECTED', { siblingEmitted: false, spawned: true, compilerInvoked: true, unitOnly: false });
    }
    return {
      ok: true,
      code: 'SIBLING_COMPILED',
      siblingEmitted: true,
      calibrated: false,
      productionAdmitted: false,
      nativeProbe: false,
      spawned: true,
      compilerInvoked: true,
      cacheHit: false,
      programHash: admittedEarly.hashes.programHash,
      boundsHash: admittedEarly.hashes.boundsHash,
      sourceHash: admittedEarly.hashes.sourceHash,
      schemaHash: admittedEarly.hashes.schemaHash,
      seedHash: admittedEarly.hashes.seedHash,
      blueprintSha256: fileHash(compiled.blueprintPath),
      blueprintPath: compiled.blueprintPath,
      stdlibSha256: STDLIB_TREE_SHA256,
      stdlibFiles: 65,
      previousProgramHash,
      unitOnly: false,
      compilerIdentity: 'aiken-realpath',
      ledgerExecution: false,
      nativePins: false,
    };
  }
  const seedText = readFileSync(join(DESIGNER, 'financial-seed-custody-v5.json'), 'utf8');
  const admitted = admitCustody({ sourceText, seedText, manifest });
  if (!admitted.ok || !admitted.hashes?.programHash) {
    return closed('SIBLING_COMPILE_REJECTED', { siblingEmitted: false, spawned: false, unitOnly: false });
  }
  if (admitted.hashes.programHash === previousProgramHash) {
    return closed('SIBLING_COMPILE_REJECTED', { siblingEmitted: false, spawned: false, unitOnly: false });
  }
  const blueprint = JSON.parse(readFileSync(cached.blueprintPath, 'utf8'));
  const titles = (blueprint.validators || []).map((item) => item.title);
  if (!titles.some((title) => title.includes('financial_head'))) {
    return closed('SIBLING_COMPILE_REJECTED', { siblingEmitted: false, spawned: false });
  }
  return {
    ok: true,
    code: 'SIBLING_COMPILED',
    siblingEmitted: true,
    calibrated: false,
    productionAdmitted: false,
    nativeProbe: false,
    spawned: false,
    compilerInvoked: false,
    cacheHit: true,
    programHash: admitted.hashes.programHash,
    boundsHash: admitted.hashes.boundsHash,
    sourceHash: admitted.hashes.sourceHash,
    schemaHash: admitted.hashes.schemaHash,
    seedHash: admitted.hashes.seedHash,
    blueprintSha256: cached.blueprintSha256,
    stdlibSha256: STDLIB_TREE_SHA256,
    stdlibFiles: 65,
    previousProgramHash,
    unitOnly: false,
    compilerIdentity: 'aiken-realpath',
    ledgerExecution: false,
    nativePins: false,
  };
}

export function prepareEnterpriseKey({ cliPath, runDir, authority, networkMagic, magic } = {}) {
  const real = realCli(cliPath);
  if (real && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_KEYGEN_NOT_ARMED', { keyCreated: false, spawnAttempted: false });
  }
  const boundMagic = genesisMagicText(networkMagic ?? magic);
  if (!boundMagic.ok) return closed(boundMagic.code, { keyCreated: false, spawnAttempted: false, genesisMagicBound: false });
  const gate = loadAuthority(authority);
  const stubDespiteAuthority = !gate.ok
    && !real
    && gate.code !== 'EXECUTION_AUTHORITY_ABSENT'
    && gate.code !== 'AUTHORITY_UNREADABLE';
  if (!gate.ok && !stubDespiteAuthority) return { ...gate, keyCreated: false };
  if (!cliPath || !existsSync(cliPath) || !runDir) return closed('KEYGEN_FAILED', { keyCreated: false, spawnAttempted: false });
  const keyPath = join(runDir, 'payment.skey');
  const vkeyPath = join(runDir, 'payment.vkey');
  const result = spawnSync(cliPath, [
    'address', 'key-gen',
    '--signing-key-file', keyPath,
    '--verification-key-file', vkeyPath,
  ], { encoding: 'utf8' });
  if (result.status !== 0 || !existsSync(keyPath)) return closed('KEYGEN_FAILED', { keyCreated: false, spawned: true });
  chmodSync(keyPath, 0o600);
  const st = statSync(keyPath);
  if ((st.mode & 0o777) !== 0o600) return closed('KEY_MODE_REFUSED', { keyCreated: true });
  if (typeof process.getuid === 'function' && st.uid !== process.getuid()) return closed('KEY_NOT_OWNED', { keyCreated: true });
  const addrPath = join(runDir, 'payment.addr');
  const address = spawnSync(cliPath, [
    'address', 'build',
    '--payment-verification-key-file', vkeyPath,
    '--testnet-magic', boundMagic.text,
    '--out-file', addrPath,
  ], { encoding: 'utf8' });
  let addr = '';
  if (address.status === 0) {
    addr = existsSync(addrPath) ? readFileSync(addrPath, 'utf8').trim() : String(address.stdout || '').trim();
  }
  if (!addr) return closed('ENTERPRISE_ADDRESS_FAILED', { keyCreated: true, addressBuilt: false, spawned: true });
  return {
    ok: true,
    code: 'ENTERPRISE_KEY_FILE',
    keyPath,
    address: addr,
    addressBuilt: true,
    mode: '0600',
    uidMatches: true,
    spawned: true,
    unitOnly: gate.unitFixture === true || !real,
    nativePins: false,
    ledgerExecution: false,
    wallet: false,
  };
}

function utf8Of(node) {
  if (!node || typeof node.bytes !== 'string' || !/^[0-9a-fA-F]*$/.test(node.bytes) || node.bytes.length % 2 !== 0) return null;
  return Buffer.from(node.bytes, 'hex').toString('utf8');
}

function exactQuantity(value) {
  if (value == null) return null;
  const read = readKernelQuantity(value);
  return read.ok ? read.value : null;
}

function refuseKernel(reason, field) {
  return {
    ok: false,
    code: 'KERNEL_AMOUNT_REFUSED',
    reason,
    field,
    ledgerExecution: false,
    nativePins: false,
    confirmed: false,
  };
}

function takeInt(node, field, sink) {
  if (sink.refused) return null;
  if (!node || node.int == null) {
    sink.refused = refuseKernel('null-quantity', field);
    return null;
  }
  const read = readKernelQuantity(node.int);
  if (!read.ok) {
    sink.refused = refuseKernel(read.reason, field);
    return null;
  }
  return read.value;
}

function plutusToFields(plutus) {
  const fields = plutus?.fields;
  if (!Array.isArray(fields) || fields.length !== 13) return null;
  const life = fields[12]?.fields;
  if (!Array.isArray(life) || life.length < 9) return null;
  const sink = {};
  const rows = (list, map) => (Array.isArray(list?.list) ? list.list.map(map) : []);
  const balance = (node, index) => ({
    party: utf8Of(node?.fields?.[0]),
    asset: utf8Of(node?.fields?.[1]),
    amount: takeInt(node?.fields?.[2], `lifecycle.balances[${index}].amount`, sink),
  });
  const allowance = (node, index) => ({
    party: utf8Of(node?.fields?.[0]),
    asset: utf8Of(node?.fields?.[1]),
    remaining: takeInt(node?.fields?.[2], `lifecycle.allowances[${index}].remaining`, sink),
    spent: takeInt(node?.fields?.[3], `lifecycle.allowances[${index}].spent`, sink),
  });
  const work = life[8]?.fields || [];
  const parsed = {
    program_hash: fields[0]?.bytes || null,
    bounds_hash: fields[1]?.bytes || null,
    source_hash: fields[2]?.bytes || null,
    schema_hash: fields[3]?.bytes || null,
    seed_hash: fields[4]?.bytes || null,
    domain_network: utf8Of(fields[5]),
    domain_deployment: utf8Of(fields[6]),
    instance_id: fields[7]?.bytes || null,
    transport_revision: takeInt(fields[8], 'transport_revision', sink),
    transport_remaining: takeInt(fields[9], 'transport_remaining', sink),
    ordinary_locked: takeInt(fields[10], 'ordinary_locked', sink),
    ordinary_revision: takeInt(fields[11], 'ordinary_revision', sink),
    lifecycle: {
      schema_version: utf8Of(life[0]),
      balances: rows(life[1], balance),
      allowances: rows(life[2], allowance),
      obligations: life[3]?.list || [],
      usedTransferIds: rows(life[4], (node) => utf8Of(node)),
      usedAllocationIds: rows(life[5], (node) => utf8Of(node)),
      usedOriginationIds: rows(life[6], (node) => utf8Of(node)),
      usedAccrualIds: rows(life[7], (node) => utf8Of(node)),
      work: {
        remaining: takeInt(work[0], 'lifecycle.work.remaining', sink),
        spent: takeInt(work[1], 'lifecycle.work.spent', sink),
        closure_reserve: takeInt(work[2], 'lifecycle.work.closure_reserve', sink),
      },
    },
  };
  if (sink.refused) return sink.refused;
  return parsed;
}

function asHead(parsed) {
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) return null;
  const missing = HEAD_FIELDS.filter((name) => parsed[name] == null);
  if (missing.length || parsed.lifecycle?.work?.spent == null || !Array.isArray(parsed.lifecycle?.balances)) return null;
  return parsed;
}

function word64Of(value) {
  if (typeof value === 'bigint') return readWord64Coin(value.toString(), undefined);
  if (typeof value === 'number') return readWord64Coin(undefined, value);
  if (typeof value === 'string') return readWord64Coin(value, undefined);
  return { ok: false, reason: 'noncanonical' };
}

function assetQty(value, assetName) {
  if (!value || typeof value !== 'object') return null;
  let seen = false;
  for (const [policy, assets] of Object.entries(value)) {
    if (policy === 'lovelace' || !assets || typeof assets !== 'object') continue;
    if (assets[assetName] != null) {
      const read = word64Of(assets[assetName]);
      if (!read.ok) return { refused: true, code: 'WORD64_COIN_REFUSED', reason: read.reason };
      return read.value;
    }
    seen = true;
  }
  return seen || Object.keys(value).length > 0 ? 0 : null;
}

function frameFromHeadFields(fields, extras = {}) {
  if (fields?.ok === false) return fields;
  const life = fields?.lifecycle;
  if (!life) return null;
  const sink = {};
  const keep = (value, field, { required = false, coin = false } = {}) => {
    if (sink.refused) return null;
    if (value == null) {
      if (required) {
        sink.refused = coin
          ? { ok: false, code: 'WORD64_COIN_REFUSED', reason: 'null-quantity', field, ledgerExecution: false, nativePins: false, confirmed: false }
          : refuseKernel('null-quantity', field);
      }
      return null;
    }
    const read = coin ? word64Of(value) : readKernelQuantity(value);
    if (!read.ok) {
      sink.refused = coin
        ? { ok: false, code: 'WORD64_COIN_REFUSED', reason: read.reason, field, ledgerExecution: false, nativePins: false, confirmed: false }
        : refuseKernel(read.reason, field);
      return null;
    }
    return read.value;
  };
  const amount = (party) => {
    const row = (life.balances || []).find((item) => item.party === party && item.asset === 'Cash');
    if (!row) return null;
    return keep(row.amount, `balances.${party}`, { required: true });
  };
  const allowance = (party) => {
    const row = (life.allowances || []).find((item) => item.party === party && item.asset === 'Cash');
    if (!row) return null;
    return {
      remaining: keep(row.remaining, `allowances.${party}.remaining`, { required: true }),
      spent: keep(row.spent, `allowances.${party}.spent`, { required: true }),
    };
  };
  const frame = {
    program_hash: fields.program_hash,
    bounds_hash: fields.bounds_hash,
    source_hash: fields.source_hash,
    schema_hash: fields.schema_hash,
    seed_hash: fields.seed_hash,
    domain_network: fields.domain_network,
    domain_deployment: fields.domain_deployment,
    balances: { Escrow: amount('Escrow'), Borrower: amount('Borrower'), Lender: amount('Lender') },
    allowances: { Escrow: allowance('Escrow'), Borrower: allowance('Borrower'), Lender: allowance('Lender') },
    obligations: life.obligations || [],
    usedAllocationIds: life.usedAllocationIds || [],
    usedOriginationIds: life.usedOriginationIds || [],
    usedAccrualIds: life.usedAccrualIds || [],
    work: {
      remaining: keep(life.work?.remaining, 'work.remaining', { required: true }),
      spent: keep(life.work?.spent, 'work.spent', { required: true }),
      closureReserve: keep(life.work?.closureReserve ?? life.work?.closure_reserve, 'work.closureReserve', { required: true }),
    },
    usedTransferIds: life.usedTransferIds || [],
    transport_revision: keep(fields.transport_revision, 'transport_revision', { required: true }),
    transport_remaining: keep(fields.transport_remaining, 'transport_remaining', { required: true }),
    ordinary_locked: keep(fields.ordinary_locked, 'ordinary_locked', { required: true }),
    ordinary_revision: keep(fields.ordinary_revision, 'ordinary_revision', { required: true }),
    sourceHash: fields.source_hash,
    instanceId: fields.instance_id,
    headQuantity: keep(extras.headQuantity ?? null, 'headQuantity', { coin: true }),
    continuingCash: keep(extras.continuingCash ?? null, 'continuingCash', { coin: true }),
    payoutCash: keep(extras.payoutCash ?? null, 'payoutCash', { coin: true }),
    runId: extras.runId || fields.runId || null,
    bodySha256: extras.bodySha256 || fields.bodySha256 || null,
    outputRef: extras.outputRef || null,
  };
  if (sink.refused) return sink.refused;
  return frame;
}

export function decodePostedDatum(plutus, extras = {}) {
  const fields = plutusToFields(plutus);
  if (!fields) return null;
  return frameFromHeadFields(fields, extras);
}

function datumNode(body) {
  if (!body || typeof body !== 'object') return null;
  return body.inlineDatum?.json || body.inlineDatum || body.datum || body.inline_datum || null;
}

function headFromUtxoMap(parsed, expectedTxid) {
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) return null;
  for (const [ref, body] of Object.entries(parsed)) {
    const match = String(ref).match(/^([0-9a-f]{64})#(\d+)$/);
    if (!match) continue;
    const datum = datumNode(body);
    const parsedFields = plutusToFields(datum);
    if (parsedFields?.ok === false) {
      return {
        fields: parsedFields,
        entry: body,
        txid: match[1],
        index: Number(match[2]),
        txidLinked: expectedTxid ? match[1] === expectedTxid : false,
      };
    }
    const fields = parsedFields || asHead(datum);
    if (!fields) continue;
    return {
      fields,
      entry: body,
      txid: match[1],
      index: Number(match[2]),
      txidLinked: expectedTxid ? match[1] === expectedTxid : false,
    };
  }
  return null;
}

export function collectHeadReadback({
  cliPath, socket = '', magic = null, networkMagic = null, expectedTxid = '', outputIndex = null, payoutIndex = null, expectedFrame = null, link = null,
} = {}) {
  const boundMagic = genesisMagicText(networkMagic ?? magic);
  if (!boundMagic.ok) return closed(boundMagic.code, { spawned: false, invented: false, genesisMagicBound: false });
  if (!cliPath || !existsSync(cliPath)) return closed('READBACK_ABSENT', { spawned: false, invented: false });
  if (realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_QUERY_NOT_ARMED', { spawned: false, invented: false });
  }
  const selected = /^[0-9a-f]{64}$/.test(String(expectedTxid || ''));
  if (selected && (!Number.isInteger(outputIndex) || outputIndex < 0)) {
    return closed('TX_INDEX_ABSENT', { spawned: false, invented: false, genesisMagicBound: true });
  }
  const txIn = selected ? `${expectedTxid}#${outputIndex}` : '';
  const query = (ref) => {
    const args = [
      'query', 'utxo',
      '--socket-path', socket,
      '--testnet-magic', boundMagic.text,
      '--output-json',
    ];
    if (ref) args.push('--tx-in', ref);
    const result = spawnSync(cliPath, args, { encoding: 'utf8' });
    if (result.status !== 0) return null;
    try {
      return parseJsonExact(String(result.stdout || ''));
    } catch {
      return null;
    }
  };
  const parsed = query(txIn);
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
    return closed('READBACK_ABSENT', { spawned: true, invented: false, selector: txIn ? '--tx-in' : 'txid-unselected' });
  }
  if (parsed.present === true) {
    return closed('READBACK_FIELDS_ABSENT', { spawned: true, booleanIgnored: true, confirmed: false, queryProvenance: true });
  }
  const mapped = headFromUtxoMap(parsed, expectedTxid);
  if (mapped?.fields?.ok === false) {
    return closed(mapped.fields.code, {
      spawned: true,
      reason: mapped.fields.reason,
      field: mapped.fields.field,
      confirmed: false,
      queryProvenance: true,
    });
  }
  const fields = mapped?.fields || asHead(parsed);
  if (!fields) {
    const missing = HEAD_FIELDS.filter((name) => parsed[name] == null);
    return closed('READBACK_FIELDS_ABSENT', { spawned: true, missing, confirmed: false, queryProvenance: true });
  }
  const unitOnly = !realCli(cliPath) || parsed.evidenceClass === 'unit-fixture' || fields.evidenceClass === 'unit-fixture';
  const txidLinked = mapped?.txidLinked === true;
  const continuingRaw = mapped?.entry ? assetQty(mapped.entry.value, '43617368') : null;
  if (continuingRaw?.refused === true) {
    return closed(continuingRaw.code, { spawned: true, reason: continuingRaw.reason, field: 'continuingCash', confirmed: false, queryProvenance: true });
  }
  const headRaw = mapped?.entry ? assetQty(mapped.entry.value, '48656164') : null;
  if (headRaw?.refused === true) {
    return closed(headRaw.code, { spawned: true, reason: headRaw.reason, field: 'headQuantity', confirmed: false, queryProvenance: true });
  }
  const continuingCash = continuingRaw;
  const headQuantity = headRaw;
  let payoutCash = null;
  if (selected && expectedFrame && Number.isInteger(payoutIndex) && payoutIndex >= 0) {
    const payoutRef = `${expectedTxid}#${payoutIndex}`;
    const payout = query(payoutRef);
    const payoutEntry = payout?.[payoutRef] || null;
    if (payoutEntry?.value) {
      const payoutRaw = assetQty(payoutEntry.value, '43617368');
      if (payoutRaw?.refused === true) {
        return closed(payoutRaw.code, { spawned: true, reason: payoutRaw.reason, field: 'payoutCash', confirmed: false, queryProvenance: true });
      }
      payoutCash = payoutRaw;
    }
  }
  const decoded = frameFromHeadFields(fields, {
    continuingCash: fields.continuingCash ?? continuingCash,
    payoutCash: fields.payoutCash ?? payoutCash,
    headQuantity: fields.headQuantity ?? headQuantity,
    runId: mapped?.entry?.runId || fields.runId || null,
    bodySha256: mapped?.entry?.bodySha256 || fields.bodySha256 || null,
    outputRef: mapped ? `${mapped.txid}#${mapped.index}` : fields.outputRef || null,
  });
  if (decoded?.ok === false) {
    return closed(decoded.code, {
      spawned: true,
      reason: decoded.reason,
      field: decoded.field,
      confirmed: false,
      queryProvenance: true,
    });
  }
  let financialMatch = false;
  let financialCode = null;
  let mismatches = null;
  if (expectedFrame && decoded) {
    const compared = compareFinancialFrame(decoded, expectedFrame, link || {});
    financialMatch = compared.financialMatch === true;
    financialCode = compared.code || null;
    mismatches = compared.mismatches || null;
  }
  return {
    ok: true,
    code: 'HEAD_READBACK',
    fields: HEAD_FIELDS,
    fieldCount: HEAD_FIELDS.length,
    spawned: true,
    confirmed: false,
    unitOnly,
    queryProvenance: true,
    selector: txIn ? '--tx-in' : 'txid-unselected',
    financialMatch,
    financialCode,
    mismatches,
    decoded,
    txid: mapped?.txid || null,
    txidLinked,
    helperConfirmed: false,
    ledgerExecution: false,
    nativePins: false,
    workSpent: exactQuantity(fields.lifecycle?.work?.spent),
  };
}

export function decideConfirmation(readback, receiptStatus) {
  if (!readback || readback.code !== 'HEAD_READBACK' || readback.fieldCount !== 13) {
    return closed('READBACK_FIELDS_ABSENT', {
      confirmed: false,
      booleanIgnored: readback?.booleanIgnored === true,
      receiptStatusIgnored: receiptStatus || null,
    });
  }
  if (readback.unitOnly === true) {
    return {
      ok: false,
      code: 'READBACK_UNIT_ONLY',
      confirmed: false,
      fieldCount: 13,
      unitOnly: true,
      ledgerExecution: false,
      nativePins: false,
      receiptStatusIgnored: receiptStatus || null,
    };
  }
  if (readback.queryProvenance !== true || readback.txidLinked !== true || readback.helperConfirmed !== true) {
    return closed(readback.txidLinked === true ? 'READBACK_UNPROVEN' : 'READBACK_TXID_UNLINKED', {
      confirmed: false,
      fieldCount: 13,
      receiptStatusIgnored: receiptStatus || null,
    });
  }
  if (readback.financialMatch !== true) {
    return closed(readback.financialCode || 'READBACK_FINANCIAL_MISMATCH', {
      confirmed: false,
      fieldCount: 13,
      mismatches: readback.mismatches || null,
      receiptStatusIgnored: receiptStatus || null,
    });
  }
  return {
    ok: true,
    code: 'FINANCIAL_READBACK_CONFIRMED',
    confirmed: true,
    fieldCount: 13,
    unitOnly: false,
    ledgerExecution: false,
    nativePins: false,
    receiptStatusIgnored: receiptStatus || null,
  };
}

export function selectRunInputs(utxos) {
  if (!Array.isArray(utxos) || utxos.length < 5) return closed('INPUTS_NOT_DISJOINT');
  const ids = utxos.map((item) => `${item.txId}#${item.index}`);
  if (ids.some((id) => !/^[0-9a-f]{64}#\d+$/.test(id)) || new Set(ids).size !== utxos.length) {
    return closed('INPUTS_NOT_DISJOINT');
  }
  if (utxos.length === 5) {
    const [head, feeA, colA, feeB, colB] = utxos;
    return {
      ok: true,
      code: 'INPUTS_SELECTED',
      head,
      left: { fee: feeA, collateral: colA },
      right: { fee: feeB, collateral: colB },
      shared: ['head'],
      ledgerExecution: false,
      nativePins: false,
    };
  }
  if (utxos.length < 12) return closed('INPUTS_INCOMPLETE', { count: utxos.length, required: 12 });
  const [cashSeed, head, mintFee, mintCollateral, headFee, headCollateral, releaseFee, releaseCol, refundFee, refundCol, cancelFee, cancelCol] = utxos;
  if (`${cashSeed.txId}#${cashSeed.index}` === `${head.txId}#${head.index}`) return closed('CASH_SEED_REUSED');
  return {
    ok: true,
    code: 'INPUTS_SELECTED',
    cashSeed,
    head,
    mintFee,
    mintCollateral,
    headFee,
    headCollateral,
    left: { fee: releaseFee, collateral: releaseCol },
    right: { fee: refundFee, collateral: refundCol },
    cancel: { fee: cancelFee, collateral: cancelCol },
    shared: ['head'],
    ledgerExecution: false,
    nativePins: false,
  };
}

function intervalsOverlap(left, right) {
  const a0 = Number(left?.invalidBefore);
  const a1 = Number(left?.invalidHereafter);
  const b0 = Number(right?.invalidBefore);
  const b1 = Number(right?.invalidHereafter);
  if (![a0, a1, b0, b1].every(Number.isInteger)) return false;
  return a0 < a1 && b0 < b1 && a0 < b1 && b0 < a1;
}

function refId(row) {
  if (!row?.txId && row?.txId !== '') return '';
  return `${row.txId}#${row.index}`;
}

export function selectRacePartners(inputs, spend) {
  const release = spend?.release;
  const cancel = spend?.cancel;
  const refund = spend?.refund;
  const left = inputs?.left;
  const right = inputs?.cancel;
  const refused = (code) => ({
    ok: false,
    code,
    overlap: null,
    leftKind: 'release',
    rightKind: 'cancel',
    refundSeparate: true,
    confirmed: false,
    cashQuantity: 100,
    shared: ['head'],
  });
  if (!release?.bodyFile || !cancel?.bodyFile || !left?.fee || !right?.fee || !left?.collateral || !right?.collateral) {
    return refused('RACE_BODY_ABSENT');
  }
  if (refund?.bodyFile && cancel.bodyFile === refund.bodyFile) return refused('RACE_REFUND_NOT_CANCEL');
  if (refId(left.fee) === refId(right.fee) || refId(left.collateral) === refId(right.collateral)) {
    return refused('RACE_INPUTS_NOT_DISJOINT');
  }
  if (!intervalsOverlap(release, cancel)) return refused('RACE_WINDOW_DISJOINT');
  return {
    ok: true,
    code: 'RACE_RELEASE_CANCEL',
    overlap: 'release-cancel',
    leftKind: 'release',
    rightKind: 'cancel',
    refundSeparate: !refund?.bodyFile || refund.bodyFile !== cancel.bodyFile,
    cashQuantity: 100,
    shared: ['head'],
    confirmed: false,
    left,
    right,
    leftBodyFile: release.bodyFile,
    rightBodyFile: cancel.bodyFile,
    leftBodySha256: release.bodyFileSha256 || '',
    rightBodySha256: cancel.bodyFileSha256 || '',
  };
}

function bodyFileReady(file, sha256) {
  if (!file || !existsSync(file)) return false;
  if (!sha256) return true;
  return createHash('sha256').update(readFileSync(file)).digest('hex') === sha256;
}

function racePostJudgement(queryText, outRef, action, expectedFrame = null, redeemer = null) {
  const absent = { postCode: 'RACE_POST_ABSENT', judged: null };
  const start = String(queryText || '').indexOf('{');
  if (start < 0 || !action) return absent;
  let parsed;
  try {
    parsed = JSON.parse(String(queryText).slice(start));
  } catch {
    return absent;
  }
  const row = parsed?.[outRef];
  if (!row || row.evidenceClass === 'unit-fixture') return absent;
  const decoded = row.decoded || row.post || null;
  if (!decoded || decoded.work == null) return absent;
  const judged = judgeEconomicAction(decoded, action, { redeemer: redeemer || row.redeemer || decoded.redeemer || null });
  if (!judged.ok) return { postCode: 'RACE_POST_MISMATCH', judged };
  if (!expectedFrame) return { postCode: 'RACE_EXPECTED_POST_ABSENT', judged };
  const compared = compareFinancialFrame(decoded, expectedFrame, {});
  if (!compared.ok) return { postCode: 'RACE_POST_MISMATCH', judged, compared };
  return { postCode: 'ECONOMIC_ACTION_MATCHED', judged, compared };
}

export async function runOwnedRace({
  submitBin, pollBin, signBin, logPath, head, left, right, socket = 'unit-socket',
  leftBodyFile = '', rightBodyFile = '', leftBodySha256 = '', rightBodySha256 = '',
  signingKeyPath = '', leftSigningKey = '', rightSigningKey = '', networkMagic = null, magic = null,
  overlap = null, cashQuantity = 100,
} = {}) {
  const boundMagic = genesisMagicText(networkMagic ?? magic);
  if (!boundMagic.ok) return closed(boundMagic.code, { submitted: false, genesisMagicBound: false, assembled: false });
  if (!left?.actor || !right?.actor || left.actor === right.actor) return closed('RACE_ACTORS_NOT_DISTINCT');
  if (!left.resource || !right.resource || left.resource === right.resource) return closed('RACE_ACTORS_NOT_DISTINCT');
  if (!bodyFileReady(leftBodyFile, leftBodySha256) || !bodyFileReady(rightBodyFile, rightBodySha256)) {
    return closed('RACE_BODY_ABSENT', { submitted: false, ledgerSubmitted: false, replay: { separate: true, executed: false, expectedClass: 'BadInputsUTxO' } });
  }
  if (!signingKeyPath || !existsSync(signingKeyPath)) return closed('RACE_SIGNATURE_ABSENT', { submitted: false });
  const keyStat = statSync(signingKeyPath);
  if ((keyStat.mode & 0o777) !== 0o600) return closed('KEY_MODE_REFUSED', { submitted: false });
  const pair = buildConcurrentPair(
    { outRef: `${head.txId}#${head.index}` },
    {
      fee: { outRef: `${left.fee.txId}#${left.fee.index}` },
      collateral: { outRef: `${left.collateral.txId}#${left.collateral.index}` },
      requiredSigners: [left.actor],
    },
    {
      fee: { outRef: `${right.fee.txId}#${right.fee.index}` },
      collateral: { outRef: `${right.collateral.txId}#${right.collateral.index}` },
      requiredSigners: [right.actor],
    },
  );
  if (!pair.ok) return pair;
  if (pair.members[0].head !== pair.members[1].head) return closed('RACE_SHARED_NOT_ONLY_HEAD');
  if (!signBin || !existsSync(signBin) || !submitBin || !existsSync(submitBin) || !pollBin || !existsSync(pollBin)) {
    return closed('RACE_SIGNATURE_ABSENT', { submitted: false });
  }
  if (realCli(submitBin) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_SUBMIT_NOT_ARMED', {
      submitted: false,
      bodiesReady: true,
      replay: { separate: true, executed: false, expectedClass: 'BadInputsUTxO' },
    });
  }
  const unitOnly = !realCli(submitBin);
  const env = { ...process.env, MORIARTY_RACE_LOG: logPath };
  const sides = [
    { fee: pair.members[0].fee, actor: left.actor, resource: left.resource, bodyFile: leftBodyFile, signingKey: leftSigningKey || signingKeyPath, expectedFrame: left.expectedFrame, redeemer: left.redeemer },
    { fee: pair.members[1].fee, actor: right.actor, resource: right.resource, bodyFile: rightBodyFile, signingKey: rightSigningKey || signingKeyPath, expectedFrame: right.expectedFrame, redeemer: right.redeemer },
  ];
  for (const side of sides) {
    const witnessFile = join(dirname(side.bodyFile), `witness-${side.actor}-${side.resource}.json`);
    const assembledFile = join(dirname(side.bodyFile), `assembled-${side.actor}-${side.resource}.json`);
    side.txFile = assembledFile;
    const signed = spawnSync(signBin, [
      'conway', 'transaction', 'witness',
      '--tx-body-file', side.bodyFile,
      '--signing-key-file', side.signingKey,
      '--testnet-magic', boundMagic.text,
      '--out-file', witnessFile,
    ], { encoding: 'utf8', env });
    if (signed.status !== 0) return closed('RACE_SIGNATURE_ABSENT', { submitted: false, spawned: true, unitOnly: true, signedIsProof: false });
    const assembled = spawnSync(signBin, [
      'conway', 'transaction', 'assemble',
      '--tx-body-file', side.bodyFile,
      '--witness-file', witnessFile,
      '--out-file', assembledFile,
    ], { encoding: 'utf8', env });
    if (assembled.status !== 0) return closed('RACE_ASSEMBLE_FAILED', { submitted: false, spawned: true, assembled: false });
  }
  const events = [];
  const submits = [];
  for (const side of sides) {
    const run = spawnSync(submitBin, [
      'conway', 'transaction', 'submit',
      '--tx-file', side.txFile,
      '--socket-path', socket,
      '--testnet-magic', boundMagic.text,
    ], { encoding: 'utf8', env });
    side.submitStatus = run.status;
    side.submitText = `${run.stdout || ''}\n${run.stderr || ''}`;
    side.submittedAt = new Date().toISOString();
    events.push({ event: 'submit', actor: side.actor, resource: side.resource, at: side.submittedAt });
    submits.push(side);
  }
  for (const side of sides) {
    const txidRun = spawnSync(pollBin, [
      'conway', 'transaction', 'txid',
      '--tx-file', side.txFile,
      '--output-text',
    ], { encoding: 'utf8', env });
    const txidText = String(txidRun.stdout || '').trim();
    side.txid = /^[0-9a-f]{64}$/.test(txidText) ? txidText : null;
    side.outputPresent = false;
    side.queryText = '';
    if (!side.txid) continue;
    const query = spawnSync(pollBin, [
      'query', 'utxo',
      '--tx-in', `${side.txid}#0`,
      '--output-json',
      '--socket-path', socket,
      '--testnet-magic', boundMagic.text,
    ], { encoding: 'utf8', env });
    side.queryStatus = query.status;
    side.queryText = `${query.stdout || ''}\n${query.stderr || ''}`;
    side.outputPresent = query.status === 0 && side.queryText.includes(`${side.txid}#0`);
    events.push({ event: 'poll', actor: side.actor, outRef: side.txid ? `${side.txid}#0` : null });
  }
  const winners = sides.filter((side) => side.outputPresent === true);
  const winner = winners.length === 1 ? winners[0] : null;
  const loser = winner ? sides.find((side) => side !== winner) : null;
  const observed = Boolean(loser && loser.submitText.includes('BadInputsUTxO'));
  const winnerOutRef = winner?.txid ? `${winner.txid}#0` : null;
  const winnerAction = !winner ? null : (winner === sides[0] ? 'release' : (overlap === 'release-refund' ? 'refund' : 'cancel'));
  let post = winnerOutRef ? racePostJudgement(winner.queryText, winnerOutRef, winnerAction, winner.expectedFrame, winner.redeemer) : { postCode: 'RACE_POST_ABSENT', judged: null };
  if (winner && !unitOnly) {
    if (!winner.expectedFrame || !winner.redeemer) post = { postCode: 'RACE_EXPECTED_POST_ABSENT', judged: null };
    else {
      const readback = collectHeadReadback({ cliPath: pollBin, socket, networkMagic: boundMagic.text,
        expectedTxid: winner.txid, outputIndex: 0, payoutIndex: 1, expectedFrame: winner.expectedFrame });
      const judged = readback.decoded ? judgeEconomicAction(readback.decoded, winnerAction, { redeemer: winner.redeemer }) : null;
      post = { postCode: readback.financialMatch === true && judged?.ok === true ? 'ECONOMIC_ACTION_MATCHED' : 'RACE_POST_MISMATCH', judged, readback };
    }
  }
  let code = 'RACE_WINNER_UNOBSERVED';
  if (winners.length > 1) code = 'RACE_WINNER_AMBIGUOUS';
  else if (winner && !observed) code = 'RACE_LOSER_RECEIPT_ABSENT';
  else if (winner && observed && post.postCode === 'RACE_POST_MISMATCH') code = 'RACE_POST_MISMATCH';
  else if (unitOnly) code = 'RACE_UNIT_OBSERVATION';
  else if (winner && observed && post.postCode === 'ECONOMIC_ACTION_MATCHED') code = 'RACE_OBSERVED';
  else if (winner && observed) code = 'RACE_INCOMPLETE';
  const replayExecuted = unitOnly !== true && code === 'RACE_OBSERVED';
  const replay = {
    separate: true,
    executed: replayExecuted,
    observed,
    expectedClass: 'BadInputsUTxO',
    ledgerSubmitted: false,
    assumedSecondIsLoser: false,
    loserTxFile: loser?.txFile || null,
    winnerTxid: winner?.txid || null,
    resubmitted: false,
  };
  return {
    ok: false,
    code,
    submitCount: submits.length,
    submitsBeforeConfirms: submits.length === 2,
    events,
    resubmitted: false,
    winnerOutRef,
    postCode: post.postCode,
    postJudge: post.judged?.code || null,
    winnerReadback: post.readback || null,
    loserCode: winner && !observed ? 'RACE_LOSER_RECEIPT_ABSENT' : null,
    submissions: sides.map((side) => ({
      actor: side.actor,
      resource: side.resource,
      submitText: side.submitText,
      submitStatus: side.submitStatus,
      submittedAt: side.submittedAt,
      txid: side.txid || null,
      outRef: side.txid ? `${side.txid}#0` : null,
      bodyFile: side.bodyFile,
    })),
    actors: [left.actor, right.actor],
    resources: [left.resource, right.resource],
    sharedHead: pair.members[0].head,
    overlap: overlap || null,
    cashQuantity,
    refundRaced: overlap === 'release-refund',
    confirmed: false,
    failureClass: observed ? 'BadInputsUTxO' : null,
    ledgerExecution: false,
    ledgerSubmitted: false,
    submitted: unitOnly ? false : submits.length === 2,
    assembled: true,
    genesisMagicBound: true,
    networkMagic: Number(boundMagic.text),
    nativePins: false,
    unitOnly,
    assumedSecondIsLoser: false,
    poll: 'query-utxo-tx-in',
    winnerTxid: winner?.txid || null,
    winnerActor: winner?.actor || null,
    replay,
  };
}

function mergeProductionClock(time, capture) {
  if (!time) return time;
  const genuine = capture
    && capture.probeExecuted === true
    && capture.evidenceClass === 'node-genesis-v3-probe'
    && capture.origin === 'executeParityCapture'
    && capture.productionAdmitted === true
    && capture.networkCommandsRun === true
    && capture.unitOnly !== true;
  if (!genuine) return { ...time, origin: null, probeExecuted: time?.probeExecuted === true && time?.origin === 'executeParityCapture' };
  return {
    ...time,
    unitOnly: false,
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: capture.lucidPosix,
    cliPosix: capture.cliPosix,
    scriptContextLower: capture.scriptContextLower,
    bindings: capture.bindings,
    transcriptPath: capture.transcriptPath,
    queriedSeed: capture.queriedSeed || null,
    origin: 'executeParityCapture',
    capture,
    measuredBounds: time.measuredBounds || null,
  };
}

function stageFailure(stages) {
  const observed = new Set(['READBACK_UNIT_ONLY', 'WITNESS_CONSTRUCTOR_REJECTED']);
  const order = ['authority', 'source', 'launcher', 'cash', 'protocol', 'budget', 'outputFee', 'time', 'clockParity', 'sibling', 'keys', 'funder', 'ownedProducer', 'financial', 'dispatch', 'utxo', 'inputs', 'mint', 'body', 'rebuild', 'posts', 'spend', 'witness', 'witnessCount', 'race', 'receipt', 'readback', 'confirmation', 'faucet', 'runtimeInputs'];
  for (const name of order) {
    const stage = stages[name];
    if (stage && stage.ok === false && stage.skipped !== true && !observed.has(stage.code)) return stage.code;
  }
  return null;
}

function runUnitOnly(stages) {
  const names = ['launcher', 'protocol', 'outputFee', 'time', 'sibling', 'keys', 'funder', 'ownedProducer', 'financial', 'utxo', 'witness', 'race', 'readback', 'faucet'];
  const flags = names.map((name) => stages[name]).filter((stage) => stage && stage.unitOnly != null);
  if (!flags.length) return true;
  return flags.some((stage) => stage.unitOnly === true);
}

async function compileAdmittedSource(sourcePath, evidenceRoot = null) {
  const sha256 = fileHash(sourcePath);
  const seedPath = join(DESIGNER, 'financial-seed-custody-v5.json');
  const seedText = readFileSync(seedPath, 'utf8');
  const manifest = JSON.parse(readFileSync(MANIFEST_PATH, 'utf8'));
  const { admitCustody } = await import('./admit-custody.mjs');
  const admitted = admitCustody({ sourceText: readFileSync(sourcePath, 'utf8'), seedText, manifest });
  if (!admitted.ok) {
    return closed(admitted.code || 'SOURCE_ADMIT_FAILED', {
      compiled: false,
      sha256,
      admitted: false,
      path: sourcePath,
    });
  }
  const identity = aikenIdentity();
  const seedHash = createHash('sha256').update(seedText).digest('hex');
  const manifestHash = fileHash(MANIFEST_PATH);
  const cachePath = sourceCachePath(sha256);
  if (cachePath && identity) {
    const cached = JSON.parse(readFileSync(cachePath, 'utf8'));
    const same = cached.sourceHash === sha256
      && cached.seedHash === seedHash
      && cached.manifestHash === manifestHash
      && cached.aikenRealpath === identity.real
      && cached.aikenVersion === identity.version
      && cached.checkStatus === 0
      && cached.buildStatus === 0
      && cached.blueprintPath
      && existsSync(cached.blueprintPath)
      && fileHash(cached.blueprintPath) === cached.blueprintSha256;
    if (same) {
      return {
        ok: true,
        code: 'SOURCE_COMPILED',
        sha256,
        path: sourcePath,
        compiled: true,
        admitted: true,
        cacheHit: true,
        compilerInvoked: false,
        checkStatus: 0,
        buildStatus: 0,
        blueprintPath: cached.blueprintPath,
        blueprintSha256: cached.blueprintSha256,
        aiken: identity.real,
        aikenVersion: identity.version,
        hashes: admitted.hashes,
        instanceId: admitted.instanceId,
        unitOnly: false,
        ledgerExecution: false,
        nativePins: false,
      };
    }
  }
  const { compileCustody } = await import('./compile-custody.mjs');
  const compileRoot = evidenceRoot || REPAIR_ROOT;
  const outDir = join(compileRoot, 'checks/source-compile', sha256);
  const compiled = compileCustody({
    sourcePath,
    seedPath,
    casesPath: join(DESIGNER, 'cases.json'),
    expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
    expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
    residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
    manifest,
    outDir,
  });
  if (!compiled.ok || !compiled.blueprintPath || !existsSync(compiled.blueprintPath)) {
    return closed(compiled.code || 'SOURCE_COMPILE_FAILED', {
      compiled: false,
      sha256,
      admitted: true,
      path: sourcePath,
      checkStatus: compiled.check?.status ?? null,
      buildStatus: compiled.build?.status ?? null,
      missing: compiled.check?.missing || compiled.missingTests || null,
    });
  }
  const publication = verifyCustodyPublication({ outDir });
  if (!publication.ok) {
    return closed(publication.code || 'STALE_MANIFEST', {
      compiled: false,
      sha256,
      admitted: true,
      path: sourcePath,
      checkStatus: compiled.check?.status ?? null,
      buildStatus: compiled.build?.status ?? null,
    });
  }
  const cacheDir = evidenceRoot ? join(evidenceRoot, 'checks/source-cache') : SOURCE_CACHE;
  mkdirSync(cacheDir, { recursive: true });
  const blueprintPath = join(cacheDir, `${sha256}.plutus.json`);
  cpSync(compiled.blueprintPath, blueprintPath);
  const blueprintSha256 = fileHash(blueprintPath);
  const record = {
    sourceHash: sha256,
    seedHash,
    manifestHash,
    aikenRealpath: identity?.real || null,
    aikenVersion: identity?.version || null,
    checkStatus: compiled.check?.status ?? null,
    buildStatus: compiled.build?.status ?? null,
    blueprintPath,
    blueprintSha256,
    commands: ['check', 'build --uplc'],
  };
  const recordPath = join(cacheDir, `${sha256}.json`);
  writeFileSync(recordPath, `${JSON.stringify(record, null, 2)}\n`);
  return {
    ok: true,
    code: 'SOURCE_COMPILED',
    sha256,
    path: sourcePath,
    compiled: true,
    admitted: true,
    cacheHit: false,
    compilerInvoked: true,
    checkStatus: record.checkStatus,
    buildStatus: record.buildStatus,
    blueprintPath,
    blueprintSha256,
    aiken: record.aikenRealpath,
    aikenVersion: record.aikenVersion,
    hashes: admitted.hashes,
    instanceId: admitted.instanceId,
    commands: record.commands,
    unitOnly: false,
    ledgerExecution: false,
    nativePins: false,
  };
}

function dataBytes(text) {
  return { bytes: Buffer.from(text, 'utf8').toString('hex') };
}

export function initialHeadDatum(hashes, instanceId) {
  const fields = {
    program_hash: hashes.programHash,
    bounds_hash: hashes.boundsHash,
    source_hash: hashes.sourceHash,
    schema_hash: hashes.schemaHash,
    seed_hash: hashes.seedHash,
    domain_network: 'yaci-isolated-t1',
    domain_deployment: 't1-custody-revision03',
    instance_id: instanceId,
    transport_revision: 0,
    transport_remaining: 1,
    ordinary_locked: 1,
    ordinary_revision: 0,
    lifecycle: {
      schema_version: 'moriarty-financial-lifecycle-state/1',
      balances: [
        { party: 'Escrow', asset: 'Cash', amount: 100 },
        { party: 'Borrower', asset: 'Cash', amount: 0 },
        { party: 'Lender', asset: 'Cash', amount: 0 },
      ],
      allowances: [
        { party: 'Escrow', asset: 'Cash', remaining: 100, spent: 0 },
        { party: 'Borrower', asset: 'Cash', remaining: 0, spent: 0 },
        { party: 'Lender', asset: 'Cash', remaining: 0, spent: 0 },
      ],
      obligations: [],
      work: { remaining: 1024, spent: 0, closure_reserve: 16 },
      constructorWorkCount: 49,
      collectionCapacity: 128,
    },
  };
  const balance = (row) => ({ constructor: 0, fields: [dataBytes(row.party), dataBytes(row.asset), { int: row.amount }] });
  const allowance = (row) => ({ constructor: 0, fields: [dataBytes(row.party), dataBytes(row.asset), { int: row.remaining }, { int: row.spent }] });
  const plutus = {
    constructor: 0,
    fields: [
      { bytes: fields.program_hash },
      { bytes: fields.bounds_hash },
      { bytes: fields.source_hash },
      { bytes: fields.schema_hash },
      { bytes: fields.seed_hash },
      dataBytes(fields.domain_network),
      dataBytes(fields.domain_deployment),
      { bytes: fields.instance_id },
      { int: 0 },
      { int: 1 },
      { int: 1 },
      { int: 0 },
      {
        constructor: 0,
        fields: [
          dataBytes(fields.lifecycle.schema_version),
          { list: fields.lifecycle.balances.map(balance) },
          { list: fields.lifecycle.allowances.map(allowance) },
          { list: [] },
          { list: [] },
          { list: [] },
          { list: [] },
          { list: [] },
          { constructor: 0, fields: [{ int: 1024 }, { int: 0 }, { int: 16 }] },
        ],
      },
    ],
  };
  return { fields, plutus };
}

function openFees(inputs) {
  if (inputs?.headFee && inputs?.headCollateral) return { fee: inputs.headFee, collateral: inputs.headCollateral };
  return { fee: inputs.left.fee, collateral: inputs.left.collateral };
}

function canonicalSlot(value) {
  if (typeof value === 'bigint') {
    const text = value.toString();
    return /^(?:0|[1-9][0-9]*)$/.test(text) ? text : null;
  }
  if (typeof value === 'number') {
    if (!Number.isSafeInteger(value) || value < 0) return null;
    return String(value);
  }
  if (typeof value === 'string' && /^(?:0|[1-9][0-9]*)$/.test(value)) return value;
  return null;
}

export function consumedHeadInterval(windows) {
  if (!windows || typeof windows !== 'object') return { ok: false, code: 'TIME_WINDOW_ABSENT', cliIntervalEmitted: false };
  const pair = windows.releaseSlot != null && windows.partitionSlot != null
    ? [windows.releaseSlot, windows.partitionSlot]
    : [windows.releaseSlots?.startSlot, windows.releaseSlots?.endSlot];
  const invalidBefore = canonicalSlot(pair[0]);
  const invalidHereafter = canonicalSlot(pair[1]);
  if (!invalidBefore || !invalidHereafter) {
    return { ok: false, code: 'TIME_WINDOW_ABSENT', cliIntervalEmitted: false, invalidBefore: null, invalidHereafter: null };
  }
  return {
    ok: true,
    code: 'SLOT_WINDOW_BOUND',
    invalidBefore,
    invalidHereafter,
    cliInterval: 'half-open-slots',
    cliIntervalEmitted: false,
    ledgerExecution: false,
  };
}

export function boundSpendInterval(kind, lower, upper, clock = null) {
  const timeSeparation = bindSpendTime({
    kind,
    lower: String(lower),
    upper: String(upper),
    zeroTime: clock?.zeroTime || '0',
    zeroSlot: clock?.zeroSlot || '0',
    slotLengthMs: '1000',
  });
  if (!timeSeparation.ok) {
    return {
      ok: false,
      code: timeSeparation.code,
      detail: timeSeparation.detail,
      kind,
      timeSeparation,
      encodeSpendBodyCalled: false,
      cliIntervalEmitted: false,
      invalidBefore: null,
      invalidHereafter: null,
      redeemerLower: timeSeparation.redeemerLower,
      redeemerUpper: timeSeparation.redeemerUpper,
      ledgerExecution: false,
    };
  }
  return {
    ok: true,
    code: timeSeparation.code,
    kind,
    timeSeparation,
    invalidBefore: timeSeparation.invalidBefore,
    invalidHereafter: timeSeparation.invalidHereafter,
    redeemerLower: timeSeparation.redeemerLower,
    redeemerUpper: timeSeparation.redeemerUpper,
    encodeSpendBodyCalled: false,
    cliIntervalEmitted: true,
    cliInterval: 'half-open-slots',
    ledgerExecution: false,
  };
}

function firstCoinText(values) {
  for (const value of values) {
    if (typeof value === 'string' && /^(?:0|[1-9][0-9]*)$/.test(value)) return value;
    if (typeof value === 'bigint' && value >= 0n) return value.toString();
  }
  return undefined;
}

function firstCoinMirror(values) {
  for (const value of values) {
    if (typeof value === 'number') return value;
  }
  return undefined;
}

function consumeLedgerCoin(primary, record, kind) {
  if (primary != null && (typeof primary === 'string' || typeof primary === 'number' || typeof primary === 'bigint')) {
    const text = typeof primary === 'number' ? undefined : primary;
    const mirror = typeof primary === 'number' ? primary : undefined;
    const read = readWord64Coin(text, mirror);
    return read.ok ? read.digits : primary;
  }
  if (!record || typeof record !== 'object') return null;
  const text = kind === 'fee'
    ? firstCoinText([record.feeText, record.parsed?.fee, record.fee])
    : firstCoinText([
      record.minimumLovelace,
      record.lovelaceText,
      record.parsed?.lovelaceText,
      record.parsed?.lovelace,
      record.lovelace,
    ]);
  const mirror = kind === 'fee'
    ? firstCoinMirror([record.fee, record.parsed?.fee])
    : firstCoinMirror([record.lovelace, record.parsed?.lovelace, record.minimumLovelace]);
  if (text == null && mirror == null) return null;
  const read = readWord64Coin(text, mirror);
  return read.ok ? read.digits : (text ?? mirror);
}

function bodySpec(stages, request) {
  const magic = stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null;
  const observed = stages.time?.code === 'SLOT_OBSERVED';
  const interval = observed ? consumedHeadInterval(stages.time?.windows) : null;
  const separated = interval?.ok === true;
  return {
    head: stages.inputs.head,
    ...openFees(stages.inputs),
    outputLovelace: consumeLedgerCoin(request.outputLovelace, stages.outputFee, 'coin'),
    feeLovelace: consumeLedgerCoin(request.feeLovelace, stages.outputFee, 'fee'),
    invalidBefore: separated ? interval.invalidBefore : 40,
    invalidHereafter: separated ? interval.invalidHereafter : 100,
    timeSeparation: interval,
    requiredSignerHash: request.requiredSignerHash,
    paymentVkeyCbor: request.paymentVkeyCbor || null,
    payoutVkeyHex: request.payoutVkeyHex || null,
    networkMagic: magic,
  };
}

export async function bindHelperReceipt(cborHex, opts = {}) {
  const label = opts.label || `t1-native-64-${randomBytes(4).toString('hex')}`;
  const receiptRoot = opts.receiptRoot || RECEIPT_ROOT;
  const dir = join(receiptRoot, label);
  const evidenceDir = join(opts.evidenceRoot || REPAIR_ROOT, 'checks/receipts', label);
  const calls = { submit: 0, poll: 0 };
  let lastPoll = null;
  const real = realCli(opts.cliPath || CARDANO_CLI);
  const armed = process.env.MORIARTY_ALLOW_REAL_NODE === '1';
  const unitOnly = opts.unitOnly === true || !real || !armed;
  const submit = async (hex) => {
    calls.submit += 1;
    if (typeof opts.submit === 'function') return opts.submit(hex);
    if (!real || !armed) {
      const file = join(dir, 'unit-txid.json');
      mkdirSync(dir, { recursive: true });
      writeFileSync(file, JSON.stringify({ type: 'Tx ConwayEra', description: 'unit txid preimage', cborHex: hex }));
      const txid = spawnSync(opts.cliPath || CARDANO_CLI, ['conway', 'transaction', 'txid', '--tx-file', file, '--output-text'], { encoding: 'utf8' });
      const got = String(txid.stdout || '').trim();
      if (txid.status !== 0 || !/^[0-9a-f]{64}$/.test(got)) throw new Error('TXID_FAILED');
      return got;
    }
    const boundMagic = genesisMagicText(opts.networkMagic ?? opts.magic);
    if (!boundMagic.ok || !opts.socket) throw new Error(boundMagic.ok ? 'GENESIS_MAGIC_ABSENT' : boundMagic.code);
    const file = join(dir, 'signed.json');
    writeFileSync(file, JSON.stringify({ type: 'Tx ConwayEra', description: 'signed financial transaction', cborHex: hex }));
    const txid = spawnSync(opts.cliPath || CARDANO_CLI, ['conway', 'transaction', 'txid', '--tx-file', file, '--output-text'], { encoding: 'utf8' });
    if (txid.status !== 0) throw new Error('TXID_FAILED');
    const got = String(txid.stdout || '').trim();
    const submitted = spawnSync(opts.cliPath || CARDANO_CLI, [
      'conway', 'transaction', 'submit', '--tx-file', file, '--socket-path', opts.socket, '--testnet-magic', boundMagic.text,
    ], { encoding: 'utf8' });
    if (submitted.status !== 0) throw new Error('SUBMIT_FAILED');
    return got;
  };
  const pollConfirmation = async (info) => {
    calls.poll += 1;
    if (typeof opts.pollConfirmation === 'function') {
      const supplied = await opts.pollConfirmation(info);
      lastPoll = supplied;
      return supplied;
    }
    if (unitOnly) {
      lastPoll = {
        status: 'unconfirmed',
        code: 'UNIT_PROVIDER_POLL_LABELLED',
        unitOnly: true,
        providerQueried: false,
        ledgerConfirmed: false,
      };
      return lastPoll;
    }
    const boundMagic = genesisMagicText(opts.networkMagic ?? opts.magic);
    if (!boundMagic.ok || !opts.socket) {
      return { status: 'unconfirmed', unitOnly: false, providerQueried: false, code: boundMagic.ok ? 'SOCKET_ABSENT' : boundMagic.code };
    }
    const envelope = join(dir, 'signed.json');
    if (!existsSync(envelope)) return { status: 'unconfirmed', unitOnly: false, providerQueried: false, code: 'TXID_ABSENT' };
    const txid = spawnSync(opts.cliPath || CARDANO_CLI, ['conway', 'transaction', 'txid', '--tx-file', envelope, '--output-text'], { encoding: 'utf8' });
    const got = String(txid.stdout || '').trim();
    if (txid.status !== 0 || !/^[0-9a-f]{64}$/.test(got)) {
      return { status: 'unconfirmed', unitOnly: false, providerQueried: false, code: 'TXID_ABSENT' };
    }
    if (!Number.isInteger(opts.outputIndex) || opts.outputIndex < 0) {
      return { status: 'unconfirmed', unitOnly: false, providerQueried: false, code: 'TX_INDEX_ABSENT', txid: got };
    }
    const queried = spawnSync(opts.cliPath || CARDANO_CLI, [
      'query', 'utxo', '--socket-path', opts.socket, '--testnet-magic', boundMagic.text,
      '--output-json', '--tx-in', `${got}#${opts.outputIndex}`,
    ], { encoding: 'utf8' });
    let present = false;
    if (queried.status === 0) {
      try {
        const parsed = JSON.parse(queried.stdout || '{}');
        present = Boolean(parsed && Object.prototype.hasOwnProperty.call(parsed, `${got}#${opts.outputIndex}`));
      } catch {
        present = false;
      }
    }
    lastPoll = {
      status: present ? 'confirmed' : 'unconfirmed',
      unitOnly: false,
      providerQueried: true,
      txid: got,
      outputIndex: opts.outputIndex,
      code: queried.status !== 0 ? 'PROVIDER_QUERY_FAILED' : (present ? 'PROVIDER_OUTPUT_PRESENT' : 'PROVIDER_OUTPUT_ABSENT'),
    };
    return lastPoll;
  };
  try {
    const bound = await bindReceipt({
      helperPath: RECEIPT_HELPER_PATH,
      label,
      signedCborHex: cborHex,
      cliPath: opts.cliPath || CARDANO_CLI,
      receiptRoot,
      submit,
      pollConfirmation,
      confirmationBudgetMs: opts.confirmationBudgetMs ?? 0,
      callerHash: opts.localTxid,
    });
    const receipt = bound.receipt || {};
    const helperConfirmed = receipt.status === 'confirmed';
    const ledgerConfirmed = helperConfirmed === true && unitOnly !== true && real && armed;
    const failureClass = bound.failureClass || receipt.failureClass || null;
    if (existsSync(dir)) {
      mkdirSync(evidenceDir, { recursive: true });
      cpSync(dir, evidenceDir, { recursive: true });
    }
    return {
      ok: ledgerConfirmed,
      code: helperConfirmed
        ? (ledgerConfirmed ? 'RECEIPT_CONFIRMED' : 'RECEIPT_UNIT_CONFIRMATION_IGNORED')
        : (failureClass || receipt.status || 'RECEIPT_RECORDED'),
      invoked: true,
      callbacksSupplied: true,
      calls,
      helperSha256: bound.helperSha256 || null,
      helperStatus: receipt.status || null,
      helperConfirmed,
      confirmed: ledgerConfirmed,
      unitOnly,
      unitConfirmationIgnored: helperConfirmed && unitOnly === true,
      signedFlagIsNotProof: true,
      ledgerExecution: false,
      nativePins: false,
      failureClass,
      computedHash: receipt.computedHash || null,
      bodySha256: opts.bodySha256 || null,
      localTxid: opts.localTxid || null,
      receiptRoot,
      label,
      providerQueried: lastPoll?.providerQueried === true,
      kept: opts.keepReceipt === true,
    };
  } catch (error) {
    return closed(error?.failureClass || 'receipt-error', {
      invoked: true,
      callbacksSupplied: true,
      calls,
      confirmed: false,
      helperConfirmed: false,
      unitOnly: true,
      failureClass: error?.failureClass || null,
      signedFlagIsNotProof: true,
    });
  } finally {
    if (opts.keepReceipt !== true) rmSync(dir, { recursive: true, force: true });
  }
}

function inputRef(input) {
  return `${input.txId}#${input.index}`;
}

export function terminalSpendPair(kind, terminal, request) {
  const sibling = request?.calibratedSiblingTerminals?.[kind];
  if (request?.productionSiblingAdmitted === true && sibling && sibling.lower != null && sibling.upper != null) {
    return { lower: String(sibling.lower), upper: String(sibling.upper), probed: false, productionSibling: true };
  }
  const probe = request?.representableSpendBounds?.[kind];
  if (probe && probe.lower != null && probe.upper != null) {
    return { lower: String(probe.lower), upper: String(probe.upper), probed: true, productionSibling: false };
  }
  const lower = terminal?.args?.lower ?? terminal?.redeemer?.lower;
  const upper = terminal?.args?.upper ?? terminal?.redeemer?.upper;
  return {
    lower: lower == null ? '' : String(lower),
    upper: upper == null ? '' : String(upper),
    probed: false,
    productionSibling: false,
  };
}

function redeemerForSpend(terminal, pair) {
  const plutus = terminal?.redeemer?.plutus || null;
  if (!pair?.probed || !plutus || !Array.isArray(plutus.fields)) return plutus;
  const lower = Number(pair.lower);
  const upper = Number(pair.upper);
  if (!Number.isSafeInteger(lower) || !Number.isSafeInteger(upper)) return plutus;
  const fields = plutus.fields.slice();
  fields[3] = { int: lower };
  fields[4] = { int: upper };
  return { ...plutus, fields };
}

export function buildApprovedFunding(stages, request, spec) {
  const manifest = JSON.parse(readFileSync(MANIFEST_PATH, 'utf8'));
  const hashes = stages.source?.hashes;
  if (!stages.inputs?.cashSeed || !stages.inputs.mintFee || !stages.inputs.mintCollateral) {
    return {
      cashCompile: null,
      headApply: null,
      mint: closed('CASH_SEED_ABSENT', { cashCanMint: false, ledgerFunding: false }),
      body: closed('HEAD_OPEN_BLOCKED', { cashCanMint: false, ledgerFunding: false }),
    };
  }
  if (!hashes?.programHash || !stages.source?.blueprintPath) {
    return {
      cashCompile: null,
      headApply: null,
      mint: closed('SOURCE_COMPILE_FAILED', { compiled: false, cashCanMint: false }),
      body: closed('HEAD_OPEN_BLOCKED', { cashCanMint: false }),
    };
  }
  const seed = stages.inputs.cashSeed;
  const cashRoot = request.evidenceRoot ? join(request.evidenceRoot, 'checks', 'cash-policy') : CASH_WORK;
  const workDir = join(cashRoot, `${seed.txId}-${seed.index}`);
  const frozenCash = join(FROZEN_CASH_WORK, `${seed.txId}-${seed.index}`, 'cash-policy.json');
  if (!existsSync(join(workDir, 'cash-policy.json')) && existsSync(frozenCash)) {
    mkdirSync(workDir, { recursive: true });
    writeFileSync(join(workDir, 'cash-policy.json'), readFileSync(frozenCash));
  }
  const cashCompile = buildApprovedCashPolicy({
    sourcePath: request.cashDesign.sourcePath,
    seedTxId: seed.txId,
    seedIndex: seed.index,
    workDir,
  });
  if (!cashCompile.ok) {
    return { cashCompile, headApply: null, mint: { ...cashCompile, cashCanMint: false }, body: closed('HEAD_OPEN_BLOCKED', { cashCanMint: false }) };
  }
  const head = stages.inputs.head;
  const headApply = applyHeadToCashPolicy({
    blueprintPath: stages.source.blueprintPath,
    cashPolicyId: cashCompile.policyId,
    hashes,
    manifest,
    seedTxId: head.txId,
    seedIndex: head.index,
    workDir: join(workDir, 'head'),
  });
  if (!headApply.ok) {
    return { cashCompile, headApply, mint: closed('HEAD_APPLY_FAILED', { cashCanMint: false }), body: headApply };
  }
  const cashRef = inputRef(seed);
  const headRef = inputRef(head);
  const common = {
    outputLovelace: spec.outputLovelace,
    feeLovelace: spec.feeLovelace,
    invalidBefore: spec.invalidBefore,
    invalidHereafter: spec.invalidHereafter,
    requiredSignerHash: spec.requiredSignerHash,
    paymentVkeyCbor: spec.paymentVkeyCbor,
    networkMagic: spec.networkMagic ?? request.networkMagic ?? request.magic ?? null,
    offlineExecutionUnits: { memory: 800000, steps: 400000 },
    workRoot: workDir,
  };
  const offlineBody = {
    protocolParamsPath: OFFLINE_PROTOCOL_PARAMS,
  };
  const payoutVkeyHex = request.payoutVkeyHex || request.payoutVkeyCbor || `5820${'34'.repeat(32)}`;
  const mint = encodeUnsignedBody({
    ...common,
    ...offlineBody,
    head: seed,
    fee: stages.inputs.mintFee,
    collateral: stages.inputs.mintCollateral,
    mintAssets: [{ quantity: '100', policyId: cashCompile.policyId, assetName: '43617368' }],
    mintScriptPath: cashCompile.scriptPath,
    assets: [{ quantity: '100', policyId: cashCompile.policyId, assetName: '43617368' }],
  }, request.cliPath);
  const mintStage = {
    ...mint,
    cashPolicyId: cashCompile.policyId,
    compiledScript: mint.ok === true,
    markerNotNativeFunding: false,
    cashCanMint: false,
    ledgerFunding: false,
    forbidsPolicy: '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9',
  };
  const holding = mint.txid ? `${mint.txid}#0` : '';
  if (!mint.ok || holding === '' || holding === cashRef || headRef === cashRef) {
    return {
      cashCompile,
      headApply,
      mint: mintStage,
      body: closed('HEAD_OPEN_BLOCKED', { cashSeedReused: headRef === cashRef, cashCanMint: false }),
    };
  }
  const datum = initialHeadDatum(hashes, headApply.instanceId);
  const headFees = openFees(stages.inputs);
  const artifactDir = request.evidenceRoot ? join(request.evidenceRoot, 'artifacts') : join(REPAIR_ROOT, 'artifacts');
  mkdirSync(artifactDir, { recursive: true });
  const opened = encodeUnsignedBody({
    ...common,
    head: head,
    extraTxIns: [holding],
    fee: headFees.fee,
    collateral: headFees.collateral,
    outFile: join(artifactDir, 'head-open.body.json'),
    ...offlineBody,
    mintAssets: [{ quantity: '1', policyId: headApply.appliedHash, assetName: '48656164' }],
    mintScriptPath: headApply.scriptPath,
    outputScriptPath: headApply.scriptPath,
    inlineDatumJson: JSON.stringify(datum.plutus),
    assets: [
      { quantity: '1', policyId: headApply.appliedHash, assetName: '48656164' },
      { quantity: '100', policyId: cashCompile.policyId, assetName: '43617368' },
    ],
  }, request.cliPath);
  const body = {
    ...opened,
    mintTxid: mint.txid || null,
    mintSeparate: mint.ok === true,
    headMintsCash: false,
    cashSeedReused: false,
    holding,
    kernelCash: '100',
    headAsset: '48656164',
    headPolicyId: headApply.appliedHash,
    cashPolicyId: cashCompile.policyId,
    instanceId: headApply.instanceId,
    datumFields: datum.fields,
    compiledScript: opened.ok === true,
    ledgerFunding: false,
    constructorWorkCount: 49,
    collectionCapacity: 128,
  };
  if (!opened.ok) return { cashCompile, headApply, mint: mintStage, body, posts: null, spend: null };
  let posts = null;
  try {
    posts = loadSourceTerminals(readFileSync(stages.source.path || request.sourcePath, 'utf8'), stages.sibling?.calibrated ? { bounds: stages.sibling.measuredBounds } : null);
  } catch (error) {
    posts = closed('SOURCE_POST_REJECTED', { detail: error?.message || 'source-post-threw', bodyBuilt: false });
  }
  const spend = {};
  if (posts?.ok) {
    const openedRef = parseRef(holding);
    for (const kind of ['release', 'refund', 'cancel']) {
      const terminal = posts.terminals[kind];
      const side = kind === 'release' ? stages.inputs.left : kind === 'refund' ? stages.inputs.right : stages.inputs.cancel;
      if (!openedRef || !side?.fee || !side?.collateral || !terminal?.ok) {
        spend[kind] = closed('INPUTS_INCOMPLETE', { kind, bodyBuilt: false });
        continue;
      }
      const postDatum = datumFromPost(hashes, headApply.instanceId, terminal);
      const outFile = join(artifactDir, `${kind}.body.json`);
      const pair = terminalSpendPair(kind, terminal, request);
      const interval = boundSpendInterval(kind, pair.lower, pair.upper, stages.sibling?.calibrated ? stages.sibling : null);
      const decoded = postDatum?.plutus
        ? decodePostedDatum(postDatum.plutus, { headQuantity: 1, continuingCash: 0, payoutCash: 100 })
        : null;
      const sideRedeemer = {
        recipient: terminal.redeemer?.payoutParty || null,
        transferId: terminal.redeemer?.transferId || null,
        amount: terminal.redeemer?.amount,
      };
      if (!interval.ok) {
        spend[kind] = {
          ...interval,
          frame: decoded,
          redeemer: sideRedeemer,
          economicAction: judgeEconomicAction(decoded, kind, { redeemer: sideRedeemer }),
          obligationsPreserved: true,
          fenceBypass: false,
          bodyBuilt: false,
          bodyFile: null,
        };
        continue;
      }
      spend[kind] = {
        ...encodeSpendBody({
          kind,
          head: openedRef,
          fee: side.fee,
          collateral: side.collateral,
          requiredSignerHash: spec.requiredSignerHash,
          paymentVkeyCbor: spec.paymentVkeyCbor,
          payoutVkeyHex,
          networkMagic: common.networkMagic,
          offlineExecutionUnits: common.offlineExecutionUnits,
          workRoot: workDir,
          spendScriptPath: headApply.scriptPath,
          redeemerJson: JSON.stringify(redeemerForSpend(terminal, pair)),
          invalidBefore: interval.invalidBefore,
          invalidHereafter: interval.invalidHereafter,
          feeLovelace: spec.feeLovelace,
          protocolParamsPath: OFFLINE_PROTOCOL_PARAMS,
          outFile,
          outputs: [
            {
              lovelace: spec.outputLovelace,
              lovelaceText: typeof spec.outputLovelace === 'string' ? spec.outputLovelace : undefined,
              inlineDatumJson: JSON.stringify(postDatum.plutus),
              assets: [{ quantity: '1', policyId: headApply.appliedHash, assetName: '48656164' }],
            },
            {
              lovelace: spec.outputLovelace,
              lovelaceText: typeof spec.outputLovelace === 'string' ? spec.outputLovelace : undefined,
              assets: [{ quantity: '100', policyId: cashCompile.policyId, assetName: '43617368' }],
            },
          ],
        }, request.cliPath),
        frame: expectedFrame(terminal, { ...hashes, instanceId: headApply.instanceId }),
        obligationsPreserved: true,
        fenceBypass: false,
        timeSeparation: interval.timeSeparation,
        encodeSpendBodyCalled: true,
        cliIntervalEmitted: true,
      };
    }
  }
  return { cashCompile, headApply, mint: mintStage, body, posts, spend };
}

function parseRef(text) {
  const match = String(text || '').match(/^([0-9a-f]{64})#(\d+)$/);
  if (!match) return null;
  return { txId: match[1], index: Number(match[2]) };
}

function costExceeds(protocol, cost, txBytes) {
  const units = protocol?.maxTxExecutionUnits;
  if (!units || !cost || cost.memory == null || cost.steps == null) return false;
  try {
    if (BigInt(cost.memory) > BigInt(units.memory) || BigInt(cost.steps) > BigInt(units.steps)) return true;
  } catch {
    return true;
  }
  return protocol.maxTxSize != null && txBytes != null && Number(txBytes) > Number(protocol.maxTxSize);
}

export async function adoptHelper77(request = {}) {
  const flags = {
    nativeBudgetMeasured: false,
    ledgerExecuted: false,
    ledgerExecution: false,
    liveProtocolQueried: false,
    budgetPin: false,
    valuesAreBudgetPins: false,
    feeAppliesToSignedTx: false,
    selfApproval: false,
    helper77: true,
    unitOnly: true,
    evidenceClass: 'offline-pinned-protocol-file',
  };
  try {
    const calculated = request.kind === 'body-fee'
      ? await calculateBodyMinimumFee(request)
      : await calculateOutputMinimum(request);
    return {
      ...calculated,
      ...flags,
      ok: calculated.calculated === true,
      code: request.kind === 'body-fee' ? 'HELPER77_FEE_OBSERVED' : 'HELPER77_COIN_OBSERVED',
      nativeBudgetMeasured: false,
      ledgerExecuted: false,
      ledgerExecution: false,
      budgetPin: false,
      feeEstablishesFeeAfterRealExecutionUnits: false,
    };
  } catch (error) {
    const code = error instanceof OutputFeeError ? error.code : 'HELPER77_FAILED';
    return { ok: false, code, ...flags, detail: error?.message || 'helper77-threw' };
  }
}

const FINANCIAL_SLOTS = [
  'fin-cash-seed', 'fin-head', 'fin-mint-fee', 'fin-mint-collateral',
  'fin-head-fee', 'fin-head-collateral', 'fin-release-fee', 'fin-release-collateral',
  'fin-refund-fee', 'fin-refund-collateral', 'fin-cancel-fee', 'fin-cancel-collateral',
];

function inputsFromDispatch(dispatch) {
  if (!dispatch || !Array.isArray(dispatch.results)) return null;
  const rows = FINANCIAL_SLOTS.map((id) => dispatch.results.find((item) => item.id === id) || null);
  if (rows.some((item) => item == null)) return null;
  const failed = rows.find((item) => item.ok !== true || !/^[0-9a-f]{64}$/.test(String(item.txId || '')));
  if (failed) return { ok: false, code: failed.code || 'FINANCIAL_ROLES_ABSENT' };
  if (rows.some((item) => !/^(?:0|[1-9][0-9]*)$/.test(String(item.lovelaceText || '')))) {
    return { ok: false, code: 'BALANCE_INPUT_MALFORMED' };
  }
  const utxos = rows.map((item) => ({ txId: item.txId, index: item.index, lovelaceText: String(item.lovelaceText) }));
  const selected = selectRunInputs(utxos);
  if (!selected.ok) return selected;
  return {
    ...selected,
    twelveInputFrameSatisfied: true,
    roleSource: 'producer-dispatch',
    inputValues: utxos.map((item) => ({ ref: `${item.txId}#${item.index}`, lovelaceText: item.lovelaceText })),
  };
}

function valuesFor(inputs, names) {
  const rows = inputs.inputValues || [];
  return names.map((name) => {
    const input = inputs[name] || null;
    const ref = input ? `${input.txId}#${input.index}` : '';
    return rows.find((row) => row.ref === ref) || null;
  }).filter((row) => row);
}

function protocolFeeParts(protocolPath) {
  const text = readFileSync(protocolPath, 'utf8');
  const pick = (...names) => {
    for (const name of names) {
      const matched = text.match(new RegExp(`"${name}"\\s*:\\s*([0-9]+)`));
      if (matched) return matched[1];
    }
    return null;
  };
  const minFeeA = pick('minFeeA', 'txFeePerByte');
  const minFeeB = pick('minFeeB', 'txFeeFixed');
  if (!minFeeA || !minFeeB) return null;
  return { minFeeA, minFeeB };
}

function feeSeed(parts, coin) {
  let fee = parts.minFeeB;
  if (fee === coin) fee = (BigInt(parts.minFeeB) + BigInt(parts.minFeeA)).toString();
  if (!/^[1-9][0-9]*$/.test(fee) || fee === coin || fee === '100') return null;
  return fee;
}

function enterpriseAddress(cliPath, vkeyHex, magicText, workDir) {
  const vkeyPath = join(workDir, 'pay.vkey');
  writeFileSync(vkeyPath, JSON.stringify({
    type: 'PaymentVerificationKeyShelley_ed25519',
    description: 'Role verification key. Not a signing key.',
    cborHex: vkeyHex,
  }));
  const built = spawnSync(cliPath, [
    'address', 'build', '--payment-verification-key-file', vkeyPath, '--testnet-magic', String(magicText),
  ], { encoding: 'utf8' });
  const address = String(built.stdout || '').trim();
  if (built.status !== 0 || !address) return { ok: false, code: 'BODY_ENCODE_FAILED', detail: built.stderr || '' };
  return { ok: true, address };
}

function unitCliIdentity(request) {
  const cliPath = request.cliPath || CARDANO_CLI;
  return typeof request.ownedProducer?.unitSpawn === 'function' && !realCli(cliPath) ? 'unit-stub' : null;
}

async function measuredCoin(request, workDir, address, datum, assets, referenceScriptPath) {
  const protocolPath = OFFLINE_PROTOCOL_PARAMS;
  const protocolSha = createHash('sha256').update(readFileSync(protocolPath)).digest('hex');
  const calculated = await calculateOutputMinimum({
    cliPath: request.cliPath || CARDANO_CLI,
    protocolPath,
    protocolProvenance: protocolSha,
    cliIdentity: unitCliIdentity(request),
    workDir,
    output: {
      address,
      lovelace: '0',
      assets,
      datumData: datum || undefined,
      referenceScriptPath: referenceScriptPath || undefined,
    },
  });
  return { ...calculated, protocolSha };
}

async function convergeFee(encodeAtFee, request, workDir, referenceScriptSize, coin) {
  const protocolPath = OFFLINE_PROTOCOL_PARAMS;
  const parts = protocolFeeParts(protocolPath);
  if (!parts) return { ok: false, code: 'FEE_UNMEASURED', body: null, fee: null };
  let fee = feeSeed(parts, coin);
  if (!fee) return { ok: false, code: 'FEE_UNMEASURED', body: null, fee: null };
  let body = null;
  let feeResult = null;
  for (let pass = 0; pass < 4; pass += 1) {
    body = encodeAtFee(fee);
    if (!body.ok) return { ok: false, code: body.code, body, fee, feeResult };
    feeResult = await calculateBodyMinimumFee({
      cliPath: request.cliPath || CARDANO_CLI,
      protocolPath,
      protocolProvenance: createHash('sha256').update(readFileSync(protocolPath)).digest('hex'),
      cliIdentity: unitCliIdentity(request),
      bodyPath: body.bodyFile,
      workDir: join(workDir, `fee-${pass}`),
      witnessCount: 1,
      witnessCountProvenance: 'ASSUMED-unsigned-body-no-key-analysis',
      referenceScriptSize,
      requiresReferenceScriptFee: referenceScriptSize != null,
    });
    const next = String(feeResult.parsed?.fee || '');
    if (!/^[1-9][0-9]*$/.test(next) || next === '100' || next === body.quantities.outputLovelace) {
      return { ok: false, code: 'FEE_UNMEASURED', body, fee: next, feeResult };
    }
    if (next === fee) return { ok: true, code: 'FEE_CONVERGED', body, fee, feeResult, passes: pass + 1 };
    fee = next;
  }
  return { ok: false, code: 'FEE_DRIFT', body, fee, feeResult };
}

async function recountSignedWitnessFee(stages, request) {
  const counted = stages.witnessCount?.counted;
  const body = stages.body;
  const currentFee = stages.outputFee?.parsed?.fee;
  if (stages.witnessCount?.witnessCountVerified !== true || !Number.isInteger(counted) || counted < 1) return;
  if (!body?.bodyFile || !body?.cborHex || currentFee == null) return;
  const preserved = body.cborHex;
  const protocolPath = OFFLINE_PROTOCOL_PARAMS;
  let recounted;
  try {
    recounted = await calculateBodyMinimumFee({
      cliPath: request.cliPath || CARDANO_CLI,
      protocolPath,
      protocolProvenance: createHash('sha256').update(readFileSync(protocolPath)).digest('hex'),
      cliIdentity: unitCliIdentity(request),
      bodyPath: body.bodyFile,
      workDir: join(request.evidenceRoot || tmpdir(), 'fee-recount'),
      witnessCount: counted,
      witnessCountProvenance: 'counted-from-signing-key-paths',
      referenceScriptSize: stages.outputFee.referenceScriptSize ?? null,
      requiresReferenceScriptFee: stages.outputFee.referenceScriptSize != null,
    });
  } catch (error) {
    stages.outputFee = {
      ...stages.outputFee,
      feeWitnessRecounted: false,
      feeMatchesSignedWitnessCount: false,
      feeRecountCode: error?.code || 'FEE_UNMEASURED',
      nativeBudgetMeasured: false,
      ledgerExecution: false,
    };
    return;
  }
  const next = String(recounted.parsed?.fee || '');
  const stored = JSON.parse(readFileSync(body.bodyFile, 'utf8'));
  const filePreserved = stored.cborHex === preserved && stages.body.cborHex === preserved;
  const matched = next === String(currentFee) && String(recounted.witnessCount) === String(counted) && filePreserved;
  stages.outputFee = {
    ...stages.outputFee,
    feeWitnessRecounted: true,
    feeWitnessCount: String(counted),
    witnessCount: String(counted),
    witnessCountProvenance: recounted.witnessCountProvenance,
    feeMatchesSignedWitnessCount: matched,
    recountedFee: next,
    nativeBudgetMeasured: false,
    ledgerExecution: false,
    code: matched ? stages.outputFee.code : 'FEE_WITNESS_COUNT_DIVERGED',
    ok: matched ? stages.outputFee.ok : false,
  };
}

function plutusCostUnits(cliPath, bodyFile, context = {}) {
  const before = createHash('sha256').update(readFileSync(cliPath)).digest('hex');
  const mode = context.mode === 'online' ? 'online' : 'offline';
  const built = plutusCostArgv({
    cli: cliPath,
    txFile: bodyFile,
    mode,
    startTime: context.startTimeUtc,
    startTimePosix: context.startTimePosix,
    byronGenesis: context.byronGenesis,
    eraHistoryFile: context.eraHistoryFile,
    utxoFile: context.utxoFile,
    protocolParamsFile: OFFLINE_PROTOCOL_PARAMS,
    socket: context.socket,
    magic: context.magic,
    magicSource: context.magicSource,
  });
  if (!built.ok) {
    return {
      cost: { ok: false, code: built.code || 'EVALUATION_CONTEXT_ABSENT' },
      cliSha256: before,
      cliSha256After: before,
      cliDigestStable: true,
      status: null,
      spawned: false,
    };
  }
  if (mode === 'online' && realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return {
      cost: { ok: false, code: 'REAL_NODE_NOT_ARMED', command: built.command },
      cliSha256: before,
      cliSha256After: before,
      cliDigestStable: true,
      status: null,
      spawned: false,
    };
  }
  const args = built.argv.slice(1);
  const run = spawnSync(cliPath, args, { encoding: 'utf8' });
  const after = createHash('sha256').update(readFileSync(cliPath)).digest('hex');
  let parsed = run.stdout;
  try { parsed = JSON.parse(run.stdout); } catch { /* consumePlutusCost rejects text that is not a cost list */ }
  let cost = run.status === 0 ? consumePlutusCost(parsed, { unitOnly: !realCli(cliPath) }) : { ok: false, code: 'EVALUATION_UNAVAILABLE' };
  if (cost.ok === true && cost.cost?.memory === 1 && cost.cost?.steps === 1) {
    cost = { ...cost, ok: false, code: 'SUPPLIED_UNITS_NOT_A_MEASUREMENT' };
  }
  if (cost.ok === true && Array.isArray(cost.items) && cost.items.some((item) => !/^[0-9a-f]{56}$/i.test(String(item.scriptHash || '')))) {
    cost = { ...cost, ok: false, code: 'SCRIPT_HASH_REQUIRED' };
  }
  return { cost, cliSha256: before, cliSha256After: after, cliDigestStable: before === after, status: run.status };
}

function matchRedeemers(cost, bodyOrRows) {
  const body = bodyOrRows && !Array.isArray(bodyOrRows) ? bodyOrRows : { redeemers: bodyOrRows };
  const items = Array.isArray(cost?.items) ? cost.items : [];
  const rows = Array.isArray(body.redeemers) ? body.redeemers : [];
  if (items.length === 0 || items.length !== rows.length) return null;
  const used = new Set();
  const matched = [];
  for (const item of items) {
    const scriptHash = String(item.scriptHash || '').toLowerCase();
    if (!/^[0-9a-f]{56}$/.test(scriptHash)) return null;
    const hit = rows.findIndex((row, index) => !used.has(index)
      && row.memory === item.executionUnits.memory
      && row.steps === item.executionUnits.steps
      && Number.isInteger(row.tag)
      && Number.isInteger(row.index));
    if (hit < 0) return null;
    if (!redeemerPointerAccepts(rows[hit], {
      ordinaryInputs: body.ordinaryInputs,
      scriptRef: body.scriptRef,
      mintPolicyCount: body.mintPolicyCount,
    })) return null;
    used.add(hit);
    matched.push({
      scriptHash,
      tag: rows[hit].tag,
      index: rows[hit].index,
      memory: rows[hit].memory,
      steps: rows[hit].steps,
    });
  }
  return matched;
}

function rebuildWithUnits(encodeAtFee, fee, costWrap, reeval) {
  const cost = costWrap.cost;
  const refused = (code, extra = {}) => ({
    ok: false,
    code,
    applied: false,
    scriptDataHash: null,
    scriptDataHashSource: null,
    scriptDataHashInvented: false,
    nativeBudgetMeasured: false,
    submitBlocked: true,
    matchedRedeemer: null,
    ...extra,
  });
  if (costWrap.cliDigestStable !== true) return refused('CLI_DIGEST_DRIFT', { body: null });
  if (cost?.code === 'SUPPLIED_UNITS_NOT_A_MEASUREMENT') return refused('SUPPLIED_UNITS_NOT_A_MEASUREMENT', { body: null });
  if (cost?.ok === false) return refused(cost.code || 'EVALUATION_UNAVAILABLE', { body: null });
  if (!cost || cost.ok !== true || !Number.isInteger(cost.cost?.memory) || !Number.isInteger(cost.cost?.steps)) {
    return {
      ok: true,
      code: 'PLUTUS_COST_REBUILD_HELD',
      applied: false,
      scriptDataHash: null,
      scriptDataHashSource: null,
      scriptDataHashInvented: false,
      nativeBudgetMeasured: false,
      submitBlocked: true,
      matchedRedeemer: null,
      observedMemory: cost?.cost?.memory ?? null,
      observedSteps: cost?.cost?.steps ?? null,
      cliSha256: costWrap.cliSha256,
      cliSha256After: costWrap.cliSha256After,
      body: null,
    };
  }
  const units = { memory: cost.cost.memory, steps: cost.cost.steps, measured: cost.unitOnly === false };
  let previous = null;
  let body = null;
  let matched = null;
  for (let pass = 0; pass < 4; pass += 1) {
    body = encodeAtFee(fee, units);
    if (!body.ok) return refused(body.code, { body });
    if (!body.scriptDataHash || body.scriptDataHashSource !== 'conway-body-key-11') {
      return refused('SCRIPT_DATA_HASH_ABSENT', { body, observedMemory: units.memory, observedSteps: units.steps });
    }
    matched = matchRedeemers(cost, body);
    if (!matched) return refused('REDEEMER_POINTER_UNMATCHED', { body, scriptDataHash: body.scriptDataHash, observedMemory: units.memory, observedSteps: units.steps });
    const again = reeval(body.bodyFile);
    if (again.cliDigestStable !== true) return refused('CLI_DIGEST_DRIFT', { body });
    if (!again.cost || again.cost.ok !== true) return refused(again.cost?.code || 'EVALUATION_UNAVAILABLE', { body });
    if (again.cost.cost.memory !== units.memory || again.cost.cost.steps !== units.steps) {
      if (again.cost.cost.memory === 1 && again.cost.cost.steps === 1) return refused('SUPPLIED_UNITS_NOT_A_MEASUREMENT', { body });
      units.memory = again.cost.cost.memory;
      units.steps = again.cost.cost.steps;
      previous = null;
      continue;
    }
    if (previous && previous === body.scriptDataHash) {
      const integrity = recomputeScriptDataHash(body.cborHex, {
        spentOutputs: body.spentOutputs,
        spendingInputs: body.spendingInputs,
        referenceUtxos: body.referenceUtxos,
        referenceInputs: body.referenceInputs,
        spendScripts: body.spendScripts,
        protocolPath: body.protocolParamsPath,
      });
      if (integrity.matchesStored !== true || integrity.recomputedHex !== body.scriptDataHash) {
        return refused('SCRIPT_DATA_INTEGRITY_MISMATCH', {
          body,
          scriptDataHash: body.scriptDataHash,
          scriptDataHashSource: body.scriptDataHashSource,
          scriptDataHashInvented: false,
          integrity,
        });
      }
      return {
        ok: true,
        code: 'PLUTUS_COST_REBUILD_APPLIED',
        applied: true,
        scriptDataHash: body.scriptDataHash,
        scriptDataHashSource: 'conway-body-key-11',
        scriptDataHashInvented: false,
        scriptDataHashRecomputed: integrity.recomputedHex,
        integrityMatchesStored: true,
        fromWitnessPresent: integrity.fromWitnessPresent === true,
        nativeBudgetMeasured: false,
        submitBlocked: true,
        matchedRedeemer: matched[0],
        observedMemory: units.memory,
        observedSteps: units.steps,
        cliSha256: costWrap.cliSha256,
        cliSha256After: again.cliSha256After,
        passes: pass + 1,
        body,
      };
    }
    previous = body.scriptDataHash;
  }
  return refused('SCRIPT_DATA_HASH_DRIFT', {
    body,
    scriptDataHash: body?.scriptDataHash || null,
    observedMemory: units.memory,
    observedSteps: units.steps,
  });
}

export async function assembleConsumedFinancial(stages, request) {
  const inputs = stages.inputs;
  const workDir = join(request.evidenceRoot || join(tmpdir(), 'moriarty-financial'), 'assemble');
  mkdirSync(workDir, { recursive: true });
  const datum = request.detailedData;
  if (!datum || datum.constructor !== 0 || !Array.isArray(datum.fields) || datum.fields.length !== 13) {
    const absent = closed('DETAILED_DATUM_ABSENT', { fieldCount: Array.isArray(datum?.fields) ? datum.fields.length : 0 });
    return { outputFee: absent, mint: absent, body: absent, rebuild: absent, spend: absent };
  }
  const magic = stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null;
  const magicText = genesisMagicText(magic);
  if (!magicText.ok) {
    const refused = closed(magicText.code, { spawned: false });
    return { outputFee: refused, mint: refused, body: refused, rebuild: refused, spend: refused };
  }
  const headInterval = consumedHeadInterval(stages.time?.windows);
  if (!headInterval.ok) {
    const window = closed(headInterval.code || 'TIME_WINDOW_ABSENT', { spawned: false, cliIntervalEmitted: false });
    return { outputFee: window, mint: window, body: window, rebuild: window, spend: window };
  }
  const vkey = String(request.paymentVkeyCbor || '');
  const address = enterpriseAddress(request.cliPath || CARDANO_CLI, vkey, magicText.text, workDir);
  if (!address.ok) {
    const failed = closed(address.code, { detail: address.detail, spawned: false });
    return { outputFee: failed, mint: failed, body: failed, rebuild: failed, spend: failed };
  }
  const refStat = request.referenceScriptPath ? statSync(request.referenceScriptPath) : null;
  if (request.referenceScriptPath && (!refStat?.isFile())) {
    const missing = closed('REFERENCE_SCRIPT_MISSING', { spawned: false });
    return { outputFee: missing, mint: missing, body: missing, rebuild: missing, spend: missing };
  }
  const referenceScriptSize = refStat ? refStat.size : null;
  const cashAssets = [{ policyId: UNIT_CASH_POLICY, assetNameHex: '43617368', quantity: 100 }];
  let coin;
  try {
    coin = await measuredCoin(request, join(workDir, 'coin'), address.address, datum, cashAssets, request.referenceScriptPath || null);
  } catch (error) {
    const failed = closed(error?.code || 'HELPER77_FAILED', { detail: error?.message || 'coin-threw', spawned: false });
    return { outputFee: failed, mint: failed, body: failed, rebuild: failed, spend: failed };
  }
  const measuredText = firstCoinText([
    coin.minimumLovelace,
    coin.lovelaceText,
    coin.parsed?.lovelaceText,
    coin.parsed?.lovelace,
    coin.lovelace,
  ]);
  const measuredMirror = firstCoinMirror([coin.lovelace, coin.parsed?.lovelace, coin.minimumLovelace]);
  const coinRead = readWord64Coin(measuredText, measuredMirror);
  if (!coinRead.ok) {
    const failed = closed('LEDGER_COIN_REFUSED', {
      reason: coinRead.reason || 'noncanonical',
      spawned: false,
      minAdaMeasured: false,
      protocolMinAda: 'unmeasured',
    });
    return { outputFee: failed, mint: failed, body: failed, rebuild: failed, spend: failed };
  }
  const outputLovelace = coinRead.digits;
  let startTimeUtc = null;
  if (request.time?.genesisPath && existsSync(request.time.genesisPath)) {
    try { startTimeUtc = JSON.parse(readFileSync(request.time.genesisPath, 'utf8')).systemStart || null; } catch { startTimeUtc = null; }
  }
  const eraHistoryFile = join(workDir, 'era-history.json');
  const utxoFile = join(workDir, 'cost-utxo.json');
  writeFileSync(eraHistoryFile, `${JSON.stringify({ type: 'EraHistory', description: 'unit era-history envelope, not a node query', cborHex: '9fff' })}\n`);
  const utxoBody = {};
  for (const row of inputs.inputValues || []) utxoBody[row.ref] = { value: { lovelace: row.lovelaceText } };
  writeFileSync(utxoFile, `${JSON.stringify(utxoBody)}\n`);
  const costContext = {
    mode: request.costMode === 'online' ? 'online' : 'offline',
    startTimeUtc,
    eraHistoryFile,
    utxoFile,
    socket: request.costSocket || request.readback?.socket || request.time?.socket || '',
    magic: Number(magicText.text),
    magicSource: request.costMagicSource || stages.ownedProducer?.schema71?.magicSource || null,
  };
  const base = {
    outputLovelace,
    invalidBefore: headInterval.invalidBefore,
    invalidHereafter: headInterval.invalidHereafter,
    paymentVkeyCbor: vkey,
    networkMagic: Number(magicText.text),
    protocolParamsPath: OFFLINE_PROTOCOL_PARAMS,
    requireChange: true,
  };
  const mintFile = join(workDir, 'cash-mint.body.json');
  const encodeMint = (fee, units = null) => encodeUnsignedBody({
    ...base,
    feeLovelace: fee,
    head: inputs.cashSeed,
    fee: inputs.mintFee,
    collateral: inputs.mintCollateral,
    inputValues: valuesFor(inputs, ['cashSeed', 'mintFee', 'mintCollateral']),
    outFile: mintFile,
    offlineExecutionUnits: units,
    mintAssets: [{ quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' }],
    assets: [{ quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' }],
  }, request.cliPath);
  const minted = await convergeFee(encodeMint, request, join(workDir, 'mint-fee'), referenceScriptSize, outputLovelace);
  if (!minted.ok) {
    const mint = {
      ...(minted.body || closed(minted.code)),
      cashPolicyId: UNIT_CASH_POLICY,
      compiledScript: false,
      markerNotNativeFunding: true,
      cashCanMint: false,
      ledgerFunding: false,
      kernelCash: '100',
    };
    return { outputFee: { ...coin, ok: false, code: minted.code }, mint, body: closed('HEAD_OPEN_BLOCKED'), rebuild: closed(minted.code), spend: null };
  }
  const mintCost = plutusCostUnits(request.cliPath || CARDANO_CLI, minted.body.bodyFile, costContext);
  const mintRebuild = rebuildWithUnits(encodeMint, minted.fee, mintCost, (file) => plutusCostUnits(request.cliPath || CARDANO_CLI, file, costContext));
  const mint = {
    ...(mintRebuild.body || minted.body || closed(mintRebuild.code)),
    code: mintRebuild.ok ? 'UNSIGNED_BODY' : mintRebuild.code,
    ok: mintRebuild.ok === true,
    cashPolicyId: UNIT_CASH_POLICY,
    compiledScript: false,
    markerNotNativeFunding: true,
    cashCanMint: false,
    ledgerFunding: false,
    kernelCash: '100',
    rebuild: mintRebuild,
  };
  if (!mintRebuild.ok) {
    return { outputFee: { ...coin, ok: false, code: mintRebuild.code }, mint, body: closed('HEAD_OPEN_BLOCKED'), rebuild: mintRebuild, spend: null };
  }
  const holding = `${mint.txid}#${mint.headOutputIndex}`;
  const headFile = join(workDir, 'head-open.body.json');
  const headInputs = [
    ...valuesFor(inputs, ['head', 'headFee', 'headCollateral']),
    { ref: holding, lovelaceText: outputLovelace },
  ];
  const encodeHead = (fee, units = null) => encodeUnsignedBody({
    ...base,
    feeLovelace: fee,
    head: inputs.head,
    fee: inputs.headFee,
    collateral: inputs.headCollateral,
    extraTxIns: [holding],
    inputValues: headInputs,
    outFile: headFile,
    offlineExecutionUnits: units,
    inlineDatumJson: JSON.stringify(datum),
    mintAssets: [{ quantity: '1', policyId: UNIT_HEAD_POLICY, assetName: '48656164' }],
    assets: [
      { quantity: '1', policyId: UNIT_HEAD_POLICY, assetName: '48656164' },
      { quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' },
    ],
  }, request.cliPath);
  const openedFee = await convergeFee(encodeHead, request, join(workDir, 'head-fee'), referenceScriptSize, outputLovelace);
  let body = openedFee.body ? {
    ...openedFee.body,
    holding,
    mintTxid: mint.txid,
    mintSeparate: mint.ok === true,
    headMintsCash: false,
    kernelCash: '100',
    headAsset: '48656164',
    datumFields: datum.fields,
    datumFieldCount: datum.fields.length,
    detailedConstructor: true,
    markerNotNativeFunding: true,
    ledgerFunding: false,
  } : closed(openedFee.code || 'HEAD_OPEN_BLOCKED');
  if (!openedFee.ok) {
    return {
      outputFee: { ...coin, ...openedFee.feeResult, ok: false, code: openedFee.code, referenceScriptSize },
      mint,
      body,
      rebuild: closed(openedFee.code),
      spend: null,
    };
  }
  const costWrap = plutusCostUnits(request.cliPath || CARDANO_CLI, body.bodyFile, costContext);
  const rebuild = rebuildWithUnits(encodeHead, openedFee.fee, costWrap, (file) => plutusCostUnits(request.cliPath || CARDANO_CLI, file, costContext));
  if (rebuild.body?.ok) {
    body = {
      ...body,
      ...rebuild.body,
      holding,
      mintTxid: mint.txid,
      datumFieldCount: 13,
      detailedConstructor: true,
      markerNotNativeFunding: true,
      ledgerFunding: false,
    };
  }
  const spend = {};
  const openedRef = { txId: body.txid, index: body.headOutputIndex };
  const openFields = plutusToFields(datum);
  const identity = {
    programHash: openFields?.program_hash || null,
    boundsHash: openFields?.bounds_hash || null,
    sourceHash: openFields?.source_hash || null,
    schemaHash: openFields?.schema_hash || null,
    seedHash: openFields?.seed_hash || null,
    instanceId: openFields?.instance_id || null,
  };
  const sourcePath = stages.sibling?.calibrated ? stages.sibling.sourcePath : (request.sourcePath || request.sibling?.sourcePath || null);
  let posts = null;
  if (!sourcePath || !existsSync(sourcePath)) posts = closed('SOURCE_POST_ABSENT');
  else {
    try {
      posts = loadSourceTerminals(readFileSync(sourcePath, 'utf8'), stages.sibling?.calibrated ? { bounds: stages.sibling.measuredBounds } : null);
    } catch (error) {
      posts = closed('SOURCE_POST_REJECTED', { detail: error?.message || 'source-post-threw' });
    }
  }
  const effectMap = {};
  for (const kind of ['release', 'refund', 'cancel']) {
    const side = kind === 'release' ? inputs.left : kind === 'refund' ? inputs.right : inputs.cancel;
    const sideValues = [
      { ref: `${openedRef.txId}#${openedRef.index}`, lovelaceText: outputLovelace },
      ...inputs.inputValues.filter((row) => row.ref === `${side.fee.txId}#${side.fee.index}` || row.ref === `${side.collateral.txId}#${side.collateral.index}`),
    ];
    const terminal = posts?.ok ? posts.terminals[kind] : null;
    const pair = terminalSpendPair(kind, terminal, request);
    const kindLower = pair.lower;
    const kindUpper = pair.upper;
    if (!terminal?.ok || !/^(?:0|[1-9][0-9]*)$/.test(String(kindLower ?? '')) || !/^(?:0|[1-9][0-9]*)$/.test(String(kindUpper ?? ''))) {
      spend[kind] = closed(terminal?.ok ? 'TIME_WINDOW_ABSENT' : (posts?.code || 'SOURCE_POST_ABSENT'), { kind, spawned: false });
      continue;
    }
    const interval = boundSpendInterval(kind, kindLower, kindUpper, stages.sibling?.calibrated ? stages.sibling : null);
    const postDatum = datumFromPost(identity, identity.instanceId, terminal);
    const decoded = postDatum?.plutus
      ? decodePostedDatum(postDatum.plutus, { headQuantity: 1, continuingCash: 0, payoutCash: 100 })
      : null;
    const sideRedeemer = {
      recipient: terminal.redeemer?.payoutParty || null,
      transferId: terminal.redeemer?.transferId || null,
      amount: terminal.redeemer?.amount,
    };
    if (!interval.ok) {
      spend[kind] = {
        ...interval,
        frame: decoded,
        redeemer: sideRedeemer,
        effectDatumClass: postDatum ? 'source-terminal-plutus' : 'open-head-datum',
        spendMintEmpty: true,
        economicAction: judgeEconomicAction(decoded, kind, { redeemer: sideRedeemer }),
        bodyBuilt: false,
        bodyFile: null,
      };
      continue;
    }
    const encoded = encodeSpendBody({
      kind,
      head: openedRef,
      fee: side.fee,
      collateral: side.collateral,
      paymentVkeyCbor: vkey,
      payoutVkeyHex: kind === 'release'
        ? (request.lenderVkeyCbor || request.payoutVkeyCbor)
        : (request.borrowerVkeyCbor || request.payoutVkeyCbor),
      spendScriptPath: request.spendScriptPath,
      redeemerJson: JSON.stringify(redeemerForSpend(terminal, pair) || { constructor: 0, fields: [{ bytes: Buffer.from(kind, 'utf8').toString('hex') }] }),
      invalidBefore: String(interval.invalidBefore),
      invalidHereafter: String(interval.invalidHereafter),
      feeLovelace: openedFee.fee,
      networkMagic: Number(magicText.text),
      protocolParamsPath: OFFLINE_PROTOCOL_PARAMS,
      offlineExecutionUnits: rebuild.applied ? { memory: rebuild.observedMemory, steps: rebuild.observedSteps, measured: false } : null,
      requireChange: true,
      outFile: join(workDir, `${kind}.body.json`),
      inputValues: sideValues,
      outputs: [
        {
          lovelace: outputLovelace,
          lovelaceText: outputLovelace,
          inlineDatumJson: JSON.stringify(postDatum?.plutus || datum),
          assets: [{ quantity: '1', policyId: UNIT_HEAD_POLICY, assetName: '48656164' }],
        },
        {
          lovelace: outputLovelace,
          lovelaceText: outputLovelace,
          assets: [{ quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' }],
        },
      ],
    }, request.cliPath);
    spend[kind] = {
      ...encoded,
      invalidBefore: interval.invalidBefore,
      invalidHereafter: interval.invalidHereafter,
      timeSeparation: interval.timeSeparation,
      encodeSpendBodyCalled: true,
      cliIntervalEmitted: true,
      frame: decoded,
      redeemer: sideRedeemer,
      effectDatumClass: postDatum ? 'source-terminal-plutus' : 'open-head-datum',
      spendMintEmpty: true,
      economicAction: judgeEconomicAction(decoded, kind, { redeemer: sideRedeemer }),
    };
    if (postDatum && encoded.ok && encoded.txid) {
      const headIndex = Number.isInteger(encoded.builtHeadOutputIndex) ? encoded.builtHeadOutputIndex : 0;
      const payoutIndex = Number.isInteger(encoded.builtPayoutOutputIndex) ? encoded.builtPayoutOutputIndex : 1;
      if (headIndex !== payoutIndex) {
        effectMap[`${encoded.txid}#${headIndex}`] = {
          address: encoded.outputAddress,
          value: { lovelace: '2000000', [UNIT_HEAD_POLICY]: { '48656164': '1' } },
          inlineDatum: postDatum.plutus,
          evidenceClass: 'unit-fixture',
        };
        effectMap[`${encoded.txid}#${payoutIndex}`] = {
          address: encoded.payoutAddress,
          value: { lovelace: '2000000', [UNIT_CASH_POLICY]: { '43617368': '100' } },
          evidenceClass: 'unit-fixture',
        };
      }
    }
  }
  if (request.evidenceRoot && Object.keys(effectMap).length) {
    writeFileSync(join(request.evidenceRoot, 'effect-utxo.json'), JSON.stringify(effectMap));
  }
  const failedSpend = ['release', 'refund', 'cancel'].map((kind) => spend[kind]).find((item) => item?.ok === false);
  const spendStage = failedSpend
    ? { ...spend, ok: false, code: failedSpend.code }
    : { ...spend, ok: true, code: 'SPEND_BODIES_ENCODED' };
  return {
    outputFee: {
      ...coin,
      ...openedFee.feeResult,
      ok: true,
      code: 'FINANCIAL_OUTPUT_FEE',
      nativeBudgetMeasured: false,
      ledgerExecution: false,
      referenceScriptSize,
      referenceScriptOmitted: referenceScriptSize == null,
      lovelaceText: outputLovelace,
      lovelace: typeof coinRead.value === 'number' ? coinRead.value : null,
      parsed: { lovelace: outputLovelace, lovelaceText: outputLovelace, fee: openedFee.fee },
    },
    mint,
    body,
    rebuild,
    spend: spendStage,
    posts,
  };
}

function hex56List(values) {
  if (!Array.isArray(values)) return [];
  return values
    .map((value) => String(value || '').trim().toLowerCase())
    .filter((value) => /^[0-9a-f]{56}$/.test(value));
}

function witnessKeyContext(body) {
  const ordinaryKeyHashes = hex56List(body?.ordinaryKeyHashes);
  const collateralKeyHashes = hex56List(body?.collateralKeyHashes);
  const extraKeyHashes = hex56List(body?.signatories);
  const collateralPresent = Boolean(body?.collateralRef) || body?.collateralInLedgerBody === true;
  const resolved = extraKeyHashes.length > 0
    && ordinaryKeyHashes.length > 0
    && (!collateralPresent || collateralKeyHashes.length > 0);
  return { resolved, ordinaryKeyHashes, collateralKeyHashes, extraKeyHashes };
}

export async function runNativeCaller(request = {}) {
  const stages = {};
  let launched = null;
  try {
    stages.runtimeInputs = loadRuntimeInputs();
    stages.authority = loadAuthority(request.authority);
    if (request.cashDesign) stages.cashDesign = loadCashDesignAuthority(request.cashDesign);
    if (request.sourcePath) {
      if (!existsSync(request.sourcePath)) stages.source = closed('SOURCE_ABSENT', { compiled: false });
      else {
        try {
          stages.source = await compileAdmittedSource(request.sourcePath, request.evidenceRoot || null);
        } catch (error) {
          stages.source = closed('SOURCE_COMPILE_FAILED', {
            compiled: false,
            detail: error?.code || error?.message || 'compile-threw',
          });
        }
      }
    }
    if (request.launch) {
      stages.launcher = await inspectLauncher({
        ...request.launch,
        authority: request.authority,
        startNode: request.launch.startNode === true,
      });
      launched = stages.launcher?.pgid ? stages.launcher : null;
    }
    if (typeof request.cashSource === 'string') stages.cash = assessCashPolicy(request.cashSource, request.cashVotes);
    if (request.protocol) stages.protocol = queryProtocolParameters(request.protocol);
    if (stages.protocol?.queried === true && request.cost) {
      if (costExceeds(stages.protocol, request.cost, request.txBytes)) {
        stages.budget = compareBudget({
          queried: true,
          source: stages.protocol.source,
          unitOnly: stages.protocol.unitOnly,
          maxTxExecutionUnits: stages.protocol.maxTxExecutionUnits,
          maxTxSize: stages.protocol.maxTxSize,
          utxoCostPerByte: stages.protocol.utxoCostPerByte ?? null,
          coinsPerUTxOByte: stages.protocol.coinsPerUTxOByte ?? null,
          cost: request.cost,
          txBytes: request.txBytes,
          executionSource: 'supplied-not-measured',
        });
      } else {
        stages.budget = closed('EVALUATION_UNAVAILABLE', {
          submitBlocked: true,
          suppliedCostIgnored: true,
          nativeBudgetMeasured: false,
          minAdaMeasured: false,
          protocolMinAda: 'unmeasured',
          reason: 'supplied-cost-is-not-a-measurement',
        });
      }
    }
    if (request.time) {
      stages.time = measureSlotLength({
        ...request.time,
        genesisPath: stages.launcher?.shelleyGenesisPath || request.time.genesisPath,
        runDir: request.launch?.runDir || request.time.runDir,
      });
    }
    if (request.parityCapture) {
      stages.parityCapture = executeParityCapture(request.parityCapture);
    }
    if (request.time) {
      stages.productionTime = productionParityGate(mergeProductionClock(stages.time, stages.parityCapture));
    }
    const cliPosix = stages.time?.cliPosix ?? request.time?.cliPosix ?? null;
    const lucidPosix = stages.time?.lucidPosix ?? request.time?.lucidPosix ?? null;
    const scriptContextLower = stages.time?.scriptContextLower ?? request.time?.scriptContextLower ?? null;
    if (cliPosix != null || lucidPosix != null || scriptContextLower != null) {
      const parity = compareClockParity({ cliPosix, lucidPosix, scriptContextLower });
      stages.clockParity = parity.code === 'POSIX_PARITY_NOT_RUN'
        ? { ...parity, ok: true, skipped: true }
        : parity;
    }
    if (stages.productionTime?.code === 'PARITY_ADMITTED' && request.sibling) {
      stages.sibling = compileGuardSibling({
        ...request.sibling,
        evidenceRoot: request.evidenceRoot || null,
        production: stages.productionTime,
      });
    } else if (request.sibling) {
      // Not admitted: do not spawn. A non-aiken path is a UNIT refusal, not an unmeasured compile.
      stages.sibling = closed('SIBLING_NOT_COMPILED_IN_THIS_RUN', {
        siblingEmitted: false,
        spawned: false,
        compilerInvoked: false,
        unitOnly: !realAiken(request.sibling.compilerBin),
      });
    }
    if (request.keys) stages.keys = prepareEnterpriseKey({ ...request.keys, authority: request.authority });
    if (request.outputFee) stages.outputFee = await adoptHelper77(request.outputFee);
    if (request.funder) stages.funder = selectOwnedFunder(request.funder);
    if (request.ownedProducer) {
      stages.ownedProducer = produceOwnedRuntime({ ...request.ownedProducer, executeNow: request.ownedProducer.executeNow === true });
      stages.financial = stages.ownedProducer.financial || stages.ownedProducer.schema71?.financial || null;
      const producerReady = stages.ownedProducer.schema71?.inputsReady === true;
      const unitSpawn = request.ownedProducer.unitSpawn;
      const latch = process.env.MORIARTY_ALLOW_REAL_NODE === '1';
      if (producerReady && typeof unitSpawn === 'function') {
        stages.dispatch = dispatchOwnedProducer(stages.ownedProducer.schema71, {
          unitOnly: true,
          spawn: unitSpawn,
        });
      } else if (producerReady && latch && realCli(CARDANO_CLI) && typeof unitSpawn !== 'function') {
        stages.dispatch = dispatchOwnedProducer(stages.ownedProducer.schema71, {
          unitOnly: false,
          spawn: realProducerSpawn,
        });
      }
      if (stages.dispatch?.financial) stages.financial = stages.dispatch.financial;
    }
    if (request.utxoQuery) stages.utxo = queryOwnedUtxos(request.utxoQuery);
    const utxos = request.utxos || stages.utxo?.utxos;
    if (utxos) stages.inputs = selectRunInputs(utxos);
    else if (stages.dispatch) {
      const consumed = inputsFromDispatch(stages.dispatch);
      if (consumed) stages.inputs = consumed;
    }
    // A bare real-cli financial body, or any real-cli body after a time request, stays closed until parity is admitted.
    // An adopted cash design with no time request still encodes the unsigned financial227 body.
    if (stages.sibling?.nativeProbe === true && stages.sibling.measuredBounds
        && isAuthenticProductionAdmission(stages.sibling.productionAdmission)) {
      stages.source = { ...stages.source, path: stages.sibling.sourcePath,
        blueprintPath: stages.sibling.blueprintPath,
        hashes: { programHash: stages.sibling.programHash, boundsHash: stages.sibling.boundsHash, sourceHash: stages.sibling.sourceHash, schemaHash: stages.sibling.schemaHash, seedHash: stages.sibling.seedHash } };
      const bounds = stages.sibling.measuredBounds;
      const partition = BigInt(bounds.partition);
      const releaseUpper = (partition - 1n).toString(10);
      request.productionSiblingAdmitted = true;
      request.calibratedSiblingTerminals = {
        release: { lower: bounds.releaseLower, upper: releaseUpper },
        cancel: { lower: bounds.releaseLower, upper: releaseUpper },
        refund: { lower: bounds.partition, upper: bounds.refundInclusive },
      };
    }
    const cliIsReal = realCli(request.cliPath || '');
    const timeOpen = request.time != null || request.parityCapture != null;
    const nativeBodyBlocked = request.financialBody === true && cliIsReal && (
      timeOpen ? stages.productionTime?.code !== 'PARITY_ADMITTED' : !request.cashDesign
    );
    // The connected encoder still uses UNIT policy/script/budget inputs. A genuine
    // parity receipt cannot promote those placeholders into a production body.
    const productionConsumerIncomplete = request.financialBody === true && cliIsReal
      && stages.sibling?.productionAdmitted === true;
    if (productionConsumerIncomplete) {
      stages.body = closed('PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE', {
        submitBlocked: true, bodyBuilt: false, cliIntervalEmitted: false,
        reason: 'current-policy-script-budget-minAda-binding-required',
      });
      stages.spend = closed('PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE', { bodyBuilt: false, encodeSpendBodyCalled: false });
    }
    if (nativeBodyBlocked) {
      stages.body = closed('PARITY_NOT_ADMITTED', { ledgerFunding: false, submitBlocked: true, bodyBuilt: false, cliIntervalEmitted: false });
      stages.spend = closed('PARITY_NOT_ADMITTED', { bodyBuilt: false, cliIntervalEmitted: false });
    }
    const constraintBlocked = stages.budget?.code === 'BUDGET_EXCEEDED';
    if (!nativeBodyBlocked && !productionConsumerIncomplete && request.buildBody && stages.inputs?.ok && !constraintBlocked) {
      const spec = bodySpec(stages, request);
      if (stages.time?.code === 'SLOT_OBSERVED' && spec.timeSeparation?.ok !== true) {
        const refused = spec.timeSeparation?.code || 'TIME_WINDOW_ABSENT';
        stages.body = closed(refused, { ledgerFunding: false, submitBlocked: true, bodyBuilt: false, cliIntervalEmitted: false });
        stages.spend = closed(refused, { bodyBuilt: false, cliIntervalEmitted: false, encodeSpendBodyCalled: false });
      } else if (request.financialBody && stages.inputs?.twelveInputFrameSatisfied === true) {
        const consumed = await assembleConsumedFinancial(stages, request);
        stages.outputFee = consumed.outputFee;
        stages.mint = consumed.mint;
        stages.body = consumed.body;
        stages.rebuild = consumed.rebuild;
        stages.spend = consumed.spend;
        stages.posts = consumed.posts;
      } else if (request.financialBody && stages.cashDesign?.designAdopted === true) {
        const funded = await buildApprovedFunding(stages, request, spec);
        stages.cashCompile = funded.cashCompile;
        stages.headApply = funded.headApply;
        stages.mint = funded.mint;
        stages.body = funded.body;
        stages.posts = funded.posts;
        if (funded.spend) {
          const failed = ['release', 'refund', 'cancel'].map((kind) => funded.spend[kind]).find((item) => item?.ok === false);
          stages.spend = failed
            ? { ...funded.spend, ok: false, code: failed.code }
            : { ...funded.spend, ok: true, code: 'SPEND_BODIES_ENCODED' };
        }
      } else if (request.financialBody && stages.cashDesign && stages.cashDesign.ok === false) {
        stages.mint = closed(stages.cashDesign.code, { cashCanMint: false, compiledScript: false, ledgerFunding: false });
        stages.body = closed('HEAD_OPEN_BLOCKED', { cashCanMint: false, ledgerFunding: false });
      } else if (request.financialBody) {
        stages.mint = encodeUnsignedBody({
          ...spec,
          mintAssets: [{ quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' }],
          assets: [{ quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' }],
        }, request.cliPath);
        stages.mint = {
          ...stages.mint,
          cashPolicyId: UNIT_CASH_POLICY,
          compiledScript: false,
          markerNotNativeFunding: true,
          forbidsPolicy: '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9',
        };
        const opened = encodeUnsignedBody({
          ...spec,
          mintAssets: [{ quantity: '1', policyId: UNIT_HEAD_POLICY, assetName: '48656164' }],
          inlineDatumJson: JSON.stringify(initialHeadDatum(stages.source?.hashes || {}, stages.source?.instanceId || '').plutus),
          assets: [
            { quantity: '1', policyId: UNIT_HEAD_POLICY, assetName: '48656164' },
            { quantity: '100', policyId: UNIT_CASH_POLICY, assetName: '43617368' },
          ],
        }, request.cliPath);
        stages.body = {
          ...opened,
          mintTxid: stages.mint.txid || null,
          mintSeparate: stages.mint.ok === true,
          headMintsCash: false,
          kernelCash: '100',
          headAsset: '48656164',
          markerNotNativeFunding: true,
        };
      } else {
        stages.body = encodeUnsignedBody(spec, request.cliPath);
      }
      if (stages.body?.ok && stages.sibling?.programHash) {
        stages.body = {
          ...stages.body,
          guardProgramHash: stages.sibling.programHash,
          guardSourceHash: stages.sibling.sourceHash,
          guardUsed: stages.sibling.programHash !== stages.sibling.previousProgramHash,
        };
      }
    } else if (request.buildBody && constraintBlocked) {
      stages.body = closed('BUDGET_EXCEEDED', { submitBlocked: true });
    }
    if (stages.body?.ok && stages.budget?.code === 'EVALUATION_UNAVAILABLE') {
      const bodyPath = stages.spend?.release?.bodyFile || stages.body.bodyFile || '';
      const measured = acquireBodyEvaluation({
        cliPath: request.cliPath || CARDANO_CLI,
        bodyPath,
        socket: request.protocol?.socket || request.utxoQuery?.socket || request.readback?.socket || '',
        execute: true,
      });
      stages.evaluation = measured;
      stages.budget = {
        ...measured,
        ok: measured.nativeBudgetMeasured === true,
        submitBlocked: measured.nativeBudgetMeasured !== true,
        suppliedCostIgnored: measured.nativeBudgetMeasured !== true,
        nativeBudgetMeasured: measured.nativeBudgetMeasured === true,
        minAdaMeasured: false,
        protocolMinAda: 'unmeasured',
        code: measured.nativeBudgetMeasured === true ? 'BODY_EVALUATED' : (measured.code || 'EVALUATION_UNAVAILABLE'),
      };
      if (measured.nativeBudgetMeasured === true && stages.protocol) {
        const compared = compareBudget({
          queried: true,
          source: 'cardano-cli-query',
          unitOnly: false,
          queryProvenance: measured.queryProvenance,
          measuredByEvaluator: true,
          executionSource: 'body-evaluator',
          maxTxExecutionUnits: stages.protocol.maxTxExecutionUnits,
          maxTxSize: stages.protocol.maxTxSize,
          utxoCostPerByte: stages.protocol.utxoCostPerByte,
          cost: measured.cost,
          txBytes: measured.txBytes,
          offlineUnits: false,
        });
        stages.budget = { ...compared, evaluation: measured };
      }
    }
    const submitBlocked = stages.budget?.submitBlocked === true;
    const witnessKey = stages.keys?.keyPath || request.witnessKeyPath;
    if (stages.body?.cborHex && witnessKey && !submitBlocked) {
      stages.witness = bindWitness({
        signingKeyPath: witnessKey,
        txBodyCborHex: stages.body.cborHex,
        txBodyType: stages.body.type,
        cborSha256: stages.body.cborSha256,
        networkMagic: stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null,
      }, request.cliPath || CARDANO_CLI);
      if (stages.witness?.witnessCborHex) {
        const keys = witnessKeyContext(stages.body);
        const counted = bindSignedWitnessCount({
          unitOnly: stages.witness.unitOnly !== false,
          signedCborHex: stages.witness.witnessCborHex,
          signingKeyCount: 1,
          bodyCborHex: stages.body.cborHex,
          neededKeys: {
            ordinaryKeyHashes: keys.ordinaryKeyHashes,
            collateralKeyHashes: keys.collateralKeyHashes,
            extraKeyHashes: keys.extraKeyHashes,
            inputsResolved: keys.resolved,
          },
        });
        stages.witnessCount = {
          ...counted,
          contextResolved: keys.resolved,
          signatureVerified: keys.resolved ? counted.signatureVerified === true : false,
          signatureCode: keys.resolved ? counted.signatureCode : (counted.signatureCode || 'WITNESS_CONTEXT_UNRESOLVED'),
        };
      } else {
        stages.witnessCount = {
          ok: false,
          code: stages.witness?.code || 'WITNESS_SET_NOT_COUNTED',
          witnessCountVerified: false,
          signatureVerified: false,
          ledgerExecution: false,
          nativeBudgetMeasured: false,
          unitOnly: stages.witness?.unitOnly !== false,
        };
      }
    }
    if (request.financialBody && stages.witnessCount?.witnessCountVerified === true) {
      await recountSignedWitnessFee(stages, request);
    }
    if (request.race && stages.inputs?.ok && !submitBlocked) {
      const partners = selectRacePartners(stages.inputs, stages.spend);
      const opened = parseRef(stages.body?.holding) || stages.inputs.head || null;
      if (!partners.ok || !opened || !(stages.keys?.keyPath || request.race.signingKeyPath)) {
        stages.race = closed(partners.code || 'RACE_BODY_ABSENT', {
          submitted: false,
          confirmed: false,
          ledgerSubmitted: false,
          spawned: false,
          refundSeparate: partners.refundSeparate !== false,
          leftKind: partners.leftKind,
          rightKind: partners.rightKind,
          replay: { separate: true, executed: false, expectedClass: 'BadInputsUTxO' },
        });
      } else {
        stages.race = await runOwnedRace({
          signBin: request.race.signBin,
          submitBin: request.race.submitBin,
          pollBin: request.race.pollBin,
          logPath: request.race.logPath,
          socket: request.utxoQuery?.socket || request.launch?.socketPath || request.race.socket || 'unit-socket',
          head: opened,
          left: { ...partners.left, actor: request.race.leftActor, resource: request.race.leftResource, expectedFrame: stages.spend?.[partners.leftKind]?.frame, redeemer: stages.posts?.terminals?.[partners.leftKind]?.redeemer },
          right: { ...partners.right, actor: request.race.rightActor, resource: request.race.rightResource, expectedFrame: stages.spend?.[partners.rightKind]?.frame, redeemer: stages.posts?.terminals?.[partners.rightKind]?.redeemer },
          leftBodyFile: partners.leftBodyFile,
          rightBodyFile: partners.rightBodyFile,
          leftBodySha256: partners.leftBodySha256,
          rightBodySha256: partners.rightBodySha256,
          signingKeyPath: stages.keys?.keyPath || request.race.signingKeyPath,
          networkMagic: stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null,
          overlap: partners.overlap,
          cashQuantity: partners.cashQuantity,
        });
        stages.race = {
          ...stages.race,
          partnerCode: partners.code,
          refundSeparate: partners.refundSeparate,
          leftKind: partners.leftKind,
          rightKind: partners.rightKind,
        };
      }
    } else if (request.race && submitBlocked) {
      stages.race = closed(stages.budget.code || 'EVALUATION_UNAVAILABLE', { submitBlocked: true, spawned: false });
    }
    if (request.financialBody && stages.body?.cborHex) {
      stages.receipt = await bindHelperReceipt(stages.body.cborHex, {
        cliPath: request.cliPath || CARDANO_CLI,
        networkMagic: stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null,
        socket: request.readback?.socket || request.utxoQuery?.socket || '',
        localTxid: stages.body.txid,
        bodySha256: stages.body.cborSha256,
        confirmationBudgetMs: 40,
        outputIndex: stages.body.headOutputIndex,
        evidenceRoot: request.evidenceRoot,
        receiptRoot: request.receiptRoot,
        keepReceipt: request.keepReceipt === true,
        label: request.receiptLabel,
      });
    }
    if (request.readback) {
      const release = stages.spend?.release;
      const headTxid = stages.body?.txid || '';
      const headIndex = stages.body?.headOutputIndex;
      stages.readback = collectHeadReadback({
        ...request.readback,
        magic: request.readback.magic ?? stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null,
        expectedTxid: headTxid || release?.txid || '',
        outputIndex: Number.isInteger(request.readback.outputIndex)
          ? request.readback.outputIndex
          : (Number.isInteger(headIndex) ? headIndex : release?.builtHeadOutputIndex),
        payoutIndex: Number.isInteger(request.readback.payoutIndex)
          ? request.readback.payoutIndex
          : (Number.isInteger(stages.body?.builtPayoutOutputIndex) ? stages.body.builtPayoutOutputIndex : release?.builtPayoutOutputIndex),
        expectedFrame: null,
        link: {
          runId: stages.utxo?.runId || null,
          bodySha256: release?.cborSha256 || stages.body?.cborSha256 || null,
        },
      });
      if (release?.frame && release.txid) {
        const effect = collectHeadReadback({
          ...request.readback,
          magic: request.readback.magic ?? stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null,
          expectedTxid: release.txid,
          outputIndex: Number.isInteger(release.builtHeadOutputIndex) ? release.builtHeadOutputIndex : 0,
          payoutIndex: Number.isInteger(release.builtPayoutOutputIndex) ? release.builtPayoutOutputIndex : 1,
          expectedFrame: release.frame,
          link: { runId: stages.utxo?.runId || null },
        });
        stages.readback.effectTxid = effect.txid || null;
        stages.readback.effectQueryCode = effect.code || null;
        stages.readback.financialMatch = effect.financialMatch === true;
        stages.readback.financialCode = effect.financialCode || null;
        stages.readback.mismatches = effect.mismatches || null;
        stages.readback.decoded = effect.decoded || null;
        stages.readback.effectUnitOnly = effect.unitOnly === true;
        stages.readback.economicAction = judgeEconomicAction(effect.decoded, 'release', {
          redeemer: release.redeemer || null,
        });
      }
      const byAction = {};
      for (const kind of ['release', 'refund', 'cancel']) {
        const side = stages.spend?.[kind];
        if (!side?.frame || !side.txid) continue;
        const observed = collectHeadReadback({
          ...request.readback,
          magic: request.readback.magic ?? stages.ownedProducer?.schema71?.magic ?? request.networkMagic ?? request.magic ?? null,
          expectedTxid: side.txid,
          outputIndex: Number.isInteger(side.builtHeadOutputIndex) ? side.builtHeadOutputIndex : 0,
          payoutIndex: Number.isInteger(side.builtPayoutOutputIndex) ? side.builtPayoutOutputIndex : 1,
          expectedFrame: side.frame,
          link: { runId: stages.utxo?.runId || null },
        });
        const judged = judgeEconomicAction(observed.decoded, kind, { redeemer: side.redeemer || null });
        const providerBound = observed.unitOnly !== true
          && observed.spawned === true
          && observed.txidLinked === true
          && observed.decoded != null;
        byAction[kind] = {
          financialMatch: observed.financialMatch === true && judged.ok === true,
          financialCode: observed.financialCode || null,
          judgeCode: judged.code,
          mismatches: judged.ok ? null : (judged.mismatches || observed.mismatches || null),
          txid: observed.txid || null,
          confirmed: providerBound && judged.ok === true && observed.financialMatch === true,
          providerQueried: providerBound,
          unitOnly: observed.unitOnly === true,
          spendMintEmpty: side.spendMintEmpty === true,
          recipient: judged.recipient || null,
          payoutPaymentHash: side.payoutPaymentHash || null,
        };
      }
      stages.readback.byAction = byAction;
      const receipt = stages.receipt;
      const bodySha = release?.cborSha256 || stages.body?.cborSha256;
      const txid = release?.txid || stages.body?.txid;
      stages.readback.helperConfirmed = receipt?.helperConfirmed === true
        && receipt.unitOnly !== true
        && receipt.computedHash === txid
        && receipt.bodySha256 === bodySha;
      stages.confirmation = decideConfirmation(stages.readback, request.receiptStatus);
    }
    if (request.faucet && stages.keys?.ok && !constraintBlocked) {
      const owned = stages.runtimeInputs?.adopted === true ? (stages.runtimeInputs.faucetInputs || []) : [];
      if (!stages.keys.address) {
        stages.faucet = closed('FAUCET_ADDRESS_ABSENT', { spawned: false, unitOnly: true });
      } else if (!owned.length) {
        stages.faucet = closed('FAUCET_INPUTS_ABSENT', { spawned: false, unitOnly: true, ledgerExecution: false, nativePins: false });
      } else if (realCli(request.faucet.bin) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
        stages.faucet = closed('REAL_SUBMIT_NOT_ARMED', { spawned: false, bodiesReady: true });
      } else {
        const args = ['conway', 'transaction', 'build-raw'];
        for (const input of owned) args.push('--tx-in', String(input));
        args.push('--tx-out', `${stages.keys.address}+10000000`, '--fee', '200000', '--out-file', 'faucet-body.json');
        const faucet = spawnSync(request.faucet.bin, args, {
          encoding: 'utf8',
          env: { ...process.env, MORIARTY_RACE_LOG: request.faucet.logPath || '' },
        });
        stages.faucet = {
          ok: false,
          code: faucet.status === 0 ? 'FAUCET_BODY_SHAPED' : 'FAUCET_FAILED',
          spawned: true,
          unitOnly: !realCli(request.faucet.bin),
          ledgerExecution: false,
          nativePins: false,
          submitted: false,
        };
      }
    }
    const failed = stageFailure(stages);
    const unitOnly = runUnitOnly(stages);
    const submitsInvoked = (stages.race?.submitCount || 0) >= 2;
    return {
      ok: failed == null,
      code: failed || (stages.authority?.ok ? 'CALLER_UNIT_PIPELINE' : stages.authority?.code || 'EXECUTION_AUTHORITY_ABSENT'),
      ledgerExecution: false,
      nativePins: false,
      ledgerSubmitted: false,
      confirmed: stages.confirmation?.confirmed === true,
      submitted: submitsInvoked && unitOnly !== true,
      unitOnly,
      ceilingRemoved: false,
      startedNode: stages.launcher?.code === 'OWNED_NODE_SPAWNED',
      stages,
    };
  } finally {
    if (launched?.pgid) stopOwnedGroup(launched.pgid);
  }
}
