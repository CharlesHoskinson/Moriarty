/** Source lowering shared by the versioned expression frontend.
 * Financial state classification and operation schemas always come from trusted Σ.
 */
import type { Expression, Span, Statement, Ensures } from './frontend.ts';
import type { ExpressionSpan } from './expression-wire-v1.ts';
import { ExpressionFailure } from './expression-wire-v1.ts';
import type { Schema } from './expression-types-v1.ts';

export interface SourceCoreNode {
  constructor: string;
  operands: Record<string, any>;
  span: ExpressionSpan;
}
export function sourceSpan(span: Span): ExpressionSpan {
  return { kind: 'source', start: String(span.start), end: String(span.end) };
}
export function sourceFailure(code: string, span: Span): never {
  throw new ExpressionFailure(code, sourceSpan(span));
}
export function sourceNode(constructor: string, operands: Record<string, any>, span: Span): SourceCoreNode {
  return { constructor, operands, span: sourceSpan(span) };
}

const BINARY: Record<string, string> = {
  '+': 'Add', '-': 'Sub', '*': 'Mul', 'and': 'And', 'or': 'Or',
  '==': 'Eq', '<': 'Lt', '<=': 'Lte', '>': 'Gt', '>=': 'Gte',
};
/** Extend only the explicitly reviewed surface forms through the supplied callback. */
export function createSourceLowering(schema: Schema, parameters: Set<string>,
    extension: (expression: Expression, lower: (e: Expression) => SourceCoreNode) => SourceCoreNode | undefined) {
  const locals = new Set<string>();
  function lower(e: Expression): SourceCoreNode {
    if (e.tag === 'BooleanLiteral') return sourceNode('LitBool', { value: e.value }, e.span);
    if (e.tag === 'StringLiteral') return sourceNode('LitText', { value: e.decoded }, e.span);
    if (e.tag === 'Identifier') {
      if (locals.has(e.name)) return sourceNode('ReadLocal', { name: e.name }, e.span);
      if (parameters.has(e.name)) return sourceNode('ReadArg', { name: e.name }, e.span);
      return sourceNode('ReadLocal', { name: e.name }, e.span);
    }
    if (e.tag === 'Unary') return sourceNode('Not', { value: lower(e.operand) }, e.span);
    if (e.tag === 'Binary' || e.tag === 'Comparison') {
      const operands = { left: lower(e.left), right: lower(e.right) };
      if (e.operator === '!=') return sourceNode('Not', { value: sourceNode('Eq', operands, e.span) }, e.span);
      return sourceNode(BINARY[e.operator], operands, e.span);
    }
    if (e.tag === 'Projection') {
      if (e.object.tag === 'Identifier' && ['pre', 'post', 'next'].includes(e.object.name))
        return sourceNode('ReadPre', { view: e.object.name, field: e.field }, e.span);
      const projected = extension(e, lower);
      if (projected) return projected;
      return sourceNode('ProjectField', { record: lower(e.object), field: e.field }, e.span);
    }
    if (e.tag === 'Call' && ['floor_div', 'ceil_div'].includes(e.name)) {
      if (e.arguments.length !== 2) return sourceFailure('SOURCE_ARITY', e.span);
      return sourceNode(e.name === 'floor_div' ? 'FloorDiv' : 'CeilDiv',
        { left: lower(e.arguments[0]), right: lower(e.arguments[1]) }, e.span);
    }
    return extension(e, lower) ?? sourceFailure('SOURCE_CALL', e.span);
  }
  function statement(s: Statement | Ensures): SourceCoreNode {
    if (s.tag === 'Requires' || s.tag === 'Ensures')
      return sourceNode(s.tag === 'Requires' ? 'Require' : 'Ensure', { condition: lower(s.expression) }, s.span);
    if (s.tag === 'Let') {
      const node = sourceNode('Let', { name: s.name, value: lower(s.expression) }, s.span);
      locals.add(s.name);
      return node;
    }
    if (s.tag === 'Next') return sourceNode('NextWrite', { field: s.name, value: lower(s.expression) }, s.span);
    if (s.type.arguments.length || !Object.hasOwn(schema.operations, s.type.name))
      return sourceFailure('SOURCE_OPERATION', s.span);
    if (s.expression) return sourceNode('Emit', { operation: s.type.name, fields: lower(s.expression) }, s.span);
    return sourceNode('Emit', { operation: s.type.name,
      fields: sourceNode('ConstructRecord', { recordType: schema.operations[s.type.name],
        fields: s.fields.map(f => ({ name: f.name, value: lower(f.expression) })) }, s.span) }, s.span);
  }
  return { lower, statement };
}
