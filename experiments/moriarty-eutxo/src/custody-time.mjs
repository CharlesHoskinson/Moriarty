/**
 * Offline time arithmetic for the approved T1 recipe.
 * A successful plan is integer math only. It is not a node measurement.
 * TIME-CALIBRATION-RECIPE.json names some uint128 overflows IMPOSSIBLE_BOUNDARY.
 * addU128 in the approved codec returns OVERFLOW. This module keeps OVERFLOW.
 */

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
  };
}

export function calibrateGuards(evidence) {
  if (evidence === undefined || evidence === null) return refuse('PARITY_ABSENT');
  if (typeof evidence !== 'object') return refuse('MALFORMED_DATA');
  if (evidence.probeExecuted !== true) return refuse('PARITY_ABSENT');
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
  return refuse('CALIBRATED_SOURCE_NOT_THIS_PHASE');
}
