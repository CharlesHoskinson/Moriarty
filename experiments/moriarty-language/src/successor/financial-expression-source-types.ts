/** Exact source spellings for the separately reviewed financial expression profile. */
import { GENERIC_PRIMARIES, FINANCIAL_GENERIC_PRIMARIES, SOURCE_KEYWORDS } from './frontend.ts';
import type { Expression, TypeNode, Span } from './frontend.ts';
import type { Schema, ValueType } from './financial-expression-types-v1.ts';
import { ExpressionFailure } from './expression-wire-v1.ts';
import { sourceFailure, sourceNode } from './expression-source-lower.ts';
import type { SourceCoreNode } from './expression-source-lower.ts';

const SCALARS = ['UInt64', 'UInt128', 'UInt256', 'SInt128', 'Bool', 'Text', 'Unit'];
const INTRINSICS = ['u256', 'some_value', 'quanta', 'mantissa', 'is_negative', 'magnitude', ...FINANCIAL_GENERIC_PRIMARIES, 'u64', 'u128', 'i128', 'amount', 'shares', 'rate', 'price',
  'access_field', 'access_index', 'floor_div', 'ceil_div', ...GENERIC_PRIMARIES];
const TERM_RESERVED = new Set([...SOURCE_KEYWORDS, ...INTRINSICS, ...SCALARS,
  'pre', 'post', 'obs', 'Amount', 'Shares', 'Rate', 'Price', 'Quantity', 'Units',
  'Record', 'Enum', 'Operation', 'Option', 'Collection', 'Asset', 'Variant', 'AmountProduct', 'ScaledAmount', 'SignedScaledAmount', 'SignedAmount', 'NetAmount']);
export function reservedSourceTerm(name: string): boolean { return TERM_RESERVED.has(name); }
const METADATA_RESERVED = new Set([...SOURCE_KEYWORDS, ...GENERIC_PRIMARIES, ...FINANCIAL_GENERIC_PRIMARIES]);
export function asciiIdentifier(s: string): boolean {
  return /^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(s) && !/[\r\n\u2028\u2029]/.test(s);
}
export function validateSourceSchemaNames(s: Schema): void {
  const names: string[] = [];
  for (const key of ['units', 'assets', 'vaults', 'parties']) names.push(...s[key]);
  for (const key of ['recordTypes', 'enumTypes', 'variantTypes', 'fields', 'args', 'observations', 'operations']) names.push(...Object.keys(s[key]));
  for (const fields of Object.values(s.recordTypes)) names.push(...Object.keys(fields as object));
  for (const cases of Object.values(s.variantTypes)) names.push(...Object.keys(cases as object));
  for (const members of Object.values(s.enumTypes)) names.push(...members as string[]);
  if (names.some(n => !asciiIdentifier(n) || METADATA_RESERVED.has(n))) throw new ExpressionFailure('SOURCE_SCHEMA_NAME');
  const prefixes = new Set(['pre', 'post', 'obs']);
  for (const key of ['units', 'assets', 'vaults', 'parties']) s[key].forEach((n: string) => prefixes.add(n));
  for (const key of ['recordTypes', 'variantTypes', 'fields', 'args', 'observations', 'operations']) Object.keys(s[key]).forEach(n => prefixes.add(n));
  if (Object.keys(s.enumTypes).some(n => prefixes.has(n))) throw new ExpressionFailure('SOURCE_SCHEMA_NAME');
}
function arity(t: TypeNode, n: number): TypeNode[] {
  if (t.arguments.length !== n) sourceFailure('SOURCE_TYPE_SHAPE', t.span);
  return t.arguments;
}
function nominal(t: TypeNode): string {
  if (t.numeric || t.arguments.length) sourceFailure('SOURCE_TYPE_SHAPE', t.span);
  return t.name;
}
function numeric(t: TypeNode): string {
  if (!t.numeric || t.arguments.length || t.name === '-0') sourceFailure('SOURCE_TYPE_SHAPE', t.span);
  return t.name;
}
function units(t: TypeNode): string[][] {
  if (t.numeric || t.name !== 'Units' || t.arguments.length % 2) sourceFailure('SOURCE_TYPE_SHAPE', t.span);
  const pairs: string[][] = [];
  for (let i = 0; i < t.arguments.length; i += 2) pairs.push([nominal(t.arguments[i]), numeric(t.arguments[i + 1])]);
  return pairs;
}
export function sourceType(t: TypeNode): ValueType {
  if (t.numeric) return sourceFailure('SOURCE_TYPE_SHAPE', t.span);
  if (SCALARS.includes(t.name)) { arity(t, 0); return [t.name]; }
  if (['Amount', 'Record', 'Enum', 'Operation', 'Variant', 'SignedAmount', 'NetAmount'].includes(t.name)) return [t.name, nominal(arity(t, 1)[0])];
  if (t.name === 'AmountProduct') { const a = arity(t, 2); return [t.name, nominal(a[0]), nominal(a[1])]; }
  if (['ScaledAmount', 'SignedScaledAmount'].includes(t.name)) { const a = arity(t, 2); return [t.name, nominal(a[0]), numeric(a[1])]; }
  if (t.name === 'Shares') { const a = arity(t, 2); return ['Shares', nominal(a[0]), nominal(a[1])]; }
  if (t.name === 'Rate') return ['Rate', numeric(arity(t, 1)[0])];
  if (t.name === 'Price') { const a = arity(t, 3); return ['Price', nominal(a[0]), nominal(a[1]), numeric(a[2])]; }
  if (t.name === 'Quantity') { const a = arity(t, 2); return ['Quantity', units(a[0]), numeric(a[1])]; }
  if (t.name === 'Option') return ['Option', sourceType(arity(t, 1)[0])];
  if (t.name === 'Collection') { const a = arity(t, 2); return ['Collection', sourceType(a[0]), numeric(a[1])]; }
  return sourceFailure('SOURCE_TYPE_SHAPE', t.span);
}
function integer(e: Expression, unsigned = false): string {
  if (e.tag !== 'IntegerLiteral' || e.value === '-0' || (unsigned && e.value.startsWith('-')))
    return sourceFailure('SOURCE_LITERAL_SHAPE', e.span);
  return e.value;
}
function identity(e: Expression): string {
  if (e.tag !== 'Identifier') return sourceFailure('SOURCE_LITERAL_SHAPE', e.span);
  return e.name;
}
function count(actual: number, expected: number, span: Span): void {
  if (actual !== expected) sourceFailure('SOURCE_ARITY', span);
}
export function sourceExtension(schema: Schema) {
  return (e: Expression, lower: (e: Expression) => SourceCoreNode): SourceCoreNode | undefined => {
    const n = (tag: string, operands: Record<string, any>) => sourceNode(tag, operands, e.span);
    if (e.tag === 'Conditional') return n('Select', { condition: lower(e.condition), consequent: lower(e.consequent), alternative: lower(e.alternative) });
    if (e.tag === 'IntegerLiteral') {
      const value = integer(e);
      return value.startsWith('-') ? n('LitSInt', { value }) : n('LitUInt', { width: '128', value });
    }
    if (e.tag === 'Index') return n('ProjectIndex', { collection: lower(e.object), index: lower(e.index) });
    if (e.tag === 'RecordExpression') return n('ConstructRecord', { recordType: nominal(e.recordType),
      fields: e.fields.map(f => ({ name: f.name, value: lower(f.expression) })) });
    if (e.tag === 'Projection' && e.object.tag === 'Identifier') {
      if (e.object.name === 'obs') return n('ReadObs', { name: e.field });
      if (Object.hasOwn(schema.enumTypes, e.object.name)) return n('ConstructEnum', { enumType: e.object.name, member: e.field });
    }
    if (e.tag !== 'Call') return undefined;
    const args = e.arguments, types = e.typeArguments ?? [];
    if (['variant', 'project_variant', 'to_uint'].includes(e.name) || (['amount', 'shares'].includes(e.name) && types.length)) {
      count(types.length, ['variant', 'shares'].includes(e.name) ? 2 : 1, e.span);
      count(args.length, 1, e.span);
      if (e.name === 'variant') return n('ConstructVariant', { family: nominal(types[0]), tag: nominal(types[1]), value: lower(args[0]) });
      if (e.name === 'project_variant') return n('ProjectVariant', { tag: nominal(types[0]), value: lower(args[0]) });
      if (e.name === 'to_uint') return n('ConvertUInt', { width: numeric(types[0]), value: lower(args[0]) });
      if (e.name === 'amount') return n('ConstructAmount', { asset: nominal(types[0]), value: lower(args[0]) });
      return n('ConstructShares', { vault: nominal(types[0]), holder: nominal(types[1]), value: lower(args[0]) });
    }
    if (['some_value', 'quanta', 'mantissa', 'is_negative', 'magnitude', 'u256'].includes(e.name)) {
      count(types.length, 0, e.span); count(args.length, 1, e.span);
      if (e.name === 'u256') return n('LitUInt', { width: '256', value: integer(args[0]) });
      if (e.name === 'some_value') return n('ProjectSome', { value: lower(args[0]) });
      return n('ScalarValue', { component: e.name === 'is_negative' ? 'negative' : e.name, value: lower(args[0]) });
    }

    if (['some', 'none', 'collection', 'quantity'].includes(e.name)) {
      count(types.length, ['collection', 'quantity'].includes(e.name) ? 2 : 1, e.span);
      if (e.name === 'collection') return n('ConstructCollection', { elementType: sourceType(types[0]), capacity: numeric(types[1]), items: args.map(lower) });
      count(args.length, e.name === 'none' ? 0 : 1, e.span);
      if (e.name === 'quantity') return n('LitQuantity', { units: units(types[0]), scale: numeric(types[1]), mantissa: integer(args[0]) });
      return e.name === 'some' ? n('ConstructSome', { elementType: sourceType(types[0]), value: lower(args[0]) })
        : n('ConstructNone', { elementType: sourceType(types[0]) });
    }
    count(types.length, 0, e.span);
    if (['u64', 'u128', 'i128'].includes(e.name)) {
      count(args.length, 1, e.span);
      return e.name === 'i128' ? n('LitSInt', { value: integer(args[0]) })
        : n('LitUInt', { width: e.name === 'u64' ? '64' : '128', value: integer(args[0]) });
    }
    if (e.name === 'amount') {
      count(args.length, 2, e.span);
      return n('LitAmount', { value: integer(args[0], true), asset: identity(args[1]) });
    }
    if (e.name === 'shares') {
      count(args.length, 3, e.span);
      return n('LitShares', { value: integer(args[0], true), vault: identity(args[1]), holder: identity(args[2]) });
    }
    if (e.name === 'rate') {
      count(args.length, 2, e.span);
      return n('LitRate', { mantissa: integer(args[0]), scale: integer(args[1], true) });
    }
    if (e.name === 'price') {
      count(args.length, 4, e.span);
      return n('LitPrice', { mantissa: integer(args[0], true), base: identity(args[1]), quote: identity(args[2]), scale: integer(args[3], true) });
    }
    if (e.name === 'access_index') {
      count(args.length, 2, e.span);
      return n('AccessIndex', { collection: lower(args[0]), index: lower(args[1]) });
    }
    if (e.name === 'access_field') {
      count(args.length, 2, e.span);
      const record = lower(args[0]);
      const field = args[1];
      if (field.tag !== 'StringLiteral' || !asciiIdentifier(field.decoded)) return sourceFailure('SOURCE_LITERAL_SHAPE', field.span);
      return n('AccessField', { record, field: field.decoded });
    }
    return undefined;
  };
}
