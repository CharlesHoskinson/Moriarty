/**
 * Evidence gates for the T1 native caller.
 * A boolean, a copied hash, or a unit fixture does not create RuntimePins.
 * Offline inspect does not exec a node. A reviewed or unit-fixture authority
 * can spawn an owned process group. The real node stays unarmed in this process
 * unless MORIARTY_ALLOW_REAL_NODE=1. Unit fixtures do not create RuntimePins.
 */
import { spawn, spawnSync } from 'node:child_process';
import { createHash, randomBytes } from 'node:crypto';
import { existsSync, mkdirSync, openSync, readFileSync, realpathSync, statSync } from 'node:fs';
import { appendFileSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import net from 'node:net';
import { homedir, tmpdir } from 'node:os';
import { isAbsolute, join, resolve, sep } from 'node:path';
import { scriptHashV3 } from './apply-head.mjs';
import { genesisDigest, instanceDigest } from './same-script-identity.mjs';
import {
  PATHS as RUNTIME65_PATHS,
  addressBuildArgv,
  addressKeyGenArgv,
  assessReadiness,
  calculateMinFeeArgv,
  faucetBuildArgv,
  fundingConfirmArgv,
  futureStartArgv,
  initialAddrArgv,
  inspectGenesis,
  portClaim,
  queryEraHistoryArgv,
  queryTipArgv,
  rejectLauncher,
  signArgv,
  submitArgv,
  yaciCreateNodeArgv,
} from '../owned/runtime-inputs-65/runtime-inputs.mjs';
import {
  buildOwnedProducerPlan,
  consumePlutusCost,
  dispatchOwnedProducer,
  plutusCostArgv,
  realProducerSpawn,
} from './native-producer.mjs';
import { genesisMagicText } from './native-body.mjs';
import { parseJsonExact } from './native-bootstrap.mjs';
import { refuseCallerMinAdaFormula } from '../owned/runtime-schemas-71/schemas.mjs';

export const CARDANO_CLI = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-cli';
export const CARDANO_NODE = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-node';
export const TASK_ROOT = '/home/charl/moriarty-eutxo-validation-2026-10-12';
export const CALLER_OUT = join(TASK_ROOT, 't1-native-adapters-57/checks/cli-custody');
export const REPAIR_OUT = CALLER_OUT;
// Historical native50 digest. Not an acceptance key.
export const NATIVE_MANIFEST_DIGEST = 'c7f97ef913d912226bf80b054a538deb70d8133ebcf8c307ac11c3eb5b743007';
const UNIT_VERDICT = 'APPROVE_SCOPED_T1_NATIVE_CALLER';
const NATIVE_VERDICT = 'APPROVE_SCOPED_T1_NATIVE_IMPLEMENTATION';
const CASH_VERDICT = 'APPROVE_SCOPED_T1_CASH_FIXTURE_DESIGN';

const HEX28 = /^[0-9a-f]{56}$/;
const HEX32 = /^[0-9a-f]{64}$/;
const HISTORICAL = [
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
const PHASE1 = ['InvalidWitnessesUTXOW', 'BadInputsUTxO', 'OutsideValidityInterval', 'ValueNotConservedUTxO'];
const ownedQueries = new Map();

function closed(code, extra = {}) {
  return {
    ok: false,
    code,
    ledgerExecution: false,
    nativePins: false,
    runtimePinsCreated: false,
    confirmed: false,
    submitted: false,
    ...extra,
  };
}

export function resolveOutputPath(requested, fallback = REPAIR_OUT) {
  const chosen = resolve(requested || fallback);
  const segments = chosen.split(sep);
  const hit = HISTORICAL.find((name) => segments.includes(name));
  if (hit) return closed('OUTPUT_PATH_FORBIDDEN', { path: chosen, historical: hit });
  return { ok: true, code: 'OUTPUT_PATH_OK', path: chosen, ledgerExecution: false, nativePins: false };
}

function keyPathOf(pins) {
  if (typeof pins.keyPath === 'string') return pins.keyPath;
  if (pins.keyPaths && typeof pins.keyPaths.payment === 'string') return pins.keyPaths.payment;
  if (Array.isArray(pins.keyPaths) && typeof pins.keyPaths[0] === 'string') return pins.keyPaths[0];
  return null;
}

function exactNativeVerdict(row) {
  if (!row || typeof row.verdict !== 'string' || row.verdict.includes('CHANGES_REQUIRED')) return false;
  if (row.verdict === CASH_VERDICT) return false;
  if (row.evidenceClass === 'unit-fixture') return row.verdict === UNIT_VERDICT;
  if (row.evidenceClass === 'reviewed-implementation') return row.verdict === NATIVE_VERDICT;
  return false;
}

function coveredManifest(authority, reviewA, reviewB, candidate) {
  const note = { requiredManifestDigest: null, selfApproval: false, hardcodedNative50: false, spawnAttempted: false };
  if (typeof authority.manifest !== 'string' || typeof authority.reviewAuthority !== 'string') {
    return closed('AUTHORITY_MANIFEST_ABSENT', note);
  }
  if (!existsSync(authority.manifest) || !existsSync(authority.reviewAuthority)) {
    return closed('AUTHORITY_MANIFEST_ABSENT', note);
  }
  let manifestPath;
  let authorityPath;
  try {
    manifestPath = realpathSync(authority.manifest);
    authorityPath = realpathSync(authority.reviewAuthority);
  } catch {
    return closed('AUTHORITY_MANIFEST_ABSENT', note);
  }
  if (manifestPath === authorityPath) return closed('AUTHORITY_SELF_REFERENCE', note);
  const manifestBytes = readFileSync(manifestPath);
  const manifestHash = createHash('sha256').update(manifestBytes).digest('hex');
  let external;
  let manifest;
  try {
    external = JSON.parse(readFileSync(authorityPath, 'utf8'));
    manifest = JSON.parse(manifestBytes.toString('utf8'));
  } catch {
    return closed('AUTHORITY_UNREADABLE', note);
  }
  if (external.manifestSha256 !== manifestHash || reviewA.candidate !== manifestHash || reviewB.candidate !== manifestHash || candidate !== manifestHash) {
    return closed('AUTHORITY_MANIFEST_MISMATCH', { ...note, candidate, manifestHash });
  }
  if (manifest.manifestSha256 || manifest.sha256) return closed('AUTHORITY_SELF_REFERENCE', note);
  if (!Array.isArray(manifest.files) || manifest.files.length < 1) return closed('AUTHORITY_MANIFEST_MISMATCH', note);
  if (reviewA.sourceHash !== reviewB.sourceHash || reviewA.sourceHash !== manifest.sourceHash) {
    return closed('AUTHORITY_SOURCE_MISMATCH', note);
  }
  let sourceCovered = false;
  for (const file of manifest.files) {
    if (!file?.path || !existsSync(file.path)) return closed('AUTHORITY_BYTES_MISMATCH', { ...note, path: file?.path || null });
    const bytes = readFileSync(file.path);
    const hash = createHash('sha256').update(bytes).digest('hex');
    if (hash !== file.sha256 || bytes.length !== file.bytes) return closed('AUTHORITY_BYTES_MISMATCH', { ...note, path: file.path });
    if (hash === manifest.sourceHash) sourceCovered = true;
  }
  if (!sourceCovered) return closed('AUTHORITY_SOURCE_MISMATCH', note);
  return { ok: true, manifestHash, sourceHash: manifest.sourceHash };
}

export function loadAuthority(authority) {
  const note = { requiredManifestDigest: null, selfApproval: false, hardcodedNative50: false, spawnAttempted: false };
  if (!authority || typeof authority !== 'object' || authority.approved === true && !authority.reviewA) {
    return closed('EXECUTION_AUTHORITY_ABSENT', note);
  }
  for (const key of ['reviewA', 'reviewB', 'candidate']) {
    if (typeof authority[key] !== 'string' || !existsSync(authority[key])) {
      return closed('EXECUTION_AUTHORITY_ABSENT', { ...note, missing: key });
    }
  }
  let reviewA;
  let reviewB;
  try {
    reviewA = JSON.parse(readFileSync(authority.reviewA, 'utf8'));
    reviewB = JSON.parse(readFileSync(authority.reviewB, 'utf8'));
  } catch {
    return closed('AUTHORITY_UNREADABLE', note);
  }
  const candidate = createHash('sha256').update(readFileSync(authority.candidate)).digest('hex');
  if (reviewA.selfApproval === true || reviewB.selfApproval === true) return closed('AUTHORITY_SELF_APPROVAL', note);
  if (!exactNativeVerdict(reviewA) || !exactNativeVerdict(reviewB)) return closed('AUTHORITY_NOT_APPROVED', note);
  if (reviewA.candidate !== candidate || reviewB.candidate !== candidate) return closed('AUTHORITY_CANDIDATE_MISMATCH', note);
  if (reviewA.verdict !== reviewB.verdict) return closed('AUTHORITY_VERDICT_MISMATCH', note);
  if (reviewA.evidenceClass !== reviewB.evidenceClass) return closed('AUTHORITY_CLASS_MISMATCH', note);
  if (reviewA.evidenceClass !== 'unit-fixture' && reviewA.evidenceClass !== 'reviewed-implementation') {
    return closed('AUTHORITY_CLASS_ABSENT', note);
  }
  if (reviewA.evidenceClass === 'unit-fixture') {
    return {
      ok: true,
      code: 'AUTHORITY_AGREED',
      candidate,
      verdict: reviewA.verdict,
      evidenceClass: 'unit-fixture',
      reviewClass: 'unit-fixture',
      unitFixture: true,
      ledgerExecution: false,
      nativePins: false,
      selfApproval: false,
      hardcodedNative50: false,
      requiredManifestDigest: null,
      spawnAttempted: false,
    };
  }
  if (reviewA.reviewClass !== 'native-implementation' || reviewB.reviewClass !== 'native-implementation') {
    return closed('AUTHORITY_CLASS_MISMATCH', note);
  }
  if (!reviewA.reviewer || !reviewB.reviewer || reviewA.reviewer === reviewB.reviewer) {
    return closed('AUTHORITY_REVIEWERS_NOT_DISTINCT', note);
  }
  const covered = coveredManifest(authority, reviewA, reviewB, candidate);
  if (!covered.ok) return covered;
  return {
    ok: true,
    code: 'AUTHORITY_AGREED',
    candidate: covered.manifestHash,
    sourceHash: covered.sourceHash,
    verdict: reviewA.verdict,
    evidenceClass: 'reviewed-implementation',
    reviewClass: 'native-implementation',
    unitFixture: false,
    ledgerExecution: false,
    nativePins: false,
    selfApproval: false,
    hardcodedNative50: false,
    requiredManifestDigest: covered.manifestHash,
    spawnAttempted: false,
  };
}

export function assessCashPolicy(sourceText, votes) {
  if (typeof sourceText !== 'string' || sourceText.trim().length < 20) {
    return closed('CASH_SOURCE_ABSENT', {
      cashCanMint: false,
      financialValidatorProof: false,
      seam: 'cash-source-not-supplied',
    });
  }
  const mintBody = sourceText.match(/mint\s*\([^)]*\)\s*\{([^{}]*)\}/);
  const failClosed = (mintBody && /\bfail\b/.test(mintBody[1]))
    || sourceText.includes('This policy cannot mint');
  if (failClosed) {
    return closed('CASH_POLICY_FAIL_CLOSED', {
      cashCanMint: false,
      financialValidatorProof: false,
      seam: 'compiled-cash-fails-closed',
    });
  }
  if (votes?.approved === true && !votes.reviewA) {
    return closed('CASH_POLICY_VOTES_ABSENT', { cashCanMint: false, booleanIgnored: true, seam: 'boolean-is-not-a-vote' });
  }
  if (!votes?.reviewA || !votes?.reviewB || !votes?.candidate) {
    return closed('CASH_POLICY_VOTES_ABSENT', {
      cashCanMint: false,
      financialValidatorProof: false,
      seam: 'needs-two-agreeing-vote-files',
    });
  }
  const authority = loadAuthority(votes);
  if (!authority.ok) return { ...authority, cashCanMint: false, financialValidatorProof: false };
  const sourceHash = createHash('sha256').update(sourceText).digest('hex');
  const reviewA = JSON.parse(readFileSync(votes.reviewA, 'utf8'));
  const reviewB = JSON.parse(readFileSync(votes.reviewB, 'utf8'));
  if (reviewA.policySourceSha256 !== sourceHash || reviewB.policySourceSha256 !== sourceHash) {
    return closed('CASH_POLICY_SOURCE_MISMATCH', { cashCanMint: false, financialValidatorProof: false });
  }
  if (authority.unitFixture) {
    return {
      ok: true,
      code: 'CASH_POLICY_UNIT_ACCEPTED',
      cashCanMint: false,
      ledgerFunding: false,
      unitFixture: true,
      financialValidatorProof: false,
      nativePins: false,
      ledgerExecution: false,
      runtimePinsCreated: false,
      confirmed: false,
      submitted: false,
      seam: 'unit-policy-not-native-funding',
    };
  }
  if (reviewA.reviewClass !== 'cash-policy') {
    return closed('CASH_REVIEW_CLASS_MISMATCH', {
      cashCanMint: false,
      ledgerFunding: false,
      financialValidatorProof: false,
      seam: 'native-code-votes-are-not-a-cash-mint',
    });
  }
  return {
    ok: true,
    code: 'CASH_POLICY_SHAPE_ONLY',
    cashCanMint: false,
    ledgerFunding: false,
    unitFixture: false,
    financialValidatorProof: false,
    nativePins: false,
    ledgerExecution: false,
    runtimePinsCreated: false,
    confirmed: false,
    submitted: false,
    seam: 'cash-policy-class-without-two-design-votes',
  };
}

const CASH_DESIGN_CANDIDATE = '01d288d564843c174d2ce23363dbfe8d2555b9290eb3f62143f7f4f874efe138';
const CASH_DESIGN_SOURCE = '3f897ac9ef56c01e11d191dba31736da508dc77982ca484f1684e66013b242c4';
const CASH_DESIGN_VERDICT = 'APPROVE_SCOPED_T1_CASH_FIXTURE_DESIGN';

export function loadCashDesignAuthority(design) {
  const refused = (code, extra = {}) => closed(code, {
    cashCanMint: false,
    ledgerFunding: false,
    designAdopted: false,
    reviewClass: 'cash-fixture-design',
    ...extra,
  });
  if (!design || typeof design !== 'object') return refused('CASH_DESIGN_ABSENT');
  for (const key of ['reviewA', 'reviewB', 'candidate', 'sourcePath']) {
    if (typeof design[key] !== 'string' || !existsSync(design[key])) return refused('CASH_DESIGN_ABSENT', { missing: key });
  }
  let reviewA;
  let reviewB;
  try {
    reviewA = JSON.parse(readFileSync(design.reviewA, 'utf8'));
    reviewB = JSON.parse(readFileSync(design.reviewB, 'utf8'));
  } catch {
    return refused('CASH_DESIGN_UNREADABLE');
  }
  const candidate = createHash('sha256').update(readFileSync(design.candidate)).digest('hex');
  const sourceHash = createHash('sha256').update(readFileSync(design.sourcePath)).digest('hex');
  if (!reviewA.reviewer || !reviewB.reviewer || reviewA.reviewer === reviewB.reviewer) {
    return refused('CASH_DESIGN_REVIEWERS_NOT_DISTINCT');
  }
  if (reviewA.verdict !== CASH_DESIGN_VERDICT || reviewB.verdict !== CASH_DESIGN_VERDICT) {
    return refused('CASH_DESIGN_VERDICT_REJECTED', { verdict: reviewA.verdict || null });
  }
  if (reviewA.reviewClass === 'native-implementation' || reviewB.reviewClass === 'native-implementation') {
    return refused('CASH_DESIGN_CLASS_MISMATCH');
  }
  if (reviewA.candidateSha256 !== candidate || reviewB.candidateSha256 !== candidate) {
    return refused('CASH_DESIGN_CANDIDATE_MISMATCH', { candidate });
  }
  if (candidate !== CASH_DESIGN_CANDIDATE) return refused('CASH_DESIGN_MANIFEST_MISMATCH', { candidate });
  if (sourceHash !== CASH_DESIGN_SOURCE) return refused('CASH_DESIGN_SOURCE_MISMATCH', { sourceHash });
  return {
    ok: true,
    code: 'CASH_DESIGN_ADOPTED',
    designAdopted: true,
    cashCanMint: false,
    ledgerFunding: false,
    nativePins: false,
    ledgerExecution: false,
    runtimePinsCreated: false,
    confirmed: false,
    submitted: false,
    selfApproval: false,
    reviewClass: 'cash-fixture-design',
    candidate,
    sourceHash,
    reviewers: [reviewA.reviewer, reviewB.reviewer],
  };
}

function readJson(path) {
  try {
    return JSON.parse(readFileSync(path, 'utf8'));
  } catch {
    return null;
  }
}

function validatorCode(blueprint, namePart, purpose) {
  return blueprint?.validators?.find((item) => item.title.includes(namePart) && item.title.endsWith(`.${purpose}`)) || null;
}

export function certifyRuntimePins(pins) {
  if (pins == null || typeof pins !== 'object') return closed('NOT_EXECUTED', { nativeExecution: 'NOT_EXECUTED' });
  if (pins.unitOnly === true || pins.unitFixture === true) return closed('UNIT_FIXTURE_NOT_PINS');
  const claiming = pins.nativePins === true || pins.runtimePinsCreated === true
    || Boolean(pins.keyPath) || Boolean(pins.keyPaths) || Boolean(pins.appliedBlueprintPath) || Boolean(pins.compiledReportPath);
  if (!claiming) return closed('NOT_EXECUTED', { nativeExecution: 'NOT_EXECUTED' });
  const scriptHash = String(pins.scriptHash || '');
  const cashPolicyId = String(pins.cashPolicyId || '');
  if (!HEX28.test(scriptHash) || !HEX28.test(cashPolicyId)) return closed('MALFORMED_ID');
  if (pins.lenderVkh != null && !HEX28.test(String(pins.lenderVkh))) return closed('MALFORMED_ID');
  if (pins.borrowerVkh != null && !HEX28.test(String(pins.borrowerVkh))) return closed('MALFORMED_ID');
  if (pins.seedTxId != null && !HEX32.test(String(pins.seedTxId))) return closed('MALFORMED_ID');
  const keyPath = keyPathOf(pins);
  if (!keyPath || !existsSync(keyPath)) return closed('WITNESS_KEYS_ABSENT', { constructorReached: false });
  const st = statSync(keyPath);
  if (!st.isFile() || (st.mode & 0o777) !== 0o600) {
    return closed('KEY_MODE_REFUSED', { mode: (st.mode & 0o777).toString(8).padStart(4, '0') });
  }
  if (typeof process.getuid === 'function' && st.uid !== process.getuid()) return closed('KEY_NOT_OWNED');
  if (!pins.appliedBlueprintPath || !existsSync(pins.appliedBlueprintPath)) return closed('COMPILED_ARTIFACT_ABSENT');
  const blueprint = readJson(pins.appliedBlueprintPath);
  const minted = validatorCode(blueprint, 'financial_head', 'mint');
  const cash = validatorCode(blueprint, 'cash', 'mint');
  if (!minted?.compiledCode || !cash?.compiledCode) return closed('COMPILED_ARTIFACT_ABSENT');
  const recomputedScript = scriptHashV3(minted.compiledCode);
  const recomputedCash = scriptHashV3(cash.compiledCode);
  if (recomputedScript !== scriptHash || recomputedCash !== cashPolicyId || minted.hash !== scriptHash || cash.hash !== cashPolicyId) {
    return closed('PIN_RELATION_MISMATCH', { reason: 'recomputed-script-or-cash' });
  }
  const instance = instanceDigest(scriptHash, cashPolicyId);
  if (!instance.ok) return instance;
  if (pins.instanceId && pins.instanceId !== instance.instanceId) return closed('PIN_RELATION_MISMATCH', { reason: 'instance' });
  const report = pins.compiledReportPath ? readJson(pins.compiledReportPath) : null;
  if (report?.genesisInputs) {
    const genesis = genesisDigest(report.genesisInputs);
    if (!genesis.ok || genesis.genesis !== report.genesis) return closed('PIN_RELATION_MISMATCH', { reason: 'genesis' });
    if (report.instanceId && report.instanceId !== instance.instanceId) return closed('PIN_RELATION_MISMATCH', { reason: 'report-instance' });
    if (report.appliedHash !== scriptHash || report.cashHash !== cashPolicyId) return closed('PIN_RELATION_MISMATCH', { reason: 'report-hash' });
  }
  const cashSource = typeof pins.cashSourceText === 'string'
    ? pins.cashSourceText
    : (pins.cashSourcePath && existsSync(pins.cashSourcePath) ? readFileSync(pins.cashSourcePath, 'utf8') : null);
  const cashGate = assessCashPolicy(cashSource || '', pins.cashVotes);
  const query = pins.utxoQuery;
  const remembered = query && typeof query.runId === 'string' ? ownedQueries.get(query.runId) : null;
  let transcriptText = '';
  if (remembered && existsSync(remembered.transcriptPath)) transcriptText = readFileSync(remembered.transcriptPath, 'utf8');
  const queryOk = Boolean(
    remembered
    && remembered.unitOnly !== true
    && remembered.transcriptPath === query.transcriptPath
    && remembered.socket === query.socket
    && query.spawned === true
    && query.queried === true
    && cashGate.ledgerFunding === true
    && transcriptText.includes(`runId=${query.runId}`)
    && !transcriptText.includes('evidenceClass=unit-fixture')
    && existsSync(query.socket)
    && Array.isArray(query.utxos)
    && query.utxos.length > 0,
  );
  return {
    ...cashGate,
    relationsOk: true,
    nativePins: queryOk,
    runtimePinsCreated: queryOk,
    utxoQuery: queryOk ? 'present' : 'absent',
    flagIgnored: pins.utxoOwnedByRun === true || pins.nativePins === true || query?.unitOnly === true,
    ledgerExecution: false,
  };
}

export function classifyFailure(text, evidence = {}) {
  let body = text;
  let extra = evidence && typeof evidence === 'object' ? evidence : {};
  if (text && typeof text === 'object') {
    extra = text;
    body = text.text || text.message || '';
  }
  body = String(body || '');
  const classes = [];
  for (const name of PHASE1) {
    if (body.includes(name)) classes.push({ failureClass: name, phase: 1 });
  }
  if (body.includes('ogmios') || body.includes('preflight')) classes.push({ failureClass: 'provider-preflight', phase: 'provider' });
  const collateral = extra.isValid === false || /isValid["']?\s*[:=]\s*false/.test(body) || body.includes('isValid: false');
  if (collateral) classes.push({ failureClass: 'collateral-isValid-false', phase: 'collateral' });
  if (body.includes('PlutusFailure')) classes.push({ failureClass: 'PlutusFailure', phase: 2 });
  if (classes.length === 0) return closed('UNCLASSIFIED_FAILURE', { predicateReached: false });
  if (classes.length > 1) {
    return {
      ok: true,
      failureClass: 'MIXED_FAILURE',
      phase: 'mixed',
      classes: classes.map((item) => item.failureClass),
      predicateReached: false,
      confirmed: false,
      ledgerExecution: false,
    };
  }
  const only = classes[0];
  if (only.failureClass !== 'PlutusFailure') {
    return { ok: true, failureClass: only.failureClass, phase: only.phase, predicateReached: false, confirmed: false, ledgerExecution: false };
  }
  const corroborated = extra.isValid === true && extra.signed === true && extra.phase1Absent !== false;
  return {
    ok: true,
    failureClass: 'PlutusFailure',
    phase: 2,
    predicateReached: corroborated,
    corroboration: corroborated ? 'signed-body-isValid-true' : 'isValid-true-absent',
    confirmed: false,
    ledgerExecution: false,
  };
}

function checkPort(port) {
  return new Promise((resolvePort) => {
    const server = net.createServer();
    server.once('error', (error) => {
      resolvePort({ ok: false, code: error.code === 'EADDRINUSE' ? 'PORT_BUSY' : 'PORT_CHECK_FAILED' });
    });
    server.listen({ port, host: '127.0.0.1', exclusive: true }, () => {
      const address = server.address();
      server.close(() => resolvePort({ ok: true, port: typeof address === 'object' && address ? address.port : port }));
    });
  });
}

function ownPgid() {
  const stat = readFileSync(`/proc/${process.pid}/stat`, 'utf8');
  return Number(stat.slice(stat.lastIndexOf(')') + 2).split(' ')[2]);
}

function pgidOf(pid) {
  const stat = readFileSync(`/proc/${pid}/stat`, 'utf8');
  return Number(stat.slice(stat.lastIndexOf(')') + 2).split(' ')[2]);
}

function refusedCommand(request) {
  const text = `${request.wrapper || ''} ${request.command || ''} ${(request.args || []).join(' ')}`;
  if (/pkill|global-reset|start\.mjs|npm\s+start/.test(text)) return 'YACI_WRAPPER_REFUSED';
  return null;
}

function isRealNode(nodeBin) {
  try {
    return realpathSync(nodeBin) === realpathSync(CARDANO_NODE);
  } catch {
    return false;
  }
}

export function stopOwnedGroup(pgid) {
  const own = ownPgid();
  if (!Number.isInteger(pgid) || pgid <= 1 || pgid === own) {
    return closed('CLEANUP_REFUSED', { stopped: false, target: pgid, own });
  }
  try {
    process.kill(-pgid, 'SIGTERM');
  } catch (error) {
    if (error.code !== 'ESRCH') return closed('CLEANUP_FAILED', { stopped: false, target: pgid });
  }
  return { ok: true, code: 'OWNED_GROUP_STOPPED', stopped: true, target: pgid, own, ledgerExecution: false, nativePins: false };
}

function pathInside(child, parent) {
  const root = resolve(parent);
  const target = resolve(child);
  return target === root || target.startsWith(root + sep);
}

function writeOwnedNodeLayout(runDir, configDir) {
  const genesisDir = join(configDir, 'genesis');
  mkdirSync(genesisDir, { recursive: true });
  const databasePath = join(runDir, 'db');
  mkdirSync(databasePath, { recursive: true });
  const shelley = {
    slotLength: 1,
    epochLength: 3600,
    networkMagic: 42,
    systemStart: '2026-01-01T00:00:00Z',
  };
  const shelleyPath = join(genesisDir, 'shelley-genesis.json');
  writeFileSync(shelleyPath, `${JSON.stringify(shelley)}\n`, { mode: 0o600 });
  for (const name of ['alonzo-genesis.json', 'conway-genesis.json', 'byron-genesis.json']) {
    writeFileSync(join(genesisDir, name), `${JSON.stringify({
      networkMagic: 42,
      evidenceClass: 'layout-not-a-chain',
      genesisComplete: false,
    })}\n`, { mode: 0o600 });
  }
  const topologyPath = join(configDir, 'topology.json');
  writeFileSync(topologyPath, `${JSON.stringify({ Producers: [] })}\n`, { mode: 0o600 });
  const configPath = join(configDir, 'config.json');
  const config = {
    Protocol: 'Cardano',
    ShelleyGenesisFile: 'genesis/shelley-genesis.json',
    AlonzoGenesisFile: 'genesis/alonzo-genesis.json',
    ConwayGenesisFile: 'genesis/conway-genesis.json',
    ByronGenesisFile: 'genesis/byron-genesis.json',
    TestShelleyHardForkAtEpoch: 0,
    TestAllegraHardForkAtEpoch: 0,
    TestMaryHardForkAtEpoch: 0,
    TestAlonzoHardForkAtEpoch: 0,
    TestBabbageHardForkAtEpoch: 0,
    TestConwayHardForkAtEpoch: 0,
  };
  writeFileSync(configPath, `${JSON.stringify(config)}\n`, { mode: 0o600 });
  return { configPath, topologyPath, databasePath, shelleyPath };
}

export async function launchOwnedRuntime(request, prior = {}) {
  const refused = refusedCommand(request);
  if (refused) return closed(refused, { started: false, spawnAttempted: false });
  const nodeBin = request.nodeBin || prior.nodeBin || CARDANO_NODE;
  if (String(nodeBin).includes('yaci') || String(request.command || '').includes('npm')) {
    return closed('YACI_WRAPPER_REFUSED', { started: false, spawnAttempted: false });
  }
  const runDir = request.runDir;
  if (!runDir || !isAbsolute(runDir) || resolve(runDir) === resolve(homedir())) {
    return closed('OWNED_HOME_REQUIRED', { started: false, spawnAttempted: false });
  }
  const authority = loadAuthority(request.authority);
  const binaryPresent = existsSync(nodeBin);
  const realNode = binaryPresent && isRealNode(nodeBin);
  if (authority.ok && authority.unitFixture && realNode) {
    return closed('UNIT_FIXTURE_CANNOT_START_REAL_NODE', { started: false, spawnAttempted: false, authority: authority.code });
  }
  if (realNode && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_NODE_NOT_ARMED', {
      started: false,
      spawnAttempted: false,
      authority: authority.code || null,
      binaryPresent: true,
    });
  }
  const stubDespiteAuthority = !authority.ok
    && !realNode
    && authority.code !== 'EXECUTION_AUTHORITY_ABSENT'
    && authority.code !== 'AUTHORITY_UNREADABLE';
  if (!authority.ok && !stubDespiteAuthority) {
    return { ...authority, started: false, binaryPresent, spawnAttempted: false };
  }
  const socketPath = request.socketPath || join(runDir, 'node.socket');
  if (!pathInside(socketPath, runDir)) return closed('SOCKET_NOT_OWNED', { started: false, spawnAttempted: false });
  const configDir = request.configDir || join(runDir, 'config');
  if (!pathInside(configDir, runDir) || resolve(configDir) === resolve(runDir)) {
    return closed('CONFIG_NOT_OWNED', { started: false, spawnAttempted: false });
  }
  if (!binaryPresent) return closed('NODE_BINARY_ABSENT', { started: false, spawnAttempted: false });
  if (!existsSync(runDir)) mkdirSync(runDir, { recursive: true });
  const homeStat = statSync(runDir);
  if (!homeStat.isDirectory() || (typeof process.getuid === 'function' && homeStat.uid !== process.getuid())) {
    return closed('OWNED_HOME_REQUIRED', { started: false, spawnAttempted: false });
  }
  mkdirSync(configDir, { recursive: true });
  const port = prior.port || await checkPort(request.port ?? 0);
  if (!port.ok) return closed(port.code || 'PORT_CHECK_FAILED', { started: false, spawnAttempted: false, port });
  const layout = writeOwnedNodeLayout(runDir, configDir);
  const configPath = layout.configPath;
  const args = [
    'run',
    '--topology', layout.topologyPath,
    '--database-path', layout.databasePath,
    '--socket-path', socketPath,
    '--host-addr', '127.0.0.1',
    '--port', String(port.port),
    '--config', configPath,
  ];
  const stdoutPath = join(runDir, 'child.stdout');
  const stderrPath = join(runDir, 'child.stderr');
  const proof = join(runDir, 'proof.txt');
  rmSync(proof, { force: true });
  const child = spawn(prior.setsidPath || 'setsid', [nodeBin, ...args], {
    cwd: runDir,
    env: { PATH: process.env.PATH || '', HOME: runDir, CARDANO_NODE_SOCKET_PATH: socketPath },
    detached: true,
    stdio: ['ignore', openSync(stdoutPath, 'w'), openSync(stderrPath, 'w')],
  });
  let spawnError = null;
  child.on('error', (error) => {
    spawnError = error?.code || 'SPAWN_FAILED';
  });
  child.unref();
  const deadline = Date.now() + 2000;
  const ready = () => (realNode ? existsSync(socketPath) : existsSync(proof));
  while (!ready() && !spawnError && Date.now() < deadline) {
    await new Promise((resolveWait) => setTimeout(resolveWait, 20));
  }
  let provedPid = null;
  try {
    const text = existsSync(proof) ? readFileSync(proof, 'utf8') : '';
    const found = text.match(/^pid=(\d+)$/m);
    if (found) provedPid = Number(found[1]);
  } catch {
    provedPid = null;
  }
  const tracked = provedPid || child.pid;
  let childPgid = null;
  try {
    childPgid = pgidOf(tracked);
  } catch {
    childPgid = null;
  }
  const proved = realNode
    ? childPgid != null && childPgid !== ownPgid() && existsSync(socketPath) && !spawnError
    : childPgid != null && childPgid !== ownPgid() && existsSync(proof) && provedPid != null && !spawnError;
  return {
    ok: proved,
    code: proved ? 'OWNED_NODE_SPAWNED' : 'OWNED_NODE_START_FAILED',
    started: proved,
    spawnAttempted: true,
    pid: tracked,
    pgid: childPgid,
    ownPgid: ownPgid(),
    runDir,
    socketPath,
    configPath,
    shelleyGenesisPath: layout.shelleyPath,
    command: args,
    port,
    unitOnly: authority.unitFixture === true || !realNode,
    ledgerExecution: false,
    nativePins: false,
    binaryPresent: true,
    setsid: true,
    genesisComplete: false,
    readiness: realNode ? 'socket' : 'stub-proof-file',
  };
}

export function produceOwnedRuntime(request = {}) {
  const approved = request.approvedBootstrap === true && typeof request.runDir === 'string' && isAbsolute(request.runDir);
  const home = approved ? join(request.runDir, 'home') : (request.ownedHome || RUNTIME65_PATHS.exampleHome);
  const shape = {
    yaciBin: RUNTIME65_PATHS.yaci,
    ownedHome: home,
    packagedConfigDir: RUNTIME65_PATHS.yaciConfig,
    cli: RUNTIME65_PATHS.cli,
    runId: 't1-65-example',
    nodePort: 14011,
    submitApiPort: 14012,
    slotLengthSeconds: '1',
    blockTimeSeconds: '1',
    epochLengthSlots: '3600',
    era: 'conway',
    generateNewKeys: true,
    overwrite: true,
    executeNow: false,
    start: false,
  };
  const actorSkey = join(home, 'actors/payment.skey');
  const actorVkey = join(home, 'actors/payment.vkey');
  const socketPath = join(home, 'node.sock');
  const createNode = yaciCreateNodeArgv(shape);
  const futureStart = futureStartArgv(shape);
  const held = request.executeNow === true || request.spawn === true;
  const heldProducer = {
    ok: held ? false : createNode.ok === true,
    code: held ? 'REAL_NODE_NOT_ARMED' : 'RUNTIME_PRODUCER_HELD',
    spawned: false,
    executeNow: false,
    ledgerExecution: false,
    nativePins: false,
    nativeBudgetMeasured: false,
    ledgerFunding: false,
    runtimePinsCreated: false,
    liveHome: false,
    slotLengthSeconds: 1,
    epochLengthSlots: 3600,
    helpEpochDefault: 600,
    createNode,
    futureStart,
    ports: portClaim(),
    emptyGenesis: inspectGenesis({}),
    readiness: assessReadiness({ allowRealNode: false, ownedHome: home, socketPath }),
    proofTxt: assessReadiness({ allowRealNode: true, proofTxtOnly: true, ownedHome: home }),
    wrapper: rejectLauncher(['node', 'start.mjs']),
    keygen: addressKeyGenArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      signingKeyFile: actorSkey,
      verificationKeyFile: actorVkey,
    }),
    addressBuild: addressBuildArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      paymentVkeyFile: actorVkey,
    }),
    initialAddr: initialAddrArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      verificationKeyFile: actorVkey,
    }),
    faucetBuild: faucetBuildArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      outFile: join(home, 'faucet-body.json'),
    }),
    minFee: calculateMinFeeArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      txBodyFile: join(home, 'faucet-body.json'),
      protocolParamsFile: join(home, 'protocol.json'),
      witnessCount: 1,
    }),
    sign: signArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      signingKeyFile: actorSkey,
      txBodyFile: join(home, 'faucet-body.json'),
      outFile: join(home, 'faucet.signed'),
    }),
    submit: submitArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      socketPath,
      txFile: join(home, 'faucet.signed'),
    }),
    confirm: fundingConfirmArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      socketPath,
      address: request.actorAddress || null,
    }),
    tip: queryTipArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      socketPath,
      allowRealNode: false,
    }),
    eraHistory: queryEraHistoryArgv({
      cli: RUNTIME65_PATHS.cli,
      ownedHome: home,
      socketPath,
      allowRealNode: false,
    }),
  };
  const schema71 = buildOwnedProducerPlan({
    cli: RUNTIME65_PATHS.cli,
    ownedHome: home,
    runId: request.runId || 't1-65-example',
    nodePort: request.nodePort ?? shape.nodePort,
    submitApiPort: request.submitApiPort ?? shape.submitApiPort,
    shelleyGenesisPath: request.shelleyGenesisPath || null,
    magic: request.magic,
    magicSource: request.magicSource,
    supplyLovelace: request.supplyLovelace,
    nodeBin: request.nodeBin || CARDANO_NODE,
    topLevelGenesis: request.topLevelGenesis === true,
    slotLength: request.slotLength,
    genesisProfile: request.genesisProfile,
    proofTxtOnly: request.proofTxtOnly === true,
    launcher: request.launcher,
    evaluate: request.evaluate === true,
    command: request.command,
    socketPath: request.socketPath,
    txFile: request.txFile,
    financialFrame: request.financialFrame === true,
    costMode: request.costMode,
    eraHistoryFile: request.eraHistoryFile,
    utxoFile: request.utxoFile,
    protocolParamsFile: request.protocolParamsFile,
    outputs: request.outputs,
    feeRequest: request.feeRequest,
    byronGenesis: request.byronGenesis,
    costOutFile: request.costOutFile,
    templates: request.templates,
    approvedBootstrap: request.approvedBootstrap === true,
    nowMs: request.nowMs,
    leadSeconds: request.leadSeconds,
    runDir: request.runDir,
  });
  if (approved && schema71.inputsReady === true) {
    const nodeStep = (schema71.sequence || []).find((item) => item.id === 'S7-node-run');
    heldProducer.createNode = {
      ok: Array.isArray(nodeStep?.argv),
      code: 'APPROVED_NODE_ARGV',
      spawned: false,
      executeNow: false,
      nodeStarted: false,
      ledgerExecution: false,
      argv: nodeStep?.argv || null,
    };
    heldProducer.ok = held ? false : heldProducer.createNode.ok === true;
    heldProducer.code = held ? 'REAL_NODE_NOT_ARMED' : 'RUNTIME_PRODUCER_HELD';
  }
  heldProducer.schema71 = schema71;
  const armed = request.arm === true || request.armed === true;
  if (!held) return heldProducer;
  if (!armed) {
    return { ...heldProducer, ok: false, code: 'REAL_NODE_NOT_ARMED', spawned: false, executeNow: false };
  }
  const authority = loadAuthority(request.authority);
  if (!authority.ok) {
    return {
      ...heldProducer,
      ok: false,
      code: authority.code === 'EXECUTION_AUTHORITY_ABSENT' ? 'REAL_NODE_NOT_ARMED' : authority.code,
      spawned: false,
      executeNow: false,
      authority,
    };
  }
  if (authority.unitFixture || authority.evidenceClass !== 'reviewed-implementation') {
    return {
      ...heldProducer,
      ok: false,
      code: 'UNIT_FIXTURE_CANNOT_START_REAL_NODE',
      spawned: false,
      executeNow: false,
      authority,
    };
  }
  if (!schema71.inputsReady) {
    return {
      ...heldProducer,
      ok: false,
      code: schema71.code,
      spawned: false,
      executeNow: false,
      authorityAccepted: true,
      inputsReady: false,
      blockers: schema71.blockers,
    };
  }
  if (process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return {
      ...heldProducer,
      ok: false,
      code: 'REAL_NODE_NOT_ARMED',
      spawned: false,
      executeNow: false,
      authorityAccepted: true,
      inputsReady: true,
      wouldSpawn: schema71.sequence,
    };
  }
  return {
    ...heldProducer,
    ...dispatchOwnedProducer(schema71, { spawn: realProducerSpawn, unitOnly: false }),
    authorityAccepted: true,
    inputsReady: true,
  };
}

export function loadRuntimeInputs(root = join(TASK_ROOT, 't1-runtime-inputs-65')) {
  const names = ['RUNTIME-HANDOFF.md', 'RESULT.md', 'MACHINE.json', 'INPUT-SHA256.json'];
  const missing = names.filter((name) => !existsSync(join(root, name)));
  if (missing.length) {
    return closed('RUNTIME_INPUTS_ABSENT', { missing, adopted: false, partialLogsIgnored: true, spawned: false });
  }
  let machine;
  try {
    machine = JSON.parse(readFileSync(join(root, 'MACHINE.json'), 'utf8'));
  } catch {
    return closed('RUNTIME_INPUTS_INCOMPLETE', { adopted: false, spawned: false });
  }
  if (machine.outcome === 'COMMAND_SHAPES_READY_GAPS_OPEN' && machine.executeNow === false && machine.keysGenerated === false && machine.realNodeStarted === false) {
    const producer = produceOwnedRuntime();
    return {
      ok: producer.ok === true,
      code: 'RUNTIME_COMMANDS_READY',
      adopted: true,
      liveHome: false,
      spawned: false,
      executeNow: false,
      faucetInputs: [],
      ledgerExecution: false,
      nativePins: false,
      nativeBudgetMeasured: false,
      packetOutcome: machine.outcome,
      producer,
    };
  }
  const fields = ['home', 'socketPath', 'shelleyGenesisPath', 'faucetInputs', 'pid'];
  if (fields.some((key) => machine[key] == null) || !Array.isArray(machine.faucetInputs) || machine.faucetInputs.length < 1) {
    return closed('RUNTIME_INPUTS_INCOMPLETE', { adopted: false });
  }
  if (!existsSync(machine.shelleyGenesisPath) || !existsSync(machine.socketPath)) {
    return closed('RUNTIME_INPUTS_INCOMPLETE', { adopted: false });
  }
  const genesis = JSON.parse(readFileSync(machine.shelleyGenesisPath, 'utf8'));
  if (!genesis || genesis.slotLength !== 1 || Object.keys(genesis).length < 2) {
    return closed('RUNTIME_INPUTS_INCOMPLETE', { adopted: false, reason: 'genesis' });
  }
  return {
    ok: true,
    code: 'RUNTIME_INPUTS_ADOPTED',
    adopted: true,
    home: machine.home,
    socketPath: machine.socketPath,
    shelleyGenesisPath: machine.shelleyGenesisPath,
    faucetInputs: machine.faucetInputs,
    pid: machine.pid,
    ledgerExecution: false,
    nativePins: false,
    nativeBudgetMeasured: false,
  };
}

export function acquireBodyEvaluation({ cliPath = CARDANO_CLI, bodyPath = '', socket = '', execute = false, ...rest } = {}) {
  const held = {
    nativeBudgetMeasured: false,
    minAdaMeasured: false,
    submitBlocked: true,
    offlineUnitsAreNotMeasurement: true,
    protocolMinAda: 'unmeasured',
  };
  const command = 'conway transaction calculate-plutus-script-cost';
  if (rest.command === 'evaluate' || rest.evaluate === true) {
    return closed('NO_CLI_EVALUATE_COMMAND', { ...held, spawned: false, command });
  }
  if (!bodyPath || !existsSync(bodyPath)) return closed('EVALUATION_UNAVAILABLE', { ...held, reason: 'body-absent', spawned: false, command });
  if (execute !== true) return closed('EVALUATION_UNAVAILABLE', { ...held, reason: 'not-run', spawned: false, command });
  const plan = plutusCostArgv({ ...rest, cli: cliPath, cliPath, bodyPath, socket });
  if (realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_QUERY_NOT_ARMED', { ...held, spawned: false, command, argv: plan.argv || null });
  }
  if (!plan.ok) return closed(plan.code, { ...held, spawned: false, command: plan.command || command, argv: plan.argv || null });
  if (plan.mode !== 'offline' && (!socket || !existsSync(socket))) {
    return closed('EVALUATION_UNAVAILABLE', { ...held, reason: 'socket-absent', spawned: false, command, argv: plan.argv });
  }
  const result = spawnSync(cliPath, plan.argv.slice(1), { encoding: 'utf8' });
  let parsed = result.stdout || '';
  try { parsed = JSON.parse(parsed); } catch { /* a non-JSON cost body fails closed in the parser */ }
  const consumed = consumePlutusCost(parsed, { unitOnly: !realCli(cliPath) });
  if (!consumed.ok || result.status !== 0) {
    return closed(consumed.code || 'EVALUATION_UNAVAILABLE', {
      ...held,
      spawned: true,
      unitOnly: !realCli(cliPath),
      command,
      cliStatus: result.status,
    });
  }
  const measured = realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE === '1';
  return {
    ...consumed,
    ok: measured,
    code: measured ? 'BODY_EVALUATED' : 'PLUTUS_COST_PARSED',
    spawned: true,
    nativeBudgetMeasured: false,
    submitBlocked: true,
    executionSource: measured ? 'body-evaluator' : null,
    measuredByEvaluator: measured,
    queryProvenance: measured ? 'spawned-query' : 'unit-cli-spawn',
    unitOnly: !measured,
    command,
    ledgerExecution: false,
    nativePins: false,
  };
}

export async function inspectLauncher(request = {}) {
  const wrapper = `${request.wrapper || ''} ${request.command || ''}`;
  if (wrapper.includes('yaci') || wrapper.includes('npm')) return closed('YACI_WRAPPER_REFUSED', { started: false });
  const nodeBin = request.nodeBin || CARDANO_NODE;
  if (String(nodeBin).includes('yaci')) return closed('YACI_WRAPPER_REFUSED', { started: false });
  const setsid = spawnSync('sh', ['-c', 'command -v setsid'], { encoding: 'utf8' });
  const setsidPath = String(setsid.stdout || '').trim();
  const port = await checkPort(request.port ?? 0);
  let config = { skipped: true };
  if (request.configDir) {
    if (!existsSync(request.configDir)) config = { ok: false, code: 'CONFIG_ABSENT' };
    else {
      const st = statSync(request.configDir);
      config = {
        ok: st.isDirectory() && (typeof process.getuid !== 'function' || st.uid === process.getuid()),
        code: st.isDirectory() ? 'CONFIG_OWNED' : 'CONFIG_NOT_DIR',
      };
    }
  }
  if (!existsSync(nodeBin)) {
    const missing = spawnSync(nodeBin, ['--version'], { encoding: 'utf8' });
    return closed('NODE_BINARY_ABSENT', {
      started: false,
      spawnAttempted: true,
      spawnError: missing.error ? missing.error.code : null,
      setsid: Boolean(setsidPath),
      port,
      config,
    });
  }
  if (request.startNode === true || request.execute === true) {
    return launchOwnedRuntime(request, { setsidPath, port, config, nodeBin });
  }
  const version = spawnSync(nodeBin, ['--version'], { encoding: 'utf8' });
  return {
    ok: version.status === 0 && port.ok === true && Boolean(setsidPath),
    code: version.status === 0 ? 'LAUNCHER_CHECKED' : 'NODE_VERSION_FAILED',
    started: false,
    binaryPresent: true,
    version: String(version.stdout || version.stderr || '').trim().slice(0, 160),
    setsid: Boolean(setsidPath),
    processGroup: setsidPath ? 'setsid' : 'setsid-absent',
    port,
    config,
    ledgerExecution: false,
    nativePins: false,
  };
}

function realCli(cliPath) {
  try {
    return realpathSync(cliPath) === realpathSync(CARDANO_CLI);
  } catch {
    return false;
  }
}

export function parseCliUtxoJson(text) {
  const parsed = parseJsonExact(text);
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) return [];
  const utxos = [];
  for (const [ref, body] of Object.entries(parsed)) {
    const match = String(ref).match(/^([0-9a-f]{64})#(\d+)$/);
    if (!match) continue;
    utxos.push({ txId: match[1], index: Number(match[2]), value: body?.value || body || null });
  }
  return utxos;
}

export function queryOwnedUtxos({ cliPath, socket, runDir, execute = false, magic = null } = {}) {
  if (execute !== true) return closed('UTXO_QUERY_NOT_RUN', { queried: false, spawned: false });
  if (!cliPath || !existsSync(cliPath) || !socket || !existsSync(socket) || !runDir || !isAbsolute(runDir)) {
    return closed('UTXO_QUERY_ABSENT', { queried: false, spawned: false });
  }
  const real = realCli(cliPath);
  if (real && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_QUERY_NOT_ARMED', { queried: false, spawned: false });
  }
  const boundMagic = genesisMagicText(magic);
  if (!boundMagic.ok) return closed(boundMagic.code, { queried: false, spawned: false, genesisMagicBound: false });
  const runId = randomBytes(16).toString('hex');
  const transcriptPath = join(runDir, `utxo-${runId}.txt`);
  const args = [
    'query', 'utxo',
    '--socket-path', socket,
    '--testnet-magic', boundMagic.text,
    '--whole-utxo',
    '--output-json',
    '--out-file', transcriptPath,
  ];
  const result = spawnSync(cliPath, args, { encoding: 'utf8' });
  if (result.status !== 0 || !existsSync(transcriptPath)) {
    return closed('UTXO_QUERY_ABSENT', { spawned: true, queried: false, selector: '--whole-utxo' });
  }
  const text = readFileSync(transcriptPath, 'utf8');
  const trimmed = text.trim();
  const found = trimmed.startsWith('{') ? null : text.match(/^utxos=(\[.*\])$/m);
  let utxos = [];
  try {
    if (trimmed.startsWith('{')) {
      writeFileSync(`${transcriptPath}.provenance`, `runId=${runId}\n`);
      utxos = parseCliUtxoJson(trimmed);
    } else {
      appendFileSync(transcriptPath, `runId=${runId}\n`);
      if (found) utxos = JSON.parse(found[1]);
      else utxos = parseCliUtxoJson(text);
    }
  } catch (error) {
    return closed(error?.code || 'UTXO_QUERY_ABSENT', { spawned: true, queried: false, transcriptPath });
  }
  if (!Array.isArray(utxos)) return closed('UTXO_QUERY_ABSENT', { spawned: true, queried: false });
  const marked = !real || text.includes('evidenceClass=unit-fixture');
  const row = { unitOnly: marked, transcriptPath, socket, cliPath, runId };
  ownedQueries.set(runId, row);
  return {
    ok: utxos.length > 0,
    code: 'UTXO_QUERIED',
    spawned: true,
    queried: true,
    unitOnly: marked,
    transcriptPath,
    socket,
    utxos,
    runId,
    selector: '--whole-utxo',
    ledgerExecution: false,
    nativePins: false,
  };
}

export function queryProtocolParameters({ socket, cliPath = CARDANO_CLI, magic = null, execute = false } = {}) {
  if (!socket || !existsSync(socket)) {
    return closed('PROTOCOL_PARAMS_ABSENT', { queried: false, nativeBudgetMeasured: false });
  }
  if (execute !== true) {
    return closed('PROTOCOL_PARAMS_QUERY_NOT_RUN', {
      queried: false,
      socketPresent: true,
      nativeBudgetMeasured: false,
      command: 'cardano-cli query protocol-parameters --socket-path <owned> --testnet-magic <magic> --out-file <owned>',
    });
  }
  if (realCli(cliPath) && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_QUERY_NOT_ARMED', { queried: false, spawned: false, nativeBudgetMeasured: false });
  }
  const boundMagic = genesisMagicText(magic);
  if (!boundMagic.ok) return closed(boundMagic.code, { queried: false, spawned: false, nativeBudgetMeasured: false, genesisMagicBound: false });
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-params-'));
  try {
    const out = join(dir, 'protocol.json');
    const result = spawnSync(cliPath, [
      'query', 'protocol-parameters',
      '--socket-path', socket,
      '--testnet-magic', boundMagic.text,
      '--out-file', out,
    ], { encoding: 'utf8' });
    if (result.status !== 0 || !existsSync(out)) {
      return closed('PROTOCOL_PARAMS_ABSENT', { queried: false, cliStatus: result.status, nativeBudgetMeasured: false });
    }
    const params = JSON.parse(readFileSync(out, 'utf8'));
    const unitOnly = !realCli(cliPath) || params.evidenceClass === 'unit-fixture';
    return {
      ok: true,
      code: 'PROTOCOL_PARAMS_QUERIED',
      queried: true,
      spawned: true,
      source: unitOnly ? 'unit-cli-spawn' : 'cardano-cli-query',
      unitOnly,
      nativeBudgetMeasured: false,
      protocolParams: params,
      coinsPerUTxOByte: params.coinsPerUTxOByte ?? null,
      utxoCostPerByte: params.utxoCostPerByte ?? null,
      maxTxExecutionUnits: params.maxTxExecutionUnits || null,
      maxTxSize: params.maxTxSize ?? null,
      ledgerExecution: false,
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

export function compareBudget(input = {}) {
  const params = input.protocolParams || input;
  if (params.queried !== true) {
    return closed('PROTOCOL_PARAMS_ABSENT', { submitBlocked: true, nativeBudgetMeasured: false, minAdaMeasured: false });
  }
  const units = params.maxTxExecutionUnits;
  const cost = input.cost || params.cost;
  if (!units || cost == null || units.memory == null || units.steps == null || cost.memory == null || cost.steps == null) {
    return closed('PROTOCOL_PARAMS_ABSENT', { submitBlocked: true, nativeBudgetMeasured: false, minAdaMeasured: false });
  }
  let over = false;
  try {
    over = BigInt(cost.memory) > BigInt(units.memory) || BigInt(cost.steps) > BigInt(units.steps);
  } catch {
    return closed('MALFORMED_DATA', { submitBlocked: true });
  }
  const sizeOver = params.maxTxSize != null && input.txBytes != null && Number(input.txBytes) > Number(params.maxTxSize);
  const perByte = params.utxoCostPerByte ?? input.utxoCostPerByte ?? null;
  const observedAlias = perByte == null ? (params.coinsPerUTxOByte ?? input.coinsPerUTxOByte ?? null) : null;
  const rate = perByte ?? observedAlias;
  const txBytes = input.txBytes;
  const protocolMinAda = 'unmeasured';
  const minAdaMeasured = input.minAdaSource === 'calculate-min-required-utxo' && input.minAdaMeasured === true;
  const formulaRefusal = rate != null && txBytes != null && Number.isFinite(Number(txBytes))
    ? refuseCallerMinAdaFormula(rate, txBytes)
    : null;
  const evaluatorMeasured = input.measuredByEvaluator === true
    && input.executionSource === 'body-evaluator'
    && params.source === 'cardano-cli-query'
    && params.unitOnly !== true
    && params.queryProvenance === 'spawned-query';
  const nativeBudgetMeasured = evaluatorMeasured && minAdaMeasured && formulaRefusal == null;
  const quantities = {
    kernelCash: '100',
    unitPayout: '2000000',
    protocolMinAda,
  };
  if (over || sizeOver) {
    return closed('BUDGET_EXCEEDED', {
      submitBlocked: true,
      nativeBudgetMeasured: false,
      unitLogIsNotBudget: true,
      minAdaMeasured,
      quantities,
    });
  }
  return {
    ok: true,
    code: 'BUDGET_WITHIN_SUPPLIED_MAX',
    submitBlocked: evaluatorMeasured && minAdaMeasured !== true,
    nativeBudgetMeasured,
    unitOnly: params.unitOnly === true,
    minAdaMeasured,
    formulaRefusal,
    minAdaCommand: 'conway transaction calculate-min-required-utxo',
    quantities,
    ledgerExecution: false,
    nativePins: false,
  };
}

export async function submitConcurrentPair(pair, deps = {}) {
  if (!pair?.ok || !Array.isArray(pair.members) || pair.members.length !== 2) return closed('MALFORMED_DATA');
  if (!Array.isArray(pair.shared) || pair.shared.length !== 1 || pair.shared[0] !== 'head') return closed('RACE_SHARED_NOT_ONLY_HEAD');
  if (pair.members[0].fee === pair.members[1].fee || pair.members[0].collateral === pair.members[1].collateral) {
    return closed('RACE_INPUTS_NOT_DISJOINT');
  }
  const events = [];
  if (typeof deps.submit !== 'function') {
    return closed('NOT_EXECUTED', {
      events,
      replay: { separate: true, executed: false, expectedClass: 'BadInputsUTxO' },
    });
  }
  const ids = [];
  for (const member of pair.members) {
    events.push({ event: 'submit', head: member.head, fee: member.fee, collateral: member.collateral });
    ids.push(await deps.submit(member));
  }
  for (const id of ids) {
    events.push({ event: 'confirm', id: deps.unitOnly === true ? 'unit-only' : id });
    if (typeof deps.poll === 'function') await deps.poll(id);
  }
  const submitPositions = events.map((item, index) => (item.event === 'submit' ? index : -1)).filter((index) => index >= 0);
  const confirmPositions = events.map((item, index) => (item.event === 'confirm' ? index : -1)).filter((index) => index >= 0);
  const submitsBeforeConfirms = confirmPositions.every((index) => index > Math.max(...submitPositions));
  return {
    ok: submitsBeforeConfirms,
    code: 'RACE_ORDER_UNIT_ONLY',
    unitOnly: deps.unitOnly === true,
    ledgerExecution: false,
    nativeAcceptance: false,
    submitted: false,
    ledgerSubmitted: false,
    confirmed: false,
    submitCount: submitPositions.length,
    confirmCount: confirmPositions.length,
    submitsBeforeConfirms,
    events,
    replay: { separate: true, executed: false, expectedClass: 'BadInputsUTxO' },
  };
}

export function assertOwnedPath(path) {
  if (!path || !isAbsolute(path)) return closed('OUTPUT_PATH_FORBIDDEN');
  return resolveOutputPath(path, path);
}

export function ensureDir(path) {
  const checked = resolveOutputPath(path, path);
  if (!checked.ok) return checked;
  mkdirSync(checked.path, { recursive: true });
  return checked;
}
