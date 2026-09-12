/** Wire helpers for the proposed expression contract, independent of funded Core. */
export interface ExpressionSpan { kind: 'source' | 'synthetic'; start: string; end: string }
export const SYNTHETIC_SPAN: ExpressionSpan = Object.freeze({ kind: 'synthetic', start: '0', end: '0' });
export class ExpressionFailure extends Error {
  code: string;
  span: ExpressionSpan;
  path: number[];
  constructor(code: string, span: ExpressionSpan = SYNTHETIC_SPAN, path: number[] = []) {
    super(code); this.code = code; this.span = span; this.path = path;
  }
}
export function fail(code: string): never { throw new ExpressionFailure(code); }
export function bytes(s: string): number { return new TextEncoder().encode(s).length; }
export function scalarString(s: string): boolean {
  for (let i = 0; i < s.length; i++) {
    const c = s.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff) {
      const next = s.charCodeAt(++i);
      if (!(next >= 0xdc00 && next <= 0xdfff)) return false;
    } else if (c >= 0xdc00 && c <= 0xdfff) return false;
  }
  return true;
}
function scalarOrder(a: string, b: string): number {
  const aa = [...a], bb = [...b];
  for (let i = 0; i < Math.min(aa.length, bb.length); i++) {
    const d = aa[i].codePointAt(0)! - bb[i].codePointAt(0)!;
    if (d) return d;
  }
  return aa.length - bb.length;
}
/** Exact J spelling. Input here consists only of JSON.parse-owned data trees. */
export function canonical(value: any): string {
  // Type metadata has no semantic depth ceiling. Use an explicit traversal
  // stack; input byte bounds, rather than the host call stack, bound this work.
  const output: string[] = [];
  const pending: ({ value: any } | { text: string })[] = [{ value }];
  while (pending.length) {
    const item = pending.pop()!;
    if ('text' in item) { output.push(item.text); continue; }
    const v = item.value;
    if (typeof v === 'string') {
      if (!scalarString(v)) fail('INPUT_SCHEMA');
      output.push(JSON.stringify(v));
    } else if (typeof v === 'boolean') output.push(String(v));
    else if (Array.isArray(v)) {
      output.push('['); pending.push({ text: ']' });
      for (let i = v.length - 1; i >= 0; i--) {
        pending.push({ value: v[i] });
        if (i > 0) pending.push({ text: ',' });
      }
    } else {
      if (v === null || typeof v !== 'object') fail('INPUT_SCHEMA');
      const keys = Object.keys(v).sort(scalarOrder);
      output.push('{'); pending.push({ text: '}' });
      for (let i = keys.length - 1; i >= 0; i--) {
        pending.push({ value: v[keys[i]] }, { text: ':' }, { value: keys[i] });
        if (i > 0) pending.push({ text: ',' });
      }
    }
  }
  return output.join('');
}
export function parseCanonical(text: string, maxBytes: number): any {
  if (typeof text !== 'string') fail('INPUT_SCHEMA');
  if (text.length > maxBytes || bytes(text) > maxBytes) fail('INPUT_BOUND');
  // JSON.parse without a reviver and the iterative J writer support metadata
  // nesting throughout the admitted byte range. Core/schema/value depth limits
  // apply separately to their normative trees after transport decoding.
  let value: any;
  try { value = JSON.parse(text); } catch { fail('INPUT_SCHEMA'); }
  // Duplicate keys, whitespace, nonminimal escapes and noncanonical ordering
  // cannot survive exact equality with J of the parsed tree.
  if (canonical(value) !== text) fail('INPUT_SCHEMA');
  return value;
}
export function object(v: any): Record<string, any> {
  if (v === null || typeof v !== 'object' || Array.isArray(v)) fail('INPUT_SCHEMA');
  return v;
}
export function closed(v: any, keys: string[]): Record<string, any> {
  object(v);
  if (Object.keys(v).length !== keys.length || keys.some(k => !Object.hasOwn(v, k))) fail('INPUT_SCHEMA');
  return v;
}
export function identifier(v: any): asserts v is string {
  if (typeof v !== 'string' || !/^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(v) || v.includes('\n')) fail('INPUT_SCHEMA');
}
export function decimal(v: any): asserts v is string {
  if (typeof v !== 'string' || !/^(0|[1-9][0-9]*|-[1-9][0-9]*)$/.test(v) || v.includes('\n')) fail('INPUT_SCHEMA');
}
export function array(v: any): asserts v is any[] { if (!Array.isArray(v)) fail('INPUT_SCHEMA'); }
export function treeSize(v: any): { nodes: number; depth: number } {
  let nodes = 0, depth = 0;
  const pending = [{ value: v, depth: 1 }];
  while (pending.length) {
    const item = pending.pop()!;
    nodes++; depth = Math.max(depth, item.depth);
    const children = Array.isArray(item.value) ? item.value
      : item.value !== null && typeof item.value === 'object' ? Object.values(item.value) : [];
    for (const child of children) pending.push({ value: child, depth: item.depth + 1 });
  }
  return { nodes, depth };
}
