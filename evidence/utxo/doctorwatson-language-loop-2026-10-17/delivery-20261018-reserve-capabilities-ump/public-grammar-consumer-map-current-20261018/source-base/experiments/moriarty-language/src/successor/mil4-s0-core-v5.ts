/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: 1; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;
export interface S0RequestedOutcome {
  phase: 'TerminalSuccess' | 'RequestedFailure';
  retainedEffects: unknown[];
  retainedDuties: unknown[];
}
/** An experiment assumption. Possession of this tuple does not authenticate it. */
export interface S0LocalStipulation {
  intent: S0Intent; state: S0State; round: string; expectedSuccessor: string;
  requestedOutcome: S0RequestedOutcome;
}
const TERMINAL_SUCCESS: S0RequestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
};

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map((key) => [key, stableValue(record[key])]));
  }
  return value;
}
function sameValue(left: unknown, right: unknown): boolean {
  return JSON.stringify(stableValue(left)) === JSON.stringify(stableValue(right));
}
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 * A localStipulation is a finite-comparison assumption supplied by the caller;
 * matching it does not authenticate any external fact or qualify the result.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
  requestedOutcome: S0RequestedOutcome = TERMINAL_SUCCESS,
  localStipulation?: S0LocalStipulation | null,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind !== 'Transfer' && intent?.kind !== 'Repay')
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || !requestedOutcome || typeof requestedOutcome !== 'object'
      || !['TerminalSuccess', 'RequestedFailure'].includes(requestedOutcome.phase)
      || !Array.isArray(requestedOutcome.retainedEffects)
      || !Array.isArray(requestedOutcome.retainedDuties)
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient) && id(intent.feeRecipient)
      && (!state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === intent.recipient)
        || !state.balances.some((v) => v.account === intent.feeRecipient)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.obligations.length !== 0
        || state.allowances.length !== 1)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient)
      && id(intent.feeRecipient)
      && distinct([intent.signer, intent.recipient, intent.feeRecipient])
      && (state.balances.length !== 3 || state.obligations.length !== 0
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== intent.recipient
        || state.balances[2].account !== intent.feeRecipient
        || state.allowances.length !== 1
        || state.allowances[0].owner !== intent.signer)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Repay' && id(intent.signer) && id(intent.obligationId)) {
    const debt = state.obligations.find((v) => v.id === intent.obligationId);
    if (!debt || debt.asset !== state.asset || debt.debtor !== intent.signer
        || debt.creditor === intent.signer
        || debt.status !== 'Outstanding'
        || !state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === debt.creditor)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.balances.length !== 2 || state.obligations.length !== 1
        || state.allowances.length !== 1
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== debt.creditor
        || state.allowances[0].owner !== intent.signer) {
      return reject('stage', 'S0_STAGE_UNSUPPORTED');
    }
  }
  if (localStipulation !== undefined && (localStipulation === null
      || !sameValue(localStipulation.intent, intent)
      || !sameValue(localStipulation.state, state)
      || localStipulation.round !== state.round
      || !opaque(localStipulation.expectedSuccessor)
      || !sameValue(localStipulation.requestedOutcome, requestedOutcome))) {
    return reject('stage', 'S0_STAGE_PREMISE');
  }

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    gross = v + fee;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    if (n > BigInt(obligation.outstanding)) return reject('effect', 'S0_EFFECT_RANGE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: intent.preHead, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (localStipulation !== undefined
      && proposedPostHead !== localStipulation!.expectedSuccessor) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (requestedOutcome.phase !== 'TerminalSuccess'
      || requestedOutcome.retainedEffects.length !== 0
      || requestedOutcome.retainedDuties.length !== 0) {
    return reject('failure', 'S0_FAILURE_UNSUPPORTED');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}
