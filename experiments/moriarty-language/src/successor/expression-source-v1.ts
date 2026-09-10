/** Source-only entry to local /1 expression evaluation. No financial operation
 * interpreter, authorization, funded preparation, signing or ledger path is called. */
import { parseExpressionSource, EXPRESSION_SOURCE_PROFILE } from './expression-source-frontend.ts';
import { SuccessorSyntaxError } from './frontend.ts';
import type { ActionDecl, Program } from './frontend.ts';
import { createExpressionContractV1, EXPRESSION_CONTRACT_V1 } from './expression-v1.ts';
import type { ExpressionResult } from './expression-v1.ts';
import { ExpressionFailure, SYNTHETIC_SPAN, bytes, canonical, closed, decimal, parseCanonical, scalarString } from './expression-wire-v1.ts';
import { schemaShape, validateSchema, same } from './expression-types-v1.ts';
import type { Schema } from './expression-types-v1.ts';
import { createSourceLowering, sourceFailure, sourceSpan } from './expression-source-lower.ts';
import type { SourceCoreNode } from './expression-source-lower.ts';
import { sourceExtension, sourceType, reservedSourceTerm, validateSourceSchemaNames } from './expression-source-types.ts';

export type SourceRejected = Extract<ExpressionResult, { status: 'Rejected' }>;
export interface SourceElaborated {
  judgmentResult: 'SourceElaborated';
  sourceProfile: typeof EXPRESSION_SOURCE_PROFILE;
  contract: typeof EXPRESSION_CONTRACT_V1;
  agreement: string;
  action: string;
  staticWorkBound: string;
  core: { statements: SourceCoreNode[]; span: ReturnType<typeof sourceSpan> };
}
export interface SourceChecked {
  judgmentResult: 'SourceChecked';
  sourceProfile: typeof EXPRESSION_SOURCE_PROFILE;
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
    if (d.tag === 'ConstDecl' || d.tag === 'StateDecl') sourceFailure('SOURCE_DECLARATION', d.span);
    if (d.tag === 'ActionDecl') {
      if (reservedSourceTerm(d.name)) sourceFailure('SOURCE_RESERVED_NAME', d.span);
    } else {
      if (d.tag === 'AssetDecl' && (d.type.numeric || d.type.name !== 'Asset' || d.type.arguments.length))
        sourceFailure('SOURCE_DECLARATION_TYPE', d.type.span);
      const roster = d.tag === 'UnitDecl' ? schema.units : d.tag === 'PartyDecl' ? schema.parties : schema.assets;
      if (!roster.includes(d.name)) sourceFailure('SOURCE_DECLARATION_NAME', d.span);
    }
  }
  const action = actions[0], parameters = new Set<string>();
  const otherDeclarations = new Set<string>();
  for (const key of ['units', 'assets', 'vaults', 'parties']) schema[key].forEach((n: string) => otherDeclarations.add(n));
  for (const key of ['fields', 'observations', 'recordTypes', 'enumTypes', 'operations']) Object.keys(schema[key]).forEach(n => otherDeclarations.add(n));
  for (const p of action.parameters) {
    if (parameters.has(p.name)) sourceFailure('SOURCE_PARAMETER_NAMES', p.span);
    parameters.add(p.name);
    if (reservedSourceTerm(p.name) || otherDeclarations.has(p.name)) sourceFailure('SOURCE_RESERVED_NAME', p.span);
  }
  if (parameters.size !== Object.keys(schema.args).length || [...parameters].some(n => !Object.hasOwn(schema.args, n)))
    sourceFailure('SOURCE_PARAMETER_NAMES', action.span);
  for (const p of action.parameters) if (!same(sourceType(p.type), schema.args[p.name]))
    sourceFailure('SOURCE_PARAMETER_TYPE', p.type.span);
  return action;
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
/** Trusted Σ is immutable text. Every request owns new trees, including checking
 * and returned Core. The public evaluator never consumes caller-supplied Core. */
export function createExpressionSourceV1(schemaCanonicalJSON: string) {
  const schemaIsText = typeof schemaCanonicalJSON === 'string';
  const trustedText = schemaIsText ? schemaCanonicalJSON : '';
  function transport(source: string): void {
    if (!schemaIsText) throw new ExpressionFailure('INPUT_SCHEMA');
    if (trustedText.length > COMPONENT_BYTES || bytes(trustedText) > COMPONENT_BYTES) throw new ExpressionFailure('INPUT_BOUND');
    sourceTransport(source);
  }
  function compile(source: string): SourceElaborated | SourceRejected {
    const program = parseExpressionSource(source);
    const schema: Schema = parseCanonical(trustedText, COMPONENT_BYTES);
    schemaShape(schema); validateSchema(schema); validateSourceSchemaNames(schema);
    const action = sourceAction(program, schema);
    const lowering = createSourceLowering(schema, new Set(action.parameters.map(p => p.name)), sourceExtension(schema));
    const statements = [...action.statements, ...action.postconditions].map(statement => {
      if (statement.tag === 'Let' && reservedSourceTerm(statement.name)) sourceFailure('SOURCE_RESERVED_NAME', statement.span);
      return lowering.statement(statement);
    });
    const core = { statements, span: sourceSpan(action.span) };
    const checked = createExpressionContractV1(trustedText).check(canonical({ contract: EXPRESSION_CONTRACT_V1,
      source, core, Pre: {}, Args: {}, Obs: {}, workInitial: '0' }));
    if ('status' in checked) return checked;
    return { judgmentResult: 'SourceElaborated', sourceProfile: EXPRESSION_SOURCE_PROFILE,
      contract: EXPRESSION_CONTRACT_V1, agreement: program.agreement.name, action: action.name,
      staticWorkBound: workBound(core), core };
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
        return { judgmentResult: 'SourceChecked', sourceProfile: EXPRESSION_SOURCE_PROFILE, staticWorkBound: compiled.staticWorkBound };
      } catch (error) { return rejection(error, source); }
    },
    evaluate(source: string, snapshotCanonicalJSON: string): ExpressionResult {
      try {
        transport(source);
        const input = snapshots(snapshotCanonicalJSON);
        const compiled = compile(source);
        if ('status' in compiled) return compiled;
        return createExpressionContractV1(trustedText).evaluate(canonical({ contract: EXPRESSION_CONTRACT_V1,
          source, core: compiled.core, ...input }));
      } catch (error) { return rejection(error, source); }
    },
  });
}
