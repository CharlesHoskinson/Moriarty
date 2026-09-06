import type { CoreState, Effect } from './core.js';

export type IntentPolicy = {
  profile: 'loan' | 'swap';
  accounting: Record<string, { asset: string; account: string }>;
  transfers: Array<{ asset: string; from: string; to: string; maxAmount: string }>;
  minimumCredits: Array<{ asset: string; to: string; minAmount: string }>;
  fees: Array<{ asset: string; from: string; to: string; maxAmount: string }>;
  dues: Array<{
    kind: 'DueCreated' | 'DueSettled'; bucket: 'principal' | 'interest'; dueId: string;
    debtor: string; creditor: string; denomination: string; asset?: string; maxAmount: string;
  }>;
  allowedWrites: string[];
};

type Rejection = { outcome: 'rejected'; code: string; message: string };
export type PolicyOutcome =
  | { outcome: 'checked'; predicate: 'intent-effects'; assurance: 'local-check-only' }
  | Rejection;

const U128_MAX = (1n << 128n) - 1n;
const MAX_TEXT = 128;
const MAX_ITEMS = 16;
const MAX_FIELDS = 64;
const MAX_BYTES = 65_536;
const MAX_NODES = 4096;
const RESERVED_KEYS = new Set(['constructor', 'prototype', '__proto__']);
const own = (value: object, key: string) => Object.prototype.hasOwnProperty.call(value, key);
class IntakeError extends Error {}
function safeJsonValue(value: unknown, label: string): unknown {
  const active = new Set<object>(); let nodes = 0; let bytes = 0;
  const count = (textValue: string) => {
    bytes += new TextEncoder().encode(textValue).length;
    if (bytes > MAX_BYTES) throw new IntakeError(`${label} exceeds its byte bound`);
  };
  const walk = (item: unknown, depth: number): unknown => {
    if (++nodes > MAX_NODES || depth > 40) throw new IntakeError(`${label} exceeds its structural bound`);
    if (item === null || typeof item === 'boolean') return item;
    if (typeof item === 'string') { count(item); return item; }
    if (typeof item !== 'object') throw new IntakeError(`${label} is not JSON-only`);
    if (active.has(item)) throw new IntakeError(`${label} is cyclic`);
    const proto = Object.getPrototypeOf(item);
    if (Array.isArray(item)) {
      if (proto !== Array.prototype || item.length > MAX_FIELDS) throw new IntakeError(`${label} has an invalid or oversized array`);
      const descriptors = Object.getOwnPropertyDescriptors(item) as unknown as Record<string, PropertyDescriptor>;
      const keys = Reflect.ownKeys(descriptors);
      if (keys.some(key => typeof key === 'symbol') || keys.length !== item.length + 1 || descriptors.length.value !== item.length)
        throw new IntakeError(`${label} has an active or sparse array`);
      active.add(item);
      const result = Array.from({length:item.length}, (_, index) => {
        const descriptor = descriptors[String(index)];
        if (!descriptor?.enumerable || !own(descriptor, 'value')) throw new IntakeError(`${label} has an accessor array element`);
        return walk(descriptor.value, depth + 1);
      });
      active.delete(item); return result;
    }
    if (proto !== Object.prototype && proto !== null) throw new IntakeError(`${label} has a custom prototype`);
    const descriptors = Object.getOwnPropertyDescriptors(item);
    const keys = Reflect.ownKeys(descriptors);
    if (keys.length > MAX_FIELDS || keys.some(key => typeof key === 'symbol')) throw new IntakeError(`${label} has too many or symbolic fields`);
    const result: Record<string, unknown> = Object.create(null); active.add(item);
    for (const rawKey of keys) {
      const key = rawKey as string; const descriptor = descriptors[key];
      if (RESERVED_KEYS.has(key) || !descriptor.enumerable || !own(descriptor, 'value')) throw new IntakeError(`${label} has an active, hidden, or reserved field`);
      count(key); result[key] = walk(descriptor.value, depth + 1);
    }
    active.delete(item); return result;
  };
  const safe = walk(value, 0);
  if (new TextEncoder().encode(JSON.stringify(safe)).length > MAX_BYTES) throw new IntakeError(`${label} exceeds its byte bound`);
  return safe;
}
const object = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null && !Array.isArray(value)
  && (Object.getPrototypeOf(value) === Object.prototype || Object.getPrototypeOf(value) === null);
const exact = (value: Record<string, unknown>, required: string[], optional: string[] = []) => {
  const allowed = new Set([...required, ...optional]);
  return required.every(key => own(value, key)) && Object.keys(value).every(key => allowed.has(key));
};
const text = (value: unknown): value is string => typeof value === 'string' && value.length > 0 && value.length <= MAX_TEXT
  && !/[\u0000-\u001f\u007f]/.test(value);
const uint = (value: unknown): value is string =>
  typeof value === 'string' && /^(0|[1-9][0-9]*)$/.test(value) && value.length <= 39 && BigInt(value) <= U128_MAX;
const boundedArray = (value: unknown): value is unknown[] => Array.isArray(value) && value.length <= MAX_ITEMS;
const reject = (code: string, message: string): Rejection => ({ outcome: 'rejected', code, message });

const PROFILE_FIELDS = {
  swap: ['reserveA', 'reserveB', 'traderA', 'traderB', 'providerA', 'providerB', 'closed'],
  loan: ['notional', 'principalDue', 'interestDue', 'principalPaid', 'interestPaid', 'borrowerCash', 'lenderCash', 'cursor', 'closed'],
} as const;
const ACCOUNT_FIELDS = {
  swap: ['reserveA', 'reserveB', 'traderA', 'traderB', 'providerA', 'providerB'],
  loan: ['borrowerCash', 'lenderCash'],
} as const;

function validState(value: unknown, profile: 'loan' | 'swap'): value is CoreState {
  if (!object(value) || !exact(value, ['instance', 'revision', 'remaining', 'values']) || !text(value.instance)
    || !uint(value.revision) || !uint(value.remaining) || !object(value.values)) return false;
  const values = value.values as Record<string, unknown>;
  const expected = PROFILE_FIELDS[profile];
  const keys = Object.keys(values);
  return keys.length <= MAX_FIELDS && keys.length === expected.length && expected.every(key => own(values, key) && uint(values[key]));
}

function validAccounting(policy: Record<string, unknown>, profile: 'loan' | 'swap'): boolean {
  if (!object(policy.accounting)) return false;
  const required = ACCOUNT_FIELDS[profile];
  if (Object.keys(policy.accounting).length !== required.length || !required.every(field => own(policy.accounting as object, field))) return false;
  const accounting = policy.accounting as Record<string, unknown>;
  for (const field of required) {
    const entry = accounting[field];
    if (!object(entry) || !exact(entry, ['asset', 'account']) || !text(entry.asset) || !text(entry.account)) return false;
  }
  const a = accounting as Record<string, { asset: string; account: string }>;
  if (profile === 'loan') return a.borrowerCash.asset === a.lenderCash.asset && a.borrowerCash.account !== a.lenderCash.account;
  const sameRoles = a.reserveA.account === a.reserveB.account && a.traderA.account === a.traderB.account
    && a.providerA.account === a.providerB.account;
  const distinctRoles = new Set([a.reserveA.account, a.traderA.account, a.providerA.account]).size === 3;
  const assetA = a.reserveA.asset;
  const assetB = a.reserveB.asset;
  return sameRoles && distinctRoles && assetA !== assetB
    && a.traderA.asset === assetA && a.providerA.asset === assetA
    && a.traderB.asset === assetB && a.providerB.asset === assetB;
}

function validPolicy(value: unknown): value is IntentPolicy {
  if (!object(value) || !exact(value, ['profile', 'accounting', 'transfers', 'minimumCredits', 'fees', 'dues', 'allowedWrites'])
    || (value.profile !== 'loan' && value.profile !== 'swap') || !validAccounting(value, value.profile)) return false;
  if (![value.transfers, value.minimumCredits, value.fees, value.dues, value.allowedWrites].every(boundedArray)) return false;
  const transfers = value.transfers as unknown[];
  const minimumCredits = value.minimumCredits as unknown[];
  const fees = value.fees as unknown[];
  const dues = value.dues as unknown[];
  const allowedWrites = value.allowedWrites as unknown[];
  const writeSet = new Set<string>();
  for (const field of allowedWrites) {
    if (!text(field) || !PROFILE_FIELDS[value.profile].includes(field as never) || writeSet.has(field)) return false;
    writeSet.add(field);
  }
  const identities = new Set<string>();
  const allowances = [...transfers.map(entry => ['transfer', entry]), ...fees.map(entry => ['fee', entry])] as Array<[string, unknown]>;
  for (const [category, entry] of allowances) {
    if (!object(entry) || !exact(entry, ['asset', 'from', 'to', 'maxAmount']) || !text(entry.asset) || !text(entry.from)
      || !text(entry.to) || !uint(entry.maxAmount) || entry.from === entry.to) return false;
    const id = `${category}\0${entry.asset}\0${entry.from}\0${entry.to}`;
    if (identities.has(id)) return false;
    identities.add(id);
  }
  for (const entry of minimumCredits) {
    if (!object(entry) || !exact(entry, ['asset', 'to', 'minAmount']) || !text(entry.asset) || !text(entry.to) || !uint(entry.minAmount)) return false;
    const id = `minimum\0${entry.asset}\0${entry.to}`;
    if (identities.has(id)) return false;
    identities.add(id);
  }
  for (const entry of dues) {
    if (!object(entry) || !exact(entry, ['kind', 'bucket', 'dueId', 'debtor', 'creditor', 'denomination', 'maxAmount'], ['asset'])
      || (entry.kind !== 'DueCreated' && entry.kind !== 'DueSettled') || (entry.bucket !== 'principal' && entry.bucket !== 'interest')
      || !text(entry.dueId) || !text(entry.debtor) || !text(entry.creditor) || !text(entry.denomination) || !uint(entry.maxAmount)
      || entry.debtor === entry.creditor || (entry.kind === 'DueCreated' ? own(entry, 'asset') : !text(entry.asset))) return false;
    const id = `due\0${entry.kind}\0${entry.dueId}\0${entry.debtor}\0${entry.creditor}\0${entry.denomination}\0${entry.asset ?? ''}`;
    if (identities.has(id)) return false;
    identities.add(id);
  }
  return value.profile === 'swap' ? dues.length === 0 : true;
}

type ParsedEffect = { kind: 'Transfer' | 'Fee' | 'DueCreated' | 'DueSettled'; fields: Record<string, string> };
function validEffect(effect: unknown): effect is ParsedEffect {
  if (!object(effect) || !exact(effect, ['kind', 'fields']) || !object(effect.fields)) return false;
  const kind = effect.kind;
  if (kind !== 'Transfer' && kind !== 'Fee' && kind !== 'DueCreated' && kind !== 'DueSettled') return false;
  const required = kind === 'Transfer' || kind === 'Fee'
    ? ['asset', 'from', 'to', 'amount']
    : kind === 'DueCreated'
      ? ['dueId', 'debtor', 'creditor', 'denomination', 'amount']
      : ['dueId', 'debtor', 'creditor', 'denomination', 'asset', 'amount'];
  const fields = effect.fields as Record<string, unknown>;
  if (!exact(fields, required)) return false;
  return required.every(key => key === 'amount' ? uint(fields[key]) : text(fields[key]))
    && (kind === 'Transfer' || kind === 'Fee' ? fields.from !== fields.to : fields.debtor !== fields.creditor);
}

class PolicyError extends Error {
  readonly code: string;
  constructor(code: string, message: string) { super(message); this.code = code; }
}
const add = (map: Map<string, bigint>, key: string, amount: bigint) => {
  const total = (map.get(key) ?? 0n) + amount;
  if (total > U128_MAX || total < -U128_MAX) throw new PolicyError('AggregateOverflow', 'Aggregate effect arithmetic exceeds UInt128 magnitude.');
  map.set(key, total);
};
const allowanceKey = (fields: Record<string, string>) => `${fields.asset}\0${fields.from}\0${fields.to}`;

function checkParsedIntentEffects(before: CoreState, after: CoreState, effects: Effect[], policy: IntentPolicy): PolicyOutcome {
  if (!validPolicy(policy)) return reject('MalformedPolicy', 'Intent policy is malformed or exceeds a bound.');
  if (!validState(before, policy.profile) || !validState(after, policy.profile)) return reject('MalformedState', 'State is malformed, unknown, or exceeds a bound.');
  if (!boundedArray(effects) || !effects.every(validEffect)) return reject('MalformedEffect', 'Effect is malformed, unknown, or exceeds a bound.');
  const revision = BigInt(before.revision);
  const remaining = BigInt(before.remaining);
  if (before.instance !== after.instance || revision === U128_MAX || BigInt(after.revision) !== revision + 1n || remaining === 0n || BigInt(after.remaining) !== remaining - 1n)
    return reject('InvalidTransition', 'State envelope does not make exactly one bounded transition.');

  if (policy.profile === 'loan') {
    const borrower = policy.accounting.borrowerCash.account;
    const lender = policy.accounting.lenderCash.account;
    const loanAsset = policy.accounting.borrowerCash.asset;
    const bindingsValid = policy.dues.every(due => due.debtor === borrower && due.creditor === lender
      && due.denomination === 'USD' && due.dueId === `${before.instance}:${due.bucket}`
      && (due.kind === 'DueCreated' ? due.asset === undefined : due.asset === loanAsset));
    if (!bindingsValid) return reject('MalformedPolicy', 'Loan due allowances do not match the instance, accounting roles, denomination, bucket, or settlement asset.');
  }

  const changed = PROFILE_FIELDS[policy.profile].filter(field => before.values[field] !== after.values[field]);
  const allowedWrites = new Set(policy.allowedWrites);
  if (changed.some(field => !allowedWrites.has(field))) return reject('WriteNotAllowed', 'A changed state field is not allowed by intent.');

  const transferCaps = new Map(policy.transfers.map(a => [`${a.asset}\0${a.from}\0${a.to}`, BigInt(a.maxAmount)]));
  const feeCaps = new Map(policy.fees.map(a => [`${a.asset}\0${a.from}\0${a.to}`, BigInt(a.maxAmount)]));
  const transferTotals = new Map<string, bigint>();
  const feeTotals = new Map<string, bigint>();
  const balanceDeltas = new Map<string, bigint>();
  const accountNets = new Map<string, bigint>();
  const dueTotals = new Map<string, bigint>();
  const accountField = new Map(Object.entries(policy.accounting).map(([field, entry]) => [`${entry.asset}\0${entry.account}`, field]));
  for (const allowance of [...policy.transfers, ...policy.fees]) {
    if (!accountField.has(`${allowance.asset}\0${allowance.from}`) || !accountField.has(`${allowance.asset}\0${allowance.to}`))
      return reject('UnsupportedMapping', 'A policy allowance account or asset has no profile accounting field.');
  }
  for (const minimum of policy.minimumCredits) {
    if (!accountField.has(`${minimum.asset}\0${minimum.to}`))
      return reject('UnsupportedMapping', 'A minimum-credit account or asset has no profile accounting field.');
  }

  for (const effect of effects as ParsedEffect[]) {
    const f = effect.fields;
    if (effect.kind === 'Transfer' || effect.kind === 'Fee') {
      const key = allowanceKey(f);
      const totals = effect.kind === 'Fee' ? feeTotals : transferTotals;
      const caps = effect.kind === 'Fee' ? feeCaps : transferCaps;
      if (!caps.has(key)) return reject('EffectNotAllowed', `${effect.kind} is outside the intent allowance.`);
      add(totals, key, BigInt(f.amount));
      if ((totals.get(key) ?? 0n) > caps.get(key)!) return reject(effect.kind === 'Fee' ? 'FeeLimitExceeded' : 'EffectLimitExceeded', `${effect.kind} cumulative amount exceeds its cap.`);
      const fromField = accountField.get(`${f.asset}\0${f.from}`);
      const toField = accountField.get(`${f.asset}\0${f.to}`);
      if (!fromField || !toField) return reject('UnsupportedMapping', 'An effect account or asset has no profile accounting field.');
      add(balanceDeltas, fromField, -BigInt(f.amount));
      add(balanceDeltas, toField, BigInt(f.amount));
      add(accountNets, `${f.asset}\0${f.from}`, -BigInt(f.amount));
      add(accountNets, `${f.asset}\0${f.to}`, BigInt(f.amount));
      continue;
    }
    const allowance = policy.dues.find(a => a.kind === effect.kind && a.dueId === f.dueId && a.debtor === f.debtor
      && a.creditor === f.creditor && a.denomination === f.denomination && (a.asset ?? '') === (f.asset ?? ''));
    if (!allowance) return reject('EffectNotAllowed', `${effect.kind} is outside the due allowance.`);
    const key = `${effect.kind}\0${f.dueId}\0${f.debtor}\0${f.creditor}\0${f.denomination}\0${f.asset ?? ''}`;
    add(dueTotals, key, BigInt(f.amount));
    if (dueTotals.get(key)! > BigInt(allowance.maxAmount)) return reject('EffectLimitExceeded', `${effect.kind} cumulative amount exceeds its cap.`);
    add(dueTotals, `${effect.kind}\0${allowance.bucket}`, BigInt(f.amount));
  }

  for (const minimum of policy.minimumCredits) {
    if ((accountNets.get(`${minimum.asset}\0${minimum.to}`) ?? 0n) < BigInt(minimum.minAmount)) return reject('MinimumCreditNotMet', 'A required net minimum credit was not met.');
  }
  for (const field of ACCOUNT_FIELDS[policy.profile]) {
    const actual = BigInt(after.values[field]) - BigInt(before.values[field]);
    if (actual !== (balanceDeltas.get(field) ?? 0n)) return reject('AccountingMismatch', `Balance field ${field} does not project the effects.`);
  }

  if (policy.profile === 'loan') {
    const createdPrincipal = dueTotals.get('DueCreated\0principal') ?? 0n;
    const createdInterest = dueTotals.get('DueCreated\0interest') ?? 0n;
    const settledPrincipal = dueTotals.get('DueSettled\0principal') ?? 0n;
    const settledInterest = dueTotals.get('DueSettled\0interest') ?? 0n;
    const settledTotal = settledPrincipal + settledInterest;
    if (settledTotal > U128_MAX) throw new PolicyError('AggregateOverflow', 'Aggregate settled due amount exceeds UInt128.');
    const delta = (field: string) => BigInt(after.values[field]) - BigInt(before.values[field]);
    if (delta('principalDue') !== createdPrincipal - settledPrincipal || delta('interestDue') !== createdInterest - settledInterest
      || delta('principalPaid') !== settledPrincipal || delta('interestPaid') !== settledInterest
      || BigInt(before.values.notional) - BigInt(after.values.notional) !== createdPrincipal)
      return reject('LoanAccountingMismatch', 'Loan due, paid, or notional accounting does not project the due effects.');
    const loanAsset = policy.accounting.borrowerCash.asset;
    const borrower = policy.accounting.borrowerCash.account;
    const lender = policy.accounting.lenderCash.account;
    const backing = transferTotals.get(`${loanAsset}\0${borrower}\0${lender}`) ?? 0n;
    if (backing !== settledTotal) return reject('LoanAccountingMismatch', 'Settled dues do not have an equal borrower-to-lender transfer.');
  }
  return { outcome: 'checked', predicate: 'intent-effects', assurance: 'local-check-only' };
}

/*
 * Transfer and fee caps are cumulative gross limits for each exact
 * asset/from/to edge. Refunds never restore authority on an edge. This is a
 * restricted exact-plan envelope; a future user-level budget belongs in a
 * richer Intent IR rather than changing these caps to net totals.
 */
export function checkIntentEffects(before: CoreState, after: CoreState, effects: Effect[], policy: IntentPolicy): PolicyOutcome {
  let safeBefore: unknown;
  let safeAfter: unknown;
  let safeEffects: unknown;
  let safePolicy: unknown;
  try { safePolicy = safeJsonValue(policy, 'intent policy'); }
  catch { return reject('MalformedPolicy', 'Intent policy is active, malformed, or exceeds a bound.'); }
  try {
    safeBefore = safeJsonValue(before, 'before state');
    safeAfter = safeJsonValue(after, 'after state');
  } catch { return reject('MalformedState', 'State is active, malformed, or exceeds a bound.'); }
  try { safeEffects = safeJsonValue(effects, 'effects'); }
  catch { return reject('MalformedEffect', 'Effects are active, malformed, or exceed a bound.'); }
  try {
    return checkParsedIntentEffects(safeBefore as CoreState, safeAfter as CoreState, safeEffects as Effect[], safePolicy as IntentPolicy);
  } catch (error) {
    if (error instanceof PolicyError) return reject(error.code, error.message);
    return reject('MalformedInput', 'Policy checking rejected malformed bounded input.');
  }
}
