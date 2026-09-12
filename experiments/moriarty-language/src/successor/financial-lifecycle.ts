/**
 * Bounded loan origination and interest-accrual lifecycle kernel.
 *
 * `prepareFinancialLifecycle` admits compact JSON and runs Transfer, Repay,
 * Originate and Accrue into a Prepared candidate. It is not signing, proof,
 * ledger state or authenticated time.
 */

export const LIFECYCLE_VERSION = 'moriarty-financial-lifecycle/1';
export const LIFECYCLE_STATE_VERSION = 'moriarty-financial-lifecycle-state/1';

export const LIFECYCLE_BOUNDS = Object.freeze({
  sourceUtf8Bytes: 65536,
  collectionCapacity: 128,
  identifierCharacters: 64,
  maxScale: 18,
  uint128Max: '340282366920938463463374607431768211455',
  uint64Max: '18446744073709551615',
  signed128Max: '170141183460469231731687303715884105727',
} as const);

export type Identifier = string;
export type UInt128Text = string;
export type UInt64Text = string;
export type AllocationRule = 'AccrualFirst' | 'PrincipalFirst' | 'ProRata';
export type Rounding = 'none' | 'floor' | 'ceil';
export type AccrualRounding = 'floor' | 'ceil';
export type ObligationStatus = 'Outstanding' | 'Settled';

export interface Conversion {
  mantissa: UInt128Text;
  scale: UInt128Text;
  rounding: Rounding;
}

export interface AccrualTerms {
  numerator: UInt128Text;
  denominator: UInt128Text;
  rounding: AccrualRounding;
  periodSeconds: UInt64Text;
  firstPeriodStart: UInt64Text;
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

export interface LifecycleObligation {
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
  originationId: Identifier;
  originationTransferId: Identifier;
  initialPrincipal: UInt128Text;
  nominalLiabilityCap: UInt128Text;
  liabilityIncurred: UInt128Text;
  accrualTerms: AccrualTerms;
  lastAccruedPeriod: UInt64Text;
  nextAccrualAt: UInt64Text;
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

export interface OriginateAction {
  kind: 'Originate';
  obligationId: Identifier;
  transferId: Identifier;
  originationId: Identifier;
  debtor: Identifier;
  creditor: Identifier;
  nominalAmount: UInt128Text;
  denomination: Identifier;
  settlementAsset: Identifier;
  conversion: Conversion;
  allocationRule: AllocationRule;
  accrualTerms: AccrualTerms;
  nominalLiabilityCap: UInt128Text;
}

export interface AccrueAction {
  kind: 'Accrue';
  accrualId: Identifier;
  obligationId: Identifier;
  periodIndex: UInt64Text;
  observedTime: UInt64Text;
}

export type Action = TransferAction | RepayAction | OriginateAction | AccrueAction;

export interface LifecycleState {
  schemaVersion: typeof LIFECYCLE_STATE_VERSION;
  balances: Balance[];
  allowances: Allowance[];
  obligations: LifecycleObligation[];
  usedTransferIds: Identifier[];
  usedAllocationIds: Identifier[];
  usedOriginationIds: Identifier[];
  usedAccrualIds: Identifier[];
  work: Work;
}

export interface LifecycleInput {
  schemaVersion: string;
  state: LifecycleState;
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

export interface OriginationEffect {
  obligationId: Identifier;
  transferId: Identifier;
  originationId: Identifier;
  debtor: Identifier;
  creditor: Identifier;
  nominalAmount: UInt128Text;
  denomination: Identifier;
  settlementAsset: Identifier;
  conversion: Conversion;
  allocationRule: AllocationRule;
  accrualTerms: AccrualTerms;
  nominalLiabilityCap: UInt128Text;
  kind: 'Origination';
  settlementAmount: UInt128Text;
  liabilityIncurred: UInt128Text;
  lastAccruedPeriod: UInt64Text;
  nextAccrualAt: UInt64Text;
}

export interface AccrualEffect {
  kind: 'Accrual';
  accrualId: Identifier;
  obligationId: Identifier;
  debtor: Identifier;
  creditor: Identifier;
  denomination: Identifier;
  periodIndex: UInt64Text;
  observedTime: UInt64Text;
  eligibleAt: UInt64Text;
  nextAccrualAt: UInt64Text;
  principalBasis: UInt128Text;
  numerator: UInt128Text;
  denominator: UInt128Text;
  rounding: AccrualRounding;
  interestAmount: UInt128Text;
  previousAccrued: UInt128Text;
  accrued: UInt128Text;
  previousOutstanding: UInt128Text;
  outstanding: UInt128Text;
  previousLiabilityIncurred: UInt128Text;
  liabilityIncurred: UInt128Text;
  nominalLiabilityCap: UInt128Text;
}

export type Effect = TransferAction | RepaymentEffect | OriginationEffect | AccrualEffect;

export interface PreparedLifecycle {
  status: 'Prepared';
  schemaVersion: string;
  post: LifecycleState;
  effects: Effect[];
}

export interface RejectedLifecycle {
  status: 'Rejected';
  code: string;
  actionIndex: number | null;
}

export type LifecycleResult = PreparedLifecycle | RejectedLifecycle;

const UINT128_MAX = BigInt(LIFECYCLE_BOUNDS.uint128Max);
const UINT64_MAX = BigInt(LIFECYCLE_BOUNDS.uint64Max);
const SIGNED128_MAX = BigInt(LIFECYCLE_BOUNDS.signed128Max);
const IDENTIFIER_BODY = /^[A-Za-z][A-Za-z0-9_]{0,63}$/;
const AMOUNT_BODY = /^(0|[1-9][0-9]*)$/;

const INPUT_KEYS = ['schemaVersion', 'state', 'actions'] as const;
const STATE_KEYS = [
  'schemaVersion',
  'balances',
  'allowances',
  'obligations',
  'usedTransferIds',
  'usedAllocationIds',
  'usedOriginationIds',
  'usedAccrualIds',
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
  'originationId',
  'originationTransferId',
  'initialPrincipal',
  'nominalLiabilityCap',
  'liabilityIncurred',
  'accrualTerms',
  'lastAccruedPeriod',
  'nextAccrualAt',
] as const;
const CONVERSION_KEYS = ['mantissa', 'scale', 'rounding'] as const;
const ACCRUAL_TERMS_KEYS = [
  'numerator',
  'denominator',
  'rounding',
  'periodSeconds',
  'firstPeriodStart',
] as const;
const TRANSFER_KEYS = ['kind', 'id', 'from', 'to', 'asset', 'amount'] as const;
const REPAY_KEYS = [
  'kind',
  'allocationId',
  'transferId',
  'obligationId',
  'payer',
  'nominalAmount',
] as const;
const ORIGINATE_KEYS = [
  'kind',
  'obligationId',
  'transferId',
  'originationId',
  'debtor',
  'creditor',
  'nominalAmount',
  'denomination',
  'settlementAsset',
  'conversion',
  'allocationRule',
  'accrualTerms',
  'nominalLiabilityCap',
] as const;
const ACCRUE_KEYS = [
  'kind',
  'accrualId',
  'obligationId',
  'periodIndex',
  'observedTime',
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

function rejected(code: string, actionIndex: number | null): RejectedLifecycle {
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

function addU64(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum > UINT64_MAX ? null : sum;
}

function mulU64(a: bigint, b: bigint): bigint | null {
  const product = a * b;
  return product > UINT64_MAX ? null : product;
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
  if (value.length < 1 || value.length > LIFECYCLE_BOUNDS.identifierCharacters) {
    return false;
  }
  const match = IDENTIFIER_BODY.exec(value);
  if (match === null || match.index !== 0 || match[0] !== value) {
    return false;
  }
  return true;
}

function parseUnsigned(value: unknown, max: bigint): bigint | null {
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
  if (n < 0n || n > max) {
    return null;
  }
  if (n.toString(10) !== value) {
    return null;
  }
  return n;
}

function parseUInt128(value: unknown): bigint | null {
  return parseUnsigned(value, UINT128_MAX);
}

function parseUInt64(value: unknown): bigint | null {
  return parseUnsigned(value, UINT64_MAX);
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

function requireUInt64Text(value: unknown): Res<string> {
  if (typeof value !== 'string' || parseUInt64(value) === null) {
    return bad('INVALID_AMOUNT');
  }
  return ok(value);
}

function requireSignedBound(value: unknown, positive: boolean): Res<string> {
  const parsed = requireUInt128Text(value);
  if (!parsed.ok) {
    return parsed;
  }
  const n = BigInt(parsed.value);
  if (n > SIGNED128_MAX) {
    return bad('INVALID_AMOUNT');
  }
  if (positive && n === 0n) {
    return bad('INVARIANT');
  }
  return parsed;
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
  if (value.length > LIFECYCLE_BOUNDS.collectionCapacity) {
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
  if (source.length > LIFECYCLE_BOUNDS.sourceUtf8Bytes) {
    return bad('INPUT_UTF16_LENGTH');
  }
  if (hasLoneSurrogate(source)) {
    return bad('INPUT_LONE_SURROGATE');
  }
  const bytes = new TextEncoder().encode(source);
  if (bytes.length > LIFECYCLE_BOUNDS.sourceUtf8Bytes) {
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
  if (scaleN > BigInt(LIFECYCLE_BOUNDS.maxScale)) {
    return bad('INVARIANT');
  }
  return ok({
    mantissa: mantissa.value,
    scale: scale.value,
    rounding: roundingValue,
  });
}

function parseAccrualTerms(value: unknown): Res<AccrualTerms> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, ACCRUAL_TERMS_KEYS);
  if (keys !== null) {
    return keys;
  }
  const numerator = requireUInt128Text(value.numerator);
  if (!numerator.ok) {
    return numerator;
  }
  const denominator = requireUInt128Text(value.denominator);
  if (!denominator.ok) {
    return denominator;
  }
  const roundingValue = value.rounding;
  if (roundingValue !== 'floor' && roundingValue !== 'ceil') {
    return bad('SCHEMA');
  }
  const periodSeconds = requireUInt64Text(value.periodSeconds);
  if (!periodSeconds.ok) {
    return periodSeconds;
  }
  const firstPeriodStart = requireUInt64Text(value.firstPeriodStart);
  if (!firstPeriodStart.ok) {
    return firstPeriodStart;
  }
  if (BigInt(denominator.value) === 0n) {
    return bad('INVARIANT');
  }
  if (BigInt(periodSeconds.value) === 0n) {
    return bad('INVARIANT');
  }
  return ok({
    numerator: numerator.value,
    denominator: denominator.value,
    rounding: roundingValue,
    periodSeconds: periodSeconds.value,
    firstPeriodStart: firstPeriodStart.value,
  });
}

function expectedNextAccrualAt(terms: AccrualTerms, lastAccruedPeriod: string): bigint | null {
  const last = BigInt(lastAccruedPeriod);
  const plus = addU64(last, 1n);
  if (plus === null) {
    return null;
  }
  const scaled = mulU64(plus, BigInt(terms.periodSeconds));
  if (scaled === null) {
    return null;
  }
  return addU64(BigInt(terms.firstPeriodStart), scaled);
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

function parseObligation(value: unknown): Res<LifecycleObligation> {
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
  const principal = requireSignedBound(value.principal, false);
  if (!principal.ok) {
    return principal;
  }
  const accrued = requireSignedBound(value.accrued, false);
  if (!accrued.ok) {
    return accrued;
  }
  const outstanding = requireSignedBound(value.outstanding, false);
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
  const originationId = requireIdentifier(value.originationId);
  if (!originationId.ok) {
    return originationId;
  }
  const originationTransferId = requireIdentifier(value.originationTransferId);
  if (!originationTransferId.ok) {
    return originationTransferId;
  }
  const initialPrincipal = requireSignedBound(value.initialPrincipal, true);
  if (!initialPrincipal.ok) {
    return initialPrincipal;
  }
  const nominalLiabilityCap = requireSignedBound(value.nominalLiabilityCap, true);
  if (!nominalLiabilityCap.ok) {
    return nominalLiabilityCap;
  }
  const liabilityIncurred = requireSignedBound(value.liabilityIncurred, true);
  if (!liabilityIncurred.ok) {
    return liabilityIncurred;
  }
  const accrualTerms = parseAccrualTerms(value.accrualTerms);
  if (!accrualTerms.ok) {
    return accrualTerms;
  }
  const lastAccruedPeriod = requireUInt64Text(value.lastAccruedPeriod);
  if (!lastAccruedPeriod.ok) {
    return lastAccruedPeriod;
  }
  const nextAccrualAt = requireUInt64Text(value.nextAccrualAt);
  if (!nextAccrualAt.ok) {
    return nextAccrualAt;
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
  const initial = BigInt(initialPrincipal.value);
  const incurred = BigInt(liabilityIncurred.value);
  const cap = BigInt(nominalLiabilityCap.value);
  if (p > initial || initial > incurred || incurred > cap) {
    return bad('INVARIANT');
  }
  if (a > incurred - initial) {
    return bad('INVARIANT');
  }
  const expected = expectedNextAccrualAt(accrualTerms.value, lastAccruedPeriod.value);
  if (expected === null || expected !== BigInt(nextAccrualAt.value)) {
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
    originationId: originationId.value,
    originationTransferId: originationTransferId.value,
    initialPrincipal: initialPrincipal.value,
    nominalLiabilityCap: nominalLiabilityCap.value,
    liabilityIncurred: liabilityIncurred.value,
    accrualTerms: accrualTerms.value,
    lastAccruedPeriod: lastAccruedPeriod.value,
    nextAccrualAt: nextAccrualAt.value,
  });
}

function requireActionAmount(value: unknown): Res<string> {
  const parsed = requireUInt128Text(value);
  if (!parsed.ok) {
    return parsed;
  }
  if (BigInt(parsed.value) > SIGNED128_MAX) {
    return bad('INVALID_AMOUNT');
  }
  return parsed;
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
    const nominalAmount = requireActionAmount(value.nominalAmount);
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
  if (kind === 'Originate') {
    const keys = closedKeys(value, ORIGINATE_KEYS);
    if (keys !== null) {
      return keys;
    }
    const obligationId = requireIdentifier(value.obligationId);
    if (!obligationId.ok) {
      return obligationId;
    }
    const transferId = requireIdentifier(value.transferId);
    if (!transferId.ok) {
      return transferId;
    }
    const originationId = requireIdentifier(value.originationId);
    if (!originationId.ok) {
      return originationId;
    }
    const debtor = requireIdentifier(value.debtor);
    if (!debtor.ok) {
      return debtor;
    }
    const creditor = requireIdentifier(value.creditor);
    if (!creditor.ok) {
      return creditor;
    }
    const nominalAmount = requireActionAmount(value.nominalAmount);
    if (!nominalAmount.ok) {
      return nominalAmount;
    }
    const denomination = requireIdentifier(value.denomination);
    if (!denomination.ok) {
      return denomination;
    }
    const settlementAsset = requireIdentifier(value.settlementAsset);
    if (!settlementAsset.ok) {
      return settlementAsset;
    }
    const conversion = parseConversion(value.conversion);
    if (!conversion.ok) {
      return conversion;
    }
    const rule = value.allocationRule;
    if (rule !== 'AccrualFirst' && rule !== 'PrincipalFirst' && rule !== 'ProRata') {
      return bad('SCHEMA');
    }
    const accrualTerms = parseAccrualTerms(value.accrualTerms);
    if (!accrualTerms.ok) {
      return accrualTerms;
    }
    const nominalLiabilityCap = requireActionAmount(value.nominalLiabilityCap);
    if (!nominalLiabilityCap.ok) {
      return nominalLiabilityCap;
    }
    return ok({
      kind: 'Originate',
      obligationId: obligationId.value,
      transferId: transferId.value,
      originationId: originationId.value,
      debtor: debtor.value,
      creditor: creditor.value,
      nominalAmount: nominalAmount.value,
      denomination: denomination.value,
      settlementAsset: settlementAsset.value,
      conversion: conversion.value,
      allocationRule: rule,
      accrualTerms: accrualTerms.value,
      nominalLiabilityCap: nominalLiabilityCap.value,
    });
  }
  if (kind === 'Accrue') {
    const keys = closedKeys(value, ACCRUE_KEYS);
    if (keys !== null) {
      return keys;
    }
    const accrualId = requireIdentifier(value.accrualId);
    if (!accrualId.ok) {
      return accrualId;
    }
    const obligationId = requireIdentifier(value.obligationId);
    if (!obligationId.ok) {
      return obligationId;
    }
    const periodIndex = requireUInt64Text(value.periodIndex);
    if (!periodIndex.ok) {
      return periodIndex;
    }
    const observedTime = requireUInt64Text(value.observedTime);
    if (!observedTime.ok) {
      return observedTime;
    }
    return ok({
      kind: 'Accrue',
      accrualId: accrualId.value,
      obligationId: obligationId.value,
      periodIndex: periodIndex.value,
      observedTime: observedTime.value,
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

function parseState(value: unknown): Res<LifecycleState> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, STATE_KEYS);
  if (keys !== null) {
    return keys;
  }
  if (value.schemaVersion !== LIFECYCLE_STATE_VERSION) {
    return bad('SCHEMA');
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
  const originationIds: string[] = [];
  const originationTransferIds: string[] = [];
  let periodSum = 0n;
  for (let i = 0; i < obligations.value.length; i++) {
    const obligation = obligations.value[i];
    if (obligation === undefined) {
      return bad('SCHEMA');
    }
    obligationIds.push(obligation.id);
    originationIds.push(obligation.originationId);
    originationTransferIds.push(obligation.originationTransferId);
    const next = addU128(periodSum, BigInt(obligation.lastAccruedPeriod));
    if (next === null) {
      return bad('INVARIANT');
    }
    periodSum = next;
  }
  if (!uniqueStrings(obligationIds) || !uniqueStrings(originationIds)
    || !uniqueStrings(originationTransferIds)) {
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
  const usedOriginationIds = parseIdentifierList(value.usedOriginationIds);
  if (!usedOriginationIds.ok) {
    return usedOriginationIds;
  }
  const usedAccrualIds = parseIdentifierList(value.usedAccrualIds);
  if (!usedAccrualIds.ok) {
    return usedAccrualIds;
  }
  const work = parseWork(value.work);
  if (!work.ok) {
    return work;
  }
  if (usedOriginationIds.value.length !== originationIds.length) {
    return bad('INVARIANT');
  }
  for (let i = 0; i < originationIds.length; i++) {
    const id = originationIds[i];
    if (id === undefined || !usedOriginationIds.value.includes(id)) {
      return bad('INVARIANT');
    }
  }
  for (let i = 0; i < originationTransferIds.length; i++) {
    const id = originationTransferIds[i];
    if (id === undefined || !usedTransferIds.value.includes(id)) {
      return bad('INVARIANT');
    }
  }
  if (periodSum !== BigInt(usedAccrualIds.value.length)) {
    return bad('INVARIANT');
  }
  return ok({
    schemaVersion: LIFECYCLE_STATE_VERSION,
    balances: balances.value,
    allowances: allowances.value,
    obligations: obligations.value,
    usedTransferIds: usedTransferIds.value,
    usedAllocationIds: usedAllocationIds.value,
    usedOriginationIds: usedOriginationIds.value,
    usedAccrualIds: usedAccrualIds.value,
    work: work.value,
  });
}

function parseInput(value: unknown): Res<LifecycleInput> {
  if (!isRecord(value)) {
    return bad('SCHEMA');
  }
  const keys = closedKeys(value, INPUT_KEYS);
  if (keys !== null) {
    return keys;
  }
  if (value.schemaVersion !== LIFECYCLE_VERSION) {
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
    schemaVersion: LIFECYCLE_VERSION,
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

function copyAccrualTerms(terms: AccrualTerms): AccrualTerms {
  return {
    numerator: terms.numerator,
    denominator: terms.denominator,
    rounding: terms.rounding,
    periodSeconds: terms.periodSeconds,
    firstPeriodStart: terms.firstPeriodStart,
  };
}

function copyObligation(item: LifecycleObligation): LifecycleObligation {
  return {
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
    originationId: item.originationId,
    originationTransferId: item.originationTransferId,
    initialPrincipal: item.initialPrincipal,
    nominalLiabilityCap: item.nominalLiabilityCap,
    liabilityIncurred: item.liabilityIncurred,
    accrualTerms: copyAccrualTerms(item.accrualTerms),
    lastAccruedPeriod: item.lastAccruedPeriod,
    nextAccrualAt: item.nextAccrualAt,
  };
}

function copyIds(ids: readonly Identifier[]): Identifier[] {
  const out: Identifier[] = [];
  for (let i = 0; i < ids.length; i++) {
    const id = ids[i];
    if (id !== undefined) {
      out.push(id);
    }
  }
  return out;
}

function copyState(state: LifecycleState): LifecycleState {
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
  const obligations: LifecycleObligation[] = [];
  for (let i = 0; i < state.obligations.length; i++) {
    const item = state.obligations[i];
    if (item === undefined) {
      continue;
    }
    obligations.push(copyObligation(item));
  }
  return {
    schemaVersion: LIFECYCLE_STATE_VERSION,
    balances,
    allowances,
    obligations,
    usedTransferIds: copyIds(state.usedTransferIds),
    usedAllocationIds: copyIds(state.usedAllocationIds),
    usedOriginationIds: copyIds(state.usedOriginationIds),
    usedAccrualIds: copyIds(state.usedAccrualIds),
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

function findObligationIndex(items: readonly LifecycleObligation[], id: string): number {
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
  post: LifecycleState,
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
    if (post.balances.length >= LIFECYCLE_BOUNDS.collectionCapacity) {
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
  if (post.usedTransferIds.length >= LIFECYCLE_BOUNDS.collectionCapacity) {
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
  post: LifecycleState,
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
  if (post.usedAllocationIds.length >= LIFECYCLE_BOUNDS.collectionCapacity) {
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

function applyOriginate(
  post: LifecycleState,
  action: OriginateAction,
  usedObligationIds: Set<string>,
  usedOriginationIds: Set<string>,
  step: Map<string, StepTransfer>,
): ApplyResult {
  const nominal = BigInt(action.nominalAmount);
  const cap = BigInt(action.nominalLiabilityCap);
  if (nominal === 0n || cap === 0n) {
    return bad('ZERO_AMOUNT');
  }
  if (usedObligationIds.has(action.obligationId) || usedOriginationIds.has(action.originationId)) {
    return bad('DUPLICATE');
  }
  const funded = step.get(action.transferId);
  if (funded === undefined) {
    return bad('TRANSFER_NOT_IN_STEP');
  }
  if (
    funded.from !== action.creditor ||
    funded.to !== action.debtor ||
    funded.asset !== action.settlementAsset
  ) {
    return bad('TRANSFER_MISMATCH');
  }
  if (funded.remaining !== funded.amount) {
    return bad('TRANSFER_ALREADY_ALLOCATED');
  }
  const settlement = convertNominal(nominal, action.conversion);
  if (!settlement.ok) {
    return settlement;
  }
  if (settlement.value === 0n) {
    return bad('DUST');
  }
  if (settlement.value !== funded.amount) {
    return bad('TRANSFER_AMOUNT_MISMATCH');
  }
  const nextAt = addU64(
    BigInt(action.accrualTerms.firstPeriodStart),
    BigInt(action.accrualTerms.periodSeconds),
  );
  if (nextAt === null) {
    return bad('OVERFLOW');
  }
  if (nominal > cap) {
    return bad('LIABILITY_CAP_EXCEEDED');
  }
  if (
    post.obligations.length >= LIFECYCLE_BOUNDS.collectionCapacity
    || post.usedOriginationIds.length >= LIFECYCLE_BOUNDS.collectionCapacity
  ) {
    return bad('CAPACITY');
  }
  funded.remaining = 0n;
  const obligation: LifecycleObligation = {
    id: action.obligationId,
    debtor: action.debtor,
    creditor: action.creditor,
    denomination: action.denomination,
    settlementAsset: action.settlementAsset,
    principal: action.nominalAmount,
    accrued: '0',
    outstanding: action.nominalAmount,
    allocationRule: action.allocationRule,
    conversion: copyConversion(action.conversion),
    status: 'Outstanding',
    originationId: action.originationId,
    originationTransferId: action.transferId,
    initialPrincipal: action.nominalAmount,
    nominalLiabilityCap: action.nominalLiabilityCap,
    liabilityIncurred: action.nominalAmount,
    accrualTerms: copyAccrualTerms(action.accrualTerms),
    lastAccruedPeriod: '0',
    nextAccrualAt: dec(nextAt),
  };
  post.obligations.push(obligation);
  post.usedOriginationIds.push(action.originationId);
  usedObligationIds.add(action.obligationId);
  usedOriginationIds.add(action.originationId);
  return ok({
    obligationId: action.obligationId,
    transferId: action.transferId,
    originationId: action.originationId,
    debtor: action.debtor,
    creditor: action.creditor,
    nominalAmount: action.nominalAmount,
    denomination: action.denomination,
    settlementAsset: action.settlementAsset,
    conversion: copyConversion(action.conversion),
    allocationRule: action.allocationRule,
    accrualTerms: copyAccrualTerms(action.accrualTerms),
    nominalLiabilityCap: action.nominalLiabilityCap,
    kind: 'Origination',
    settlementAmount: dec(settlement.value),
    liabilityIncurred: action.nominalAmount,
    lastAccruedPeriod: '0',
    nextAccrualAt: obligation.nextAccrualAt,
  });
}

function applyAccrue(
  post: LifecycleState,
  action: AccrueAction,
  usedAccrualIds: Set<string>,
): ApplyResult {
  const obligationIndex = findObligationIndex(post.obligations, action.obligationId);
  if (obligationIndex < 0) {
    return bad('MISSING_OBLIGATION');
  }
  const obligation = post.obligations[obligationIndex];
  if (obligation === undefined) {
    return bad('MISSING_OBLIGATION');
  }
  if (obligation.status !== 'Outstanding') {
    return bad('NOT_OUTSTANDING');
  }
  if (usedAccrualIds.has(action.accrualId)) {
    return bad('DUPLICATE');
  }
  const last = BigInt(obligation.lastAccruedPeriod);
  const periodIndex = BigInt(action.periodIndex);
  const expectedPeriod = addU64(last, 1n);
  if (expectedPeriod === null || periodIndex !== expectedPeriod) {
    return bad('PERIOD_SEQUENCE');
  }
  const observed = BigInt(action.observedTime);
  const eligibleAt = BigInt(obligation.nextAccrualAt);
  if (observed < eligibleAt) {
    return bad('PERIOD_NOT_ELIGIBLE');
  }
  const principal = BigInt(obligation.principal);
  const numerator = BigInt(obligation.accrualTerms.numerator);
  const denominator = BigInt(obligation.accrualTerms.denominator);
  const product = mulU128(principal, numerator);
  if (product === null) {
    return bad('OVERFLOW');
  }
  let interest = product / denominator;
  const remainder = product % denominator;
  if (obligation.accrualTerms.rounding === 'ceil' && remainder !== 0n) {
    const ceiled = addU128(interest, 1n);
    if (ceiled === null) {
      return bad('OVERFLOW');
    }
    interest = ceiled;
  }
  const previousAccrued = BigInt(obligation.accrued);
  const previousOutstanding = BigInt(obligation.outstanding);
  const previousIncurred = BigInt(obligation.liabilityIncurred);
  const nextAccrued = addU128(previousAccrued, interest);
  if (nextAccrued === null) {
    return bad('OVERFLOW');
  }
  const nextOutstanding = addU128(previousOutstanding, interest);
  if (nextOutstanding === null) {
    return bad('OVERFLOW');
  }
  const nextIncurred = addU128(previousIncurred, interest);
  if (nextIncurred === null) {
    return bad('OVERFLOW');
  }
  const periodPlus = addU64(periodIndex, 1n);
  if (periodPlus === null) {
    return bad('OVERFLOW');
  }
  const scaled = mulU64(periodPlus, BigInt(obligation.accrualTerms.periodSeconds));
  if (scaled === null) {
    return bad('OVERFLOW');
  }
  const nextAt = addU64(BigInt(obligation.accrualTerms.firstPeriodStart), scaled);
  if (nextAt === null) {
    return bad('OVERFLOW');
  }
  if (nextAccrued > SIGNED128_MAX || nextOutstanding > SIGNED128_MAX || nextIncurred > SIGNED128_MAX) {
    return bad('NOMINAL_RANGE');
  }
  if (nextIncurred > BigInt(obligation.nominalLiabilityCap)) {
    return bad('LIABILITY_CAP_EXCEEDED');
  }
  if (post.usedAccrualIds.length >= LIFECYCLE_BOUNDS.collectionCapacity) {
    return bad('CAPACITY');
  }
  obligation.accrued = dec(nextAccrued);
  obligation.outstanding = dec(nextOutstanding);
  obligation.liabilityIncurred = dec(nextIncurred);
  obligation.lastAccruedPeriod = action.periodIndex;
  obligation.nextAccrualAt = dec(nextAt);
  post.usedAccrualIds.push(action.accrualId);
  usedAccrualIds.add(action.accrualId);
  return ok({
    kind: 'Accrual',
    accrualId: action.accrualId,
    obligationId: action.obligationId,
    debtor: obligation.debtor,
    creditor: obligation.creditor,
    denomination: obligation.denomination,
    periodIndex: action.periodIndex,
    observedTime: action.observedTime,
    eligibleAt: dec(eligibleAt),
    nextAccrualAt: obligation.nextAccrualAt,
    principalBasis: obligation.principal,
    numerator: obligation.accrualTerms.numerator,
    denominator: obligation.accrualTerms.denominator,
    rounding: obligation.accrualTerms.rounding,
    interestAmount: dec(interest),
    previousAccrued: dec(previousAccrued),
    accrued: obligation.accrued,
    previousOutstanding: dec(previousOutstanding),
    outstanding: obligation.outstanding,
    previousLiabilityIncurred: dec(previousIncurred),
    liabilityIncurred: obligation.liabilityIncurred,
    nominalLiabilityCap: obligation.nominalLiabilityCap,
  });
}

function publishPost(post: LifecycleState): LifecycleResult | null {
  const compact = JSON.stringify(post);
  if (new TextEncoder().encode(compact).length > LIFECYCLE_BOUNDS.sourceUtf8Bytes) {
    return rejected('RESULT_BOUND', null);
  }
  const admitted = parseState(JSON.parse(compact) as unknown);
  if (!admitted.ok) {
    return rejected(admitted.code, null);
  }
  return null;
}

function runActions(input: LifecycleInput): LifecycleResult {
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
  const usedObligationIds = new Set<string>(post.obligations.map((item) => item.id));
  const usedOriginationIds = new Set<string>(post.usedOriginationIds);
  const usedAccrualIds = new Set<string>(post.usedAccrualIds);
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
    } else if (action.kind === 'Originate') {
      applied = applyOriginate(post, action, usedObligationIds, usedOriginationIds, step);
    } else if (action.kind === 'Accrue') {
      applied = applyAccrue(post, action, usedAccrualIds);
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
  const bound = publishPost(post);
  if (bound !== null) {
    return bound;
  }
  return {
    status: 'Prepared',
    schemaVersion: LIFECYCLE_VERSION,
    post,
    effects,
  };
}

export function prepareFinancialLifecycle(source: string): LifecycleResult {
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

/** Full lifecycle state admission from a primitive JSON string. */
export function admitFinancialLifecycleStateJSON(
  text: unknown,
): { ok: true; value: LifecycleState } | { ok: false; result: RejectedLifecycle | { status: 'Rejected'; code: string } } {
  if (typeof text !== 'string') {
    return { ok: false, result: { status: 'Rejected', code: 'INPUT_SCHEMA' } };
  }
  if (text.length > LIFECYCLE_BOUNDS.sourceUtf8Bytes) {
    return { ok: false, result: { status: 'Rejected', code: 'INPUT_BOUND' } };
  }
  const bytes = new TextEncoder().encode(text);
  if (bytes.length > LIFECYCLE_BOUNDS.sourceUtf8Bytes) {
    return { ok: false, result: { status: 'Rejected', code: 'INPUT_BOUND' } };
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch {
    return { ok: false, result: { status: 'Rejected', code: 'INPUT_SCHEMA' } };
  }
  let owned: unknown;
  try {
    owned = JSON.parse(JSON.stringify(parsed));
  } catch {
    return { ok: false, result: { status: 'Rejected', code: 'INPUT_SCHEMA' } };
  }
  const state = parseState(owned);
  if (!state.ok) {
    return { ok: false, result: rejected(state.code, null) };
  }
  return { ok: true, value: copyState(state.value) };
}
