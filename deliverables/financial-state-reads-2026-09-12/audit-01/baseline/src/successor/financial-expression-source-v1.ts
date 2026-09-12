/** Source-only entry to local /1 expression evaluation. No financial operation
 * interpreter, authorization, funded preparation, signing or ledger path is called. */
import { parseFinancialExpressionSource, FINANCIAL_EXPRESSION_SOURCE_PROFILE } from './financial-expression-source-frontend.ts';
import { SuccessorSyntaxError } from './frontend.ts';
import type { ActionDecl, Program } from './frontend.ts';
import { createFinancialExpressionContractV1, FINANCIAL_EXPRESSION_CONTRACT_V1 } from './financial-expression-v1.ts';
import type { ExpressionResult } from './financial-expression-v1.ts';
import { ExpressionFailure, SYNTHETIC_SPAN, bytes, canonical, closed, decimal, parseCanonical, scalarString } from './expression-wire-v1.ts';
import { schemaShape, validateSchema, same } from './financial-expression-types-v1.ts';
import type { Schema } from './financial-expression-types-v1.ts';
import { sourceFailure, sourceSpan } from './expression-source-lower.ts';
import type { SourceCoreNode } from './expression-source-lower.ts';
import { createFinancialSourceLowering } from './financial-expression-source-lower.ts';
import { sourceType, reservedSourceTerm, validateSourceSchemaNames } from './financial-expression-source-types.ts';

export type SourceRejected = Extract<ExpressionResult, { status: 'Rejected' }>;
export interface SourceElaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof FINANCIAL_EXPRESSION_SOURCE_PROFILE;
  contract: typeof FINANCIAL_EXPRESSION_CONTRACT_V1;
  agreement: string;
  action: string;
  staticWorkBound: string;
  core: { statements: SourceCoreNode[]; span: ReturnType<typeof sourceSpan> };
}
export interface SourceChecked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof FINANCIAL_EXPRESSION_SOURCE_PROFILE;
  staticWorkBound: string;
}
const TRANSPORT_BYTES = 2_000_000;
const COMPONENT_BYTES = 65536;
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
function sourceAction(program: Program, schema: Schema): ActionDecl {
  const declarations = program.agreement.declarations;
  const actions = declarations.filter((d): d is ActionDecl => d.tag === 'ActionDecl');
  if (actions.length !== 1) sourceFailure('SOURCE_ACTION_COUNT', program.agreement.span);
  const names = new Set<string>();
  for (const d of declarations) {
    if (names.has(d.name)) sourceFailure('SOURCE_DUPLICATE_DECLARATION', d.span);
    names.add(d.name);
    if (d.tag === 'ConstDecl' || d.tag === 'StateDecl'
      || d.tag === 'UninitializedStateDecl' || d.tag === 'RecordDecl' || d.tag === 'OperationDecl') {
      sourceFailure('SOURCE_DECLARATION', d.span);
    }
    if (d.tag === 'ActionDecl') {
      if (reservedSourceTerm(d.name)) sourceFailure('SOURCE_RESERVED_NAME', d.span);
    } else {
      if (d.tag === 'AssetDecl' && (d.type.numeric || d.type.name !== 'Asset' || d.type.arguments.length))
        sourceFailure('SOURCE_DECLARATION_TYPE', d.type.span);
      const roster = d.tag === 'UnitDecl' ? schema.units : d.tag === 'PartyDecl' ? schema.parties : schema.assets;
      if (!roster.includes(d.name)) sourceFailure('SOURCE_DECLARATION_NAME', d.span);
    }
  }
  const action = actions[0];
  validateFinancialActionParameters(action, schema);
  return action;
}
export function validateFinancialActionParameters(action: ActionDecl, schema: Schema): void {
  const parameters = new Set<string>();
  const otherDeclarations = new Set<string>();
  for (const key of ['units', 'assets', 'vaults', 'parties']) schema[key].forEach((n: string) => otherDeclarations.add(n));
  for (const key of ['fields', 'observations', 'recordTypes', 'enumTypes', 'variantTypes', 'operations']) Object.keys(schema[key]).forEach(n => otherDeclarations.add(n));
  for (const p of action.parameters) {
    if (parameters.has(p.name)) sourceFailure('SOURCE_PARAMETER_NAMES', p.span);
    parameters.add(p.name);
    if (reservedSourceTerm(p.name) || otherDeclarations.has(p.name)) sourceFailure('SOURCE_RESERVED_NAME', p.span);
  }
  if (parameters.size !== Object.keys(schema.args).length || [...parameters].some(n => !Object.hasOwn(schema.args, n)))
    sourceFailure('SOURCE_PARAMETER_NAMES', action.span);
  for (const p of action.parameters) if (!same(sourceType(p.type), schema.args[p.name]))
    sourceFailure('SOURCE_PARAMETER_TYPE', p.type.span);
}
function workBound(core: object): string {
  let count = 0;
  const pending: any[] = [core];
  while (pending.length) {
    const value = pending.pop();
    if (value === null || typeof value !== 'object') continue;
    if (Object.hasOwn(value, 'constructor') && typeof value.constructor === 'string') count++;
    pending.push(...Object.values(value));
  }
  return String(count);
}
/** Original source, parsed action and validated schema only. Never consumes caller Core. */
export function lowerAndCheckFinancialAction(
  source: string,
  action: ActionDecl,
  schema: Schema,
  schemaCanonicalJSON: string,
): { core: { statements: SourceCoreNode[]; span: ReturnType<typeof sourceSpan> }; staticWorkBound: string } | SourceRejected {
  const lowering = createFinancialSourceLowering(schema, new Set(action.parameters.map(p => p.name)));
  const statements = [...action.statements, ...action.postconditions].map(statement => {
    if (statement.tag === 'Let' && reservedSourceTerm(statement.name)) sourceFailure('SOURCE_RESERVED_NAME', statement.span);
    return lowering.statement(statement);
  });
  const core = { statements, span: sourceSpan(action.span) };
  const checked = createFinancialExpressionContractV1(schemaCanonicalJSON).check(canonical({ contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
    source, core, Pre: {}, Args: {}, Obs: {}, workInitial: '0' }));
  if ('status' in checked) return checked;
  return { core, staticWorkBound: workBound(core) };
}
/** Trusted Σ is immutable text. Every request owns new trees, including checking
 * and returned Core. The public evaluator never consumes caller-supplied Core. */
export function createFinancialExpressionSourceV1(schemaCanonicalJSON: string) {
  const schemaIsText = typeof schemaCanonicalJSON === 'string';
  const trustedText = schemaIsText ? schemaCanonicalJSON : '';
  function transport(source: string): void {
    if (!schemaIsText) throw new ExpressionFailure('INPUT_SCHEMA');
    if (trustedText.length > COMPONENT_BYTES || bytes(trustedText) > COMPONENT_BYTES) throw new ExpressionFailure('INPUT_BOUND');
    sourceTransport(source);
  }
  function compile(source: string): SourceElaborated | SourceRejected {
    const program = parseFinancialExpressionSource(source);
    const schema: Schema = parseCanonical(trustedText, COMPONENT_BYTES);
    schemaShape(schema); validateSchema(schema); validateSourceSchemaNames(schema);
    const action = sourceAction(program, schema);
    const lowered = lowerAndCheckFinancialAction(source, action, schema, trustedText);
    if ('status' in lowered) return lowered;
    return { judgmentResult: 'SourceElaborated', sourceProfile: FINANCIAL_EXPRESSION_SOURCE_PROFILE,
      contract: FINANCIAL_EXPRESSION_CONTRACT_V1, agreement: program.agreement.name, action: action.name,
      staticWorkBound: lowered.staticWorkBound, core: lowered.core };
  }
  return Object.freeze({
    elaborate(source: string): SourceElaborated | SourceRejected {
      try { transport(source); return compile(source); } catch (error) { return rejection(error, source); }
    },
    check(source: string): SourceChecked | SourceRejected {
      try {
        transport(source);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return { judgmentResult: 'SourceChecked', sourceProfile: FINANCIAL_EXPRESSION_SOURCE_PROFILE, staticWorkBound: compiled.staticWorkBound };
      } catch (error) { return rejection(error, source); }
    },
    evaluate(source: string, snapshotCanonicalJSON: string): ExpressionResult {
      try {
        transport(source);
        const input = snapshots(snapshotCanonicalJSON);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return createFinancialExpressionContractV1(trustedText).evaluate(canonical({ contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
          source, core: compiled.core, ...input }));
      } catch (error) { return rejection(error, source); }
    },
  });
}
