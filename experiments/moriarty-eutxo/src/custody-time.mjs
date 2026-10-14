/**
 * Offline time arithmetic for the approved T1 recipe.
 * A successful plan is integer math only. It is not a node measurement.
 * TIME-CALIBRATION-RECIPE.json names some uint128 overflows IMPOSSIBLE_BOUNDARY.
 * addU128 in the approved codec returns OVERFLOW. This module keeps OVERFLOW.
 * CLI validity is half-open slot bounds. Redeemer lower/upper are POSIX
 * milliseconds: the slot begin, and the inclusive last millisecond.
 */
import { existsSync, readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { slotToBeginUnixTime, unixTimeToEnclosingSlot } from '../owned/node_modules/@lucid-evolution/plutus/dist/index.js';

import { isAuthenticParityCapture } from './posix-parity-harness.mjs';

const authenticAdmissions = new WeakMap();
export function isAuthenticProductionAdmission(value) {
  return value != null && authenticAdmissions.get(value) === JSON.stringify(value);
}

const UINT128_MAX = 340282366920938463463374607431768211455n;
const AMOUNT_BODY = /^(0|[1-9][0-9]*)$/;

function refuse(code) {
  return { ok: false, code, sourceEmitted: false, ledgerExecution: false };
}

function parseAmount(value) {
  if (typeof value !== 'string') return refuse('NON_CANONICAL_VALUE');
  if (value.startsWith('-')) return refuse('NEGATIVE_POSIX');
  if (!AMOUNT_BODY.test(value)) return refuse('NON_CANONICAL_VALUE');
  const n = BigInt(value);
  if (n > UINT128_MAX) return refuse('OVERFLOW');
  return { ok: true, value: n };
}

function addU128(a, b) {
  if (a < 0n || b < 0n || a > UINT128_MAX || b > UINT128_MAX) return refuse('OVERFLOW');
  const sum = a + b;
  if (sum > UINT128_MAX) return refuse('OVERFLOW');
  return { ok: true, value: sum };
}

function mulU128(a, b) {
  if (a < 0n || b < 0n || a > UINT128_MAX || b > UINT128_MAX) return refuse('OVERFLOW');
  if (a !== 0n && b > UINT128_MAX / a) return refuse('OVERFLOW');
  return { ok: true, value: a * b };
}

function slotToBegin(slot, cfg) {
  if (slot < cfg.zeroSlot) return refuse('NEGATIVE_POSIX');
  const span = mulU128(slot - cfg.zeroSlot, cfg.slotLengthMs);
  if (!span.ok) return span;
  return addU128(cfg.zeroTime, span.value);
}

function enclosing(posix, cfg) {
  if (posix < cfg.zeroTime) return refuse('NEGATIVE_POSIX');
  return { ok: true, value: cfg.zeroSlot + (posix - cfg.zeroTime) / cfg.slotLengthMs };
}

export function planWindows(input) {
  if (input === null || typeof input !== 'object') return refuse('MALFORMED_DATA');
  const zeroTime = parseAmount(input.zeroTime);
  if (!zeroTime.ok) return zeroTime;
  const zeroSlot = parseAmount(input.zeroSlot);
  if (!zeroSlot.ok) return zeroSlot;
  const slotLengthMs = parseAmount(input.slotLengthMs);
  if (!slotLengthMs.ok) return slotLengthMs;
  const tipSlot = parseAmount(input.tipSlot);
  if (!tipSlot.ok) return tipSlot;
  if (slotLengthMs.value !== 1000n) return refuse('SLOT_LENGTH_REFUSED');
  const cfg = { zeroTime: zeroTime.value, zeroSlot: zeroSlot.value, slotLengthMs: slotLengthMs.value };
  const margin = 30n;
  const window = 60n;
  const horizonMs = 1800n * 1000n;
  const lookahead = mulU128(margin + 2n * window, cfg.slotLengthMs);
  if (!lookahead.ok) return lookahead;
  if (lookahead.value > horizonMs) return refuse('HORIZON_EXCEEDED');
  const tipPosix = slotToBegin(tipSlot.value, cfg);
  if (!tipPosix.ok) return tipPosix;
  const releaseSlot = addU128(tipSlot.value, margin);
  if (!releaseSlot.ok) return releaseSlot;
  const releaseStart = slotToBegin(releaseSlot.value, cfg);
  if (!releaseStart.ok) return releaseStart;
  const windowMs = mulU128(window, cfg.slotLengthMs);
  if (!windowMs.ok) return windowMs;
  const partitionTry = addU128(releaseStart.value, windowMs.value);
  if (!partitionTry.ok) return partitionTry;
  const refundTry = addU128(partitionTry.value, windowMs.value);
  if (!refundTry.ok) return refundTry;
  const releaseEnclosing = enclosing(releaseStart.value, cfg);
  if (!releaseEnclosing.ok) return releaseEnclosing;
  const partitionEnclosing = enclosing(partitionTry.value, cfg);
  if (!partitionEnclosing.ok) return partitionEnclosing;
  const refundEnclosing = enclosing(refundTry.value, cfg);
  if (!refundEnclosing.ok) return refundEnclosing;
  if (!(partitionEnclosing.value > releaseEnclosing.value)) return refuse('IMPOSSIBLE_BOUNDARY');
  if (!(refundEnclosing.value > partitionEnclosing.value)) return refuse('IMPOSSIBLE_BOUNDARY');
  if (refundTry.value - tipPosix.value > horizonMs) return refuse('HORIZON_EXCEEDED');
  const marginMs = mulU128(margin, cfg.slotLengthMs);
  if (!marginMs.ok) return marginMs;
  const earliest = addU128(tipPosix.value, marginMs.value);
  if (!earliest.ok) return earliest;
  if (releaseStart.value < earliest.value) return refuse('INCLUSION_MARGIN');
  return {
    ok: true,
    evidenceClass: 'offline-integer-math-not-native',
    ledgerExecution: false,
    sourceEmitted: false,
    forecastHorizonSec: '1800',
    forecastIsSleep: false,
    marginSlots: '30',
    windowSlots: '60',
    slotLengthMs: '1000',
    tipSlot: tipSlot.value.toString(10),
    tipPosix: tipPosix.value.toString(10),
    releaseStart: releaseStart.value.toString(10),
    partition: partitionTry.value.toString(10),
    refundExcl: refundTry.value.toString(10),
    releaseArgs: {
      lower: releaseStart.value.toString(10),
      upper: (partitionTry.value - 1n).toString(10),
    },
    refundArgs: {
      lower: partitionTry.value.toString(10),
      upper: (refundTry.value - 1n).toString(10),
    },
    lookaheadMs: lookahead.value.toString(10),
    overflowCode: 'OVERFLOW',
    recipeOverflowLabel: 'IMPOSSIBLE_BOUNDARY',
    releaseSlots: {
      startSlot: releaseEnclosing.value.toString(10),
      endSlot: partitionEnclosing.value.toString(10),
    },
    refundSlots: {
      startSlot: partitionEnclosing.value.toString(10),
      endSlot: refundEnclosing.value.toString(10),
    },
    cancelSlots: {
      startSlot: releaseEnclosing.value.toString(10),
      endSlot: partitionEnclosing.value.toString(10),
    },
    cliInterval: 'half-open-slots',
    redeemerInterval: 'posix-begin-and-inclusive-last-ms',
  };
}

function loadClock(input) {
  if (input === null || typeof input !== 'object') return refuse('MALFORMED_DATA');
  const zeroTime = parseAmount(input.zeroTime);
  if (!zeroTime.ok) return zeroTime;
  const zeroSlot = parseAmount(input.zeroSlot);
  if (!zeroSlot.ok) return zeroSlot;
  const slotLengthMs = parseAmount(input.slotLengthMs);
  if (!slotLengthMs.ok) return slotLengthMs;
  if (slotLengthMs.value !== 1000n) return refuse('SLOT_LENGTH_REFUSED');
  return { ok: true, zeroTime: zeroTime.value, zeroSlot: zeroSlot.value, slotLengthMs: slotLengthMs.value };
}

function safeNumber(value) {
  if (value < 0n || value > BigInt(Number.MAX_SAFE_INTEGER)) return null;
  return Number(value);
}

function lucidBegin(slot, clock) {
  const slotN = safeNumber(slot);
  const zeroTime = safeNumber(clock.zeroTime);
  const zeroSlot = safeNumber(clock.zeroSlot);
  const width = safeNumber(clock.slotLengthMs);
  if (slotN == null || zeroTime == null || zeroSlot == null || width == null) return null;
  return slotToBeginUnixTime(slotN, { zeroTime, zeroSlot, slotLength: width });
}

export function slotsToRedeemer(input) {
  const clock = loadClock(input);
  if (!clock.ok) return clock;
  const start = parseAmount(input.startSlot);
  if (!start.ok) return start;
  const end = parseAmount(input.endSlot);
  if (!end.ok) return end;
  if (start.value < clock.zeroSlot) return refuse('NEGATIVE_POSIX');
  if (end.value <= start.value) return refuse('EMPTY_INTERVAL');
  const lowerSpan = mulU128(start.value - clock.zeroSlot, clock.slotLengthMs);
  if (!lowerSpan.ok) return lowerSpan;
  const lower = addU128(clock.zeroTime, lowerSpan.value);
  if (!lower.ok) return lower;
  const endSpan = mulU128(end.value - clock.zeroSlot, clock.slotLengthMs);
  if (!endSpan.ok) return endSpan;
  const endPosix = addU128(clock.zeroTime, endSpan.value);
  if (!endPosix.ok) return endPosix;
  if (endPosix.value === 0n) return refuse('NEGATIVE_POSIX');
  const lucid = lucidBegin(start.value, clock);
  if (lucid != null && BigInt(lucid) !== lower.value) return refuse('IMPOSSIBLE_BOUNDARY');
  return {
    ok: true,
    lower: lower.value.toString(10),
    upper: (endPosix.value - 1n).toString(10),
    invalidBefore: start.value.toString(10),
    invalidHereafter: end.value.toString(10),
    ledgerExecution: false,
    sourceEmitted: false,
    evidenceClass: 'offline-integer-math-not-native',
  };
}

export function posixWindowToSlots(input) {
  const clock = loadClock(input);
  if (!clock.ok) return clock;
  const lower = parseAmount(input.lower);
  if (!lower.ok) return lower;
  const upper = parseAmount(input.upper);
  if (!upper.ok) return upper;
  if (lower.value < clock.zeroTime || upper.value < clock.zeroTime) return refuse('NEGATIVE_POSIX');
  if (upper.value < lower.value) return refuse('EMPTY_INTERVAL');
  const lowerDelta = lower.value - clock.zeroTime;
  const upperDelta = upper.value - clock.zeroTime;
  if (lowerDelta % clock.slotLengthMs !== 0n) return refuse('ROUNDING');
  if (upperDelta % clock.slotLengthMs === 0n) {
    return refuse(input.kind === 'refund' ? 'REFUND_UPPER_EXCLUSIVE' : 'ROUNDING');
  }
  if (upperDelta % clock.slotLengthMs !== clock.slotLengthMs - 1n) return refuse('ROUNDING');
  const startSlot = clock.zeroSlot + lowerDelta / clock.slotLengthMs;
  const endSlot = clock.zeroSlot + (upperDelta + 1n) / clock.slotLengthMs;
  if (endSlot <= startSlot) return refuse('EMPTY_INTERVAL');
  const slotN = safeNumber(startSlot);
  const zeroTime = safeNumber(clock.zeroTime);
  const zeroSlot = safeNumber(clock.zeroSlot);
  const lowerN = safeNumber(lower.value);
  if (slotN != null && zeroTime != null && zeroSlot != null && lowerN != null) {
    const enclosing = unixTimeToEnclosingSlot(lowerN, { zeroTime, zeroSlot, slotLength: 1000 });
    if (BigInt(enclosing) !== startSlot) return refuse('IMPOSSIBLE_BOUNDARY');
  }
  return {
    ok: true,
    lower: lower.value.toString(10),
    upper: upper.value.toString(10),
    startSlot: startSlot.toString(10),
    endSlot: endSlot.toString(10),
    invalidBefore: startSlot.toString(10),
    invalidHereafter: endSlot.toString(10),
    ledgerExecution: false,
    sourceEmitted: false,
  };
}

export function bindSpendTime(input) {
  const converted = posixWindowToSlots(input || {});
  if (!converted.ok) {
    return {
      ok: false,
      code: 'FIXTURE_WINDOW_UNREPRESENTABLE',
      detail: converted.code,
      kind: input?.kind || null,
      redeemerLower: typeof input?.lower === 'string' ? input.lower : null,
      redeemerUpper: typeof input?.upper === 'string' ? input.upper : null,
      cliIntervalEmitted: false,
      ledgerExecution: false,
      production: false,
      sourceEmitted: false,
    };
  }
  return {
    ok: true,
    code: 'SLOT_POSIX_SEPARATED',
    kind: input.kind || null,
    redeemerLower: converted.lower,
    redeemerUpper: converted.upper,
    invalidBefore: converted.startSlot,
    invalidHereafter: converted.endSlot,
    cliInterval: 'half-open-slots',
    cliIntervalEmitted: false,
    ledgerExecution: false,
    production: false,
    sourceEmitted: false,
  };
}

const GUARD_EDITS = [
  ['requires lower >= 1000;', 'releaseLower', 2],
  ['requires upper < 1100;', 'partition', 2],
  ['requires lower >= 1100;', 'partition', 1],
  ['requires upper <= 1200;', 'refundInclusive', 1],
];

export function actionTimeGuards(sourceText, actionName) {
  if (typeof sourceText !== 'string' || typeof actionName !== 'string' || !/^[a-z]+$/.test(actionName)) {
    return refuse('MALFORMED_DATA');
  }
  const line = sourceText.split('\n').find((row) => row.includes(`action ${actionName}(`));
  if (!line) return refuse('GUARD_SHAPE');
  const lowerMatch = line.match(/requires lower >= (\d+);/);
  const exclusive = line.match(/requires upper < (\d+);/);
  const inclusive = line.match(/requires upper <= (\d+);/);
  if (!lowerMatch || (exclusive && inclusive) || (!exclusive && !inclusive)) return refuse('GUARD_SHAPE');
  if (exclusive) {
    const upperEx = BigInt(exclusive[1]);
    if (upperEx === 0n) return refuse('EMPTY_INTERVAL');
    return {
      ok: true,
      action: actionName,
      lower: lowerMatch[1],
      upper: (upperEx - 1n).toString(10),
      exclusiveUpper: exclusive[1],
      upperForm: 'exclusive',
    };
  }
  return {
    ok: true,
    action: actionName,
    lower: lowerMatch[1],
    upper: inclusive[1],
    exclusiveUpper: (BigInt(inclusive[1]) + 1n).toString(10),
    upperForm: 'inclusive',
  };
}

const FIXED_UTXO = new Set(['8'.repeat(64), '0'.repeat(64)]);

export function isFixedUtxoProbe(seed) {
  const tx = typeof seed === 'string' ? seed : (seed?.txId || seed?.transactionId || '');
  return FIXED_UTXO.has(String(tx).toLowerCase());
}

export function isProductionProbeTime(time) {
  return Boolean(time)
    && time.evidenceClass === 'node-genesis-v3-probe'
    && time.productionArtifact === true
    && time.unitFixture !== true
    && time.probeExecuted === true
    && isAuthenticProductionAdmission(time.productionAdmission);
}

export function substituteTimeGuards(sourceText, bounds, mode = null) {
  if (typeof sourceText !== 'string') return refuse('MALFORMED_DATA');
  if (bounds === null || typeof bounds !== 'object') return refuse('MALFORMED_DATA');
  for (const key of ['releaseLower', 'partition', 'refundInclusive']) {
    if (typeof bounds[key] !== 'string' || !AMOUNT_BODY.test(bounds[key])) return refuse('NON_CANONICAL_VALUE');
  }
  let text = sourceText;
  for (const [phrase, key, expect] of GUARD_EDITS) {
    const found = text.split(phrase).length - 1;
    if (found !== expect) return refuse('GUARD_SHAPE');
    const operator = phrase.slice('requires '.length, phrase.indexOf('1000') > -1 && key === 'releaseLower'
      ? phrase.indexOf('1000')
      : phrase.search(/\d/));
    const rewritten = `requires ${operator}${bounds[key]};`;
    text = text.split(phrase).join(rewritten);
  }
  if (text.includes('requires lower >= 1000;') || text.includes('requires upper < 1100;') || text.includes('requires lower >= 1100;') || text.includes('requires upper <= 1200;')) {
    return refuse('GUARD_SHAPE');
  }
  const production = mode?.production === true;
  return {
    ok: true,
    text,
    evidenceClass: production ? 'node-genesis-v3-probe' : 'UNIT/SYNTHETIC',
    productionArtifact: production,
    sourceEmitted: false,
    ledgerExecution: false,
    nativeProbe: false,
  };
}

function hex64(value) {
  return typeof value === 'string' && /^[0-9a-f]{64}$/.test(value);
}

function fileSha(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

export function calibrateGuards(evidence) {
  if (evidence === undefined || evidence === null) return refuse('PARITY_ABSENT');
  if (typeof evidence !== 'object') return refuse('MALFORMED_DATA');
  if (evidence.probeExecuted !== true) return refuse('PARITY_ABSENT');
  if (evidence.unitOnly === true || evidence.evidenceClass === 'UNIT/SYNTHETIC' || evidence.evidenceClass === 'UNIT' || evidence.evidenceClass === 'SYNTHETIC') {
    return refuse('UNIT_PARITY_REFUSED');
  }
  if (evidence.evidenceClass !== 'node-genesis-v3-probe') return refuse('PARITY_NOT_NATIVE');
  if (evidence.slotLengthMs !== '1000') {
    if (typeof evidence.slotLengthMs === 'string' && evidence.slotLengthMs.startsWith('-')) return refuse('NEGATIVE_POSIX');
    return refuse('SLOT_LENGTH_REFUSED');
  }
  const fields = ['lucidPosix', 'cliPosix', 'scriptContextLower'];
  for (const field of fields) {
    const parsed = parseAmount(evidence[field]);
    if (!parsed.ok) return parsed;
  }
  if (evidence.lucidPosix !== evidence.cliPosix || evidence.lucidPosix !== evidence.scriptContextLower) {
    return refuse('IMPOSSIBLE_BOUNDARY');
  }
  const bindings = evidence.bindings;
  if (!bindings) return refuse('CALIBRATED_SOURCE_NOT_THIS_PHASE');
  if (bindings.forged === true) return refuse('FORGED_PARITY');
  if (!bindings.readback || bindings.readback.present !== true) return refuse('READBACK_BINDING_ABSENT');
  if (!hex64(bindings.readback.sha256)) return refuse('READBACK_BINDING_MISMATCH');
  if (!bindings.genesis || !hex64(bindings.genesis.sha256)) return refuse('GENESIS_BINDING_MISMATCH');
  if (!bindings.artifact || !hex64(bindings.artifact.sha256)) return refuse('ARTIFACT_BINDING_MISMATCH');
  if (!bindings.body || !hex64(bindings.body.sha256)) return refuse('BODY_BINDING_MISMATCH');
  if (typeof evidence.transcriptPath !== 'string' || !existsSync(evidence.transcriptPath)) {
    return refuse('PARITY_TRANSCRIPT_ABSENT');
  }
  let transcript;
  try {
    transcript = JSON.parse(readFileSync(evidence.transcriptPath, 'utf8'));
  } catch {
    return refuse('PARITY_TRANSCRIPT_ABSENT');
  }
  if (transcript.productionAdmitted !== true || transcript.networkCommandsRun !== true) {
    return refuse('PARITY_NOT_EXECUTED');
  }
  if (transcript.unitOnly === true || transcript.evidenceClass !== 'node-genesis-v3-probe') {
    return refuse('UNIT_PARITY_REFUSED');
  }
  if (!transcript.genesisPath || !existsSync(transcript.genesisPath) || fileSha(transcript.genesisPath) !== bindings.genesis.sha256) {
    return refuse('GENESIS_BINDING_MISMATCH');
  }
  if (!transcript.artifactPath || !existsSync(transcript.artifactPath) || fileSha(transcript.artifactPath) !== bindings.artifact.sha256) {
    return refuse('ARTIFACT_BINDING_MISMATCH');
  }
  if (!transcript.bodyPath || !existsSync(transcript.bodyPath) || fileSha(transcript.bodyPath) !== bindings.body.sha256) {
    return refuse('BODY_BINDING_MISMATCH');
  }
  if (!transcript.readbackPath || !existsSync(transcript.readbackPath) || fileSha(transcript.readbackPath) !== bindings.readback.sha256) {
    return refuse('READBACK_BINDING_MISMATCH');
  }
  if (String(transcript.cliPosix) !== evidence.cliPosix || String(transcript.lucidPosix) !== evidence.lucidPosix || String(transcript.scriptContextLower) !== evidence.scriptContextLower) {
    return refuse('IMPOSSIBLE_BOUNDARY');
  }
  if (bindings.artifact.sha256 === bindings.genesis.sha256 || bindings.artifact.sha256 === bindings.body.sha256) {
    return refuse('TAUTOLOGICAL_BINDING');
  }
  if (hex64(bindings.source?.sha256) && bindings.artifact.sha256 === bindings.source.sha256) {
    return refuse('TAUTOLOGICAL_BINDING');
  }
  const queriedSeed = evidence.queriedSeed || transcript.queriedSeed || null;
  if (queriedSeed && isFixedUtxoProbe(queriedSeed)) return refuse('FIXED_UTXO_PROBE');
  if (transcript.originTool !== 'executeParityCapture' || evidence.origin !== 'executeParityCapture') {
    return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
  }
  if (typeof transcript.commandLogSha256 !== 'string' || !hex64(transcript.commandLogSha256)) {
    return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
  }
  if (!transcript.commandLogPath || !existsSync(transcript.commandLogPath) || fileSha(transcript.commandLogPath) !== transcript.commandLogSha256) {
    return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
  }
  const commandLog = readFileSync(transcript.commandLogPath, 'utf8');
  if (!commandLog.includes('query tip') || !commandLog.includes('query era-history') || !commandLog.includes(String(transcript.genesisPath))) {
    return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
  }
  const capture = evidence.capture;
  if (!isAuthenticParityCapture(capture) || capture.transcriptPath !== evidence.transcriptPath
      || capture.cliPosix !== evidence.cliPosix || capture.lucidPosix !== evidence.lucidPosix
      || capture.scriptContextLower !== evidence.scriptContextLower
      || JSON.stringify(capture.bindings) !== JSON.stringify(bindings)) {
    return refuse('CONSTRUCTED_TRANSCRIPT_NOT_AUTHENTIC');
  }
  const windows = planWindows({ zeroTime: capture.zeroTime, zeroSlot: capture.zeroSlot,
    slotLengthMs: capture.slotLengthMs, tipSlot: capture.tipSlot });
  if (!windows.ok) return windows;
  const result = {
    ok: true,
    code: 'PARITY_ADMITTED',
    sourceEmitted: false,
    productionArtifact: false,
    ledgerExecution: false,
    siblingEmitted: false,
    nativeProbe: true,
    captureOrigin: 'executeParityCapture',
    measuredBounds: { releaseLower: windows.releaseStart, partition: windows.partition, refundInclusive: windows.refundArgs.upper },
    zeroTime: capture.zeroTime, zeroSlot: capture.zeroSlot, tipSlot: capture.tipSlot,
    genesisSha256: bindings.genesis.sha256,
    queriedSeed,
  };
  authenticAdmissions.set(result, JSON.stringify(result));
  return result;
}

export function productionParityGate(time) {
  if (!time || time.unitOnly === true) {
    return {
      ok: false,
      code: 'PARITY_NOT_ADMITTED',
      siblingEmitted: false,
      productionArtifact: false,
      sourceEmitted: false,
      ledgerExecution: false,
    };
  }
  return calibrateGuards({
    probeExecuted: time.probeExecuted === true,
    evidenceClass: time.evidenceClass,
    slotLengthMs: time.slotLengthMs || time.observed,
    lucidPosix: time.lucidPosix,
    cliPosix: time.cliPosix,
    scriptContextLower: time.scriptContextLower,
    unitOnly: time.unitOnly === true,
    bindings: time.bindings,
    transcriptPath: time.transcriptPath,
    measuredBounds: time.measuredBounds || null,
    queriedSeed: time.queriedSeed || null,
    origin: time.origin || null,
    capture: time.capture || null,
  });
}
