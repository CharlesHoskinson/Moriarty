import { parseSuccessorSource } from './frontend.ts';
import type { Expression, TypeNode } from './frontend.ts';
import { FUNDED_SOURCE_VERSION, fail, identifier, uint128 } from './core.ts';
import type { CoreAction, CoreEmission, CoreExpression, CoreType, FundedCore } from './core.ts';

const SIMPLE = ['UInt', 'Bool', 'Party', 'Asset', 'TransferId', 'AllocationId', 'ObligationId'];
const IDS = ['TransferId', 'AllocationId', 'ObligationId'];
const RESERVED = new Set([...SIMPLE, 'Debt', 'Amount', 'Transfer', 'Repay', 'debt', 'amount']);
const TRANSFER = ['id', 'from', 'to', 'settlementAsset', 'amount'];
const REPAY = ['allocationId', 'transferId', 'obligationId', 'payer', 'nominalAmount'];

/** Parse and typecheck the entire source, including unselected actions. No AST/Core input API. */
export function elaborateSuccessorSource(source: string): FundedCore {
  const program = parseSuccessorSource(source);
  const units = new Set<string>();
  const terms = new Set<string>();
  const parties = new Set<string>();
  const assets = new Map<string, string>();
  const globals = new Map<string, CoreExpression>();
  const declarations = program.agreement.declarations;
  function declare(names: Set<string>, name: string): void {
    if (RESERVED.has(name)) fail('RESERVED_NAME');
    if (names.has(name)) fail('DUPLICATE_NAME');
    names.add(name);
  }
  for (const d of declarations) {
    if (d.tag === 'UnitDecl') declare(units, d.name);
    else if (d.tag === 'PartyDecl' || d.tag === 'AssetDecl' || d.tag === 'ActionDecl') declare(terms, d.name);
    else fail('UNSUPPORTED_DECLARATION');
  }
  for (const d of declarations) {
    if (d.tag === 'PartyDecl') {
      parties.add(d.name);
      globals.set(d.name, { tag: 'Literal', type: { kind: 'Party' }, value: d.name });
    } else if (d.tag === 'AssetDecl') {
      const t = d.type;
      const unit = t.arguments[0];
      if (t.name !== 'Asset' || t.arguments.length !== 1 || !unit || unit.arguments.length !== 0 || !units.has(unit.name)) fail('UNSUPPORTED_TYPE');
      assets.set(d.name, unit.name);
      globals.set(d.name, { tag: 'Literal', type: { kind: 'Asset' }, value: d.name });
    }
  }
  function type(t: TypeNode): CoreType {
    if ((t.name === 'Debt' || t.name === 'Amount') && t.arguments.length === 1) {
      const n = t.arguments[0];
      if (n.arguments.length || !(t.name === 'Debt' ? units.has(n.name) : assets.has(n.name))) fail('UNKNOWN_UNIT');
      return { kind: t.name, name: n.name };
    }
    if (SIMPLE.includes(t.name) && t.arguments.length === 0) return { kind: t.name } as CoreType;
    return fail('UNSUPPORTED_TYPE');
  }
  function expression(e: Expression, env: Map<string, CoreExpression>): CoreExpression {
    if (e.tag === 'Identifier') return env.get(e.name) ?? fail('UNKNOWN_NAME');
    if (e.tag === 'IntegerLiteral') {
      if (!uint128(e.value)) fail('UINT128');
      return { tag: 'Literal', type: { kind: 'UInt' }, value: e.value };
    }
    if (e.tag === 'BooleanLiteral') return { tag: 'Literal', type: { kind: 'Bool' }, value: e.value };
    if (e.tag === 'Call') {
      if (IDS.includes(e.name) && e.arguments.length === 1) {
        const arg = e.arguments[0];
        if (arg.tag !== 'StringLiteral' || !identifier(arg.decoded)) fail('INVALID_IDENTIFIER');
        return { tag: 'Literal', type: { kind: e.name } as CoreType, value: arg.decoded };
      }
      if ((e.name === 'debt' || e.name === 'amount') && e.arguments.length === 2) {
        const [value, name] = e.arguments;
        if (value.tag !== 'IntegerLiteral' || !uint128(value.value)) fail('UINT128');
        if (name.tag !== 'Identifier' || !(e.name === 'debt' ? units.has(name.name) : assets.has(name.name))) fail('UNKNOWN_UNIT');
        return { tag: 'Literal', type: { kind: e.name === 'debt' ? 'Debt' : 'Amount', name: name.name }, value: value.value };
      }
    }
    return fail('UNSUPPORTED_EXPRESSION');
  }
  const actions: CoreAction[] = [];
  for (const d of declarations) {
    if (d.tag !== 'ActionDecl') continue;
    if (d.postconditions.length) fail('UNSUPPORTED_STATEMENT');
    const env = new Map(globals);
    const names = new Set(terms);
    const parameters = d.parameters.map(p => {
      declare(names, p.name);
      const t = type(p.type);
      env.set(p.name, { tag: 'Parameter', name: p.name, type: t });
      return { name: p.name, type: t };
    });
    if (d.statements.length > 128) fail('EMISSION_BOUND');
    const emissions: CoreEmission[] = d.statements.map(s => {
      if (s.tag !== 'Emit') return fail('UNSUPPORTED_STATEMENT');
      if (s.type.arguments.length || (s.type.name !== 'Transfer' && s.type.name !== 'Repay')) return fail('UNSUPPORTED_EFFECT');
      const kind = s.type.name;
      const keys = kind === 'Transfer' ? TRANSFER : REPAY;
      const fields = new Map<string, CoreExpression>();
      for (const f of s.fields) {
        if (!keys.includes(f.name)) fail('UNKNOWN_FIELD');
        if (fields.has(f.name)) fail('DUPLICATE_FIELD');
        fields.set(f.name, expression(f.expression, env));
      }
      if (fields.size !== keys.length) fail('MISSING_FIELD');
      for (const name of keys) {
        const expr = fields.get(name)!;
        const expected = name === 'id' || name === 'transferId' ? 'TransferId'
          : name === 'allocationId' ? 'AllocationId' : name === 'obligationId' ? 'ObligationId'
          : name === 'settlementAsset' ? 'Asset' : name === 'amount' ? 'Amount'
          : name === 'nominalAmount' ? 'Debt' : 'Party';
        if (expr.type.kind !== expected) fail('FIELD_TYPE');
      }
      return { kind, fields: keys.map(name => ({ name, expression: fields.get(name)! })) };
    });
    actions.push({ name: d.name, parameters, emissions });
  }
  return { schemaVersion: FUNDED_SOURCE_VERSION, agreement: program.agreement.name,
    units: [...units], parties: [...parties], assets: [...assets].map(([name, unit]) => ({ name, unit })), actions };
}
