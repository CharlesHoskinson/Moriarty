import { elaborateSuccessorSource } from './elaborate.ts';
import { FUNDED_SOURCE_VERSION, FundedSourceError, fail, identifier, sameType, uint128 } from './core.ts';
import type { CoreExpression, CoreType, CoreValue, FundedCore } from './core.ts';
import { SuccessorSyntaxError } from './frontend.ts';
import { prepareRepayment, REPAYMENT_VERSION } from './repayment.ts';
import type { Action, RepaymentResult } from './repayment.ts';

export const INVOCATION_UTF8_BYTES = 65536;
function closed(value: unknown, keys: string[]): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) fail('INVOCATION_SCHEMA');
  const record = value as Record<string, unknown>;
  if (Object.keys(record).length !== keys.length || keys.some(k => !Object.hasOwn(record, k))) fail('INVOCATION_SCHEMA');
  return record;
}
function argument(value: unknown, expected: CoreType, core: FundedCore): CoreValue {
  const arg = closed(value, ['type', 'value']);
  const t = closed(arg.type, 'name' in expected ? ['kind', 'name'] : ['kind']);
  if (!sameType(t as unknown as CoreType, expected)) fail('ARGUMENT_TYPE');
  if (expected.kind === 'Bool') {
    if (typeof arg.value !== 'boolean') fail('ARGUMENT_VALUE');
  } else if (expected.kind === 'Debt' || expected.kind === 'Amount' || expected.kind === 'UInt') {
    if (!uint128(arg.value)) fail('UINT128');
  } else {
    if (!identifier(arg.value)) fail('INVALID_IDENTIFIER');
    if (expected.kind === 'Party' && !core.parties.includes(arg.value)) fail('UNKNOWN_PARTY');
    if (expected.kind === 'Asset' && !core.assets.some(a => a.name === arg.value)) fail('UNKNOWN_ASSET');
  }
  return { type: expected, value: arg.value as string | boolean };
}

/** Source-only execution: caller-supplied AST/Core and live JS objects are never admitted. */
export function prepareSuccessor(source: string, invocationJson: string): RepaymentResult {
  try {
    const core = elaborateSuccessorSource(source);
    if (typeof invocationJson !== 'string') fail('INVOCATION_TYPE');
    if (invocationJson.length > INVOCATION_UTF8_BYTES || new TextEncoder().encode(invocationJson).length > INVOCATION_UTF8_BYTES) fail('INVOCATION_BOUND');
    let parsed: unknown;
    try { parsed = JSON.parse(invocationJson); } catch { return { status: 'Rejected', code: 'INVOCATION_JSON', actionIndex: null }; }
    const input = closed(parsed, ['schemaVersion', 'action', 'arguments', 'state']);
    if (input.schemaVersion !== FUNDED_SOURCE_VERSION) fail('INVOCATION_PROFILE');
    const action = core.actions.find(a => a.name === input.action);
    if (!action) fail('UNKNOWN_ACTION');
    const args = closed(input.arguments, action.parameters.map(p => p.name));
    const values = new Map(action.parameters.map(p => [p.name, argument(args[p.name], p.type, core)]));
    function resolve(e: CoreExpression): CoreValue {
      return e.tag === 'Literal' ? { type: e.type, value: e.value } : values.get(e.name)!;
    }
    const actions: Action[] = [];
    const nominalBindings: { index: number; obligationId: string; type: CoreType }[] = [];
    for (let index = 0; index < action.emissions.length; index++) {
      const emission = action.emissions[index];
      const fields = new Map(emission.fields.map(f => [f.name, resolve(f.expression)]));
      const value = (name: string): string => fields.get(name)!.value as string;
      if (emission.kind === 'Transfer') {
        const amountType = fields.get('amount')!.type;
        if (!('name' in amountType) || amountType.name !== value('settlementAsset')) return { status: 'Rejected', code: 'SETTLEMENT_UNIT', actionIndex: index };
        actions.push({ kind: 'Transfer', id: value('id'), from: value('from'), to: value('to'), asset: value('settlementAsset'), amount: value('amount') });
      } else {
        const nominalType = fields.get('nominalAmount')!.type;
        nominalBindings.push({ index, obligationId: value('obligationId'), type: nominalType });
        actions.push({ kind: 'Repay', allocationId: value('allocationId'), transferId: value('transferId'), obligationId: value('obligationId'), payer: value('payer'), nominalAmount: value('nominalAmount') });
      }
    }
    // Tentative local execution validates the complete supplied projection. No candidate is
    // exposed until the source nominal witnesses match the retained obligation metadata.
    const result = prepareRepayment(JSON.stringify({ schemaVersion: REPAYMENT_VERSION, state: input.state, actions }));
    if (result.status === 'Rejected') return result;
    for (const binding of nominalBindings) {
      const obligation = result.post.obligations.find(o => o.id === binding.obligationId)!;
      if (!('name' in binding.type) || binding.type.name !== obligation.denomination) return { status: 'Rejected', code: 'NOMINAL_UNIT', actionIndex: binding.index };
    }
    return result;
  } catch (error) {
    if (error instanceof FundedSourceError || error instanceof SuccessorSyntaxError) return { status: 'Rejected', code: error.code, actionIndex: null };
    return { status: 'Rejected', code: 'SOURCE_INTERNAL', actionIndex: null };
  }
}
