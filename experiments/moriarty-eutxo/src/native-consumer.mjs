/**
 * Callable T1 native consumer. Authoring stage does not start a node,
 * create a key, or submit. Absent pins stay NOT_EXECUTED.
 * Receipt import re-hashes the approved helper before use.
 */
import { createHash } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import { calibrateGuards, planWindows } from './custody-time.mjs';
import { bindWitness, encodeUnsignedBody, inspectTransportEnvelope } from './native-body.mjs';
import {
  assessCashPolicy,
  certifyRuntimePins,
  classifyFailure,
  compareBudget,
  inspectLauncher,
  queryProtocolParameters,
  resolveOutputPath,
  submitConcurrentPair,
} from './native-runtime.mjs';

export { bindWitness, encodeUnsignedBody, inspectTransportEnvelope };
export {
  assessCashPolicy,
  certifyRuntimePins,
  classifyFailure,
  compareBudget,
  inspectLauncher,
  queryProtocolParameters,
  resolveOutputPath,
  submitConcurrentPair,
};

export const RECEIPT_HELPER_PATH = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/transaction-receipts/revision02/submit-with-receipt.mjs';
export const RECEIPT_HELPER_SHA256 = 'f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18';
export const UNIT_PAYOUT_LOVELACE = 2000000;
export const KERNEL_CASH = 100;
const HEX28 = /^[0-9a-f]{56}$/;
const HEX32 = /^[0-9a-f]{64}$/;
const FIXTURE_REPEAT = /^(.)\1{55}$/;

function closed(code, extra = {}) {
  return {
    ok: false,
    code,
    ledgerExecution: false,
    nativePins: false,
    runtimePinsCreated: false,
    confirmed: false,
    ...extra,
  };
}

export function hashFile(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

export function loadReceiptHelper(path = RECEIPT_HELPER_PATH) {
  const digest = hashFile(path);
  if (digest !== RECEIPT_HELPER_SHA256) {
    return closed('RECEIPT_HELPER_HASH', { digest, expected: RECEIPT_HELPER_SHA256 });
  }
  return { ok: true, digest, path, ledgerExecution: false, nativeAcceptance: false };
}

export async function importReceiptHelper(path = RECEIPT_HELPER_PATH) {
  const checked = loadReceiptHelper(path);
  if (!checked.ok) return checked;
  const helper = await import(pathToFileURL(checked.path).href);
  return { ...checked, helper };
}

export function assertPinsAbsent(pins) {
  return certifyRuntimePins(pins);
}

export function separateQuantities(input) {
  if (!input || typeof input !== 'object') return closed('MALFORMED_DATA');
  const protocolMinAda = input.protocolMinAda;
  const kernelCash = input.kernelCash;
  const unitPayout = input.unitPayout;
  if (protocolMinAda == null) return closed('MIN_ADA_UNMEASURED');
  if (String(kernelCash) !== String(KERNEL_CASH)) return closed('KERNEL_CASH', { kernelCash });
  if (String(unitPayout) !== String(UNIT_PAYOUT_LOVELACE)) return closed('UNIT_PAYOUT', { unitPayout });
  if (String(protocolMinAda) === String(unitPayout) || String(protocolMinAda) === String(kernelCash)) {
    return closed('QUANTITY_COLLAPSED');
  }
  return {
    ok: true,
    protocolMinAda: String(protocolMinAda),
    kernelCash: String(KERNEL_CASH),
    unitPayout: String(UNIT_PAYOUT_LOVELACE),
    headLovelace: String(protocolMinAda),
    headLovelaceSource: 'protocol-min-ada-planning',
    protocolMinAdaMeasured: false,
    ledgerExecution: false,
  };
}

function measuredQuery(evidence) {
  const query = evidence?.query;
  if (!query || query.queried !== true || query.unitOnly === true) return false;
  if (typeof query.socket !== 'string' || !existsSync(query.socket)) return false;
  if (typeof query.stdout !== 'string') return false;
  return query.stdout.includes(String(evidence.lucidPosix)) && query.stdout.includes('1000');
}

export function observePosix(evidence) {
  if (evidence == null) return closed('PARITY_ABSENT');
  if (typeof evidence.slotLengthMs === 'string' && evidence.slotLengthMs !== '1000') {
    return closed(evidence.slotLengthMs.startsWith('-') ? 'NEGATIVE_POSIX' : 'SLOT_LENGTH_REFUSED');
  }
  if (evidence.slotLengthMs !== '1000' && evidence.slotLengthMs !== 1000) return closed('SLOT_LENGTH_REFUSED');
  const guarded = calibrateGuards({
    ...evidence,
    slotLengthMs: '1000',
    probeExecuted: evidence.probeExecuted,
    evidenceClass: evidence.evidenceClass,
  });
  if (!guarded.ok && guarded.code !== 'CALIBRATED_SOURCE_NOT_THIS_PHASE') return guarded;
  if (!measuredQuery(evidence)) {
    return closed('PARITY_NOT_MEASURED', {
      siblingEmitted: false,
      guardsStay: [1000, 1100, 1200],
      flagIgnored: evidence.measuredByThisProcess === true,
    });
  }
  return {
    ok: true,
    code: 'PARITY_MATCHED',
    siblingEmitted: false,
    measured: true,
    guardsStay: [1000, 1100, 1200],
    ledgerExecution: false,
    nativePins: false,
  };
}

export function prepareGuardSibling(evidence, deps = {}) {
  const posix = observePosix(evidence);
  if (posix.code !== 'PARITY_MATCHED') return { ...posix, siblingEmitted: false };
  if (typeof deps.sourceText !== 'string' || deps.sourceText.length < 10000 || typeof deps.compile !== 'function') {
    return closed('SIBLING_NOT_COMPILED_IN_THIS_RUN', { siblingEmitted: false, measured: true });
  }
  if (deps.compile.unitOnly === true || deps.unitOnly === true) {
    return closed('SIBLING_COMPILE_NOT_AIKEN', { siblingEmitted: false });
  }
  const compiled = deps.compile({ sourceText: deps.sourceText, previousProgramHash: deps.previousProgramHash || null });
  if (!compiled || compiled.binary !== 'aiken' || compiled.unitOnly === true || !compiled.programHash || compiled.programHash === deps.previousProgramHash) {
    return closed('SIBLING_COMPILE_REJECTED', { siblingEmitted: false });
  }
  return {
    ok: true,
    code: 'SIBLING_COMPILED',
    siblingEmitted: true,
    programHash: compiled.programHash,
    ledgerExecution: false,
    nativePins: false,
  };
}

export function buildBodyPlan(spec) {
  if (!spec?.head || !spec.fee || !spec.collateral) return closed('MALFORMED_DATA');
  const ids = [spec.head.outRef, spec.fee.outRef, spec.collateral.outRef];
  if (new Set(ids).size !== 3) return closed('INPUTS_NOT_DISJOINT');
  if (spec.head.outRef === spec.fee.outRef || spec.head.outRef === spec.collateral.outRef) return closed('INPUTS_NOT_DISJOINT');
  return {
    ok: true,
    ledgerExecution: false,
    submitted: false,
    confirmed: false,
    body: {
      inputs: [spec.head.outRef, spec.fee.outRef, spec.collateral.outRef],
      collateral: spec.collateral.outRef,
      fee: spec.fee.outRef,
      head: spec.head.outRef,
      mint: spec.mint || null,
      validity: spec.validity || null,
      requiredSigners: spec.requiredSigners || [],
      witnesses: { bound: false, vkeyCount: (spec.requiredSigners || []).length },
    },
  };
}

export function buildConcurrentPair(head, left, right) {
  if (!head?.outRef || !left?.fee || !right?.fee || !left?.collateral || !right?.collateral) return closed('MALFORMED_DATA');
  const shared = ['fee', 'collateral'].filter((key) => left[key].outRef === right[key].outRef || left[key].outRef === head.outRef || right[key].outRef === head.outRef);
  if (shared.length) return closed('RACE_INPUTS_NOT_DISJOINT', { shared });
  if (left.fee.outRef === left.collateral.outRef || right.fee.outRef === right.collateral.outRef) return closed('INPUTS_NOT_DISJOINT');
  const a = buildBodyPlan({ head, fee: left.fee, collateral: left.collateral, validity: left.validity, requiredSigners: left.requiredSigners });
  const b = buildBodyPlan({ head, fee: right.fee, collateral: right.collateral, validity: right.validity, requiredSigners: right.requiredSigners });
  if (!a.ok || !b.ok) return a.ok ? b : a;
  return {
    ok: true,
    shared: ['head'],
    submitted: false,
    confirmed: false,
    waitForFirstConfirmation: false,
    members: [a.body, b.body],
    replay: { separate: true, expectedClass: 'BadInputsUTxO', executed: false },
    ledgerExecution: false,
  };
}

export function refuseCashMint() {
  return closed('CASH_POLICY_FAIL_CLOSED', {
    cashCanMint: false,
    submitted: false,
    financialValidatorProof: false,
    amendment: 'a mintable one-shot Cash policy needs two full design votes',
  });
}

function readbackOf(opts) {
  if (opts.readbacks == null) return { present: false, code: 'READBACK_ABSENT', invented: false };
  if (opts.readbacks.invented === true || opts.readbacks.present !== true) {
    return { present: false, code: 'READBACK_ABSENT', invented: false };
  }
  return opts.readbacks;
}

export function offlineWindows(input) {
  const planned = planWindows(input);
  return { ...planned, ledgerExecution: false, sourceEmitted: false, siblingEmitted: false };
}

export async function runNativeCase(request = {}, deps = {}) {
  if (request.mintCash === true) return refuseCashMint();
  const pins = certifyRuntimePins(request.pins);
  const posix = request.posix ? observePosix(request.posix) : closed('PARITY_ABSENT', { siblingEmitted: false });
  const sibling = request.posix ? prepareGuardSibling(request.posix, deps.sibling || {}) : { siblingEmitted: false, code: 'PARITY_ABSENT' };
  const launcher = request.launcher ? await inspectLauncher(request.launcher) : { skipped: true, started: false };
  const budget = request.budget ? compareBudget(request.budget) : {
    skipped: true,
    code: 'PROTOCOL_PARAMS_ABSENT',
    submitBlocked: request.submit === true,
    nativeBudgetMeasured: false,
  };
  const body = request.body ? encodeUnsignedBody(request.body, request.cliPath) : { skipped: true };
  const witnessRequest = request.witness
    ? {
      ...request.witness,
      txBodyCborHex: request.witness.txBodyCborHex || body.cborHex,
      txBodyType: request.witness.txBodyType || body.type,
    }
    : null;
  const witness = witnessRequest ? bindWitness(witnessRequest, request.cliPath) : { skipped: true };
  let submit = { skipped: true, submitted: false, confirmed: false };
  const blocked = pins.nativePins !== true
    || posix.measured !== true
    || request.authorizeLedger !== true
    || budget.ok !== true
    || budget.submitBlocked === true
    || body.ok !== true
    || witness.signed !== true;
  if (request.submit === true) {
    if (budget.code === 'BUDGET_EXCEEDED' || blocked) {
      submit = { ok: false, code: 'SUBMIT_BLOCKED', submitted: false, confirmed: false, submitBlocked: true };
    } else if (typeof deps.submit !== 'function') {
      submit = { ok: false, code: 'NOT_EXECUTED', submitted: false, confirmed: false };
    } else {
      await deps.submit({ unitOnly: deps.unitOnly === true });
      submit = { ok: false, code: 'SUBMIT_UNIT_ONLY', submitted: false, confirmed: false, unitOnly: true, ledgerExecution: false };
    }
  }
  let code = 'NOT_EXECUTED';
  if (request.submit === true && budget.code === 'BUDGET_EXCEEDED') code = 'BUDGET_EXCEEDED';
  else if (!pins.ok) code = pins.code;
  else if (request.submit === true && submit.submitBlocked === true) code = 'SUBMIT_BLOCKED';
  else if (pins.nativePins !== true) code = pins.code || 'NOT_EXECUTED';
  return {
    ok: false,
    code,
    ledgerExecution: false,
    nativePins: false,
    runtimePinsCreated: false,
    confirmed: false,
    submitted: false,
    siblingEmitted: sibling.siblingEmitted === true,
    startedNode: launcher.started === true,
    stages: { pins, posix, sibling, launcher, budget, body, witness, submit },
  };
}

export async function bindReceipt(opts) {
  const loaded = await importReceiptHelper(opts.helperPath);
  if (!loaded.ok) return loaded;
  try {
    const receipt = await loaded.helper.submitWithReceipt(opts);
    const utxoReadbacks = readbackOf(opts);
    const confirmed = receipt.status === 'confirmed' && utxoReadbacks.present === true;
    return {
      ok: receipt.ok === true && confirmed,
      confirmed,
      submittedIsConfirmation: false,
      timeoutIsConfirmation: false,
      receipt,
      utxoReadbacks,
      helperSha256: loaded.digest,
      ledgerExecution: false,
      nativeAcceptance: false,
    };
  } catch (error) {
    const receipt = error.receipt || null;
    return {
      ok: false,
      confirmed: false,
      submittedIsConfirmation: false,
      timeoutIsConfirmation: false,
      failureClass: error.failureClass || receipt?.failureClass || 'receipt-error',
      receipt,
      utxoReadbacks: readbackOf(opts),
      helperSha256: loaded.digest,
      ledgerExecution: false,
      nativeAcceptance: false,
    };
  }
}
