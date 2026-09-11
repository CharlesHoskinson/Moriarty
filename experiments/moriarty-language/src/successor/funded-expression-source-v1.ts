/** Local funded adapter over financial expression source and the repayment kernel.
 * Ordinary writes publish only after descriptor binding and one kernel call succeed. */
import { createFinancialExpressionSourceV1 } from './financial-expression-source-v1.ts';
import type { ExpressionResult } from './financial-expression-v1.ts';
import { ExpressionFailure, bytes, parseCanonical } from './expression-wire-v1.ts';
import { same } from './financial-expression-types-v1.ts';
import type { Schema, ValueType } from './financial-expression-types-v1.ts';
import { prepareRepayment, REPAYMENT_BOUNDS, REPAYMENT_VERSION } from './repayment.ts';
import type { Action, Effect, RejectedRepayment, RepaymentState } from './repayment.ts';

const STATE_BYTES = 65536;
const SNAPSHOT_BYTES = 2_000_000;
const SIGNED128_MAX = (1n << 127n) - 1n;
const UINT128_MAX = BigInt(REPAYMENT_BOUNDS.uint128Max);

export interface FundedExpressionPrepared {
  status: 'FundedExpressionPrepared';
  post: Record<string, unknown>;
  financialPost: RepaymentState;
  effects: Effect[];
  workRemaining: string;
}

export type FundedExpressionRejected =
  | { status: 'Rejected'; code: string }
  | SourceRejected
  | RejectedRepayment;
export type FundedExpressionResult = FundedExpressionPrepared | FundedExpressionRejected;

type SourceRejected = Extract<ExpressionResult, { status: 'Rejected' }>;
type Binding = { transferAsset: string; repayDenomination: string };

function reject(code: string): FundedExpressionRejected {
  return { status: 'Rejected', code };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function recordFields(value: unknown, expected: Record<string, unknown>): boolean {
  if (!isRecord(value)) return false;
  const keys = Object.keys(expected);
  if (Object.keys(value).length !== keys.length) return false;
  return keys.every((key) => Object.hasOwn(value, key) && same(value[key] as ValueType, expected[key] as ValueType));
}

function parseOwnedState(text: unknown): { ok: true; value: Record<string, unknown> } | { ok: false; result: FundedExpressionRejected } {
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

function snapshotWorkInitial(text: string): string | undefined {
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

function operationBinding(schema: Schema): Binding | FundedExpressionRejected {
  const operations = schema.operations;
  if (!isRecord(operations) || Object.keys(operations).length !== 2
    || typeof operations.Transfer !== 'string' || typeof operations.Repay !== 'string') {
    return reject('OPERATION_BINDING');
  }
  const transferRecord = schema.recordTypes[operations.Transfer];
  const repayRecord = schema.recordTypes[operations.Repay];
  if (!isRecord(transferRecord) || !isRecord(repayRecord)) return reject('OPERATION_BINDING');
  // Source schema cannot name a field `amount`: that spelling is reserved as a
  // financial generic. transferAmount is the Amount<asset> operand; the kernel
  // Transfer still uses amount.
  const amount = transferRecord.transferAmount;
  if (!Array.isArray(amount) || amount.length !== 2 || amount[0] !== 'Amount' || typeof amount[1] !== 'string') {
    return reject('OPERATION_BINDING');
  }
  const transferExpected = {
    from: ['Text'],
    id: ['Text'],
    settlementAsset: ['Text'],
    to: ['Text'],
    transferAmount: amount,
  };
  if (!recordFields(transferRecord, transferExpected)) return reject('OPERATION_BINDING');
  const nominal = repayRecord.nominalAmount;
  if (!Array.isArray(nominal) || nominal.length !== 3 || nominal[0] !== 'Quantity' || nominal[2] !== '0') {
    return reject('OPERATION_BINDING');
  }
  const units = nominal[1];
  if (!Array.isArray(units) || units.length !== 1 || !Array.isArray(units[0]) || units[0].length !== 2
    || typeof units[0][0] !== 'string' || units[0][1] !== '1') {
    return reject('OPERATION_BINDING');
  }
  const repayExpected = {
    allocationId: ['Text'],
    nominalAmount: nominal,
    obligationId: ['Text'],
    payer: ['Text'],
    transferId: ['Text'],
  };
  if (!recordFields(repayRecord, repayExpected)) return reject('OPERATION_BINDING');
  return { transferAsset: amount[1], repayDenomination: units[0][0] };
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

function debitExpressionWork(
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
      if ('status' in evaluated && evaluated.status === 'Rejected') return evaluated as SourceRejected;
      if (!('status' in evaluated) || evaluated.status !== 'ExpressionPrepared') return reject('INPUT_SCHEMA');
      let schema: Schema;
      try { schema = parseCanonical(schemaCanonicalJSON, STATE_BYTES); }
      catch { return reject('INPUT_SCHEMA'); }
      for (const field of Object.values(schema.fields)) {
        if (isRecord(field) && field.writeClass === 'financial') return reject('TYPE_FINANCIAL_WRITE');
      }
      const binding = operationBinding(schema);
      if ('status' in binding) return binding;
      const mapped = mapDescriptors(evaluated.descriptors, binding);
      if (!mapped.ok) return mapped.result;
      const used = unsignedDecimal(workInitial ?? remainingText);
      const remainingAfterExpression = unsignedDecimal(evaluated.workRemaining);
      if (used === undefined || remainingAfterExpression === undefined) return reject('INPUT_SCHEMA');
      const expressionUsed = used - remainingAfterExpression;
      if (expressionUsed < 0n) return reject('INPUT_SCHEMA');
      const debited = debitExpressionWork(parsedState.value, expressionUsed);
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
    },
  });
}
