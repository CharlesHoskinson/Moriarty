/** Local funded adapter over financial expression source and the repayment kernel.
 * Ordinary writes publish only after descriptor binding and one kernel call succeed. */
import { createFinancialExpressionSourceV1 } from './financial-expression-source-v1.ts';
import type { ExpressionResult } from './financial-expression-v1.ts';
import { ExpressionFailure, bytes, parseCanonical } from './expression-wire-v1.ts';
import { same } from './financial-expression-types-v1.ts';
import type { Schema, ValueType } from './financial-expression-types-v1.ts';
import { prepareRepayment, REPAYMENT_BOUNDS, REPAYMENT_VERSION } from './repayment.ts';
import type { Action, Effect, RejectedRepayment, RepaymentState } from './repayment.ts';
import {
  prepareFinancialLifecycle,
  LIFECYCLE_VERSION,
  admitFinancialLifecycleStateJSON,
} from './financial-lifecycle.ts';
import type {
  Action as LifecycleAction,
  Effect as LifecycleEffect,
  LifecycleState,
  RejectedLifecycle,
} from './financial-lifecycle.ts';

const STATE_BYTES = 65536;
const SNAPSHOT_BYTES = 2_000_000;
const SIGNED128_MAX = (1n << 127n) - 1n;
const UINT128_MAX = BigInt(REPAYMENT_BOUNDS.uint128Max);

interface PreparedEnvelope<Post, Effects> {
  status: 'FundedExpressionPrepared';
  post: Record<string, unknown>;
  financialPost: Post;
  effects: Effects;
  workRemaining: string;
}

export interface FundedExpressionPrepared extends PreparedEnvelope<RepaymentState, Effect[]> {}

export type FundedExpressionRejected =
  | { status: 'Rejected'; code: string }
  | SourceRejected
  | RejectedRepayment;
export type FundedExpressionResult = FundedExpressionPrepared | FundedExpressionRejected;

export interface LifecycleExpressionPrepared extends PreparedEnvelope<LifecycleState, LifecycleEffect[]> {}
export type LifecycleExpressionRejected = FundedExpressionRejected | RejectedLifecycle;
export type LifecycleExpressionResult = LifecycleExpressionPrepared | LifecycleExpressionRejected;

type SourceRejected = Extract<ExpressionResult, { status: 'Rejected' }>;
export type Binding = { transferAsset: string; repayDenomination: string };
export type BindingDiagnostic = { operation?: string; field?: string };
export type OperationBindingFailure = { status: 'Rejected'; code: string; diagnostic?: BindingDiagnostic };

function reject(code: string): FundedExpressionRejected {
  return { status: 'Rejected', code };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function mismatchedField(value: Record<string, unknown>, expected: Record<string, unknown>): string | undefined {
  for (const key of Object.keys(value)) {
    if (!Object.hasOwn(expected, key)) return key;
  }
  for (const key of Object.keys(expected)) {
    if (!Object.hasOwn(value, key) || !same(value[key] as ValueType, expected[key] as ValueType)) return key;
  }
  return undefined;
}

function bindReject(diagnostic?: BindingDiagnostic): OperationBindingFailure {
  return diagnostic === undefined
    ? { status: 'Rejected', code: 'OPERATION_BINDING' }
    : { status: 'Rejected', code: 'OPERATION_BINDING', diagnostic };
}

export function parseOwnedState(text: unknown): { ok: true; value: Record<string, unknown> } | { ok: false; result: FundedExpressionRejected } {
  if (typeof text !== 'string') return { ok: false, result: reject('INPUT_SCHEMA') };
  if (text.length > STATE_BYTES || bytes(text) > STATE_BYTES) return { ok: false, result: reject('INPUT_BOUND') };
  let parsed: unknown;
  try { parsed = JSON.parse(text); } catch { return { ok: false, result: reject('INPUT_SCHEMA') }; }
  let owned: unknown;
  try { owned = JSON.parse(JSON.stringify(parsed)); } catch { return { ok: false, result: reject('INPUT_SCHEMA') }; }
  if (!isRecord(owned) || !isRecord(owned.work) || typeof owned.work.remaining !== 'string') {
    return { ok: false, result: reject('INPUT_SCHEMA') };
  }
  return { ok: true, value: owned };
}

export function snapshotWorkInitial(text: string): string | undefined {
  try {
    const snapshot = parseCanonical(text, SNAPSHOT_BYTES);
    if (isRecord(snapshot) && typeof snapshot.workInitial === 'string') return snapshot.workInitial;
  } catch (error) {
    if (!(error instanceof ExpressionFailure)) throw error;
  }
  return undefined;
}

function unsignedDecimal(value: unknown): bigint | undefined {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return undefined;
  try {
    const n = BigInt(value);
    if (n.toString(10) !== value) return undefined;
    return n;
  } catch {
    return undefined;
  }
}

function signedDecimal(value: unknown): bigint | undefined {
  if (typeof value !== 'string' || !/^(0|-?[1-9][0-9]*)$/.test(value)) return undefined;
  try {
    const n = BigInt(value);
    if (n.toString(10) !== value) return undefined;
    return n;
  } catch {
    return undefined;
  }
}

export function operationBinding(schema: Schema): Binding | OperationBindingFailure {
  const operations = schema.operations;
  if (!isRecord(operations)) return bindReject();
  const names = Object.keys(operations);
  const extra = names.find((name) => name !== 'Transfer' && name !== 'Repay');
  if (names.length !== 2 || typeof operations.Transfer !== 'string' || typeof operations.Repay !== 'string') {
    if (extra !== undefined) return bindReject({ operation: extra });
    if (typeof operations.Transfer !== 'string') return bindReject({ operation: 'Transfer' });
    if (typeof operations.Repay !== 'string') return bindReject({ operation: 'Repay' });
    return bindReject();
  }
  const transferRecord = schema.recordTypes[operations.Transfer];
  const repayRecord = schema.recordTypes[operations.Repay];
  if (!isRecord(transferRecord)) return bindReject({ operation: 'Transfer' });
  if (!isRecord(repayRecord)) return bindReject({ operation: 'Repay' });
  // Source schema cannot name a field `amount`: that spelling is reserved as a
  // financial generic. transferAmount is the Amount<asset> operand; the kernel
  // Transfer still uses amount.
  const amount = transferRecord.transferAmount;
  if (!Array.isArray(amount) || amount.length !== 2 || amount[0] !== 'Amount' || typeof amount[1] !== 'string') {
    return bindReject({ operation: 'Transfer', field: 'transferAmount' });
  }
  const transferExpected = {
    from: ['Text'],
    id: ['Text'],
    settlementAsset: ['Text'],
    to: ['Text'],
    transferAmount: amount,
  };
  const transferField = mismatchedField(transferRecord, transferExpected);
  if (transferField !== undefined) return bindReject({ operation: 'Transfer', field: transferField });
  const nominal = repayRecord.nominalAmount;
  if (!Array.isArray(nominal) || nominal.length !== 3 || nominal[0] !== 'Quantity' || nominal[2] !== '0') {
    return bindReject({ operation: 'Repay', field: 'nominalAmount' });
  }
  const units = nominal[1];
  if (!Array.isArray(units) || units.length !== 1 || !Array.isArray(units[0]) || units[0].length !== 2
    || typeof units[0][0] !== 'string' || units[0][1] !== '1') {
    return bindReject({ operation: 'Repay', field: 'nominalAmount' });
  }
  const repayExpected = {
    allocationId: ['Text'],
    nominalAmount: nominal,
    obligationId: ['Text'],
    payer: ['Text'],
    transferId: ['Text'],
  };
  const repayField = mismatchedField(repayRecord, repayExpected);
  if (repayField !== undefined) return bindReject({ operation: 'Repay', field: repayField });
  return { transferAsset: amount[1], repayDenomination: units[0][0] };
}

const CONVERSION_FIELDS = {
  mantissa: ['UInt128'],
  rounding: ['Text'],
  scale: ['UInt128'],
};
const ACCRUAL_TERMS_FIELDS = {
  denominator: ['UInt128'],
  firstPeriodStart: ['UInt64'],
  numerator: ['UInt128'],
  periodSeconds: ['UInt64'],
  rounding: ['Text'],
};

function nestedRecord(schema: Schema, value: unknown): Record<string, unknown> | undefined {
  if (!Array.isArray(value) || value.length !== 2 || value[0] !== 'Record' || typeof value[1] !== 'string') {
    return undefined;
  }
  const record = schema.recordTypes[value[1]];
  return isRecord(record) ? record : undefined;
}

function quantityUnits(nominal: unknown): string | undefined {
  if (!Array.isArray(nominal) || nominal.length !== 3 || nominal[0] !== 'Quantity' || nominal[2] !== '0') {
    return undefined;
  }
  const units = nominal[1];
  if (!Array.isArray(units) || units.length !== 1 || !Array.isArray(units[0]) || units[0].length !== 2
    || typeof units[0][0] !== 'string' || units[0][1] !== '1') {
    return undefined;
  }
  return units[0][0];
}

/** Structural Transfer/Repay/Originate/Accrue binding for source/5. */
export function lifecycleOperationBinding(schema: Schema): Binding | OperationBindingFailure {
  const operations = schema.operations;
  if (!isRecord(operations)) return bindReject();
  const names = Object.keys(operations);
  const allowed = new Set(['Transfer', 'Repay', 'Originate', 'Accrue']);
  const extra = names.find((name) => !allowed.has(name));
  if (names.length !== 4
    || typeof operations.Transfer !== 'string' || typeof operations.Repay !== 'string'
    || typeof operations.Originate !== 'string' || typeof operations.Accrue !== 'string') {
    if (extra !== undefined) return bindReject({ operation: extra });
    if (typeof operations.Transfer !== 'string') return bindReject({ operation: 'Transfer' });
    if (typeof operations.Repay !== 'string') return bindReject({ operation: 'Repay' });
    if (typeof operations.Originate !== 'string') return bindReject({ operation: 'Originate' });
    if (typeof operations.Accrue !== 'string') return bindReject({ operation: 'Accrue' });
    return bindReject();
  }
  const base = operationBinding({
    ...schema,
    operations: { Transfer: operations.Transfer, Repay: operations.Repay },
    recordTypes: schema.recordTypes,
  });
  if ('status' in base) return base;
  const originateRecord = schema.recordTypes[operations.Originate];
  const accrueRecord = schema.recordTypes[operations.Accrue];
  if (!isRecord(originateRecord)) return bindReject({ operation: 'Originate' });
  if (!isRecord(accrueRecord)) return bindReject({ operation: 'Accrue' });
  const conversionRecord = nestedRecord(schema, originateRecord.conversion);
  if (conversionRecord === undefined) return bindReject({ operation: 'Originate', field: 'conversion' });
  const conversionField = mismatchedField(conversionRecord, CONVERSION_FIELDS);
  if (conversionField !== undefined) return bindReject({ operation: 'Originate', field: 'conversion' });
  const termsRecord = nestedRecord(schema, originateRecord.accrualTerms);
  if (termsRecord === undefined) return bindReject({ operation: 'Originate', field: 'accrualTerms' });
  const termsField = mismatchedField(termsRecord, ACCRUAL_TERMS_FIELDS);
  if (termsField !== undefined) return bindReject({ operation: 'Originate', field: 'accrualTerms' });
  const repayRecord = schema.recordTypes[operations.Repay] as Record<string, unknown>;
  const originateExpected = {
    accrualTerms: originateRecord.accrualTerms,
    allocationRule: ['Text'],
    conversion: originateRecord.conversion,
    creditor: ['Text'],
    debtor: ['Text'],
    denomination: ['Text'],
    nominalAmount: repayRecord.nominalAmount,
    nominalLiabilityCap: repayRecord.nominalAmount,
    obligationId: ['Text'],
    originationId: ['Text'],
    settlementAsset: ['Text'],
    transferId: ['Text'],
  };
  const originateField = mismatchedField(originateRecord, originateExpected);
  if (originateField !== undefined) return bindReject({ operation: 'Originate', field: originateField });
  const accrueExpected = {
    accrualId: ['Text'],
    obligationId: ['Text'],
    observedTime: ['UInt64'],
    periodIndex: ['UInt64'],
  };
  const accrueField = mismatchedField(accrueRecord, accrueExpected);
  if (accrueField !== undefined) return bindReject({ operation: 'Accrue', field: accrueField });
  if (base.transferAsset !== base.repayDenomination) {
    return bindReject({ operation: 'Originate', field: 'nominalAmount' });
  }
  if (quantityUnits(repayRecord.nominalAmount) !== base.repayDenomination) {
    return bindReject({ operation: 'Repay', field: 'nominalAmount' });
  }
  return base;
}

function textField(fields: Record<string, unknown>, name: string): string | undefined {
  const value = fields[name];
  return typeof value === 'string' ? value : undefined;
}

function mapDescriptors(
  descriptors: unknown[],
  binding: Binding,
): { ok: true; actions: Action[] } | { ok: false; result: FundedExpressionRejected } {
  if (descriptors.length === 0) return { ok: false, result: reject('EMPTY_BATCH') };
  const actions: Action[] = [];
  for (const descriptor of descriptors) {
    if (!isRecord(descriptor) || typeof descriptor.operation !== 'string' || !isRecord(descriptor.fields)) {
      return { ok: false, result: reject('OPERATION_BINDING') };
    }
    const fields = descriptor.fields;
    if (descriptor.operation === 'Transfer') {
      const id = textField(fields, 'id');
      const from = textField(fields, 'from');
      const to = textField(fields, 'to');
      const settlementAsset = textField(fields, 'settlementAsset');
      const amount = unsignedDecimal(fields.transferAmount);
      if (id === undefined || from === undefined || to === undefined || settlementAsset === undefined
        || amount === undefined || Object.keys(fields).length !== 5) {
        return { ok: false, result: reject('OPERATION_BINDING') };
      }
      if (settlementAsset !== binding.transferAsset) return { ok: false, result: reject('SETTLEMENT_UNIT') };
      actions.push({
        kind: 'Transfer',
        id,
        from,
        to,
        asset: settlementAsset,
        amount: fields.transferAmount as string,
      });
    } else if (descriptor.operation === 'Repay') {
      const allocationId = textField(fields, 'allocationId');
      const transferId = textField(fields, 'transferId');
      const obligationId = textField(fields, 'obligationId');
      const payer = textField(fields, 'payer');
      const nominal = signedDecimal(fields.nominalAmount);
      if (allocationId === undefined || transferId === undefined || obligationId === undefined
        || payer === undefined || nominal === undefined || Object.keys(fields).length !== 5) {
        return { ok: false, result: reject('OPERATION_BINDING') };
      }
      if (nominal < 0n || nominal > SIGNED128_MAX) return { ok: false, result: reject('NOMINAL_RANGE') };
      actions.push({
        kind: 'Repay',
        allocationId,
        transferId,
        obligationId,
        payer,
        nominalAmount: fields.nominalAmount as string,
      });
    } else {
      return { ok: false, result: reject('UNSUPPORTED_OPERATION') };
    }
  }
  return { ok: true, actions };
}

export function debitExpressionWork(
  state: Record<string, unknown>,
  expressionUsed: bigint,
): { ok: true; state: Record<string, unknown> } | { ok: false; result: FundedExpressionRejected } {
  if (!isRecord(state.work)) return { ok: false, result: reject('INPUT_SCHEMA') };
  const remaining = unsignedDecimal(state.work.remaining);
  const spent = unsignedDecimal(state.work.spent);
  if (remaining === undefined || spent === undefined) return { ok: false, result: reject('INPUT_SCHEMA') };
  const nextRemaining = remaining - expressionUsed;
  const nextSpent = spent + expressionUsed;
  if (nextRemaining < 0n || nextSpent > UINT128_MAX) return { ok: false, result: reject('OVERFLOW') };
  const next = JSON.parse(JSON.stringify(state)) as Record<string, unknown>;
  const work = next.work as Record<string, unknown>;
  work.remaining = nextRemaining.toString(10);
  work.spent = nextSpent.toString(10);
  return { ok: true, state: next };
}

function nominalUnitsMatch(post: RepaymentState, actions: Action[], denomination: string): boolean {
  for (const action of actions) {
    if (action.kind !== 'Repay') continue;
    const obligation = post.obligations.find((item) => item.id === action.obligationId);
    if (obligation === undefined || obligation.denomination !== denomination) return false;
  }
  return true;
}

/** Trusted Σ is immutable text. evaluate owns source, snapshot and repayment JSON. */
export function createFundedFinancialExpressionSourceV1(schemaCanonicalJSON: string) {
  const inner = createFinancialExpressionSourceV1(schemaCanonicalJSON);
  return Object.freeze({
    evaluate(
      source: string,
      snapshotCanonicalJSON: string,
      repaymentStateJSON: string,
    ): FundedExpressionResult {
      const parsedState = parseOwnedState(repaymentStateJSON);
      if (!parsedState.ok) return parsedState.result;
      const remainingText = (parsedState.value.work as Record<string, unknown>).remaining as string;
      const workInitial = typeof snapshotCanonicalJSON === 'string'
        ? snapshotWorkInitial(snapshotCanonicalJSON) : undefined;
      if (workInitial !== undefined && workInitial !== remainingText) return reject('WORK_MISMATCH');
      const evaluated = inner.evaluate(source, snapshotCanonicalJSON);
      return completeFundedPreparation(
        evaluated,
        schemaCanonicalJSON,
        parsedState.value,
        remainingText,
        workInitial,
      );
    },
  });
}

/** Kernel binding and one repayment call after a successful expression evaluation. */
export function completeFundedPreparation(
  evaluated: ExpressionResult,
  schemaCanonicalJSON: string,
  ownedState: Record<string, unknown>,
  remainingText: string,
  workInitial: string | undefined,
): FundedExpressionResult {
  if ('status' in evaluated && evaluated.status === 'Rejected') return evaluated as SourceRejected;
  if (!('status' in evaluated) || evaluated.status !== 'ExpressionPrepared') return reject('INPUT_SCHEMA');
  let schema: Schema;
  try { schema = parseCanonical(schemaCanonicalJSON, STATE_BYTES); }
  catch { return reject('INPUT_SCHEMA'); }
  for (const field of Object.values(schema.fields)) {
    if (isRecord(field) && field.writeClass === 'financial') return reject('TYPE_FINANCIAL_WRITE');
  }
  const binding = operationBinding(schema);
  if ('status' in binding) return reject(binding.code);
  const mapped = mapDescriptors(evaluated.descriptors, binding);
  if (!mapped.ok) return mapped.result;
  const used = unsignedDecimal(workInitial ?? remainingText);
  const remainingAfterExpression = unsignedDecimal(evaluated.workRemaining);
  if (used === undefined || remainingAfterExpression === undefined) return reject('INPUT_SCHEMA');
  const expressionUsed = used - remainingAfterExpression;
  if (expressionUsed < 0n) return reject('INPUT_SCHEMA');
  const debited = debitExpressionWork(ownedState, expressionUsed);
  if (!debited.ok) return debited.result;
  const prepared = prepareRepayment(JSON.stringify({
    schemaVersion: REPAYMENT_VERSION,
    state: debited.state,
    actions: mapped.actions,
  }));
  if (prepared.status === 'Rejected') return prepared;
  if (!nominalUnitsMatch(prepared.post, mapped.actions, binding.repayDenomination)) {
    return reject('NOMINAL_UNIT');
  }
  return {
    status: 'FundedExpressionPrepared',
    post: evaluated.post,
    financialPost: prepared.post,
    effects: prepared.effects,
    workRemaining: prepared.post.work.remaining,
  };
}

/** Prefix descriptors, debit prefix work, then one kernel preparation for Core /3. */
export function prepareStagedKernel(
  descriptors: unknown[],
  schemaCanonicalJSON: string,
  ownedState: Record<string, unknown>,
  prefixUsed: bigint,
): { ok: true; post: RepaymentState; effects: Effect[] } | { ok: false; result: FundedExpressionRejected } {
  if (descriptors.length === 0) return { ok: false, result: reject('EMPTY_BATCH') };
  let schema: Schema;
  try { schema = parseCanonical(schemaCanonicalJSON, STATE_BYTES); }
  catch { return { ok: false, result: reject('INPUT_SCHEMA') }; }
  for (const field of Object.values(schema.fields)) {
    if (isRecord(field) && field.writeClass === 'financial') return { ok: false, result: reject('TYPE_FINANCIAL_WRITE') };
  }
  const binding = operationBinding(schema);
  if ('status' in binding) return { ok: false, result: reject(binding.code) };
  const mapped = mapDescriptors(descriptors, binding);
  if (!mapped.ok) return mapped;
  const debited = debitExpressionWork(ownedState, prefixUsed);
  if (!debited.ok) return debited;
  const prepared = prepareRepayment(JSON.stringify({
    schemaVersion: REPAYMENT_VERSION,
    state: debited.state,
    actions: mapped.actions,
  }));
  if (prepared.status === 'Rejected') return { ok: false, result: prepared };
  if (!nominalUnitsMatch(prepared.post, mapped.actions, binding.repayDenomination)) {
    return { ok: false, result: reject('NOMINAL_UNIT') };
  }
  return { ok: true, post: prepared.post, effects: prepared.effects };
}

const UINT64_MAX = 18446744073709551615n;

function nestedStringRecord(value: unknown, names: readonly string[]): Record<string, string> | undefined {
  if (!isRecord(value) || Object.keys(value).length !== names.length) return undefined;
  const out: Record<string, string> = {};
  for (const name of names) {
    if (typeof value[name] !== 'string') return undefined;
    out[name] = value[name] as string;
  }
  return out;
}

function mapLifecycleDescriptors(
  descriptors: unknown[],
  binding: Binding,
): { ok: true; actions: LifecycleAction[] } | { ok: false; result: FundedExpressionRejected } {
  if (descriptors.length === 0) return { ok: false, result: reject('EMPTY_BATCH') };
  const actions: LifecycleAction[] = [];
  for (const descriptor of descriptors) {
    if (!isRecord(descriptor) || typeof descriptor.operation !== 'string' || !isRecord(descriptor.fields)) {
      return { ok: false, result: reject('OPERATION_BINDING') };
    }
    const fields = descriptor.fields;
    if (descriptor.operation === 'Transfer' || descriptor.operation === 'Repay') {
      const mapped = mapDescriptors([descriptor], binding);
      if (!mapped.ok) return mapped;
      actions.push(mapped.actions[0] as LifecycleAction);
    } else if (descriptor.operation === 'Originate') {
      const obligationId = textField(fields, 'obligationId');
      const transferId = textField(fields, 'transferId');
      const originationId = textField(fields, 'originationId');
      const debtor = textField(fields, 'debtor');
      const creditor = textField(fields, 'creditor');
      const denomination = textField(fields, 'denomination');
      const settlementAsset = textField(fields, 'settlementAsset');
      const allocationRule = textField(fields, 'allocationRule');
      const nominal = signedDecimal(fields.nominalAmount);
      const cap = signedDecimal(fields.nominalLiabilityCap);
      const conversion = nestedStringRecord(fields.conversion, ['mantissa', 'scale', 'rounding']);
      const accrualTerms = nestedStringRecord(fields.accrualTerms, [
        'numerator', 'denominator', 'rounding', 'periodSeconds', 'firstPeriodStart',
      ]);
      if (obligationId === undefined || transferId === undefined || originationId === undefined
        || debtor === undefined || creditor === undefined || denomination === undefined
        || settlementAsset === undefined || allocationRule === undefined
        || nominal === undefined || cap === undefined || conversion === undefined
        || accrualTerms === undefined || Object.keys(fields).length !== 12) {
        return { ok: false, result: reject('OPERATION_BINDING') };
      }
      if (nominal < 0n || nominal > SIGNED128_MAX || cap < 0n || cap > SIGNED128_MAX) {
        return { ok: false, result: reject('NOMINAL_RANGE') };
      }
      if (denomination !== binding.repayDenomination) return { ok: false, result: reject('NOMINAL_UNIT') };
      if (settlementAsset !== binding.transferAsset) return { ok: false, result: reject('SETTLEMENT_UNIT') };
      actions.push({
        kind: 'Originate',
        obligationId,
        transferId,
        originationId,
        debtor,
        creditor,
        nominalAmount: fields.nominalAmount as string,
        denomination,
        settlementAsset,
        conversion,
        allocationRule,
        accrualTerms,
        nominalLiabilityCap: fields.nominalLiabilityCap as string,
      } as unknown as LifecycleAction);
    } else if (descriptor.operation === 'Accrue') {
      const accrualId = textField(fields, 'accrualId');
      const obligationId = textField(fields, 'obligationId');
      const periodIndex = unsignedDecimal(fields.periodIndex);
      const observedTime = unsignedDecimal(fields.observedTime);
      if (accrualId === undefined || obligationId === undefined || periodIndex === undefined
        || observedTime === undefined || Object.keys(fields).length !== 4) {
        return { ok: false, result: reject('OPERATION_BINDING') };
      }
      if (periodIndex > UINT64_MAX || observedTime > UINT64_MAX) {
        return { ok: false, result: reject('OPERATION_BINDING') };
      }
      actions.push({
        kind: 'Accrue',
        accrualId,
        obligationId,
        periodIndex: fields.periodIndex as string,
        observedTime: fields.observedTime as string,
      });
    } else {
      return { ok: false, result: reject('UNSUPPORTED_OPERATION') };
    }
  }
  return { ok: true, actions };
}

function lifecycleNominalUnitsMatch(
  post: LifecycleState,
  actions: LifecycleAction[],
  denomination: string,
): boolean {
  for (const action of actions) {
    if (action.kind === 'Repay' || action.kind === 'Accrue') {
      const obligation = post.obligations.find((item) => item.id === action.obligationId);
      if (obligation === undefined || obligation.denomination !== denomination) return false;
    }
    if (action.kind === 'Originate' && action.denomination !== denomination) return false;
  }
  return true;
}

export function prepareStagedLifecycleKernel(
  descriptors: unknown[],
  schemaCanonicalJSON: string,
  ownedState: Record<string, unknown>,
  prefixUsed: bigint,
): { ok: true; post: LifecycleState; effects: LifecycleEffect[] } | { ok: false; result: LifecycleExpressionRejected } {
  if (descriptors.length === 0) return { ok: false, result: reject('EMPTY_BATCH') };
  let schema: Schema;
  try { schema = parseCanonical(schemaCanonicalJSON, STATE_BYTES); }
  catch { return { ok: false, result: reject('INPUT_SCHEMA') }; }
  for (const field of Object.values(schema.fields)) {
    if (isRecord(field) && field.writeClass === 'financial') return { ok: false, result: reject('TYPE_FINANCIAL_WRITE') };
  }
  const binding = lifecycleOperationBinding(schema);
  if ('status' in binding) return { ok: false, result: reject(binding.code) };
  const mapped = mapLifecycleDescriptors(descriptors, binding);
  if (!mapped.ok) return mapped;
  const debited = debitExpressionWork(ownedState, prefixUsed);
  if (!debited.ok) return debited;
  const prepared = prepareFinancialLifecycle(JSON.stringify({
    schemaVersion: LIFECYCLE_VERSION,
    state: debited.state,
    actions: mapped.actions,
  }));
  if (prepared.status === 'Rejected') return { ok: false, result: prepared };
  if (!lifecycleNominalUnitsMatch(prepared.post, mapped.actions, binding.repayDenomination)) {
    return { ok: false, result: reject('NOMINAL_UNIT') };
  }
  return { ok: true, post: prepared.post, effects: prepared.effects };
}

export function lifecycleResultBound(
  state: Record<string, unknown>,
): LifecycleExpressionRejected | null {
  const compact = JSON.stringify(state);
  if (bytes(compact) > STATE_BYTES) {
    return { status: 'Rejected', code: 'RESULT_BOUND', actionIndex: null };
  }
  const admitted = admitFinancialLifecycleStateJSON(compact);
  if (!admitted.ok) return admitted.result;
  return null;
}
