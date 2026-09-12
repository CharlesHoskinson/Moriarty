/**
 * Bounded funded-repayment transition kernel for SP03.1 and RP01 CM04/CM09.
 *
 * `prepareRepayment` admits a compact JSON string, validates a closed local
 * projection, and runs Transfer/Repay actions into a Prepared candidate.
 * It is not successor Core/K admission, signing, proof, or ledger state.
 */

export const REPAYMENT_VERSION = 'moriarty-funded-repayment/0';

export const REPAYMENT_BOUNDS = Object.freeze({
  sourceUtf8Bytes: 65536,
  collectionCapacity: 128,
  identifierCharacters: 64,
  maxScale: 18,
  uint128Max: '340282366920938463463374607431768211455',
} as const);

export type Identifier = string;
export type UInt128Text = string;
export type AllocationRule = 'AccrualFirst' | 'PrincipalFirst' | 'ProRata';
export type Rounding = 'none' | 'floor' | 'ceil';
export type ObligationStatus = 'Outstanding' | 'Settled';

export interface Conversion {
  mantissa: UInt128Text;
  scale: UInt128Text;
  rounding: Rounding;
}

export interface Balance {
  party: Identifier;
  asset: Identifier;
  amount: UInt128Text;
}

export interface Allowance {
  party: Identifier;
  asset: Identifier;
  remaining: UInt128Text;
  spent: UInt128Text;
}

export interface Work {
  remaining: UInt128Text;
  spent: UInt128Text;
  closureReserve: UInt128Text;
}

export interface Obligation {
  id: Identifier;
  debtor: Identifier;
  creditor: Identifier;
  denomination: Identifier;
  settlementAsset: Identifier;
  principal: UInt128Text;
  accrued: UInt128Text;
  outstanding: UInt128Text;
  allocationRule: AllocationRule;
  conversion: Conversion;
  status: ObligationStatus;
}

export interface TransferAction {
  kind: 'Transfer';
  id: Identifier;
  from: Identifier;
  to: Identifier;
  asset: Identifier;
  amount: UInt128Text;
}

export interface RepayAction {
  kind: 'Repay';
  allocationId: Identifier;
  transferId: Identifier;
  obligationId: Identifier;
  payer: Identifier;
  nominalAmount: UInt128Text;
}

export type Action = TransferAction | RepayAction;

export interface RepaymentState {
  balances: Balance[];
  allowances: Allowance[];
  obligations: Obligation[];
  usedTransferIds: Identifier[];
  usedAllocationIds: Identifier[];
  work: Work;
}

export interface RepaymentInput {
  schemaVersion: string;
  state: RepaymentState;
  actions: Action[];
}

export interface RepaymentEffect {
  kind: 'Repayment';
  allocationId: Identifier;
  transferId: Identifier;
  obligationId: Identifier;
  payer: Identifier;
  creditor: Identifier;
  denomination: Identifier;
  settlementAsset: Identifier;
  nominalAmount: UInt128Text;
  settlementAmount: UInt128Text;
  principalDischarged: UInt128Text;
  accruedDischarged: UInt128Text;
  remainingOutstanding: UInt128Text;
}

export type Effect = TransferAction | RepaymentEffect;

export interface PreparedRepayment {
  status: 'Prepared';
  schemaVersion: string;
  post: RepaymentState;
  effects: Effect[];
}

export interface RejectedRepayment {
  status: 'Rejected';
  code: string;
  actionIndex: number | null;
}

export type RepaymentResult = PreparedRepayment | RejectedRepayment;

const UINT128_MAX = BigInt(REPAYMENT_BOUNDS.uint128Max);
const IDENTIFIER_BODY = /^[A-Za-z][A-Za-z0-9_]{0,63}$/;
const AMOUNT_BODY = /^(0|[1-9][0-9]*)$/;

const INPUT_KEYS = ['schemaVersion', 'state', 'actions'] as const;
const STATE_KEYS = [
  'balances',
  'allowances',
  'obligations',
  'usedTransferIds',
  'usedAllocationIds',
  'work',
] as const;
const BALANCE_KEYS = ['party', 'asset', 'amount'] as const;
const ALLOWANCE_KEYS = ['party', 'asset', 'remaining', 'spent'] as const;
const WORK_KEYS = ['remaining', 'spent', 'closureReserve'] as const;
const OBLIGATION_KEYS = [
  'id',
  'debtor',
  'creditor',
  'denomination',
  'settlementAsset',
  'principal',
  'accrued',
  'outstanding',
  'allocationRule',
  'conversion',
  'status',
] as const;
const CONVERSION_KEYS = ['mantissa', 'scale', 'rounding'] as const;
const TRANSFER_KEYS = ['kind', 'id', 'from', 'to', 'asset', 'amount'] as const;
const REPAY_KEYS = [
  'kind',
  'allocationId',
  'transferId',
  'obligationId',
  'payer',
  'nominalAmount',
] as const;

interface Ok<T> {
  ok: true;
  value: T;
}

interface Fail {
  ok: false;
  code: string;
}

type Res<T> = Ok<T> | Fail;

interface StepTransfer {
  id: string;
  from: string;
  to: string;
  asset: string;
  amount: bigint;
  remaining: bigint;
}

function ok<T>(value: T): Ok<T> {
  return { ok: true, value };
}

function bad(code: string): Fail {
  return { ok: false, code };
}

function rejected(code: string, actionIndex: number | null): RejectedRepayment {
  return { status: 'Rejected', code, actionIndex };
}

function addU128(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum > UINT128_MAX ? null : sum;
}

function subU128(a: bigint, b: bigint): bigint | null {
  return b > a ? null : a - b;
}

function mulU128(a: bigint, b: bigint): bigint | null {
  const product = a * b;
  return product > UINT128_MAX ? null : product;
}

function dec(n: bigint): UInt128Text {
  return n.toString(10);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function closedKeys(
  obj: Record<string, unknown>,
  allowed: readonly string[],
): Fail | null {
  const keys = Object.keys(obj);
  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    if (key === undefined) {
      continue;
    }
    if (!allowed.includes(key)) {
      return bad('UNKNOWN_FIELD');
    }
  }
  for (let i = 0; i < allowed.length; i++) {
    const need = allowed[i];
    if (need === undefined) {
      continue;
    }
    if (!Object.hasOwn(obj, need)) {
      return bad('SCHEMA');
    }
  }
  return null;
}

function isIdentifier(value: unknown): value is string {
  if (typeof value !== 'string') {
    return false;
  }
  if (value.length < 1 || value.length > REPAYMENT_BOUNDS.identifierCharacters) {
    return false;
  }
  const match = IDENTIFIER_BODY.exec(value);
  if (match === null || match.index !== 0 || match[0] !== value) {
    return false;
  }
  return true;
}

function parseUInt128(value: unknown): bigint | null {
  if (typeof value !== 'string') {
    return null;
  }
  const match = AMOUNT_BODY.exec(value);
  if (match === null || match.index !== 0 || match[0] !== value) {
    return null;
  }
  let n: bigint;
  try {
    n = BigInt(value);
  } catch {
    return null;
  }
  if (n < 0n || n > UINT128_MAX) {
    return null;
  }
  if (n.toString(10) !== value) {
    return null;
  }
  return n;
}

function requireIdentifier(value: unknown): Res<string> {
  if (!isIdentifier(value)) {
    return bad('INVALID_IDENTIFIER');
  }
  return ok(value);
}

function requireUInt128Text(value: unknown): Res<string> {
  if (typeof value !== 'string' || parseUInt128(value) === null) {
    return bad('INVALID_AMOUNT');
  }
  return ok(value);
}

function uniqueStrings(items: readonly string[]): boolean {
  return new Set(items).size === items.length;
}

function pairKey(party: string, asset: string): string {
  return party + '\u0000' + asset;
}

function uniquePairs(items: readonly { party: string; asset: string }[]): boolean {
  const seen = new Set<string>();
  for (let i = 0; i < items.length; i++) {
    const item = items[i];
    if (item === undefined) {
      return false;
    }
    const key = pairKey(item.party, item.asset);
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);
  }
  return true;
}

function parseArray<T>(
  value: unknown,
  elem: (item: unknown) => Res<T>,
): Res<T[]> {
  if (!Array.isArray(value)) {
    return bad('SCHEMA');
  }
  if (value.length > REPAYMENT_BOUNDS.collectionCapacity) {
    return bad('CAPACITY');
  }
  const out: T[] = [];
  for (let i = 0; i < value.length; i++) {
    const parsed = elem(value[i]);
    if (!parsed.ok) {
      return parsed;
    }
    out.push(parsed.value);
  }
  return ok(out);
}

function hasLoneSurrogate(source: string): boolean {
  const n = source.length;
  for (let i = 0; i < n; i++) {
    const c = source.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff) {
      if (i + 1 >= n) {
        return true;
      }
      const d = source.charCodeAt(i + 1);
      if (d < 0xdc00 || d > 0xdfff) {
        return true;
      }
      i += 1;
    } else if (c >= 0xdc00 && c <= 0xdfff) {
      return true;
    }
  }
  return false;
}

function admitSource(source: string): Res<unknown> {
  if (source.length > REPAYMENT_BOUNDS.sourceUtf8Bytes) {
    return bad('INPUT_UTF16_LENGTH');
  }
  if (hasLoneSurrogate(source)) {
    return bad('INPUT_LONE_SURROGATE');
  }
  const bytes = new TextEncoder().encode(source);
  if (bytes.length > REPAYMENT_BOUNDS.sourceUtf8Bytes) {
    return bad('INPUT_UTF8_LENGTH');
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(source);
  } catch {
    return bad('INPUT_JSON');
  }
  let compact: string;
  try {
    compact = JSON.stringify(parsed);
  } catch {
    return bad('INPUT_ENCODING');
  }
  if (typeof compact !== 'string') {
    return bad('INPUT_ENCODING');
  }
  if (compact !== source) {
    return bad('INPUT_COMPACT');
  }
  return ok(parsed);
}

function parseConversion(value: unknown): Res<Conversion> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, CONVERSION_KEYS);
  if (keys !== null) {
    return keys;
  }
  const mantissa = requireUInt128Text(value.mantissa);
  if (!mantissa.ok) {
    return mantissa;
  }
  const scale = requireUInt128Text(value.scale);
  if (!scale.ok) {
    return scale;
  }
  const roundingValue = value.rounding;
  if (
    roundingValue !== 'none' &&
    roundingValue !== 'floor' &&
    roundingValue !== 'ceil'
  ) {
    return bad('SCHEMA');
  }
  const mantissaN = BigInt(mantissa.value);
  if (mantissaN === 0n) {
    return bad('INVARIANT');
  }
  const scaleN = BigInt(scale.value);
  if (scaleN > BigInt(REPAYMENT_BOUNDS.maxScale)) {
    return bad('INVARIANT');
  }
  return ok({
    mantissa: mantissa.value,
    scale: scale.value,
    rounding: roundingValue,
  });
}

function parseBalance(value: unknown): Res<Balance> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, BALANCE_KEYS);
  if (keys !== null) {
    return keys;
  }
  const party = requireIdentifier(value.party);
  if (!party.ok) {
    return party;
  }
  const asset = requireIdentifier(value.asset);
  if (!asset.ok) {
    return asset;
  }
  const amount = requireUInt128Text(value.amount);
  if (!amount.ok) {
    return amount;
  }
  return ok({
    party: party.value,
    asset: asset.value,
    amount: amount.value,
  });
}

function parseAllowance(value: unknown): Res<Allowance> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, ALLOWANCE_KEYS);
  if (keys !== null) {
    return keys;
  }
  const party = requireIdentifier(value.party);
  if (!party.ok) {
    return party;
  }
  const asset = requireIdentifier(value.asset);
  if (!asset.ok) {
    return asset;
  }
  const remaining = requireUInt128Text(value.remaining);
  if (!remaining.ok) {
    return remaining;
  }
  const spent = requireUInt128Text(value.spent);
  if (!spent.ok) {
    return spent;
  }
  if (addU128(BigInt(remaining.value), BigInt(spent.value)) === null) {
    return bad('INVARIANT');
  }
  return ok({
    party: party.value,
    asset: asset.value,
    remaining: remaining.value,
    spent: spent.value,
  });
}

function parseWork(value: unknown): Res<Work> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, WORK_KEYS);
  if (keys !== null) {
    return keys;
  }
  const remaining = requireUInt128Text(value.remaining);
  if (!remaining.ok) {
    return remaining;
  }
  const spent = requireUInt128Text(value.spent);
  if (!spent.ok) {
    return spent;
  }
  const closureReserve = requireUInt128Text(value.closureReserve);
  if (!closureReserve.ok) {
    return closureReserve;
  }
  const rem = BigInt(remaining.value);
  const sp = BigInt(spent.value);
  const clo = BigInt(closureReserve.value);
  const remSpent = addU128(rem, sp);
  if (remSpent === null) {
    return bad('INVARIANT');
  }
  if (addU128(remSpent, clo) === null) {
    return bad('INVARIANT');
  }
  return ok({
    remaining: remaining.value,
    spent: spent.value,
    closureReserve: closureReserve.value,
  });
}

function parseObligation(value: unknown): Res<Obligation> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, OBLIGATION_KEYS);
  if (keys !== null) {
    return keys;
  }
  const id = requireIdentifier(value.id);
  if (!id.ok) {
    return id;
  }
  const debtor = requireIdentifier(value.debtor);
  if (!debtor.ok) {
    return debtor;
  }
  const creditor = requireIdentifier(value.creditor);
  if (!creditor.ok) {
    return creditor;
  }
  const denomination = requireIdentifier(value.denomination);
  if (!denomination.ok) {
    return denomination;
  }
  const settlementAsset = requireIdentifier(value.settlementAsset);
  if (!settlementAsset.ok) {
    return settlementAsset;
  }
  const principal = requireUInt128Text(value.principal);
  if (!principal.ok) {
    return principal;
  }
  const accrued = requireUInt128Text(value.accrued);
  if (!accrued.ok) {
    return accrued;
  }
  const outstanding = requireUInt128Text(value.outstanding);
  if (!outstanding.ok) {
    return outstanding;
  }
  const rule = value.allocationRule;
  if (rule !== 'AccrualFirst' && rule !== 'PrincipalFirst' && rule !== 'ProRata') {
    return bad('SCHEMA');
  }
  const conversion = parseConversion(value.conversion);
  if (!conversion.ok) {
    return conversion;
  }
  const status = value.status;
  if (status !== 'Outstanding' && status !== 'Settled') {
    return bad('SCHEMA');
  }
  const p = BigInt(principal.value);
  const a = BigInt(accrued.value);
  const o = BigInt(outstanding.value);
  const sum = addU128(p, a);
  if (sum === null || sum !== o) {
    return bad('INVARIANT');
  }
  if (status === 'Outstanding') {
    if (o === 0n) {
      return bad('INVARIANT');
    }
  } else if (o !== 0n) {
    return bad('INVARIANT');
  }
  return ok({
    id: id.value,
    debtor: debtor.value,
    creditor: creditor.value,
    denomination: denomination.value,
    settlementAsset: settlementAsset.value,
    principal: principal.value,
    accrued: accrued.value,
    outstanding: outstanding.value,
    allocationRule: rule,
    conversion: conversion.value,
    status,
  });
}

function parseAction(value: unknown): Res<Action> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  if (!Object.hasOwn(value, 'kind')) {
    return bad('SCHEMA');
  }
  const kind = value.kind;
  if (kind === 'Transfer') {
    const keys = closedKeys(value, TRANSFER_KEYS);
    if (keys !== null) {
      return keys;
    }
    const id = requireIdentifier(value.id);
    if (!id.ok) {
      return id;
    }
    const from = requireIdentifier(value.from);
    if (!from.ok) {
      return from;
    }
    const to = requireIdentifier(value.to);
    if (!to.ok) {
      return to;
    }
    const asset = requireIdentifier(value.asset);
    if (!asset.ok) {
      return asset;
    }
    const amount = requireUInt128Text(value.amount);
    if (!amount.ok) {
      return amount;
    }
    return ok({
      kind: 'Transfer',
      id: id.value,
      from: from.value,
      to: to.value,
      asset: asset.value,
      amount: amount.value,
    });
  }
  if (kind === 'Repay') {
    const keys = closedKeys(value, REPAY_KEYS);
    if (keys !== null) {
      return keys;
    }
    const allocationId = requireIdentifier(value.allocationId);
    if (!allocationId.ok) {
      return allocationId;
    }
    const transferId = requireIdentifier(value.transferId);
    if (!transferId.ok) {
      return transferId;
    }
    const obligationId = requireIdentifier(value.obligationId);
    if (!obligationId.ok) {
      return obligationId;
    }
    const payer = requireIdentifier(value.payer);
    if (!payer.ok) {
      return payer;
    }
    const nominalAmount = requireUInt128Text(value.nominalAmount);
    if (!nominalAmount.ok) {
      return nominalAmount;
    }
    return ok({
      kind: 'Repay',
      allocationId: allocationId.value,
      transferId: transferId.value,
      obligationId: obligationId.value,
      payer: payer.value,
      nominalAmount: nominalAmount.value,
    });
  }
  if (typeof kind !== 'string') {
    return bad('SCHEMA');
  }
  return bad('UNKNOWN_ACTION');
}

function parseIdentifierList(value: unknown): Res<Identifier[]> {
  const arr = parseArray(value, requireIdentifier);
  if (!arr.ok) {
    return arr;
  }
  if (!uniqueStrings(arr.value)) {
    return bad('DUPLICATE');
  }
  return arr;
}

function parseState(value: unknown): Res<RepaymentState> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, STATE_KEYS);
  if (keys !== null) {
    return keys;
  }
  const balances = parseArray(value.balances, parseBalance);
  if (!balances.ok) {
    return balances;
  }
  if (!uniquePairs(balances.value)) {
    return bad('DUPLICATE');
  }
  const allowances = parseArray(value.allowances, parseAllowance);
  if (!allowances.ok) {
    return allowances;
  }
  if (!uniquePairs(allowances.value)) {
    return bad('DUPLICATE');
  }
  const obligations = parseArray(value.obligations, parseObligation);
  if (!obligations.ok) {
    return obligations;
  }
  const obligationIds: string[] = [];
  for (let i = 0; i < obligations.value.length; i++) {
    const obligation = obligations.value[i];
    if (obligation === undefined) {
      return bad('SCHEMA');
    }
    obligationIds.push(obligation.id);
  }
  if (!uniqueStrings(obligationIds)) {
    return bad('DUPLICATE');
  }
  const usedTransferIds = parseIdentifierList(value.usedTransferIds);
  if (!usedTransferIds.ok) {
    return usedTransferIds;
  }
  const usedAllocationIds = parseIdentifierList(value.usedAllocationIds);
  if (!usedAllocationIds.ok) {
    return usedAllocationIds;
  }
  const work = parseWork(value.work);
  if (!work.ok) {
    return work;
  }
  return ok({
    balances: balances.value,
    allowances: allowances.value,
    obligations: obligations.value,
    usedTransferIds: usedTransferIds.value,
    usedAllocationIds: usedAllocationIds.value,
    work: work.value,
  });
}

function parseInput(value: unknown): Res<RepaymentInput> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, INPUT_KEYS);
  if (keys !== null) {
    return keys;
  }
  if (value.schemaVersion !== REPAYMENT_VERSION) {
    return bad('SCHEMA');
  }
  const state = parseState(value.state);
  if (!state.ok) {
    return state;
  }
  const actions = parseArray(value.actions, parseAction);
  if (!actions.ok) {
    return actions;
  }
  if (actions.value.length === 0) {
    return bad('SCHEMA');
  }
  return ok({
    schemaVersion: REPAYMENT_VERSION,
    state: state.value,
    actions: actions.value,
  });
}

function copyConversion(conversion: Conversion): Conversion {
  return {
    mantissa: conversion.mantissa,
    scale: conversion.scale,
    rounding: conversion.rounding,
  };
}

function copyState(state: RepaymentState): RepaymentState {
  const balances: Balance[] = [];
  for (let i = 0; i < state.balances.length; i++) {
    const item = state.balances[i];
    if (item === undefined) {
      continue;
    }
    balances.push({
      party: item.party,
      asset: item.asset,
      amount: item.amount,
    });
  }
  const allowances: Allowance[] = [];
  for (let i = 0; i < state.allowances.length; i++) {
    const item = state.allowances[i];
    if (item === undefined) {
      continue;
    }
    allowances.push({
      party: item.party,
      asset: item.asset,
      remaining: item.remaining,
      spent: item.spent,
    });
  }
  const obligations: Obligation[] = [];
  for (let i = 0; i < state.obligations.length; i++) {
    const item = state.obligations[i];
    if (item === undefined) {
      continue;
    }
    obligations.push({
      id: item.id,
      debtor: item.debtor,
      creditor: item.creditor,
      denomination: item.denomination,
      settlementAsset: item.settlementAsset,
      principal: item.principal,
      accrued: item.accrued,
      outstanding: item.outstanding,
      allocationRule: item.allocationRule,
      conversion: copyConversion(item.conversion),
      status: item.status,
    });
  }
  const usedTransferIds: Identifier[] = [];
  for (let i = 0; i < state.usedTransferIds.length; i++) {
    const id = state.usedTransferIds[i];
    if (id !== undefined) {
      usedTransferIds.push(id);
    }
  }
  const usedAllocationIds: Identifier[] = [];
  for (let i = 0; i < state.usedAllocationIds.length; i++) {
    const id = state.usedAllocationIds[i];
    if (id !== undefined) {
      usedAllocationIds.push(id);
    }
  }
  return {
    balances,
    allowances,
    obligations,
    usedTransferIds,
    usedAllocationIds,
    work: {
      remaining: state.work.remaining,
      spent: state.work.spent,
      closureReserve: state.work.closureReserve,
    },
  };
}

function findPairIndex(
  items: readonly { party: string; asset: string }[],
  party: string,
  asset: string,
): number {
  for (let i = 0; i < items.length; i++) {
    const item = items[i];
    if (item !== undefined && item.party === party && item.asset === asset) {
      return i;
    }
  }
  return -1;
}

function findObligationIndex(items: readonly Obligation[], id: string): number {
  for (let i = 0; i < items.length; i++) {
    const item = items[i];
    if (item !== undefined && item.id === id) {
      return i;
    }
  }
  return -1;
}

function convertNominal(nominal: bigint, conversion: Conversion): Res<bigint> {
  const mantissa = BigInt(conversion.mantissa);
  const scale = BigInt(conversion.scale);
  const product = mulU128(nominal, mantissa);
  if (product === null) {
    return bad('OVERFLOW');
  }
  const divisor = 10n ** scale;
  if (divisor > UINT128_MAX) {
    return bad('OVERFLOW');
  }
  const quotient = product / divisor;
  const remainder = product % divisor;
  if (conversion.rounding === 'none') {
    if (remainder !== 0n) {
      return bad('INEXACT_CONVERSION');
    }
    return ok(quotient);
  }
  if (conversion.rounding === 'floor') {
    return ok(quotient);
  }
  if (remainder === 0n) {
    return ok(quotient);
  }
  const ceiled = addU128(quotient, 1n);
  if (ceiled === null) {
    return bad('OVERFLOW');
  }
  return ok(ceiled);
}

function allocateNominal(
  rule: AllocationRule,
  n: bigint,
  principal: bigint,
  accrued: bigint,
): Res<{ dP: bigint; dA: bigint }> {
  let dP: bigint;
  let dA: bigint;
  if (rule === 'AccrualFirst') {
    dA = n < accrued ? n : accrued;
    dP = n - dA;
  } else if (rule === 'PrincipalFirst') {
    dP = n < principal ? n : principal;
    dA = n - dP;
  } else {
    const total = addU128(principal, accrued);
    if (total === null) {
      return bad('OVERFLOW');
    }
    if (total === 0n) {
      return bad('INVARIANT');
    }
    const product = mulU128(n, principal);
    if (product === null) {
      return bad('OVERFLOW');
    }
    dP = product / total;
    dA = n - dP;
  }
  if (dP > principal || dA > accrued) {
    return bad('ALLOCATION_COMPONENT');
  }
  return ok({ dP, dA });
}

type ApplyResult = { ok: true; value: Effect } | Fail;

function applyTransfer(
  post: RepaymentState,
  action: TransferAction,
  usedTransferIds: Set<string>,
  step: Map<string, StepTransfer>,
): ApplyResult {
  const amount = BigInt(action.amount);
  if (amount === 0n) {
    return bad('ZERO_AMOUNT');
  }
  if (action.from === action.to) {
    return bad('SELF_TRANSFER');
  }
  if (usedTransferIds.has(action.id) || step.has(action.id)) {
    return bad('DUPLICATE');
  }
  const senderIndex = findPairIndex(post.balances, action.from, action.asset);
  if (senderIndex < 0) {
    return bad('MISSING_BALANCE');
  }
  const sender = post.balances[senderIndex];
  if (sender === undefined) {
    return bad('MISSING_BALANCE');
  }
  const senderAmt = BigInt(sender.amount);
  const nextSender = subU128(senderAmt, amount);
  if (nextSender === null) {
    return bad('INSUFFICIENT_BALANCE');
  }
  const allowanceIndex = findPairIndex(post.allowances, action.from, action.asset);
  if (allowanceIndex < 0) {
    return bad('MISSING_ALLOWANCE');
  }
  const allowance = post.allowances[allowanceIndex];
  if (allowance === undefined) {
    return bad('MISSING_ALLOWANCE');
  }
  const allowRemaining = BigInt(allowance.remaining);
  const allowSpent = BigInt(allowance.spent);
  const nextRemaining = subU128(allowRemaining, amount);
  if (nextRemaining === null) {
    return bad('INSUFFICIENT_ALLOWANCE');
  }
  const nextSpent = addU128(allowSpent, amount);
  if (nextSpent === null) {
    return bad('OVERFLOW');
  }
  const receiverIndex = findPairIndex(post.balances, action.to, action.asset);
  let nextReceiver: bigint;
  if (receiverIndex < 0) {
    if (post.balances.length >= REPAYMENT_BOUNDS.collectionCapacity) {
      return bad('CAPACITY');
    }
    nextReceiver = amount;
  } else {
    const receiver = post.balances[receiverIndex];
    if (receiver === undefined) {
      return bad('MISSING_BALANCE');
    }
    const credited = addU128(BigInt(receiver.amount), amount);
    if (credited === null) {
      return bad('OVERFLOW');
    }
    nextReceiver = credited;
  }
  if (post.usedTransferIds.length >= REPAYMENT_BOUNDS.collectionCapacity) {
    return bad('CAPACITY');
  }
  sender.amount = dec(nextSender);
  if (receiverIndex < 0) {
    post.balances.push({
      party: action.to,
      asset: action.asset,
      amount: '0',
    });
    const created = post.balances[post.balances.length - 1];
    if (created === undefined) {
      return bad('CAPACITY');
    }
    created.amount = dec(nextReceiver);
  } else {
    const receiver = post.balances[receiverIndex];
    if (receiver === undefined) {
      return bad('MISSING_BALANCE');
    }
    receiver.amount = dec(nextReceiver);
  }
  allowance.remaining = dec(nextRemaining);
  allowance.spent = dec(nextSpent);
  post.usedTransferIds.push(action.id);
  usedTransferIds.add(action.id);
  step.set(action.id, {
    id: action.id,
    from: action.from,
    to: action.to,
    asset: action.asset,
    amount,
    remaining: amount,
  });
  return ok({
    kind: 'Transfer',
    id: action.id,
    from: action.from,
    to: action.to,
    asset: action.asset,
    amount: action.amount,
  });
}

function applyRepay(
  post: RepaymentState,
  action: RepayAction,
  usedAllocationIds: Set<string>,
  step: Map<string, StepTransfer>,
): ApplyResult {
  const nominal = BigInt(action.nominalAmount);
  if (nominal === 0n) {
    return bad('ZERO_AMOUNT');
  }
  const obligationIndex = findObligationIndex(
    post.obligations,
    action.obligationId,
  );
  if (obligationIndex < 0) {
    return bad('MISSING_OBLIGATION');
  }
  const obligation = post.obligations[obligationIndex];
  if (obligation === undefined) {
    return bad('MISSING_OBLIGATION');
  }
  const outstanding = BigInt(obligation.outstanding);
  if (obligation.status !== 'Outstanding' || outstanding === 0n) {
    return bad('NOT_OUTSTANDING');
  }
  if (nominal > outstanding) {
    return bad('EXCEEDS_OUTSTANDING');
  }
  if (usedAllocationIds.has(action.allocationId)) {
    return bad('DUPLICATE');
  }
  const funded = step.get(action.transferId);
  if (funded === undefined) {
    return bad('TRANSFER_NOT_IN_STEP');
  }
  if (
    funded.from !== action.payer ||
    funded.to !== obligation.creditor ||
    funded.asset !== obligation.settlementAsset
  ) {
    return bad('TRANSFER_MISMATCH');
  }
  const settlement = convertNominal(nominal, obligation.conversion);
  if (!settlement.ok) {
    return settlement;
  }
  if (settlement.value === 0n) {
    return bad('DUST');
  }
  if (settlement.value > funded.remaining) {
    return bad('INSUFFICIENT_UNALLOCATED');
  }
  const principal = BigInt(obligation.principal);
  const accrued = BigInt(obligation.accrued);
  const parts = allocateNominal(
    obligation.allocationRule,
    nominal,
    principal,
    accrued,
  );
  if (!parts.ok) {
    return parts;
  }
  if (post.usedAllocationIds.length >= REPAYMENT_BOUNDS.collectionCapacity) {
    return bad('CAPACITY');
  }
  const nextRemaining = subU128(funded.remaining, settlement.value);
  if (nextRemaining === null) {
    return bad('INSUFFICIENT_UNALLOCATED');
  }
  funded.remaining = nextRemaining;
  const nextPrincipal = principal - parts.value.dP;
  const nextAccrued = accrued - parts.value.dA;
  const nextOutstanding = nextPrincipal + nextAccrued;
  obligation.principal = dec(nextPrincipal);
  obligation.accrued = dec(nextAccrued);
  obligation.outstanding = dec(nextOutstanding);
  obligation.status = nextOutstanding === 0n ? 'Settled' : 'Outstanding';
  post.usedAllocationIds.push(action.allocationId);
  usedAllocationIds.add(action.allocationId);
  return ok({
    kind: 'Repayment',
    allocationId: action.allocationId,
    transferId: action.transferId,
    obligationId: action.obligationId,
    payer: action.payer,
    creditor: obligation.creditor,
    denomination: obligation.denomination,
    settlementAsset: obligation.settlementAsset,
    nominalAmount: action.nominalAmount,
    settlementAmount: dec(settlement.value),
    principalDischarged: dec(parts.value.dP),
    accruedDischarged: dec(parts.value.dA),
    remainingOutstanding: obligation.outstanding,
  });
}

function runActions(input: RepaymentInput): RepaymentResult {
  const cost = BigInt(input.actions.length);
  if (BigInt(input.state.work.remaining) < cost) {
    return rejected('INSUFFICIENT_WORK', null);
  }
  if (addU128(BigInt(input.state.work.spent), cost) === null) {
    return rejected('OVERFLOW', null);
  }
  const post = copyState(input.state);
  const effects: Effect[] = [];
  const usedTransferIds = new Set<string>(post.usedTransferIds);
  const usedAllocationIds = new Set<string>(post.usedAllocationIds);
  const step = new Map<string, StepTransfer>();
  for (let actionIndex = 0; actionIndex < input.actions.length; actionIndex++) {
    const action = input.actions[actionIndex];
    if (action === undefined) {
      return rejected('SCHEMA', actionIndex);
    }
    let applied: ApplyResult;
    if (action.kind === 'Transfer') {
      applied = applyTransfer(post, action, usedTransferIds, step);
    } else if (action.kind === 'Repay') {
      applied = applyRepay(post, action, usedAllocationIds, step);
    } else {
      return rejected('UNKNOWN_ACTION', actionIndex);
    }
    if (!applied.ok) {
      return rejected(applied.code, actionIndex);
    }
    effects.push(applied.value);
  }
  const remaining = subU128(BigInt(post.work.remaining), cost);
  const spent = addU128(BigInt(post.work.spent), cost);
  if (remaining === null || spent === null) {
    return rejected('OVERFLOW', null);
  }
  post.work.remaining = dec(remaining);
  post.work.spent = dec(spent);
  return {
    status: 'Prepared',
    schemaVersion: REPAYMENT_VERSION,
    post,
    effects,
  };
}

export function prepareRepayment(source: string): RepaymentResult {
  if (typeof source !== 'string') {
    return rejected('INPUT_NOT_STRING', null);
  }
  const admitted = admitSource(source);
  if (!admitted.ok) {
    return rejected(admitted.code, null);
  }
  const parsed = parseInput(admitted.value);
  if (!parsed.ok) {
    return rejected(parsed.code, null);
  }
  return runActions(parsed.value);
}