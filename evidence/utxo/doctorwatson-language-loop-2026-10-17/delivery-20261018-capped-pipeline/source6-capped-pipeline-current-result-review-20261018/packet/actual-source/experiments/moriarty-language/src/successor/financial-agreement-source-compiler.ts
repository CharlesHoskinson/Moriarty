/** Shared declaration compilation for source-defined financial agreements. */
import {
  FINANCIAL_AGREEMENT_SOURCE_PROFILE,
  FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE,
  FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE,
  FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE,
  FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE,
  FINANCIAL_READ_GENERIC_PRIMARIES,
  FINANCIAL_POST_READ_GENERIC_PRIMARIES,
  parseSuccessorFinancialAgreementSource,
  parseSuccessorFinancialAgreementSourceV2,
  parseSuccessorFinancialAgreementSourceV3,
  parseSuccessorFinancialAgreementSourceV4,
  parseSuccessorFinancialAgreementSourceV5,
  SuccessorSyntaxError,
} from './frontend.ts';
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
} from './frontend.ts';
import {
  lowerAndCheckFinancialAction,
  validateFinancialActionParameters,
} from './financial-expression-source-v1.ts';
import type { SourceRejected } from './financial-expression-source-v1.ts';
import {
  FINANCIAL_EXPRESSION_CONTRACT_V1,
  FINANCIAL_EXPRESSION_CONTRACT_V2,
  FINANCIAL_EXPRESSION_CONTRACT_V3,
  FINANCIAL_EXPRESSION_CONTRACT_V4,
  createFinancialExpressionContractV1,
  createFinancialExpressionContractV2,
  createFinancialExpressionContractV3,
  createFinancialExpressionContractV4,
} from './financial-expression-v1.ts';
import type { ExpressionResult } from './financial-expression-v1.ts';
import {
  ExpressionFailure,
  SYNTHETIC_SPAN,
  bytes,
  canonical,
  closed,
  decimal,
  parseCanonical,
  scalarString,
} from './expression-wire-v1.ts';
import { schemaShape, validateSchema } from './financial-expression-types-v1.ts';
import type { Schema, ValueType } from './financial-expression-types-v1.ts';
import { sourceFailure, sourceSpan } from './expression-source-lower.ts';
import {
  asciiIdentifier,
  reservedSourceSchemaName,
  reservedSourceTerm,
  sourceType,
  validateSourceSchemaNames,
} from './financial-expression-source-types.ts';
import {
  completeFundedPreparation,
  lifecycleOperationBinding,
  operationBinding,
  parseOwnedState,
  snapshotWorkInitial,
} from './funded-expression-source-v1.ts';
import { admitRepaymentStateJSON } from './repayment.ts';
import { admitFinancialLifecycleStateJSON } from './financial-lifecycle.ts';
import type {
  BindingDiagnostic,
  FundedExpressionResult,
  LifecycleExpressionResult,
} from './funded-expression-source-v1.ts';

export const TRANSPORT_BYTES = 2_000_000;
export const COMPONENT_BYTES = 65536;

export type AgreementProfile =
  | typeof FINANCIAL_AGREEMENT_SOURCE_PROFILE
  | typeof FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE
  | typeof FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE
  | typeof FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE
  | typeof FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE;

function extraReserved(profile: AgreementProfile): readonly string[] {
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE
    || profile === FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE) {
    return [...FINANCIAL_READ_GENERIC_PRIMARIES, ...FINANCIAL_POST_READ_GENERIC_PRIMARIES];
  }
  return profile === FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE ? FINANCIAL_READ_GENERIC_PRIMARIES : [];
}

function readsEnabled(profile: AgreementProfile): boolean {
  return profile === FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE
    || profile === FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE
    || profile === FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE;
}

function postReadsEnabled(profile: AgreementProfile): boolean {
  return profile === FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE
    || profile === FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE;
}

export interface CompiledAgreementAction {
  action: string;
  schema: Schema;
  core: { statements: object[]; span: ReturnType<typeof sourceSpan> };
  staticWorkBound: string;
}

export interface CompiledAgreement {
  agreement: string;
  actions: CompiledAgreementAction[];
}

export function rejection(error: unknown, source: unknown): SourceRejected {
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

export function sourceTransport(source: string): void {
  if (typeof source !== 'string') throw new SuccessorSyntaxError('SOURCE_TYPE', 'source must be a string');
  if (source.length > COMPONENT_BYTES || bytes(source) > COMPONENT_BYTES)
    throw new SuccessorSyntaxError('SOURCE_BOUND', 'source exceeds UTF-8 byte bound');
}

export function snapshots(text: string): Record<string, any> {
  const r = parseCanonical(text, TRANSPORT_BYTES);
  closed(r, ['Pre', 'Args', 'Obs', 'workInitial']);
  decimal(r.workInitial);
  if (r.workInitial.length > 6 || BigInt(r.workInitial) < 0n || BigInt(r.workInitial) > 65536n)
    throw new ExpressionFailure('INPUT_BOUND');
  for (const key of ['Pre', 'Args', 'Obs']) if (bytes(canonical(r[key])) > COMPONENT_BYTES)
    throw new ExpressionFailure('INPUT_BOUND');
  return r;
}

export function evaluateCompiledAction(
  source: string,
  compiled: CompiledAgreementAction,
  snapshotCanonicalJSON: string,
  repaymentStateJSON: string,
  profile: typeof FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE,
): LifecycleExpressionResult;
export function evaluateCompiledAction(
  source: string,
  compiled: CompiledAgreementAction,
  snapshotCanonicalJSON: string,
  repaymentStateJSON: string,
  profile?: Exclude<AgreementProfile, typeof FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE>,
): FundedExpressionResult;
export function evaluateCompiledAction(
  source: string,
  compiled: CompiledAgreementAction,
  snapshotCanonicalJSON: string,
  repaymentStateJSON: string,
  profile: AgreementProfile = FINANCIAL_AGREEMENT_SOURCE_PROFILE,
): FundedExpressionResult | LifecycleExpressionResult {
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE) {
    const admitted = admitFinancialLifecycleStateJSON(repaymentStateJSON);
    if (!admitted.ok) return admitted.result;
    const remainingText = admitted.value.work.remaining;
    const workInitial = typeof snapshotCanonicalJSON === 'string'
      ? snapshotWorkInitial(snapshotCanonicalJSON) : undefined;
    if (workInitial !== undefined && workInitial !== remainingText) {
      return { status: 'Rejected', code: 'WORK_MISMATCH' };
    }
    const input = snapshots(snapshotCanonicalJSON);
    const schemaText = canonical(compiled.schema);
    return createFinancialExpressionContractV4(schemaText, repaymentStateJSON).evaluate(canonical({
      contract: FINANCIAL_EXPRESSION_CONTRACT_V4, source, core: compiled.core, ...input,
    }));
  }
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE) {
    const admitted = admitRepaymentStateJSON(repaymentStateJSON);
    if (!admitted.ok) return admitted.result;
    const remainingText = admitted.value.work.remaining;
    const workInitial = typeof snapshotCanonicalJSON === 'string'
      ? snapshotWorkInitial(snapshotCanonicalJSON) : undefined;
    if (workInitial !== undefined && workInitial !== remainingText) {
      return { status: 'Rejected', code: 'WORK_MISMATCH' };
    }
    const input = snapshots(snapshotCanonicalJSON);
    const schemaText = canonical(compiled.schema);
    return createFinancialExpressionContractV3(schemaText, repaymentStateJSON).evaluate(canonical({
      contract: FINANCIAL_EXPRESSION_CONTRACT_V3, source, core: compiled.core, ...input,
    }));
  }
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE) {
    const admitted = admitRepaymentStateJSON(repaymentStateJSON);
    if (!admitted.ok) return admitted.result;
    const remainingText = admitted.value.work.remaining;
    const workInitial = typeof snapshotCanonicalJSON === 'string'
      ? snapshotWorkInitial(snapshotCanonicalJSON) : undefined;
    if (workInitial !== undefined && workInitial !== remainingText) {
      return { status: 'Rejected', code: 'WORK_MISMATCH' };
    }
    const input = snapshots(snapshotCanonicalJSON);
    const schemaText = canonical(compiled.schema);
    const evaluated: ExpressionResult = createFinancialExpressionContractV2(schemaText, repaymentStateJSON).evaluate(canonical({
      contract: FINANCIAL_EXPRESSION_CONTRACT_V2, source, core: compiled.core, ...input,
    }));
    return completeFundedPreparation(
      evaluated,
      schemaText,
      JSON.parse(JSON.stringify(admitted.value)) as Record<string, unknown>,
      remainingText,
      workInitial,
    );
  }
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
}

function parseAgreement(source: string, profile: AgreementProfile): Program {
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE) return parseSuccessorFinancialAgreementSourceV5(source);
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE) return parseSuccessorFinancialAgreementSourceV4(source);
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE) return parseSuccessorFinancialAgreementSourceV3(source);
  return profile === FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE
    ? parseSuccessorFinancialAgreementSourceV2(source)
    : parseSuccessorFinancialAgreementSource(source);
}

function sortedUnique(names: string[]): string[] {
  return [...names].sort((a, b) => a < b ? -1 : a > b ? 1 : 0);
}

function collectKind<T extends Declaration>(declarations: Declaration[], tag: T['tag']): T[] {
  return declarations.filter((d): d is T => d.tag === tag);
}

function declareNames(program: Program, profile: AgreementProfile): {
  units: UnitDecl[];
  assets: AssetDecl[];
  parties: PartyDecl[];
  records: RecordDecl[];
  operations: OperationDecl[];
  states: UninitializedStateDecl[];
  actions: ActionDecl[];
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
  if (profile === FINANCIAL_AGREEMENT_SOURCE_PROFILE) {
    if (actions.length !== 1) sourceFailure('SOURCE_ACTION_COUNT', program.agreement.span);
  } else if (actions.length === 0) {
    sourceFailure('SOURCE_ACTION_COUNT', program.agreement.span);
  }
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
  const reserved = extraReserved(profile);
  for (const action of actions) {
    if (reservedSourceTerm(action.name, reserved)) sourceFailure('SOURCE_RESERVED_NAME', action.span);
  }
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
  return { units, assets, parties, records, operations, states, actions };
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

function requireSourceSchemaName(name: string, span: Span, extra: readonly string[] = []): void {
  if (!asciiIdentifier(name) || reservedSourceSchemaName(name, extra)) sourceFailure('SOURCE_SCHEMA_NAME', span);
}

function validateDeclaredSchemaNames(collected: {
  units: UnitDecl[];
  assets: AssetDecl[];
  parties: PartyDecl[];
  records: RecordDecl[];
  operations: OperationDecl[];
  states: UninitializedStateDecl[];
  actions: ActionDecl[];
}, extra: readonly string[] = []): void {
  for (const d of collected.units) requireSourceSchemaName(d.name, d.span, extra);
  for (const d of collected.assets) requireSourceSchemaName(d.name, d.span, extra);
  for (const d of collected.parties) requireSourceSchemaName(d.name, d.span, extra);
  for (const d of collected.records) {
    requireSourceSchemaName(d.name, d.span, extra);
    for (const field of d.fields) requireSourceSchemaName(field.name, field.span, extra);
  }
  for (const d of collected.operations) requireSourceSchemaName(d.name, d.span, extra);
  for (const d of collected.states) requireSourceSchemaName(d.name, d.span, extra);
  for (const action of collected.actions) {
    for (const parameter of action.parameters) requireSourceSchemaName(parameter.name, parameter.span, extra);
  }
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
  const extra = operations.find((operation) =>
    operation.name !== 'Transfer' && operation.name !== 'Repay'
    && operation.name !== 'Originate' && operation.name !== 'Accrue');
  if (extra) return extra.span;
  if (operations.length !== 2 && operations.length !== 4) return operations[0]?.span ?? program.agreement.span;
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

function requireProtectedBinding(
  program: Program,
  operations: OperationDecl[],
  records: RecordDecl[],
  schema: Schema,
  profile: AgreementProfile = FINANCIAL_AGREEMENT_SOURCE_PROFILE,
): void {
  const binding = profile === FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE
    ? lifecycleOperationBinding(schema)
    : operationBinding(schema);
  if ('status' in binding) {
    sourceFailure(
      'OPERATION_BINDING',
      bindingFailureSpan(program, operations, records, binding.diagnostic),
    );
  }
  if (binding.transferAsset !== binding.repayDenomination) {
    sourceFailure(
      'OPERATION_BINDING',
      mixedUnitSpan(operations, records)
        ?? bindingFailureSpan(program, operations, records),
    );
  }
}

function normalizeSchema(schema: Schema, states: UninitializedStateDecl[], program: Program): Schema {
  const schemaText = canonical(schema);
  const normalized: Schema = parseCanonical(schemaText, COMPONENT_BYTES);
  schemaShape(normalized);
  validateSchema(normalized);
  validateSourceSchemaNames(normalized);
  for (const field of Object.values(normalized.fields) as { writeClass?: string }[]) {
    if (field.writeClass === 'financial') sourceFailure('SOURCE_DECLARATION', states[0]?.span ?? program.agreement.span);
  }
  return normalized;
}

export function compileAgreementSource(source: string, profile: AgreementProfile): CompiledAgreement | SourceRejected {
  const program = parseAgreement(source, profile);
  const collected = declareNames(program, profile);
  validateDeclaredSchemaNames(collected, extraReserved(profile));
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
  const shared = {
    units: sortedUnique(collected.units.map((d) => d.name)),
    assets: sortedUnique(collected.assets.map((d) => d.name)),
    vaults: [] as string[],
    parties: sortedUnique(collected.parties.map((d) => d.name)),
    recordTypes,
    enumTypes: {},
    variantTypes: {},
    fields,
    args: {} as Record<string, ValueType>,
    observations: {},
    operations,
  };
  function actionArgs(action: ActionDecl): Record<string, ValueType> {
    const args: Record<string, ValueType> = {};
    for (const parameter of action.parameters) {
      args[parameter.name] = declaredType(parameter.type, names);
    }
    return args;
  }
  function compileAction(action: ActionDecl, schema: Schema): CompiledAgreementAction | SourceRejected {
    const lowered = lowerAndCheckFinancialAction(source, action, schema, canonical(schema), {
      contract: profile === FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE
        ? FINANCIAL_EXPRESSION_CONTRACT_V4
        : postReadsEnabled(profile)
        ? FINANCIAL_EXPRESSION_CONTRACT_V3
        : readsEnabled(profile) ? FINANCIAL_EXPRESSION_CONTRACT_V2 : FINANCIAL_EXPRESSION_CONTRACT_V1,
      extraReserved: extraReserved(profile),
      financialReads: readsEnabled(profile),
      financialPostReads: postReadsEnabled(profile),
    });
    if ('status' in lowered) return lowered;
    return {
      action: action.name,
      schema,
      core: lowered.core,
      staticWorkBound: lowered.staticWorkBound,
    };
  }
  if (profile === FINANCIAL_AGREEMENT_SOURCE_PROFILE) {
    const action = collected.actions[0];
    const schema = normalizeSchema({ ...shared, args: actionArgs(action) }, collected.states, program);
    validateFinancialActionParameters(action, schema, extraReserved(profile));
    requireProtectedBinding(program, collected.operations, collected.records, schema, profile);
    const compiled = compileAction(action, schema);
    if ('status' in compiled) return compiled;
    return { agreement: program.agreement.name, actions: [compiled] };
  }
  const sharedNormalized = normalizeSchema(shared, collected.states, program);
  requireProtectedBinding(program, collected.operations, collected.records, sharedNormalized, profile);
  const compiledActions: CompiledAgreementAction[] = [];
  for (const action of collected.actions) {
    const schema = normalizeSchema({ ...shared, args: actionArgs(action) }, collected.states, program);
    validateFinancialActionParameters(action, schema, extraReserved(profile));
    const compiled = compileAction(action, schema);
    if ('status' in compiled) return compiled;
    compiledActions.push(compiled);
  }
  return { agreement: program.agreement.name, actions: compiledActions };
}

export function requireActionName(actionName: unknown): string {
  if (typeof actionName !== 'string' || !asciiIdentifier(actionName)) {
    throw new ExpressionFailure('SOURCE_ACTION_NAME');
  }
  return actionName;
}

export function selectCompiledAction(
  actions: CompiledAgreementAction[],
  actionName: unknown,
): CompiledAgreementAction {
  const name = requireActionName(actionName);
  const selected = actions.find((item) => item.action === name);
  if (selected === undefined) throw new ExpressionFailure('SOURCE_ACTION_UNKNOWN');
  return selected;
}
