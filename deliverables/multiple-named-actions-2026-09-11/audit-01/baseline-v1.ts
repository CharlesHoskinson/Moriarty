/** Source-defined financial agreement: declarations compile to the existing schema. */
import {
  FINANCIAL_AGREEMENT_SOURCE_PROFILE,
  parseSuccessorFinancialAgreementSource,
  SuccessorSyntaxError,
} from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/frontend.ts';
import type {
  ActionDecl,
  AssetDecl,
  Declaration,
  OperationDecl,
  PartyDecl,
  Program,
  RecordDecl,
  Span,
  TypeNode,
  UninitializedStateDecl,
  UnitDecl,
} from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/frontend.ts';
import {
  lowerAndCheckFinancialAction,
  validateFinancialActionParameters,
} from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/financial-expression-source-v1.ts';
import type { SourceRejected } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/financial-expression-source-v1.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V1, createFinancialExpressionContractV1 } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/financial-expression-v1.ts';
import type { ExpressionResult } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/financial-expression-v1.ts';
import {
  ExpressionFailure,
  SYNTHETIC_SPAN,
  bytes,
  canonical,
  closed,
  decimal,
  parseCanonical,
  scalarString,
} from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/expression-wire-v1.ts';
import { schemaShape, validateSchema } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/financial-expression-types-v1.ts';
import type { Schema, ValueType } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/financial-expression-types-v1.ts';
import { sourceFailure, sourceSpan } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/expression-source-lower.ts';
import {
  asciiIdentifier,
  reservedSourceSchemaName,
  reservedSourceTerm,
  sourceType,
  validateSourceSchemaNames,
} from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/financial-expression-source-types.ts';
import {
  completeFundedPreparation,
  operationBinding,
  parseOwnedState,
  snapshotWorkInitial,
} from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/funded-expression-source-v1.ts';
import type { BindingDiagnostic, FundedExpressionResult } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/funded-expression-source-v1.ts';

export {
  FINANCIAL_AGREEMENT_SOURCE_PROFILE,
  parseSuccessorFinancialAgreementSource as parseFinancialAgreementSource,
} from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/frontend.ts';
export { formatSuccessorFinancialAgreementSource as formatFinancialAgreementSource } from '/home/charl/Moriarty/.worktrees/multiple-named-actions/experiments/moriarty-language/src/successor/format.ts';

const TRANSPORT_BYTES = 2_000_000;
const COMPONENT_BYTES = 65536;

export interface AgreementSourceElaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_PROFILE;
  contract: typeof FINANCIAL_EXPRESSION_CONTRACT_V1;
  agreement: string;
  action: string;
  staticWorkBound: string;
  schema: Schema;
  core: { statements: object[]; span: ReturnType<typeof sourceSpan> };
}
export interface AgreementSourceChecked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof FINANCIAL_AGREEMENT_SOURCE_PROFILE;
  staticWorkBound: string;
}

function rejection(error: unknown, source: unknown): SourceRejected {
  if (error instanceof ExpressionFailure) return { status: 'Rejected', code: error.code,
    span: error.span, nodePath: error.path.map(String), workUsed: '0' };
  if (error instanceof SuccessorSyntaxError) {
    const validSource = typeof source === 'string' && source.length <= COMPONENT_BYTES
      && scalarString(source) && bytes(source) <= COMPONENT_BYTES;
    return { status: 'Rejected', code: error.code,
      span: validSource ? { kind: 'source', start: String(error.start), end: String(error.end) } : SYNTHETIC_SPAN,
      nodePath: [], workUsed: '0' };
  }
  throw error;
}

function sourceTransport(source: string): void {
  if (typeof source !== 'string') throw new SuccessorSyntaxError('SOURCE_TYPE', 'source must be a string');
  if (source.length > COMPONENT_BYTES || bytes(source) > COMPONENT_BYTES)
    throw new SuccessorSyntaxError('SOURCE_BOUND', 'source exceeds UTF-8 byte bound');
}

function snapshots(text: string): Record<string, any> {
  const r = parseCanonical(text, TRANSPORT_BYTES);
  closed(r, ['Pre', 'Args', 'Obs', 'workInitial']);
  decimal(r.workInitial);
  if (r.workInitial.length > 6 || BigInt(r.workInitial) < 0n || BigInt(r.workInitial) > 65536n)
    throw new ExpressionFailure('INPUT_BOUND');
  for (const key of ['Pre', 'Args', 'Obs']) if (bytes(canonical(r[key])) > COMPONENT_BYTES)
    throw new ExpressionFailure('INPUT_BOUND');
  return r;
}

function sortedUnique(names: string[]): string[] {
  return [...names].sort((a, b) => a < b ? -1 : a > b ? 1 : 0);
}

function collectKind<T extends Declaration>(declarations: Declaration[], tag: T['tag']): T[] {
  return declarations.filter((d): d is T => d.tag === tag);
}

function declareNames(program: Program): {
  units: UnitDecl[];
  assets: AssetDecl[];
  parties: PartyDecl[];
  records: RecordDecl[];
  operations: OperationDecl[];
  states: UninitializedStateDecl[];
  action: ActionDecl;
} {
  const declarations = program.agreement.declarations;
  const units = collectKind<UnitDecl>(declarations, 'UnitDecl');
  const assets = collectKind<AssetDecl>(declarations, 'AssetDecl');
  const parties = collectKind<PartyDecl>(declarations, 'PartyDecl');
  const records = collectKind<RecordDecl>(declarations, 'RecordDecl');
  const operations = collectKind<OperationDecl>(declarations, 'OperationDecl');
  const states = collectKind<UninitializedStateDecl>(declarations, 'UninitializedStateDecl');
  const actions = collectKind<ActionDecl>(declarations, 'ActionDecl');
  for (const d of declarations) {
    if (d.tag === 'ConstDecl' || d.tag === 'StateDecl') sourceFailure('SOURCE_DECLARATION', d.span);
  }
  if (actions.length !== 1) sourceFailure('SOURCE_ACTION_COUNT', program.agreement.span);
  const unitNames = new Set<string>();
  const assetNames = new Set<string>();
  const otherNames = new Set<string>();
  for (const d of declarations) {
    if (d.tag === 'UnitDecl') {
      if (unitNames.has(d.name) || otherNames.has(d.name)) sourceFailure('SOURCE_DUPLICATE_DECLARATION', d.span);
      unitNames.add(d.name);
    } else if (d.tag === 'AssetDecl') {
      if (assetNames.has(d.name) || otherNames.has(d.name)) sourceFailure('SOURCE_DUPLICATE_DECLARATION', d.span);
      assetNames.add(d.name);
    } else {
      if (otherNames.has(d.name) || unitNames.has(d.name) || assetNames.has(d.name)) {
        sourceFailure('SOURCE_DUPLICATE_DECLARATION', d.span);
      }
      otherNames.add(d.name);
    }
  }
  const action = actions[0];
  if (reservedSourceTerm(action.name)) sourceFailure('SOURCE_RESERVED_NAME', action.span);
  for (const asset of assets) {
    if (asset.type.numeric || asset.type.name !== 'Asset' || asset.type.arguments.length) {
      sourceFailure('SOURCE_DECLARATION_TYPE', asset.type.span);
    }
  }
  for (const record of records) {
    const fields = new Set<string>();
    for (const field of record.fields) {
      if (fields.has(field.name)) sourceFailure('SOURCE_DUPLICATE_DECLARATION', field.span);
      fields.add(field.name);
    }
  }
  return { units, assets, parties, records, operations, states, action };
}

type DeclaredNames = {
  units: Set<string>;
  assets: Set<string>;
  vaults: Set<string>;
  parties: Set<string>;
  records: Set<string>;
  enums: Set<string>;
  variants: Set<string>;
  operations: Set<string>;
};

function requireDeclared(name: string, names: Set<string>, span: Span): void {
  if (!names.has(name)) sourceFailure('TYPE_NAME', span);
}

function scaleInRange(node: TypeNode): void {
  const n = BigInt(node.name);
  if (n < 0n || n > 18n) sourceFailure('TYPE_LITERAL', node.span);
}

function resolveDeclaredTypeNode(t: TypeNode, names: DeclaredNames): void {
  if (t.numeric) return;
  if (t.name === 'Option') {
    resolveDeclaredTypeNode(t.arguments[0], names);
    return;
  }
  if (t.name === 'Collection') {
    resolveDeclaredTypeNode(t.arguments[0], names);
    const capacity = BigInt(t.arguments[1].name);
    if (capacity < 0n || capacity > 128n) sourceFailure('TYPE_COLLECTION_BOUND', t.arguments[1].span);
    return;
  }
  if (t.name === 'Unit') sourceFailure('TYPE_NAME', t.span);
  if (['UInt64', 'UInt128', 'UInt256', 'SInt128', 'Bool', 'Text'].includes(t.name)) return;
  if (['Amount', 'SignedAmount', 'NetAmount'].includes(t.name)) {
    requireDeclared(t.arguments[0].name, names.assets, t.arguments[0].span);
    return;
  }
  if (t.name === 'Shares') {
    requireDeclared(t.arguments[0].name, names.vaults, t.arguments[0].span);
    requireDeclared(t.arguments[1].name, names.parties, t.arguments[1].span);
    return;
  }
  if (t.name === 'Quantity') {
    const unitsNode = t.arguments[0];
    const pairCount = unitsNode.arguments.length / 2;
    if (pairCount > 8) sourceFailure('TYPE_LITERAL', unitsNode.span);
    let previous = '';
    for (let i = 0; i < unitsNode.arguments.length; i += 2) {
      const nameNode = unitsNode.arguments[i];
      const powerNode = unitsNode.arguments[i + 1];
      requireDeclared(nameNode.name, names.units, nameNode.span);
      const power = BigInt(powerNode.name);
      if (nameNode.name <= previous || power === 0n || power < -128n || power > 127n) {
        sourceFailure('TYPE_LITERAL', nameNode.name <= previous ? nameNode.span : powerNode.span);
      }
      previous = nameNode.name;
    }
    scaleInRange(t.arguments[1]);
    return;
  }
  if (t.name === 'Rate') {
    scaleInRange(t.arguments[0]);
    return;
  }
  if (t.name === 'Price') {
    requireDeclared(t.arguments[0].name, names.assets, t.arguments[0].span);
    requireDeclared(t.arguments[1].name, names.assets, t.arguments[1].span);
    if (t.arguments[0].name === t.arguments[1].name) sourceFailure('TYPE_LITERAL', t.arguments[1].span);
    scaleInRange(t.arguments[2]);
    return;
  }
  if (t.name === 'AmountProduct') {
    requireDeclared(t.arguments[0].name, names.assets, t.arguments[0].span);
    requireDeclared(t.arguments[1].name, names.assets, t.arguments[1].span);
    if (t.arguments[0].name > t.arguments[1].name) sourceFailure('TYPE_LITERAL', t.span);
    return;
  }
  if (t.name === 'ScaledAmount' || t.name === 'SignedScaledAmount') {
    requireDeclared(t.arguments[0].name, names.assets, t.arguments[0].span);
    scaleInRange(t.arguments[1]);
    return;
  }
  if (t.name === 'Record') {
    requireDeclared(t.arguments[0].name, names.records, t.arguments[0].span);
    return;
  }
  if (t.name === 'Enum') {
    requireDeclared(t.arguments[0].name, names.enums, t.arguments[0].span);
    return;
  }
  if (t.name === 'Variant') {
    requireDeclared(t.arguments[0].name, names.variants, t.arguments[0].span);
    return;
  }
  if (t.name === 'Operation') {
    requireDeclared(t.arguments[0].name, names.operations, t.arguments[0].span);
  }
}

function declaredType(t: TypeNode, names: DeclaredNames): ValueType {
  const value = sourceType(t);
  resolveDeclaredTypeNode(t, names);
  return value;
}

function requireSourceSchemaName(name: string, span: Span): void {
  if (!asciiIdentifier(name) || reservedSourceSchemaName(name)) sourceFailure('SOURCE_SCHEMA_NAME', span);
}

function validateDeclaredSchemaNames(collected: {
  units: UnitDecl[];
  assets: AssetDecl[];
  parties: PartyDecl[];
  records: RecordDecl[];
  operations: OperationDecl[];
  states: UninitializedStateDecl[];
  action: ActionDecl;
}): void {
  for (const d of collected.units) requireSourceSchemaName(d.name, d.span);
  for (const d of collected.assets) requireSourceSchemaName(d.name, d.span);
  for (const d of collected.parties) requireSourceSchemaName(d.name, d.span);
  for (const d of collected.records) {
    requireSourceSchemaName(d.name, d.span);
    for (const field of d.fields) requireSourceSchemaName(field.name, field.span);
  }
  for (const d of collected.operations) requireSourceSchemaName(d.name, d.span);
  for (const d of collected.states) requireSourceSchemaName(d.name, d.span);
  for (const parameter of collected.action.parameters) requireSourceSchemaName(parameter.name, parameter.span);
}

function walkDeclaredCycles(records: RecordDecl[], operations: OperationDecl[]): void {
  const recordByName = new Map(records.map((record) => [record.name, record]));
  const operationRecord = new Map(operations.map((operation) => [operation.name, operation.type.name]));
  const visiting = new Set<string>();
  const done = new Set<string>();
  function walkRecord(name: string, span: Span): void {
    const id = 'R:' + name;
    if (done.has(id)) return;
    if (visiting.has(id)) sourceFailure('TYPE_SCHEMA_CYCLE', span);
    const record = recordByName.get(name);
    if (record === undefined) return;
    visiting.add(id);
    for (const field of record.fields) walkTypeNode(field.type);
    visiting.delete(id);
    done.add(id);
  }
  function walkOperation(name: string, span: Span): void {
    const id = 'O:' + name;
    if (done.has(id)) return;
    if (visiting.has(id)) sourceFailure('TYPE_SCHEMA_CYCLE', span);
    const recordName = operationRecord.get(name);
    if (recordName === undefined) return;
    visiting.add(id);
    walkRecord(recordName, span);
    visiting.delete(id);
    done.add(id);
  }
  function walkTypeNode(t: TypeNode): void {
    if (t.numeric) return;
    if (t.name === 'Option' || t.name === 'Collection') {
      if (t.arguments[0] !== undefined) walkTypeNode(t.arguments[0]);
      return;
    }
    if (t.name === 'Record' && t.arguments[0] !== undefined) walkRecord(t.arguments[0].name, t.span);
    else if (t.name === 'Operation' && t.arguments[0] !== undefined) walkOperation(t.arguments[0].name, t.span);
  }
  for (const record of records) walkRecord(record.name, record.span);
  for (const operation of operations) walkOperation(operation.name, operation.span);
}

function deriveSchema(program: Program): { schema: Schema; schemaText: string; action: ActionDecl; operations: OperationDecl[]; records: RecordDecl[] } {
  const collected = declareNames(program);
  validateDeclaredSchemaNames(collected);
  const names: DeclaredNames = {
    units: new Set(collected.units.map((d) => d.name)),
    assets: new Set(collected.assets.map((d) => d.name)),
    vaults: new Set<string>(),
    parties: new Set(collected.parties.map((d) => d.name)),
    records: new Set(collected.records.map((d) => d.name)),
    enums: new Set<string>(),
    variants: new Set<string>(),
    operations: new Set(collected.operations.map((d) => d.name)),
  };
  const operations: Record<string, string> = {};
  for (const operation of collected.operations) {
    if (operation.type.numeric || operation.type.arguments.length) {
      sourceFailure('SOURCE_DECLARATION_TYPE', operation.type.span);
    }
    requireDeclared(operation.type.name, names.records, operation.type.span);
    operations[operation.name] = operation.type.name;
  }
  const recordTypes: Record<string, Record<string, ValueType>> = {};
  for (const record of collected.records) {
    const fields: Record<string, ValueType> = {};
    for (const field of record.fields) fields[field.name] = declaredType(field.type, names);
    recordTypes[record.name] = fields;
  }
  walkDeclaredCycles(collected.records, collected.operations);
  const fields: Record<string, { type: ValueType; writeClass: 'ordinary' }> = {};
  for (const state of collected.states) {
    fields[state.name] = { type: declaredType(state.type, names), writeClass: 'ordinary' };
  }
  const args: Record<string, ValueType> = {};
  for (const parameter of collected.action.parameters) {
    args[parameter.name] = declaredType(parameter.type, names);
  }
  const schema = {
    units: sortedUnique(collected.units.map((d) => d.name)),
    assets: sortedUnique(collected.assets.map((d) => d.name)),
    vaults: [] as string[],
    parties: sortedUnique(collected.parties.map((d) => d.name)),
    recordTypes,
    enumTypes: {},
    variantTypes: {},
    fields,
    args,
    observations: {},
    operations,
  };
  const schemaText = canonical(schema);
  const normalized: Schema = parseCanonical(schemaText, COMPONENT_BYTES);
  schemaShape(normalized);
  validateSchema(normalized);
  validateSourceSchemaNames(normalized);
  for (const field of Object.values(normalized.fields) as { writeClass?: string }[]) {
    if (field.writeClass === 'financial') sourceFailure('SOURCE_DECLARATION', collected.states[0]?.span ?? program.agreement.span);
  }
  validateFinancialActionParameters(collected.action, normalized);
  const binding = operationBinding(normalized);
  if ('status' in binding) {
    sourceFailure(
      'OPERATION_BINDING',
      bindingFailureSpan(program, collected.operations, collected.records, binding.diagnostic),
    );
  }
  if (binding.transferAsset !== binding.repayDenomination) {
    sourceFailure(
      'OPERATION_BINDING',
      mixedUnitSpan(collected.operations, collected.records)
        ?? bindingFailureSpan(program, collected.operations, collected.records),
    );
  }
  return { schema: normalized, schemaText, action: collected.action, operations: collected.operations, records: collected.records };
}

function bindingFailureSpan(
  program: Program,
  operations: OperationDecl[],
  records: RecordDecl[],
  diagnostic?: BindingDiagnostic,
): Span {
  if (diagnostic?.operation !== undefined) {
    const operation = operations.find((item) => item.name === diagnostic.operation);
    if (diagnostic.field !== undefined) {
      const record = records.find((item) => item.name === operation?.type.name);
      const field = record?.fields.find((item) => item.name === diagnostic.field);
      return field?.span ?? record?.span ?? operation?.span ?? program.agreement.span;
    }
    return operation?.span ?? program.agreement.span;
  }
  const extra = operations.find((operation) => operation.name !== 'Transfer' && operation.name !== 'Repay');
  if (extra) return extra.span;
  if (operations.length !== 2) return operations[0]?.span ?? program.agreement.span;
  const transfer = operations.find((operation) => operation.name === 'Transfer');
  const repay = operations.find((operation) => operation.name === 'Repay');
  if (transfer === undefined) return repay?.span ?? program.agreement.span;
  if (repay === undefined) return transfer.span;
  const transferRecord = records.find((record) => record.name === transfer.type.name);
  return transferRecord?.span ?? transfer.span;
}

function mixedUnitSpan(operations: OperationDecl[], records: RecordDecl[]): Span | undefined {
  const repay = operations.find((operation) => operation.name === 'Repay');
  const record = records.find((item) => item.name === repay?.type.name);
  return record?.fields.find((field) => field.name === 'nominalAmount')?.span ?? repay?.span;
}

function compile(source: string): AgreementSourceElaborated | SourceRejected {
  const program = parseSuccessorFinancialAgreementSource(source);
  const derived = deriveSchema(program);
  const lowered = lowerAndCheckFinancialAction(source, derived.action, derived.schema, derived.schemaText);
  if ('status' in lowered) return lowered;
  return {
    judgmentResult: 'SourceElaborated',
    sourceProfile: FINANCIAL_AGREEMENT_SOURCE_PROFILE,
    contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
    agreement: program.agreement.name,
    action: derived.action.name,
    staticWorkBound: lowered.staticWorkBound,
    schema: derived.schema,
    core: lowered.core,
  };
}

/** Trusted Σ is derived from source text. Public APIs accept primitive strings only. */
export function createFinancialAgreementSourceV1() {
  return Object.freeze({
    elaborate(source: string): AgreementSourceElaborated | SourceRejected {
      try { sourceTransport(source); return compile(source); } catch (error) { return rejection(error, source); }
    },
    check(source: string): AgreementSourceChecked | SourceRejected {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return {
          judgmentResult: 'SourceChecked',
          sourceProfile: FINANCIAL_AGREEMENT_SOURCE_PROFILE,
          staticWorkBound: compiled.staticWorkBound,
        };
      } catch (error) { return rejection(error, source); }
    },
    evaluate(source: string, snapshotCanonicalJSON: string, repaymentStateJSON: string): FundedExpressionResult {
      try {
        sourceTransport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        const parsedState = parseOwnedState(repaymentStateJSON);
        if (!parsedState.ok) return parsedState.result;
        const remainingText = (parsedState.value.work as Record<string, unknown>).remaining as string;
        const workInitial = typeof snapshotCanonicalJSON === 'string'
          ? snapshotWorkInitial(snapshotCanonicalJSON) : undefined;
        if (workInitial !== undefined && workInitial !== remainingText) {
          return { status: 'Rejected', code: 'WORK_MISMATCH' };
        }
        const input = snapshots(snapshotCanonicalJSON);
        const schemaText = canonical(compiled.schema);
        const evaluated: ExpressionResult = createFinancialExpressionContractV1(schemaText).evaluate(canonical({
          contract: FINANCIAL_EXPRESSION_CONTRACT_V1, source, core: compiled.core, ...input,
        }));
        return completeFundedPreparation(evaluated, schemaText, parsedState.value, remainingText, workInitial);
      } catch (error) { return rejection(error, source); }
    },
  });
}
