/** Duplicate-safe bounded JSON text transport. No caller objects are accepted. */
export class LocalError extends Error {
  code: string;
  pointer: string;
  constructor(code: string, message: string, pointer = '/') {
    super(message); this.name = 'LocalError'; this.code = code; this.pointer = pointer;
  }
}
export function scalarText(value: string): boolean {
  for (let i = 0; i < value.length; i++) {
    const c = value.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff) {
      const low = value.charCodeAt(++i);
      if (!(low >= 0xdc00 && low <= 0xdfff)) return false;
    } else if (c >= 0xdc00 && c <= 0xdfff) return false;
  }
  return true;
}
export function parseBoundedJson(text: string): unknown { return parseJsonWithinLimits(text,65536,1024); }
/** Internal native response parser. */
export function parseJsonWithinLimits(text: string, byteLimit:number, stringLimit:number): unknown {
  const fail = (code: string, message: string): never => { throw new LocalError(code, message); };
  if (typeof text !== 'string') fail('BETA_JSON_TEXT', 'JSON must be text');
  if (Buffer.byteLength(text) > byteLimit) fail('BETA_JSON_BOUND', `JSON exceeds ${byteLimit} bytes`);
  if (!scalarText(text)) fail('BETA_JSON_UNICODE', 'Invalid Unicode scalar');
  let i = 0, nodes = 0;
  const ws = (): void => { while (i < text.length && /[ \t\r\n]/.test(text[i])) i++; };
  const string = (): string => {
    const start = i++;
    let escaped = false;
    while (i < text.length) {
      const c = text[i++];
      if (c === '"' && !escaped) {
        let value: string;
        try { value = JSON.parse(text.slice(start, i)) as string; }
        catch { return fail('BETA_JSON_SYNTAX', 'Invalid JSON string'); }
        if (!scalarText(value)) fail('BETA_JSON_UNICODE', 'Invalid decoded Unicode scalar');
        if (Buffer.byteLength(value) > stringLimit) fail('BETA_JSON_STRING', `JSON string exceeds ${stringLimit} bytes`);
        return value;
      }
      if (c === '\\' && !escaped) escaped = true; else escaped = false;
    }
    return fail('BETA_JSON_SYNTAX', 'Unterminated string');
  };
  const value = (depth: number): void => {
    if (depth > 32) fail('BETA_JSON_DEPTH', 'JSON depth exceeds 32');
    if (++nodes > 4096) fail('BETA_JSON_NODES', 'JSON nodes exceed 4096');
    ws();
    if (text[i] === '"') { string(); return; }
    if (text[i] === '{') {
      i++; ws(); const keys = new Set<string>();
      if (text[i] === '}') { i++; return; }
      for (;;) {
        if (text[i] !== '"') fail('BETA_JSON_SYNTAX', 'Expected object key');
        const key = string();
        if (keys.has(key)) fail('BETA_JSON_DUPLICATE', `Duplicate JSON key ${key}`);
        if (keys.size >= 64) fail('BETA_JSON_FIELDS', 'Object fields exceed 64');
        keys.add(key); ws();
        if (text[i++] !== ':') fail('BETA_JSON_SYNTAX', 'Expected colon');
        value(depth + 1); ws();
        const c = text[i++]; if (c === '}') return;
        if (c !== ',') fail('BETA_JSON_SYNTAX', 'Expected comma');
        ws();
      }
    }
    if (text[i] === '[') {
      i++; ws(); if (text[i] === ']') { i++; return; }
      for (;;) {
        value(depth + 1); ws(); const c = text[i++];
        if (c === ']') return;
        if (c !== ',') fail('BETA_JSON_SYNTAX', 'Expected comma');
      }
    }
    const rest = text.slice(i);
    const match = /^(?:true|false|null|-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?)/.exec(rest);
    if (!match) fail('BETA_JSON_SYNTAX', 'Expected JSON value');
    i += match![0].length;
  };
  value(0); ws();
  if (i !== text.length) fail('BETA_JSON_SYNTAX', 'Trailing JSON material');
  try { return JSON.parse(text); } catch { return fail('BETA_JSON_SYNTAX', 'Invalid JSON'); }
}
