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
  if (typeof value === 'string') {
    if (!scalarString(value)) fail('INPUT_SCHEMA');
    return JSON.stringify(value);
  }
  if (typeof value === 'boolean') return String(value);
  if (Array.isArray(value)) return '[' + value.map(canonical).join(',') + ']';
  if (value === null || typeof value !== 'object') fail('INPUT_SCHEMA');
  return '{' + Object.keys(value).sort(scalarOrder).map(k => canonical(k) + ':' + canonical(value[k])).join(',') + '}';
}
export function parseCanonical(text: string, maxBytes: number): any {
  if (typeof text !== 'string') fail('INPUT_SCHEMA');
  if (text.length > maxBytes || bytes(text) > maxBytes) fail('INPUT_BOUND');
  // A valid Core of constructor depth64 has JSON nesting below256. This
  // transport guard is a derived safety ceiling, not a smaller semantic bound.
  let depth = 0, quoted = false, escaped = false;
  for (const ch of text) {
    if (quoted) { if (escaped) escaped = false; else if (ch === '\\') escaped = true; else if (ch === '"') quoted = false; }
    else if (ch === '"') quoted = true;
    else if (ch === '{' || ch === '[') { if (++depth > 512) fail('INPUT_BOUND'); }
    else if (ch === '}' || ch === ']') depth--;
  }
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
  const children = Array.isArray(v) ? v : v !== null && typeof v === 'object' ? Object.values(v) : [];
  let nodes = 1, depth = 1;
  for (const child of children) { const n = treeSize(child); nodes += n.nodes; depth = Math.max(depth, n.depth + 1); }
  return { nodes, depth };
}
