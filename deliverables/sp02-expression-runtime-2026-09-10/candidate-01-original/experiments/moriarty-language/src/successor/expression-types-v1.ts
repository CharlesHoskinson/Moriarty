import { array, bytes, canonical, closed, decimal, fail, identifier, object, treeSize } from './expression-wire-v1.ts';

/** Type annotations and values retain the exact representation.md JSON trees. */
export type ValueType = any[];
export type Schema = Record<string, any>;
export const SCHEMA_KEYS = ['units', 'assets', 'vaults', 'parties', 'recordTypes', 'enumTypes', 'fields', 'args', 'observations', 'operations'];
const SCALARS = ['UInt64', 'UInt128', 'SInt128', 'Bool', 'Text', 'Unit'];
export const NUMERIC = ['UInt64', 'UInt128', 'SInt128', 'Amount', 'Shares', 'Rate', 'Price', 'Quantity'];
export function same(a: ValueType, b: ValueType): boolean { return canonical(a) === canonical(b); }
export function unitShape(v: any): void {
  array(v);
  for (const item of v) { array(item); if (item.length !== 2) fail('INPUT_SCHEMA'); identifier(item[0]); decimal(item[1]); }
}
export function typeShape(t: any): asserts t is ValueType {
  array(t); if (typeof t[0] !== 'string') fail('INPUT_SCHEMA');
  const tag = t[0];
  if (SCALARS.includes(tag)) { if (t.length !== 1) fail('INPUT_SCHEMA'); }
  else if (['Amount', 'Record', 'Enum', 'Operation'].includes(tag)) {
    if (t.length !== 2) fail('INPUT_SCHEMA'); identifier(t[1]);
  } else if (tag === 'Shares') {
    if (t.length !== 3) fail('INPUT_SCHEMA'); identifier(t[1]); identifier(t[2]);
  } else if (tag === 'Rate') {
    if (t.length !== 2) fail('INPUT_SCHEMA'); decimal(t[1]);
  } else if (tag === 'Price') {
    if (t.length !== 4) fail('INPUT_SCHEMA'); identifier(t[1]); identifier(t[2]); decimal(t[3]);
  } else if (tag === 'Quantity') {
    if (t.length !== 3) fail('INPUT_SCHEMA'); unitShape(t[1]); decimal(t[2]);
  } else if (tag === 'Option') {
    if (t.length !== 2) fail('INPUT_SCHEMA'); typeShape(t[1]);
  } else if (tag === 'Collection') {
    if (t.length !== 3) fail('INPUT_SCHEMA'); typeShape(t[1]); decimal(t[2]);
  } else fail('INPUT_SCHEMA');
}
function mapShape(v: any): void { object(v); for (const k of Object.keys(v)) identifier(k); }
export function schemaShape(s: any): asserts s is Schema {
  closed(s, SCHEMA_KEYS);
  if (bytes(canonical(s)) > 65536) fail('INPUT_BOUND');
  const size = treeSize(s); if (size.nodes > 4096 || size.depth > 64) fail('INPUT_BOUND');
  for (const key of ['units', 'assets', 'vaults', 'parties']) { array(s[key]); s[key].forEach(identifier); }
  for (const key of ['recordTypes', 'enumTypes', 'fields', 'args', 'observations', 'operations']) mapShape(s[key]);
  for (const r of Object.values(s.recordTypes)) {
    mapShape(r); if (Object.keys(r as object).length > 64) fail('INPUT_BOUND'); Object.values(r as object).forEach(typeShape);
  }
  for (const e of Object.values(s.enumTypes)) { array(e); if (e.length > 128) fail('INPUT_BOUND'); e.forEach(identifier); }
  for (const f of Object.values(s.fields)) {
    closed(f, ['type', 'writeClass']); const field = f as any; typeShape(field.type);
    if (!['ordinary', 'financial'].includes(field.writeClass)) fail('INPUT_SCHEMA');
  }
  Object.values(s.args).forEach(typeShape); Object.values(s.observations).forEach(typeShape);
  Object.values(s.operations).forEach(identifier);
}
function declared(map: any, name: string): void { if (!Object.hasOwn(map, name)) fail('TYPE_NAME'); }
export function unitDomain(units: any[], s: Schema, code: string): void {
  if (units.length > 8) fail(code);
  let previous = '';
  for (const [name, power] of units) {
    const p = BigInt(power);
    if (name <= previous || p === 0n || p < -128n || p > 127n) fail(code);
    if (!s.units.includes(name)) fail('TYPE_NAME');
    previous = name;
  }
}
export function resolve(t: ValueType, s: Schema, domainCode = 'TYPE_LITERAL'): void {
  const tag = t[0];
  if (tag === 'Unit') fail('TYPE_NAME');
  if (['UInt64', 'UInt128', 'SInt128', 'Bool', 'Text'].includes(tag)) return;
  if (tag === 'Amount') { if (!s.assets.includes(t[1])) fail('TYPE_NAME'); }
  else if (tag === 'Shares') { if (!s.vaults.includes(t[1]) || !s.parties.includes(t[2])) fail('TYPE_NAME'); }
  else if (tag === 'Rate' || tag === 'Quantity' || tag === 'Price') {
    if (tag === 'Quantity') unitDomain(t[1], s, domainCode);
    if (tag === 'Price') {
      if (!s.assets.includes(t[1]) || !s.assets.includes(t[2])) fail('TYPE_NAME');
      if (t[1] === t[2]) fail(domainCode);
    }
    const scale = BigInt(t[tag === 'Rate' ? 1 : tag === 'Quantity' ? 2 : 3]);
    if (scale < 0n || scale > 18n) fail(domainCode);
  } else if (tag === 'Record') declared(s.recordTypes, t[1]);
  else if (tag === 'Enum') declared(s.enumTypes, t[1]);
  else if (tag === 'Operation') declared(s.operations, t[1]);
  else if (tag === 'Option' || tag === 'Collection') {
    resolve(t[1], s, domainCode);
    if (tag === 'Collection' && (BigInt(t[2]) < 0n || BigInt(t[2]) > 128n)) fail('TYPE_COLLECTION_BOUND');
  } else fail('TYPE_NAME');
}
export function validateSchema(s: Schema): void {
  let declarations = 0;
  for (const key of SCHEMA_KEYS) declarations += Array.isArray(s[key]) ? s[key].length : Object.keys(s[key]).length;
  if (declarations > 256) fail('INPUT_BOUND');
  function sortedUnique(items: string[]): void {
    if (items.some((x, i) => i > 0 && x <= items[i - 1])) fail('INPUT_SCHEMA');
  }
  for (const key of ['units', 'assets', 'vaults', 'parties']) sortedUnique(s[key]);
  for (const e of Object.values(s.enumTypes)) sortedUnique(e as string[]);
  // Walk declarations in identifier order, including all nested type references.
  const definitions: [string, ValueType][] = [];
  for (const [name, fields] of Object.entries(s.recordTypes))
    for (const [field, t] of Object.entries(fields as object)) definitions.push([name + '.' + field, t]);
  for (const [name, record] of Object.entries(s.operations)) definitions.push([name, ['Record', record]]);
  for (const key of ['args', 'observations'])
    for (const [name, t] of Object.entries(s[key])) definitions.push([name, t as ValueType]);
  for (const [name, f] of Object.entries(s.fields)) definitions.push([name, (f as any).type]);
  for (const [, t] of definitions.sort((a, b) => a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0)) resolve(t, s);
  const visiting = new Set<string>(), done = new Set<string>();
  function walkType(t: ValueType): void {
    if (t[0] === 'Option' || t[0] === 'Collection') walkType(t[1]);
    else if (t[0] === 'Record') walk('R:' + t[1]);
    else if (t[0] === 'Operation') walk('O:' + t[1]);
  }
  function walk(name: string): void {
    if (visiting.has(name)) fail('TYPE_SCHEMA_CYCLE');
    if (done.has(name)) return;
    visiting.add(name);
    const key = name.slice(2);
    if (name.startsWith('R:')) for (const field of Object.keys(s.recordTypes[key]).sort()) walkType(s.recordTypes[key][field]);
    else walk('R:' + s.operations[key]);
    visiting.delete(name); done.add(name);
  }
  for (const name of [...Object.keys(s.recordTypes).map(x => 'R:' + x), ...Object.keys(s.operations).map(x => 'O:' + x)].sort()) walk(name);
}
export function numericFits(t: ValueType, value: string): boolean {
  const n = BigInt(value), tag = t[0];
  if (['SInt128', 'Quantity', 'Rate'].includes(tag)) return n >= -(1n << 127n) && n < (1n << 127n);
  return n >= 0n && n < (1n << (tag === 'UInt64' ? 64n : 128n));
}
export function valueChildren(t: ValueType, value: any, s: Schema): [ValueType, any][] {
  if (t[0] === 'Record') return Object.keys(s.recordTypes[t[1]]).sort().map(k => [s.recordTypes[t[1]][k], value[k]]);
  if (t[0] === 'Option' || t[0] === 'Collection') return value.map((v: any) => [t[1], v]);
  if (t[0] === 'Operation') return [[['Record', s.operations[t[1]]], value.fields]];
  return [];
}
export function valueShape(t: ValueType, value: any, s: Schema): void {
  const tag = t[0];
  if (NUMERIC.includes(tag)) decimal(value);
  else if (tag === 'Bool') { if (typeof value !== 'boolean') fail('INPUT_SCHEMA'); }
  else if (tag === 'Text' || tag === 'Enum') { if (typeof value !== 'string') fail('INPUT_SCHEMA'); }
  else if (tag === 'Record') closed(value, Object.keys(s.recordTypes[t[1]]));
  else if (tag === 'Option') { array(value); if (value.length > 1) fail('INPUT_SCHEMA'); }
  else if (tag === 'Collection') { array(value); if (BigInt(value.length) > BigInt(t[2])) fail('INPUT_SCHEMA'); }
  else if (tag === 'Operation') {
    closed(value, ['operation', 'fields']); if (value.operation !== t[1]) fail('INPUT_SCHEMA');
  } else fail('INPUT_SCHEMA');
  for (const [childType, child] of valueChildren(t, value, s)) valueShape(childType, child, s);
}
export function valueDomain(t: ValueType, value: any, s: Schema, code: string): void {
  if (NUMERIC.includes(t[0]) && !numericFits(t, value)) fail(code);
  if (t[0] === 'Text' && bytes(value) > 1024) fail(code);
  if (t[0] === 'Enum' && !s.enumTypes[t[1]].includes(value)) fail(code);
  for (const [ct, child] of valueChildren(t, value, s)) valueDomain(ct, child, s, code);
}
export function valueSize(t: ValueType, value: any, s: Schema): { nodes: number; depth: number } {
  let nodes = 1, depth = 1;
  for (const [ct, child] of valueChildren(t, value, s)) {
    const n = valueSize(ct, child, s); nodes += n.nodes; depth = Math.max(depth, n.depth + 1);
  }
  return { nodes, depth };
}
export function valueBound(t: ValueType, value: any, s: Schema, code: string): void {
  const size = valueSize(t, value, s);
  if (size.nodes > 4096 || size.depth > 64 || bytes(canonical({ type: t, value })) > 65536) fail(code);
}
export function aggregateBound(types: Record<string, ValueType>, values: Record<string, any>, s: Schema, code: string): void {
  let nodes = 1, depth = 1;
  for (const key of Object.keys(types).sort()) {
    valueBound(types[key], values[key], s, code);
    const n = valueSize(types[key], values[key], s); nodes += n.nodes; depth = Math.max(depth, n.depth + 1);
  }
  if (nodes > 4096 || depth > 64 || bytes(canonical(values)) > 65536) fail(code);
}
export function validateSnapshot(types: Record<string, ValueType>, values: any, s: Schema): void {
  closed(values, Object.keys(types));
  for (const key of Object.keys(types).sort()) valueShape(types[key], values[key], s);
  aggregateBound(types, values, s, 'INPUT_BOUND');
  for (const key of Object.keys(types).sort()) valueDomain(types[key], values[key], s, 'INPUT_VALUE');
}
