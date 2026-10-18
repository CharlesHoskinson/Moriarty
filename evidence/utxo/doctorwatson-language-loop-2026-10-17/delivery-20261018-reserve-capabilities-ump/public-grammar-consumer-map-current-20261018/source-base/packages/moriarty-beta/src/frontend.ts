import { createHash } from 'node:crypto';

export type Span = { start: number; end: number };
export type Diagnostic = { code: string; message: string; span: Span };
export type Value = { span: Span } & (
  | { tag: 'string'; value: string } | { tag: 'bool'; value: boolean }
  | { tag: 'scalar'; value: string } | { tag: 'qty'; asset: string; atoms: string }
  | { tag: 'tag'; name: 'None' | 'SuccessOnly' }
  | { tag: 'array'; items: Value[] } | { tag: 'record'; fields: Record<string, Value> }
  | { tag: 'call'; name: string; args: Record<string, Value> }
  | { tag: 'entity'; kind: string; name: string; fields: Record<string, Value> });
export type Declaration = { kind: string; name: string; span: Span; nameSpan: Span; value: Value };
export type Action = { name: string; intent: string; span: Span; nameSpan: Span; support: 'LocalS0' | 'SpecifiedOnly' };
export type Reference = { name: string; useSpan: Span; declarationSpan: Span };
export type FieldUse = { containerSpan: Span; key: string; span: Span };
export type Analysis = { status: 'AuthoringChecked' | 'AuthoringRejected'; sourceHash: string; diagnostics: Diagnostic[]; agreement: string | null; agreementSpan: Span | null; declarations: Declaration[]; actions: Action[]; references: Reference[]; fieldUses: FieldUse[] };

const U128 = (1n << 128n) - 1n;
const S127 = (1n << 127n) - 1n;
const bytes = (s: string) => Buffer.byteLength(s, 'utf8');
const SOURCE6_RESERVED = new Set(('profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head post_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head').split(' '));
const KINDS = new Set('domain account asset const intent obligation pool instrument observation policy grant stage episode party share_class'.split(' '));
const RESERVED = new Set([...KINDS, 'profile', 'agreement', 'action', 'uses', 'true', 'false', 'None', 'SuccessOnly']);
type Fields = Record<string, Value>;
type Schema = Record<string, string>;
// Roles are authoring checks; every non-S0 financial relation remains open.
const SCHEMAS: Record<string, Schema> = {
  atoms: { asset: 'asset', value: 'scalar' }, min: { a: 'numeric', b: 'numeric' }, max: { a: 'numeric', b: 'numeric' },
  rounds: { domain: 'domain', from: 'scalar', to: 'scalar' },
  transfer: { from: 'account', to: 'account', fee_to: 'account', value: 'qty', fee: 'qty' },
  repay: { obligation: 'obligation', payer: 'account', amount: 'qty' },
  'amm.swap_exact_input': { pool: 'pool', owner: 'account', input: 'qty', output_asset: 'asset', net_floor: 'qty', fee_cap: 'qty' },
  'amm.redeem': { pool: 'pool', owner: 'account', share_atoms: 'scalar' },
  'amm.mint': { pool: 'pool', owner: 'account', amounts: 'qty[]', minimum_share_atoms: 'scalar' },
  'lending.originate': { obligation: 'obligation', debtor: 'account', creditor: 'account', principal: 'qty', collateral: 'qty' },
  'lending.liquidate': { obligation: 'obligation' },
  'lending.roll_forward': { obligation: 'obligation', to_round: 'scalar' },
  'stablecoin.mint': { instrument: 'instrument', owner: 'account', supply: 'qty', backing: 'qty' },
  'stablecoin.redeem': { instrument: 'instrument', owner: 'account', burn: 'qty', minimum_backing: 'qty' },
  'stablecoin.emergency_settle': { instrument: 'instrument', owner: 'account', claim: 'qty' },
  'option.fix': { instrument: 'instrument', observation: 'observation' },
  'option.exercise': { instrument: 'instrument', holder: 'account' },
  'option.settle': { instrument: 'instrument', holder: 'account', payoff: 'qty' },
  'oracle.select': { observation: 'observation' },
  'governance.queue': { policy: 'policy', next_epoch: 'scalar' },
  'governance.execute': { policy: 'policy' }, 'governance.veto': { policy: 'policy' },
  'bridge.escrow': { owner: 'account', amount: 'qty', destination: 'domain', claim_id: 'string' },
  'bridge.claim': { owner: 'account', amount: 'qty', source: 'domain', claim_id: 'string' },
  'bridge.recover': { owner: 'account', amount: 'qty', claim_id: 'string' },
  'staking.deposit': { owner: 'account', shares: 'share_class', backing: 'qty' },
  'staking.reward': { shares: 'share_class', amount: 'qty' },
  'staking.slash': { shares: 'share_class', amount: 'qty' },
  'staking.unbond': { owner: 'account', shares: 'share_class', share_atoms: 'scalar' },
  'staking.withdraw': { owner: 'account', shares: 'share_class', share_atoms: 'scalar', minimum_backing: 'qty' },
};
for (const schema of Object.values(SCHEMAS)) Object.freeze(schema);
Object.freeze(SCHEMAS);
const HORIZON = new Set(Object.keys(SCHEMAS).filter(n => n.includes('.')));
class Failure extends Error { diagnostic: Diagnostic; constructor(code: string, message: string, span: Span) { super(message); this.diagnostic = { code, message, span }; } }
function fail(code: string, message: string, span: Span): never { throw new Failure(code, message, span); }
function unicode(s: string, span: Span): void {
  for (let i = 0; i < s.length; i++) {
    const c = s.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff && s.charCodeAt(i + 1) >= 0xdc00 && s.charCodeAt(i + 1) <= 0xdfff) { i++; continue; }
    if (c >= 0xd800 && c <= 0xdfff) fail('BETA_UNICODE', 'Lone Unicode surrogate', span);
  }
}
type Token = { kind: 'word' | 'number' | 'string' | 'punct' | 'comment' | 'eof'; raw: string; value: string; span: Span };
function lex(source: string): Token[] {
  if (bytes(source) > 65536) fail('BETA_SOURCE_BOUND', 'Source exceeds 65536 bytes', { start: 0, end: 0 });
  unicode(source, { start: 0, end: bytes(source) });
  const tokens: Token[] = [];
  let i = 0, offset = 0;
  const consume = (end: number, kind?: Token['kind'], value?: string) => {
    const raw = source.slice(i, end), start = offset;
    offset += bytes(raw); i = end;
    if (kind) {
      if (tokens.length >= 8191) fail('BETA_TOKEN_BOUND', 'Source exceeds 8192 tokens including trivia and EOF', { start, end: offset });
      tokens.push({ kind, raw, value: value ?? raw, span: { start, end: offset } });
    }
  };
  while (i < source.length) {
    const c = source[i];
    if (/[ \t\r\n]/.test(c)) { consume(i + 1); continue; }
    if (source.startsWith('//', i)) { let end = i + 2; while (end < source.length && source[end] !== '\r' && source[end] !== '\n') end++; consume(end, 'comment'); continue; }
    if (source.startsWith('/*', i)) {
      const end = source.indexOf('*/', i + 2);
      if (end < 0) fail('BETA_COMMENT', 'Unterminated block comment', { start: offset, end: bytes(source) });
      consume(end + 2, 'comment'); continue;
    }
    if (c === '"') {
      let end = i + 1, escaped = false, closed = false;
      for (; end < source.length; end++) {
        const x = source[end];
        if (x === '"' && !escaped) { end++; closed = true; break; }
        escaped = x === '\\' && !escaped;
      }
      const span = { start: offset, end: offset + bytes(source.slice(i, end)) };
      if (!closed) fail('BETA_STRING', 'Unterminated string', span);
      let value: string;
      try { value = JSON.parse(source.slice(i, end)); } catch { fail('BETA_STRING', 'Invalid JSON string', span); }
      unicode(value!, span);
      if (bytes(value!) > 1024) fail('BETA_STRING_BOUND', 'Decoded string exceeds 1024 bytes', span);
      consume(end, 'string', value!); continue;
    }
    if (/[0-9]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[0-9_.]/.test(source[end])) end++;
      const raw = source.slice(i, end), plain = raw.replaceAll('_', '');
      if (!/^(0|[1-9][0-9]*(?:_[0-9]+)*)(?:\.[0-9]+(?:_[0-9]+)*)?$/.test(raw) || !/^(0|[1-9][0-9]*)(?:\.[0-9]+)?$/.test(plain)) fail('BETA_NUMBER', 'Noncanonical numeric spelling', { start: offset, end: offset + bytes(raw) });
      consume(end, 'number', plain); continue;
    }
    if (/[A-Za-z]/.test(c)) {
      let end = i + 1; while (end < source.length && /[A-Za-z0-9_]/.test(source[end])) end++;
      if (end - i > 64) fail('BETA_IDENTIFIER_BOUND', 'Identifier exceeds 64 characters', { start: offset, end: offset + end - i });
      consume(end, 'word'); continue;
    }
    if ('{}[]():;,=<>.+-*'.includes(c)) { consume(i + 1, 'punct'); continue; }
    fail('BETA_CHARACTER', 'Unsupported source character; identifiers use ASCII and division is not supported', { start: offset, end: offset + bytes(String.fromCodePoint(source.codePointAt(i)!)) });
  }
  tokens.push({ kind: 'eof', raw: '', value: '', span: { start: offset, end: offset } }); return tokens;
}
function immutable<T extends Value>(v: T): T {
  Object.freeze(v.span);
  if (v.tag === 'array') Object.freeze(v.items);
  if (v.tag === 'record' || v.tag === 'entity') Object.freeze(v.fields);
  if (v.tag === 'call') Object.freeze(v.args);
  return Object.freeze(v);
}
function entity(v: Value, kind: string): Extract<Value, { tag: 'entity' }> {
  if (v.tag !== 'entity' || v.kind !== kind) fail('BETA_TYPE', `Expected ${kind} reference`, v.span); return v;
}
function scalar(v: Value): bigint { if (v.tag !== 'scalar') fail('BETA_TYPE', 'Expected unsigned scalar', v.span); return BigInt(v.value); }
function string(v: Value): string { if (v.tag !== 'string') fail('BETA_TYPE', 'Expected string', v.span); return v.value; }
function domain(v: Value): string { return entity(v, 'domain').name; }
function qty(v: Value, asset?: string, narrow = false): bigint {
  if (v.tag !== 'qty') fail('BETA_TYPE', 'Expected nominal Qty', v.span);
  if (asset !== undefined && v.asset !== asset) fail('BETA_ASSET_MISMATCH', `Expected Qty<${asset}>, received Qty<${v.asset}>`, v.span);
  const n = BigInt(v.atoms); if (narrow && n > S127) fail('BETA_S127_BOUND', 'S0 field exceeds 2^127-1', v.span); return n;
}
function transport(s: string, span: Span): void { if (!/^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(s) || SOURCE6_RESERVED.has(s)) fail('BETA_TRANSPORT_ID', 'Identifier cannot be represented by unchanged Source/6', span); }
function fields(f: Fields, required: string[], optional: string[], span: Span): void {
  for (const k of Object.keys(f)) if (!required.includes(k) && !optional.includes(k)) fail('BETA_UNKNOWN_FIELD', `Unknown field ${k}`, f[k].span);
  for (const k of required) if (!Object.hasOwn(f, k)) fail('BETA_MISSING_FIELD', `Missing field ${k}`, span);
}
type Annotation = { name: string; args: Annotation[]; span: Span };
class Parser {
  tokens: Token[]; pos = 0; nodes = 0; edges = 0; declarations: Declaration[] = []; actions: Action[] = []; references: Reference[] = []; names = new Map<string, Declaration>(); identities = new Set<string>(); agreement: string | null = null; agreementSpan: Span | null = null;
  fieldUses: FieldUse[] = [];
  constructor(tokens: Token[]) { this.tokens = tokens.filter(t => t.kind !== 'comment'); }
  peek(): Token { return this.tokens[this.pos]; }
  take(): Token { return this.tokens[this.pos++]; }
  eat(raw: string): boolean { if (this.peek().raw === raw) { this.take(); return true; } return false; }
  expect(raw: string): Token { const t = this.peek(); if (!this.eat(raw)) fail('BETA_SYNTAX', `Expected ${raw}`, t.span); return t; }
  name(): Token { const t = this.take(); if (t.kind !== 'word' || RESERVED.has(t.raw)) fail('BETA_IDENTIFIER', 'Expected declaration identifier', t.span); return t; }
  node(depth: number): void { if (depth > 64) fail('BETA_DEPTH_BOUND', 'Nesting exceeds 64', this.peek().span); if (++this.nodes > 8192) fail('BETA_NODE_BOUND', 'Analysis exceeds 8192 nodes', this.peek().span); if (++this.edges > 32768) fail('BETA_EDGE_BOUND', 'Analysis exceeds 32768 expression edges', this.peek().span); }
  value(v: Value): Value { return immutable(v); }
  lookup(t: Token): Value { const d = this.names.get(t.raw); if (!d) fail('BETA_REFERENCE', `Unknown or forward reference ${t.raw}`, t.span); if (++this.edges > 32768) fail('BETA_EDGE_BOUND', 'Analysis exceeds 32768 expression edges', t.span); this.references.push(Object.freeze({ name: t.raw, useSpan: Object.freeze(t.span), declarationSpan: Object.freeze(d.span) })); return d.value; }
  annotation(depth: number): Annotation {
    this.node(depth); const t = this.take(); if (t.kind !== 'word') fail('BETA_ANNOTATION', 'Expected type name', t.span);
    const args: Annotation[] = [];
    if (this.eat('<')) { do { if (args.length >= 64) fail('BETA_FIELD_BOUND', 'Too many type arguments', this.peek().span); args.push(this.annotation(depth + 1)); } while (this.eat(',')); this.expect('>'); }
    return { name: t.raw, args, span: { start: t.span.start, end: this.tokens[this.pos - 1].span.end } };
  }
  checkAnnotation(a: Annotation, v: Value): void {
    if (a.name === 'Qty' && a.args.length === 1 && a.args[0].args.length === 0) { const ref = this.names.get(a.args[0].name); if (!ref) fail('BETA_REFERENCE', `Unknown type asset ${a.args[0].name}`, a.span); this.lookup({ kind: 'word', raw: a.args[0].name, value: a.args[0].name, span: a.args[0].span }); entity(ref.value, 'asset'); qty(v, ref.name); return; }
    const simple: Record<string, string> = { Scalar: 'scalar', UInt128: 'scalar', String: 'string', Bool: 'bool' };
    if (a.args.length === 0 && simple[a.name]) { if (v.tag !== simple[a.name]) fail('BETA_ANNOTATION', `Value does not match ${a.name}`, v.span); return; }
    const kinds: Record<string, string> = { Domain: 'domain', Account: 'account', Asset: 'asset', Obligation: 'obligation', Pool: 'pool', Instrument: 'instrument', Observation: 'observation', Policy: 'policy', Grant: 'grant', Stage: 'stage', Episode: 'episode', Party: 'party', ShareClass: 'share_class' };
    if (kinds[a.name] && a.args.length === 0) { entity(v, kinds[a.name]); return; }
    if (['Account', 'Asset', 'Obligation'].includes(a.name) && a.args.length === 1 && !a.args[0].args.length) { const e = entity(v, kinds[a.name]); const d = this.names.get(a.args[0].name); if (!d) fail('BETA_REFERENCE', 'Unknown annotation domain', a.span); this.lookup({ kind: 'word', raw: a.args[0].name, value: a.args[0].name, span: a.args[0].span }); if (domain(e.fields.domain) !== domain(d.value)) fail('BETA_DOMAIN_MISMATCH', 'Annotation domain mismatch', a.span); return; }
    fail('BETA_ANNOTATION', `Unsupported type annotation ${a.name}`, a.span);
  }
  record(close: string, depth: number, start = this.tokens[this.pos - 1].span.start): Fields {
    const out: Fields = Object.create(null);
    const uses: { key: string; span: Span }[] = [];
    if (this.eat(close)) return out;
    while (true) {
      if (Object.keys(out).length >= 64) fail('BETA_FIELD_BOUND', 'Record/call exceeds 64 fields', this.peek().span);
      const key = this.take(); if (key.kind !== 'word') fail('BETA_SYNTAX', 'Expected field name', key.span);
      if (Object.hasOwn(out, key.raw)) fail('BETA_DUPLICATE_FIELD', `Duplicate field ${key.raw}`, key.span);
      this.expect(':'); const useStart = this.peek().span.start; out[key.raw] = this.expression(depth + 1);
      uses.push({ key: key.raw, span: Object.freeze({ start: useStart, end: this.tokens[this.pos - 1].span.end }) });
      if (this.eat(close)) break; this.expect(','); if (this.eat(close)) break;
    }
    const containerSpan = Object.freeze({ start, end: this.tokens[this.pos - 1].span.end });
    for (const use of uses) this.fieldUses.push(Object.freeze({ containerSpan, ...use }));
    return out;
  }
  expression(depth = 1, min = 0): Value {
    this.node(depth); const start = this.peek().span.start; let left = this.primary(depth);
    while (true) {
      const t = this.peek(), prec = t.raw === '*' ? 2 : ['+', '-'].includes(t.raw) ? 1 : 0;
      if (prec === 0 || prec < min) break;
      this.take(); const right = this.expression(depth + 1, prec + 1); const span = { start, end: this.tokens[this.pos - 1].span.end };
      if (left.tag !== 'scalar' && left.tag !== 'qty' || right.tag !== 'scalar' && right.tag !== 'qty') fail('BETA_TYPE', 'Arithmetic requires scalars or quantities', span);
      let asset: string | undefined;
      if (t.raw === '*') { if (left.tag === 'qty' && right.tag === 'qty') fail('BETA_TYPE', 'Qty times Qty is unsupported', span); asset = left.tag === 'qty' ? left.asset : right.tag === 'qty' ? right.asset : undefined; }
      else { if (left.tag !== right.tag || left.tag === 'qty' && right.tag === 'qty' && left.asset !== right.asset) fail('BETA_ASSET_MISMATCH', 'Addition/subtraction require identical numeric types and nominal assets', span); asset = left.tag === 'qty' ? left.asset : undefined; }
      const l = BigInt(left.tag === 'qty' ? left.atoms : left.value), r = BigInt(right.tag === 'qty' ? right.atoms : right.value);
      const n = t.raw === '*' ? l * r : t.raw === '+' ? l + r : l - r;
      this.range(n, span); left = this.value(asset ? { tag: 'qty', asset, atoms: n.toString(), span } : { tag: 'scalar', value: n.toString(), span });
    }
    return left;
  }
  range(n: bigint, span: Span): void { if (n < 0n || n > U128) fail('BETA_UINT128_BOUND', 'Unsigned arithmetic outside 0..2^128-1', span); }
  primary(depth: number): Value {
    const t = this.take(), span = t.span;
    if (t.kind === 'string') return this.value({ tag: 'string', value: t.value, span });
    if (t.kind === 'number') {
      if (this.peek().kind === 'word' && !RESERVED.has(this.peek().raw)) {
        const unit = this.take(); if (unit.span.start === t.span.end) fail('BETA_QUANTITY_SEPARATOR', 'Quantity and asset need whitespace or a comment', unit.span);
        const asset = entity(this.lookup(unit), 'asset'); const scale = Number(scalar(asset.fields.scale)); const [integer, fraction = ''] = t.value.split('.');
        if (fraction.length > scale) fail('BETA_PRECISION', 'Quantity has more decimals than asset scale', t.span);
        const n = BigInt(integer + fraction.padEnd(scale, '0')); const full = { start: t.span.start, end: unit.span.end }; this.range(n, full);
        return this.value({ tag: 'qty', asset: asset.name, atoms: n.toString(), span: full });
      }
      if (t.value.includes('.')) fail('BETA_DECIMAL_SCALAR', 'A decimal requires an asset reference', span);
      const n = BigInt(t.value); this.range(n, span); return this.value({ tag: 'scalar', value: n.toString(), span });
    }
    if (t.raw === '(') { const v = this.expression(depth + 1); this.expect(')'); return v; }
    if (t.raw === '[') {
      const items: Value[] = []; if (!this.eat(']')) while (true) { items.push(this.expression(depth + 1)); if (this.eat(']')) break; this.expect(','); if (this.eat(']')) break; }
      return this.value({ tag: 'array', items, span: { start: span.start, end: this.tokens[this.pos - 1].span.end } });
    }
    if (t.raw === '{') { const f = this.record('}', depth); return this.value({ tag: 'record', fields: f, span: { start: span.start, end: this.tokens[this.pos - 1].span.end } }); }
    if (t.kind === 'word') {
      if (t.raw === 'true' || t.raw === 'false') return this.value({ tag: 'bool', value: t.raw === 'true', span });
      if (t.raw === 'None' || t.raw === 'SuccessOnly') return this.value({ tag: 'tag', name: t.raw, span });
      let name = t.raw;
      while (this.eat('.')) { const component = this.take(); if (component.kind !== 'word') fail('BETA_SYNTAX', 'Expected qualified call component', component.span); name += '.' + component.raw; }
      if (this.eat('(')) {
        if (!Object.hasOwn(SCHEMAS, name)) fail('BETA_UNKNOWN_CALL', `Unknown call ${name}`, t.span);
        const args = this.record(')', depth, span.start), full = { start: span.start, end: this.tokens[this.pos - 1].span.end }; this.call(name, args, full);
        if (name === 'atoms') { const asset = entity(args.asset, 'asset'); return this.value({ tag: 'qty', asset: asset.name, atoms: scalar(args.value).toString(), span: full }); }
        if (name === 'min' || name === 'max') {
          const a = args.a, b = args.b;
          if (a.tag !== b.tag || a.tag === 'qty' && b.tag === 'qty' && a.asset !== b.asset) fail('BETA_ASSET_MISMATCH', 'min/max require identical numeric types', full);
          const av = a.tag === 'qty' ? qty(a) : scalar(a), bv = b.tag === 'qty' ? qty(b) : scalar(b), n = name === 'min' ? (av < bv ? av : bv) : (av > bv ? av : bv);
          return this.value(a.tag === 'qty' ? { tag: 'qty', asset: a.asset, atoms: n.toString(), span: full } : { tag: 'scalar', value: n.toString(), span: full });
        }
        return this.value({ tag: 'call', name, args, span: full });
      }
      if (name !== t.raw) fail('BETA_SYNTAX', 'Qualified names are calls only', span);
      return this.lookup(t);
    }
    fail('BETA_SYNTAX', 'Expected expression', span);
  }
  call(name: string, args: Fields, span: Span): void {
    const schema = SCHEMAS[name]; fields(args, Object.keys(schema), [], span);
    for (const [k, role] of Object.entries(schema)) {
      const v = args[k];
      if (role === 'scalar') scalar(v); else if (role === 'string') string(v); else if (role === 'qty') qty(v);
      else if (role === 'numeric') { if (v.tag !== 'qty' && v.tag !== 'scalar') fail('BETA_TYPE', 'Expected scalar or Qty', v.span); }
      else if (role === 'qty[]') { if (v.tag !== 'array') fail('BETA_TYPE', 'Expected array of Qty', v.span); for (const item of v.items) qty(item); }
      else entity(v, role);
    }
    if (name === 'rounds' && scalar(args.from) > scalar(args.to)) fail('BETA_ROUND_WINDOW', 'Inverted round window', span);
    if (name === 'transfer') {
      const endpoints = ['from', 'to', 'fee_to'].map(k => entity(args[k], 'account'));
      if (new Set(endpoints.map(e => e.name)).size !== 3) fail('BETA_ENDPOINT_ALIAS', 'Transfer requires three distinct accounts, including zero fee', span);
      const firstDomain = domain(endpoints[0].fields.domain); for (const e of endpoints) if (domain(e.fields.domain) !== firstDomain) fail('BETA_DOMAIN_MISMATCH', 'Transfer accounts have different domains', e.span);
      if (args.value.tag === 'qty') { qty(args.fee, args.value.asset); this.sameQtyDomain(args.value, firstDomain); }
    }
    if (name === 'repay') {
      const loan = entity(args.obligation, 'obligation'), payer = entity(args.payer, 'account'), asset = entity(loan.fields.asset, 'asset');
      qty(args.amount, asset.name); if (domain(loan.fields.domain) !== domain(payer.fields.domain)) fail('BETA_DOMAIN_MISMATCH', 'Repayment payer and obligation domains differ', span);
    }
    if (HORIZON.has(name)) this.horizonRelations(name, args, span);
  }
  sameQtyDomain(v: Value, d: string): void { if (v.tag !== 'qty') return; const a = this.names.get(v.asset); if (!a || domain(entity(a.value, 'asset').fields.domain) !== d) fail('BETA_DOMAIN_MISMATCH', 'Quantity asset has different domain', v.span); }
  horizonRelations(name: string, a: Fields, span: Span): void {
    const checkAsset = (v: Value | undefined, asset: Value | undefined) => { if (v && asset) qty(v, entity(asset, 'asset').name); };
    if (name.startsWith('amm.')) {
      const pool = entity(a.pool, 'pool'); const owner = entity(a.owner, 'account');
      if (domain(pool.fields.domain) !== domain(owner.fields.domain)) fail('BETA_DOMAIN_MISMATCH', 'Pool and owner domains differ', span);
      if (name === 'amm.swap_exact_input') { checkAsset(a.net_floor, a.output_asset); if (a.input.tag === 'qty') qty(a.fee_cap, a.input.asset); }
      const members = pool.fields.assets;
      if (members.tag === 'array') for (const v of [a.input, a.output_asset, ...(a.amounts?.tag === 'array' ? a.amounts.items : [])]) {
        const unit = v?.tag === 'qty' ? v.asset : v?.tag === 'entity' && v.kind === 'asset' ? v.name : undefined;
        if (unit && !members.items.some(x => x.tag === 'entity' && x.kind === 'asset' && x.name === unit)) fail('BETA_ASSET_MISMATCH', 'Asset is not a declared pool member', v!.span);
      }
    }
    if (name === 'lending.originate') checkAsset(a.principal, entity(a.obligation, 'obligation').fields.asset);
    if (name.startsWith('stablecoin.')) { const f = entity(a.instrument, 'instrument').fields; if (!f.asset || !f.backing) fail('BETA_MISSING_FIELD', 'Stablecoin instrument requires asset and backing references', span); checkAsset(a.supply ?? a.burn ?? a.claim, f.asset); checkAsset(a.backing ?? a.minimum_backing, f.backing); }
    if (name.startsWith('option.')) { const f = entity(a.instrument, 'instrument').fields; if (!f.underlying || !f.settlement) fail('BETA_MISSING_FIELD', 'Option instrument requires underlying and settlement assets', span); if (name === 'option.settle') checkAsset(a.payoff, f.settlement); }
    if (name.startsWith('staking.')) checkAsset(a.backing ?? a.amount ?? a.minimum_backing, entity(a.shares, 'share_class').fields.backing);
    // Cross-domain bridge roles are deliberate; other family arguments share a declared domain.
    { // Only source/destination are foreign bridge roles; custody and amount stay local.
      let expected: string | undefined;
      for (const [role,v] of Object.entries(a)) {
        if(name.startsWith('bridge.')&&['source','destination'].includes(role))continue;
        const d = v.tag === 'entity' && v.kind === 'domain' ? v.name : v.tag === 'entity' && v.fields.domain ? domain(v.fields.domain) : undefined;
        if (d && expected && d !== expected) fail('BETA_DOMAIN_MISMATCH', 'Operation argument domains differ', v.span); if (d) expected = d;
      }
      if (expected) for (const v of Object.values(a)) { this.sameQtyDomain(v, expected); if (v.tag === 'array') for (const x of v.items) this.sameQtyDomain(x, expected); }
    }
  }
  identity(key: string, span: Span): void { if (this.identities.has(key)) fail('BETA_DUPLICATE_ID', 'Duplicate economic identity; aliases are unsupported', span); this.identities.add(key); }
  declaration(kind: string, name: string, value: Value): Value {
    if (kind === 'const') return value;
    if (value.tag !== 'record') fail('BETA_TYPE', `${kind} requires a record`, value.span);
    const f = value.fields, span = value.span;
    if (kind === 'domain') {
      fields(f, ['id', 'chain', 'network'], [], span); const id = string(f.id); transport(id, f.id.span); string(f.chain); string(f.network); this.identity('domain:'+id, f.id.span);
    } else if (kind === 'account' || kind === 'asset' || kind === 'obligation') {
      fields(f, kind === 'account' ? ['domain', 'id'] : kind === 'asset' ? ['domain', 'id', 'scale', 'representation'] : ['domain', 'id', 'asset'], kind === 'asset' ? ['symbol'] : [], span);
      const d = domain(f.domain), id = string(f.id); transport(id, f.id.span); this.identity(kind+':'+d+':'+id, f.id.span);
      if (kind === 'asset') { if (scalar(f.scale) > 18n) fail('BETA_SCALE', 'Asset scale must be 0..18', f.scale.span); string(f.representation); if (f.symbol) string(f.symbol); }
      if (kind === 'obligation' && domain(entity(f.asset, 'asset').fields.domain) !== d) fail('BETA_DOMAIN_MISMATCH', 'Obligation asset domain differs', f.asset.span);
    } else if (kind === 'intent') this.intent(f, span);
    else this.horizonDeclaration(kind, f, span);
    return this.value({ tag: 'entity', kind, name, fields: f, span });
  }
  horizonDeclaration(kind: string, f: Fields, span: Span): void {
    const contracts: Record<string, [string[], string[]]> = {
      pool: [['domain', 'id', 'assets'], []], share_class: [['domain', 'id', 'backing'], []],
      instrument: [['domain', 'id'], ['asset', 'backing', 'underlying', 'settlement', 'strike_units', 'strike', 'exercise_round', 'collateral']],
      observation: [['id', 'domain'], ['unit', 'source', 'observed_round', 'maximum_age', 'finality', 'provenance']],
      policy: [[], ['id', 'epoch', 'source_hash', 'duty_preservation']],
      grant: [[], ['issuer', 'scope', 'expiry', 'revocation_epoch', 'gross_limit', 'work_limit', 'signers', 'evidence']],
      party: [['id', 'account'], []],
      stage: [['domain'], ['id', 'trigger', 'paired_claim', 'authority', 'reads', 'writes', 'duty', 'status', 'amount', 'evidence', 'relation', 'signed_floor', 'failure', 'operation', 'completion', 'rounding', 'retained_duties']],
      episode: [['id', 'stages'], ['pending', 'timeout', 'recovery', 'failure', 'duty', 'authority']],
    };
    const c = contracts[kind]; fields(f, c[0], c[1], span);
    if (f.id) string(f.id); if (f.domain) domain(f.domain);
    if(kind==='share_class')this.identity('share_class:'+domain(f.domain)+':'+string(f.id),f.id.span);
    if(f.signed_floor){if(f.signed_floor.tag==='qty')this.sameQtyDomain(f.signed_floor,domain(f.domain));else string(f.signed_floor);}
    if (kind === 'pool') { if (f.assets.tag !== 'array' || f.assets.items.length === 0) fail('BETA_TYPE', 'Pool assets require a nonempty asset array', f.assets.span); const seen = new Set<string>(); for (const v of f.assets.items) { const a = entity(v, 'asset'); if (domain(a.fields.domain) !== domain(f.domain)) fail('BETA_DOMAIN_MISMATCH', 'Pool asset domain differs', v.span); if (seen.has(a.name)) fail('BETA_DUPLICATE_ID', 'Duplicate pool asset', v.span); seen.add(a.name); } }
    if (kind === 'party') entity(f.account, 'account');
    if (kind === 'episode') { if (f.stages.tag !== 'array') fail('BETA_TYPE', 'Episode stages require an array', f.stages.span); for (const x of f.stages.items) entity(x, 'stage'); }
    for (const k of ['asset', 'backing', 'underlying', 'settlement']) if (f[k]) { const a = entity(f[k], 'asset'); if (f.domain && domain(a.fields.domain) !== domain(f.domain)) fail('BETA_DOMAIN_MISMATCH', 'Resource asset domain differs', f[k].span); }
    for (const k of ['epoch', 'exercise_round', 'observed_round', 'maximum_age', 'expiry', 'revocation_epoch', 'work_limit']) if (f[k]) scalar(f[k]);
    for (const k of ['collateral', 'gross_limit', 'amount']) if (f[k]) qty(f[k]);
    for (const k of 'strike_units strike unit source finality provenance source_hash duty_preservation scope evidence trigger paired_claim duty status relation failure rounding pending timeout recovery authority completion retained_duties'.split(' ')) if (f[k]) {
      if (k === 'authority' && f[k].tag === 'entity') entity(f[k], 'grant'); else string(f[k]);
    }
    for (const k of ['reads', 'writes']) if (f[k]) { if (f[k].tag !== 'array') fail('BETA_TYPE', `${k} requires a string array`, f[k].span); for (const v of f[k].items) string(v); }
    if (f.issuer) entity(f.issuer, 'account');
    if (f.signers) { if (f.signers.tag !== 'array') fail('BETA_TYPE', 'Signers must be an account array', f.signers.span); for (const x of f.signers.items) entity(x, 'account'); }
  }
  intent(f: Fields, span: Span): void {
    if (!f.operation || f.operation.tag !== 'call') fail('BETA_TYPE', 'Intent requires a named operation call', f.operation?.span ?? span);
    const op = f.operation;
    if (HORIZON.has(op.name)) {
      fields(f, ['operation'], 'authority policy nonce valid reads writes kernel completion rounding relation failure status observation continuation loss fixing earliest_round veto_before domain asset signer key pre_head gross_cap fee_cap net_floor source_hash policy_digest observations disclosures retained_effects retained_duties delegation recovery'.split(' '), span);
      if (f.domain) domain(f.domain); if (f.asset) entity(f.asset, 'asset'); if (f.signer) entity(f.signer, 'account');
      const localArgs=Object.entries(op.args).filter(([key])=>!op.name.startsWith('bridge.')||!['source','destination'].includes(key));
      let intentDomain=f.domain?domain(f.domain):undefined;
      for(const v of [f.asset,f.signer,...localArgs.map(([,v])=>v)]){
       if(!v)continue;
       const d=v.tag==='entity'&&v.kind==='domain'?v.name:v.tag==='entity'&&v.fields.domain?domain(v.fields.domain):v.tag==='qty'?domain(entity(this.names.get(v.asset)!.value,'asset').fields.domain):undefined;
       if(d&&intentDomain&&d!==intentDomain)fail('BETA_DOMAIN_MISMATCH','Intent header and operation domains differ',v.span);if(d)intentDomain=d;
      }
      if(f.valid?.tag==='call'&&f.valid.name==='rounds'&&intentDomain&&domain(f.valid.args.domain)!==intentDomain)fail('BETA_DOMAIN_MISMATCH','Intent rounds domain differs',f.valid.span);
      const bridgeAsset=op.name.startsWith('bridge.')&&op.args.amount.tag==='qty'?op.args.amount.asset:undefined;
      const boundAsset=f.asset?entity(f.asset,'asset').name:bridgeAsset;
      if(bridgeAsset&&boundAsset!==bridgeAsset)fail('BETA_ASSET_MISMATCH','Bridge intent asset must match its local amount',f.asset?.span??span);
      if(f.retained_duties){if(f.retained_duties.tag==='array'){for(const duty of f.retained_duties.items)string(duty);}else string(f.retained_duties);}
      if (f.policy) entity(f.policy, 'policy'); if (f.authority) { if (f.authority.tag === 'entity') entity(f.authority, 'grant'); else string(f.authority); }
      for (const k of ['reads', 'writes']) if (f[k]) { if (f[k].tag !== 'array') fail('BETA_TYPE', `${k} requires a string array`, f[k].span); for (const x of f[k].items) string(x); }
      for (const k of 'nonce key pre_head source_hash policy_digest kernel completion rounding relation status observation continuation loss fixing'.split(' ')) if (f[k]) string(f[k]);
      if (f.valid) { if (f.valid.tag === 'array') { if (f.valid.items.length !== 2) fail('BETA_TYPE', 'Horizon validity requires two round scalars', f.valid.span); for (const x of f.valid.items) scalar(x); } else if (f.valid.tag !== 'call' || f.valid.name !== 'rounds') fail('BETA_TYPE', 'Horizon validity requires rounds or two round scalars', f.valid.span); }
      for (const k of ['gross_cap', 'fee_cap', 'net_floor']) if (f[k]) {
       const unit=k==='net_floor'&&op.name==='amm.swap_exact_input'?entity(op.args.output_asset,'asset').name:boundAsset;qty(f[k],unit);if(intentDomain)this.sameQtyDomain(f[k],intentDomain);
      }
      for (const k of ['earliest_round', 'veto_before']) if (f[k]) scalar(f[k]);
      return;
    }
    if (op.name !== 'transfer' && op.name !== 'repay') fail('BETA_OPERATION', 'Intent operation must be transfer, repay or a recognized horizon operation', op.span);
    fields(f, 'domain asset signer key nonce pre_head valid gross_cap fee_cap net_floor operation source_hash policy_digest failure observations disclosures retained_effects retained_duties delegation recovery'.split(' '), [], span);
    const d = domain(f.domain), a = entity(f.asset, 'asset'), signer = entity(f.signer, 'account');
    if (domain(a.fields.domain) !== d || domain(signer.fields.domain) !== d) fail('BETA_DOMAIN_MISMATCH', 'Intent domain, asset and signer must agree', span);
    for (const k of ['key', 'nonce', 'pre_head', 'source_hash', 'policy_digest']) if (string(f[k]).length === 0) fail('BETA_STRING', `Empty ${k} claim`, f[k].span);
    if (f.valid.tag !== 'call' || f.valid.name !== 'rounds' || domain(f.valid.args.domain) !== d) fail('BETA_DOMAIN_MISMATCH', 'Intent requires rounds on its domain', f.valid.span);
    for (const k of ['gross_cap', 'fee_cap', 'net_floor']) qty(f[k], a.name, true);
    if (f.failure.tag !== 'tag' || f.failure.name !== 'SuccessOnly') fail('BETA_FAILURE_POLICY', 'S0 requires SuccessOnly', f.failure.span);
    for (const k of ['observations', 'disclosures', 'retained_effects', 'retained_duties']) if (f[k].tag !== 'array' || f[k].items.length !== 0) fail('BETA_FAILURE_POLICY', `S0 requires empty ${k}`, f[k].span);
    for (const k of ['delegation', 'recovery']) if (f[k].tag !== 'tag' || f[k].name !== 'None') fail('BETA_FAILURE_POLICY', `S0 requires ${k}: None`, f[k].span);
    if (op.name === 'transfer') {
      for (const k of ['from', 'to', 'fee_to']) if (domain(entity(op.args[k], 'account').fields.domain) !== d) fail('BETA_DOMAIN_MISMATCH', 'Operation account domain differs', op.args[k].span);
      if (entity(op.args.from, 'account').name !== signer.name) fail('BETA_SIGNER', 'Transfer signer must be from account', f.signer.span);
      qty(op.args.value, a.name, true); qty(op.args.fee, a.name, true);
    } else {
      const loan = entity(op.args.obligation, 'obligation');
      if (domain(loan.fields.domain) !== d || entity(loan.fields.asset, 'asset').name !== a.name) fail('BETA_DOMAIN_MISMATCH', 'Repayment obligation domain or asset differs', loan.span);
      if (entity(op.args.payer, 'account').name !== signer.name) fail('BETA_SIGNER', 'Repayment signer must be payer', f.signer.span); qty(op.args.amount, a.name, true);
      // Fee/floor economic checks stay in Core, including syntactically valid nonzero values.
    }
  }
  parse(): void {
    this.expect('profile'); const profile = this.take(); if (profile.kind !== 'string' || profile.value !== 'moriarty-beta/1') fail('BETA_PROFILE', 'Expected profile "moriarty-beta/1"', profile.span); this.expect(';'); this.expect('agreement'); const agreement = this.name(); transport(agreement.raw, agreement.span); this.agreement = agreement.raw; this.agreementSpan = Object.freeze(agreement.span); this.expect('{');
    while (!this.eat('}')) {
      if (this.declarations.length + this.actions.length >= 256) fail('BETA_DECLARATION_BOUND', 'Agreement exceeds 256 declarations/actions', this.peek().span);
      const start = this.take(); if (start.kind !== 'word' || !KINDS.has(start.raw) && start.raw !== 'action') fail('BETA_CONSTRUCT', 'Unknown or incomplete declaration', start.span);
      const name = this.name(); if (this.names.has(name.raw) || this.actions.some(a => a.name === name.raw)) fail('BETA_DUPLICATE_NAME', `Duplicate declaration ${name.raw}`, name.span);
      if (start.raw === 'action') {
        this.expect('uses'); const ref = this.name(), intent = entity(this.lookup(ref), 'intent'); const end = this.expect(';');
        const op = intent.fields.operation; if (op.tag !== 'call') fail('BETA_OPERATION', 'Action intent has no operation', op.span);
        transport(name.raw, name.span); this.actions.push({ name: name.raw, intent: ref.raw, span: { start: start.span.start, end: end.span.end }, nameSpan: name.span, support: HORIZON.has(op.name) ? 'SpecifiedOnly' : 'LocalS0' }); continue;
      }
      const annotation = this.eat(':') ? this.annotation(1) : undefined; this.expect('='); let value = this.expression(); const end = this.expect(';');
      value = this.declaration(start.raw, name.raw, value); if (annotation) this.checkAnnotation(annotation, value);
      const declaration = { kind: start.raw, name: name.raw, span: { start: start.span.start, end: end.span.end }, nameSpan: name.span, value }; this.names.set(name.raw, declaration); this.declarations.push(declaration);
    }
    if (this.peek().kind !== 'eof') fail('BETA_TRAILING_INPUT', 'Trailing input after agreement', this.peek().span);
  }
}
export function analyze(source: string): Analysis {
  const sourceHash = createHash('sha256').update(source).digest('hex'); let parser: Parser | undefined;
  try { parser = new Parser(lex(source)); parser.parse(); return { status: 'AuthoringChecked', sourceHash, diagnostics: [], agreement: parser.agreement, agreementSpan: parser.agreementSpan, declarations: parser.declarations, actions: parser.actions, references: parser.references, fieldUses: parser.fieldUses }; }
  catch (error) { if (!(error instanceof Failure)) throw error; return { status: 'AuthoringRejected', sourceHash, diagnostics: [error.diagnostic], agreement: parser?.agreement ?? null, agreementSpan: parser?.agreementSpan ?? null, declarations: [], actions: [], references: [], fieldUses: [] }; }
}
export function check(source: string): object {
  const a = analyze(source);
  return { ...a, declarations: a.declarations.map(({ kind, name, span, nameSpan }) => ({ kind, name, span, nameSpan })),
    actions: a.actions.map(x => ({ ...x, coverage: { syntax: 'Checked', names: 'Checked', quantities: 'Checked', financialRelations: x.support === 'LocalS0' ? 'DelegatedToCoreDuringLocalPreparation' : 'Open', localPreparation: x.support === 'LocalS0' ? 'AvailableUnqualified' : 'Unsupported' } })),
    operationSchemas: SCHEMAS, evidence: 'AuthoringOnly', openGates: ['authentication', 'nativeProof', 'financialCorrespondence', 'atomicLedgerAcceptance'] };
}
export function byteToPosition(source: string, byte: number): { line: number; character: number } {
  let offset = 0, line = 0, character = 0;
  const target = Math.max(0, Math.min(bytes(source), Number.isFinite(byte) ? byte : 0));
  for (let i = 0; i < source.length;) {
    const point = source.codePointAt(i)!, raw = String.fromCodePoint(point), n = bytes(raw);
    if (offset + n > target) break;
    if (raw === '\r') { if (source[i + 1] === '\n') { if (offset + 2 > target) break; offset += 2; i += 2; } else { offset += n; i++; } line++; character = 0; continue; }
    offset += n; i += raw.length; if (raw === '\n') { line++; character = 0; } else character += raw.length;
  }
  return { line, character };
}
export function format(source: string): { text: string | null; diagnostics: Diagnostic[] } {
  const a = analyze(source); if (a.diagnostics.length) return { text: null, diagnostics: a.diagnostics };
  const tokens = lex(source); let out = '', indent = 0, lineStart = true, previous = '';const delimiters:string[]=[];
  const write = (s: string) => { if (lineStart) { out += '  '.repeat(indent); lineStart = false; } out += s; };
  const space = () => { if (!lineStart && !out.endsWith(' ') && !out.endsWith('\n')) out += ' '; };
  const newline = () => { out = out.replace(/[ \t]+$/, ''); if (!out.endsWith('\n')) out += '\n'; lineStart = true; };
  for (const t of tokens) {
    const raw = t.raw;
    if (t.kind === 'eof') break;
    if (t.kind === 'comment') { space(); write(raw); if (raw.startsWith('//') || raw.includes('\n')) newline(); else space(); continue; }
    if (raw === '}') { delimiters.pop();indent = Math.max(0, indent - 1); newline(); write(raw); }
    else if (raw === '{') { space(); write(raw);delimiters.push(raw); indent++; newline(); }
    else if (raw === ';') { write(raw); newline(); }
    else if (raw === ',') { write(raw); if (delimiters.at(-1)==='{') newline(); else space(); }
    else if (raw === ':') { write(raw); space(); }
    else if (raw === '(' || raw === '[' || raw === '<' || raw === '.' || raw === ')' || raw === ']' || raw === '>') { if(raw==='('||raw==='[')delimiters.push(raw);else if(raw===')'||raw===']')delimiters.pop();if (raw === '[' && !['(', '[', '<', '.'].includes(previous)) space(); write(raw); }
    else { if (!['(', '[', '<', '.'].includes(previous)) space(); write(raw); }
    previous = raw;
  }
  newline();
  // Bound formatted output before it becomes an accepted replacement source.
  if (bytes(out) > 65536) return { text: null, diagnostics: [{ code: 'BETA_FORMAT_BOUND', message: 'Formatted source exceeds 65536 bytes', span: { start: 0, end: bytes(source) } }] };
  const verified = analyze(out); if (verified.diagnostics.length) return { text: null, diagnostics: verified.diagnostics };
  return { text: out, diagnostics: [] };
}
