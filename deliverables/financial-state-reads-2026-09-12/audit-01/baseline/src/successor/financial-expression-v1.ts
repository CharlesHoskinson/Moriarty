/** Explicit financial-expression /1 fork of the reviewed40-node machine. Never calls funded preparation or settlement. */
import { ExpressionFailure, SYNTHETIC_SPAN, array, bytes, canonical, closed, decimal,
  fail, identifier, object, parseCanonical } from './expression-wire-v1.ts';
import type { ExpressionSpan } from './expression-wire-v1.ts';
import { NUMERIC, aggregateBound, numericFits, resolve, same, schemaShape, typeShape,
  unitShape, validateSchema, validateSnapshot, valueBound, valueDomain, valueSize } from './financial-expression-types-v1.ts';
import type { Schema, ValueType } from './financial-expression-types-v1.ts';

export const FINANCIAL_EXPRESSION_CONTRACT_V1 = 'moriarty-financial-expression-contract/1';
type CoreNode = { constructor: string; operands: Record<string, any>; span: ExpressionSpan };
type Action = { statements: CoreNode[]; span: ExpressionSpan };
export type ExpressionResult =
  | { status: 'Rejected'; code: string; span: ExpressionSpan; nodePath: string[]; workUsed: string }
  | { status: 'ExpressionPrepared'; post: Record<string, any>; descriptors: any[]; workRemaining: string }
  | { judgmentResult: 'ExpressionValue'; type: ValueType; value: any; workRemaining: string };
export type ExpressionCheckResult =
  | { judgmentResult: 'ExpressionChecked' }
  | Extract<ExpressionResult, { status: 'Rejected' }>;
type Role = 'int' | 'id' | 'bool' | 'text' | 'type' | 'units' | 'view' | 'expr' | 'list' | 'fields';
const BINARY: [string, Role][] = [['left', 'expr'], ['right', 'expr']];
const OPERANDS: Record<string, [string, Role][]> = {
  ConstructShares: [['vault','id'],['holder','id'],['value','expr']],
  ConstructAmount: [['asset','id'],['value','expr']],
  ConstructVariant: [['family','id'],['tag','id'],['value','expr']],
  ProjectVariant: [['tag','id'],['value','expr']], ProjectSome: [['value','expr']],
  ConvertUInt: [['width','int'],['value','expr']], ScalarValue: [['component','id'],['value','expr']],
  Select: [['condition','expr'],['consequent','expr'],['alternative','expr']],
  LitUInt: [['width', 'int'], ['value', 'int']], LitSInt: [['value', 'int']],
  LitBool: [['value', 'bool']], LitText: [['value', 'text']],
  LitAmount: [['asset', 'id'], ['value', 'int']],
  LitQuantity: [['units', 'units'], ['scale', 'int'], ['mantissa', 'int']],
  LitShares: [['vault', 'id'], ['holder', 'id'], ['value', 'int']],
  LitRate: [['scale', 'int'], ['mantissa', 'int']],
  LitPrice: [['base', 'id'], ['quote', 'id'], ['scale', 'int'], ['mantissa', 'int']],
  ReadLocal: [['name', 'id']], ReadPre: [['view', 'view'], ['field', 'id']],
  ReadArg: [['name', 'id']], ReadObs: [['name', 'id']],
  ProjectField: [['record', 'expr'], ['field', 'id']], AccessField: [['record', 'expr'], ['field', 'id']],
  ProjectIndex: [['collection', 'expr'], ['index', 'expr']], AccessIndex: [['collection', 'expr'], ['index', 'expr']],
  ConstructRecord: [['recordType', 'id'], ['fields', 'fields']],
  ConstructEnum: [['enumType', 'id'], ['member', 'id']],
  ConstructSome: [['elementType', 'type'], ['value', 'expr']], ConstructNone: [['elementType', 'type']],
  ConstructCollection: [['elementType', 'type'], ['capacity', 'int'], ['items', 'list']],
  Add: BINARY, Sub: BINARY, Mul: BINARY, FloorDiv: BINARY, CeilDiv: BINARY,
  Eq: BINARY, Lt: BINARY, Lte: BINARY, Gt: BINARY, Gte: BINARY,
  Not: [['value', 'expr']], And: BINARY, Or: BINARY,
  Require: [['condition', 'expr']], Let: [['name', 'id'], ['value', 'expr']],
  NextWrite: [['field', 'id'], ['value', 'expr']], Ensure: [['condition', 'expr']],
  Emit: [['operation', 'id'], ['fields', 'expr']],
};
const STATEMENTS = new Set(['Require', 'Let', 'NextWrite', 'Ensure', 'Emit']);
function has(v: object, name: string): boolean { return Object.hasOwn(v, name); }
function validSpan(v: any, sourceBytes: number): v is ExpressionSpan {
  if (v === null || typeof v !== 'object' || Array.isArray(v)
      || Object.keys(v).length !== 3 || !['source', 'synthetic'].includes(v.kind)
      || !['start', 'end'].every(k => typeof v[k] === 'string' && /^(0|[1-9][0-9]*)$/.test(v[k]) && !v[k].includes('\n'))) return false;
  const start = BigInt(v.start), end = BigInt(v.end);
  return v.kind === 'synthetic' ? start === 0n && end === 0n : start <= end && end <= BigInt(sourceBytes);
}

class Machine {
  schema: Schema;
  sourceBytes = 0;
  initial = 0;
  remaining = 0;
  reducing = false;
  types = new Map<CoreNode, ValueType>();
  localTypes = new Map<string, ValueType>();
  locals = new Map<string, any>();
  written = new Set<string>();
  writes: Record<string, any> = Object.create(null);
  descriptors: any[] = [];
  pre: Record<string, any> = {};
  args: Record<string, any> = {};
  obs: Record<string, any> = {};
  located = new WeakSet<ExpressionFailure>();
  constructor(schema: Schema) { this.schema = schema; }
  site<T>(node: any, path: number[], f: () => T): T {
    try { return f(); } catch (error) {
      if (error instanceof ExpressionFailure && !this.located.has(error)) {
        error.span = validSpan(node?.span, this.sourceBytes) ? node.span : SYNTHETIC_SPAN;
        error.path = path; this.located.add(error);
      }
      throw error;
    }
  }
  spanShape(p: any): void {
    closed(p, ['kind', 'start', 'end']); decimal(p.start); decimal(p.end);
    if (!['source', 'synthetic'].includes(p.kind)) fail('INPUT_SCHEMA');
    if (!validSpan(p, this.sourceBytes)) fail('INPUT_SPAN');
  }
  children(n: CoreNode): CoreNode[] {
    const result: CoreNode[] = [];
    for (const [name, role] of OPERANDS[n.constructor]) {
      if (role === 'expr') result.push(n.operands[name]);
      else if (role === 'list') result.push(...n.operands[name]);
      else if (role === 'fields') result.push(...n.operands[name].map((x: any) => x.value));
    }
    return result;
  }
  shape(n: any, path: number[], depth: number, count: { nodes: number }): void {
    this.site(n, path, () => {
      if (depth > 64 || ++count.nodes > 4096) fail('INPUT_BOUND');
      closed(n, ['constructor', 'operands', 'span']); this.spanShape(n.span);
      if (typeof n.constructor !== 'string' || !has(OPERANDS, n.constructor)) fail('TYPE_CONSTRUCTOR');
      const roles = OPERANDS[n.constructor]; closed(n.operands, roles.map(([name]) => name));
      let index = 0;
      for (const [name, role] of roles) {
        const v = n.operands[name];
        if (role === 'expr') this.shape(v, [...path, index++], depth + 1, count);
        else if (role === 'list' || role === 'fields') {
          array(v); if (v.length > (role === 'fields' ? 64 : 128)) fail('INPUT_BOUND');
          for (const entry of v) {
            if (role === 'fields') { closed(entry, ['name', 'value']); identifier(entry.name); }
            this.shape(role === 'fields' ? entry.value : entry, [...path, index++], depth + 1, count);
          }
        } else if (role === 'id') identifier(v);
        else if (role === 'int') decimal(v);
        else if (role === 'type') typeShape(v);
        else if (role === 'units') unitShape(v);
        else if (role === 'bool') { if (typeof v !== 'boolean') fail('INPUT_SCHEMA'); }
        else if (role === 'text') { if (typeof v !== 'string') fail('INPUT_SCHEMA'); }
        else if (role === 'view' && !['pre', 'post', 'next'].includes(v)) fail('INPUT_SCHEMA');
      }
    });
  }
  requireType(actual: ValueType, expected: ValueType): void { if (!same(actual, expected)) fail('TYPE_MISMATCH'); }
  lookup(map: any, name: string): any { if (!has(map, name)) fail('TYPE_NAME'); return map[name]; }
  literal(n: CoreNode): { type: ValueType; value: any } {
    const o = n.operands;
    switch (n.constructor) {
      case 'LitUInt': if (!['64', '128', '256'].includes(o.width)) fail('TYPE_LITERAL'); return { type: ['UInt' + o.width], value: o.value };
      case 'LitSInt': return { type: ['SInt128'], value: o.value };
      case 'LitBool': return { type: ['Bool'], value: o.value };
      case 'LitText': return { type: ['Text'], value: o.value };
      case 'LitAmount': return { type: ['Amount', o.asset], value: o.value };
      case 'LitQuantity': return { type: ['Quantity', o.units, o.scale], value: o.mantissa };
      case 'LitShares': return { type: ['Shares', o.vault, o.holder], value: o.value };
      case 'LitRate': return { type: ['Rate', o.scale], value: o.mantissa };
      case 'LitPrice': return { type: ['Price', o.base, o.quote, o.scale], value: o.mantissa };
      default: throw new Error('Not a literal constructor');
    }
  }
  infer(n: CoreNode, path: number[], ensure: boolean, statement = false): ValueType {
    return this.site(n, path, () => {
      const k = n.constructor, o = n.operands, s = this.schema;
      if (STATEMENTS.has(k) !== statement) fail('TYPE_STATEMENT_PLACEMENT');
      if (k === 'Let') {
        const declarations = ['args', 'observations', 'fields', 'recordTypes', 'enumTypes', 'variantTypes', 'operations'];
        if (this.localTypes.has(o.name) || ['pre', 'post', 'next'].includes(o.name)
            || declarations.some(key => has(s[key], o.name))
            || ['units', 'assets', 'vaults', 'parties'].some(key => s[key].includes(o.name))) fail('TYPE_DUPLICATE_BINDER');
      }
      if (k === 'NextWrite') {
        const field = this.lookup(s.fields, o.field);
        if (field.writeClass !== 'ordinary') fail('TYPE_FINANCIAL_WRITE');
        if (this.written.has(o.field)) fail('TYPE_DUPLICATE_WRITE');
      }
      if (k === 'Emit') this.lookup(s.operations, o.operation);
      if (k === 'ConstructRecord') {
        const fields = this.lookup(s.recordTypes, o.recordType), seen = new Set<string>();
        for (const entry of o.fields) { if (seen.has(entry.name)) fail('TYPE_DUPLICATE_FIELD'); seen.add(entry.name); }
        if (seen.size !== Object.keys(fields).length || [...seen].some(name => !has(fields, name))) fail('TYPE_RECORD_FIELDS');
      }
      if (['ConstructSome', 'ConstructNone', 'ConstructCollection'].includes(k)) resolve(o.elementType, s);
      if (k === 'ConstructCollection' && (BigInt(o.capacity) < 0n || BigInt(o.capacity) > 128n || BigInt(o.items.length) > BigInt(o.capacity))) fail('TYPE_COLLECTION_BOUND');
      if(k==='ConstructAmount') resolve(['Amount',o.asset],s);
      if(k==='ConstructShares') resolve(['Shares',o.vault,o.holder],s);
      if(k==='ConstructVariant') this.lookup(this.lookup(s.variantTypes,o.family),o.tag);
      if(k==='ConvertUInt' && !['64','128','256'].includes(o.width)) fail('TYPE_LITERAL');
      if(k==='ScalarValue' && !['quanta','mantissa','negative','magnitude'].includes(o.component)) fail('TYPE_LITERAL');
      const childTypes = this.children(n).map((child, i) => this.infer(child, [...path, i], ensure));
      let t: ValueType;
      if (k.startsWith('Lit')) {
        const literal = this.literal(n); t = literal.type; resolve(t, s); valueDomain(t, literal.value, s, 'TYPE_LITERAL'); valueBound(t, literal.value, s, 'TYPE_LITERAL');
      } else if(k==='ConstructAmount' || k==='ConstructShares') {
        this.requireType(childTypes[0],['UInt128']);t=k==='ConstructAmount'?['Amount',o.asset]:['Shares',o.vault,o.holder];
      } else if(k==='ConstructVariant') {
        this.requireType(childTypes[0],s.variantTypes[o.family][o.tag]);t=['Variant',o.family];
      } else if(k==='ProjectVariant') {
        if(childTypes[0][0]!=='Variant') fail('TYPE_MISMATCH');t=this.lookup(s.variantTypes[childTypes[0][1]],o.tag);
      } else if(k==='ProjectSome') {
        if(childTypes[0][0]!=='Option') fail('TYPE_MISMATCH');t=childTypes[0][1];
      } else if(k==='ConvertUInt') {
        if(!['UInt64','UInt128','UInt256'].includes(childTypes[0][0])) fail('TYPE_MISMATCH');t=['UInt'+o.width];
      } else if(k==='ScalarValue') t=this.scalarType(o.component,childTypes[0]);
      else if(k==='Select') {
        this.requireType(childTypes[0],['Bool']);this.requireType(childTypes[1],childTypes[2]);
        if(['Unit','Operation'].includes(childTypes[1][0])) fail('TYPE_MISMATCH');t=childTypes[1];
      } else if (k === 'ReadLocal') {
        if (!this.localTypes.has(o.name)) fail('TYPE_NAME'); t = this.localTypes.get(o.name)!;
      } else if (k === 'ReadArg' || k === 'ReadObs') t = this.lookup(k === 'ReadArg' ? s.args : s.observations, o.name);
      else if (k === 'ReadPre') {
        if (o.view === 'next') fail('TYPE_NEXT_READ');
        if (o.view === 'post' && !ensure) fail('TYPE_POST_SCOPE');
        t = this.lookup(s.fields, o.field).type;
      } else if (k === 'ProjectField' || k === 'AccessField') {
        if (childTypes[0][0] !== 'Record') fail('TYPE_MISMATCH'); t = this.lookup(s.recordTypes[childTypes[0][1]], o.field);
      } else if (k === 'ProjectIndex' || k === 'AccessIndex') {
        if (childTypes[0][0] !== 'Collection') fail('TYPE_MISMATCH'); this.requireType(childTypes[1], ['UInt64']); t = childTypes[0][1];
      } else if (k === 'ConstructRecord') {
        o.fields.forEach((f: any, i: number) => this.requireType(childTypes[i], s.recordTypes[o.recordType][f.name])); t = ['Record', o.recordType];
      } else if (k === 'ConstructEnum') {
        if (!this.lookup(s.enumTypes, o.enumType).includes(o.member)) fail('TYPE_ENUM_MEMBER'); t = ['Enum', o.enumType];
      } else if (k === 'ConstructSome' || k === 'ConstructNone') {
        if (k === 'ConstructSome') this.requireType(childTypes[0], o.elementType); t = ['Option', o.elementType];
      } else if (k === 'ConstructCollection') {
        childTypes.forEach(ct => this.requireType(ct, o.elementType)); t = ['Collection', o.elementType, o.capacity];
      } else if (['Add', 'Sub', 'Mul', 'FloorDiv', 'CeilDiv'].includes(k)) t = this.arithmeticType(k, childTypes[0], childTypes[1], o.right);
      else if (['Eq', 'Lt', 'Lte', 'Gt', 'Gte'].includes(k)) {
        this.requireType(childTypes[0], childTypes[1]);
        if (k !== 'Eq' && !NUMERIC.includes(childTypes[0][0])) fail('TYPE_MISMATCH'); t = ['Bool'];
      } else if (['Not', 'And', 'Or'].includes(k)) {
        childTypes.forEach(ct => this.requireType(ct, ['Bool'])); t = ['Bool'];
      } else if (k === 'Require' || k === 'Ensure') { this.requireType(childTypes[0], ['Bool']); t = ['Unit']; }
      else if (k === 'Let') { this.localTypes.set(o.name, childTypes[0]); t = ['Unit']; }
      else if (k === 'NextWrite') { this.requireType(childTypes[0], s.fields[o.field].type); this.written.add(o.field); t = ['Unit']; }
      else if (k === 'Emit') { this.requireType(childTypes[0], ['Record', s.operations[o.operation]]); t = ['Unit']; }
      else throw new Error('Unimplemented admitted constructor: ' + k);
      this.types.set(n, t); return t;
    });
  }
  scalarType(component: string, child: ValueType): ValueType {
    const tag=child[0];
    if(component==='quanta' && ['Amount','Shares'].includes(tag)) return ['UInt128'];
    if(component==='mantissa' && tag==='Price') return ['UInt128'];
    if(component==='mantissa' && ['Rate','Quantity'].includes(tag)) return ['SInt128'];
    if(component==='negative' && ['Rate','Quantity','SignedAmount','NetAmount'].includes(tag)) return ['Bool'];
    if(component==='magnitude' && ['Rate','Quantity','NetAmount'].includes(tag)) return ['UInt128'];
    if(component==='magnitude' && tag==='SignedAmount') return ['UInt256'];
    fail('TYPE_MISMATCH');
  }
  arithmeticType(k: string, left: ValueType, right: ValueType, rightNode?: CoreNode): ValueType {
    if(k==='Mul') {
      const amount=left[0]==='Amount'?left:right[0]==='Amount'?right:null;
      const other=left[0]==='Amount'?right:left;
      if(amount && other[0]==='UInt128') return amount;
      if(left[0]==='Amount' && right[0]==='Amount') return ['AmountProduct',...[left[1],right[1]].sort()];
      if(amount && other[0]==='Price' && other[2]===amount[1]) return ['ScaledAmount',other[1],other[3]];
      if(amount && other[0]==='Rate') return ['SignedScaledAmount',amount[1],other[1]];
    }
    if(['FloorDiv','CeilDiv'].includes(k)) {
      if(left[0]==='AmountProduct' && right[0]==='Amount' && left.slice(1).includes(right[1])) return ['Amount',right[1]===left[1]?left[2]:left[1]];
      if(['ScaledAmount','SignedScaledAmount'].includes(left[0])) {
        if(!same(right,['UInt128']) || rightNode?.constructor!=='LitUInt' || rightNode.operands.width!=='128' || BigInt(rightNode.operands.value)!==10n**BigInt(left[2])) fail('TYPE_SCALE_DIVISOR');
        return [left[0]==='ScaledAmount'?'Amount':'SignedAmount',left[1]];
      }
    }
    if (['Mul', 'FloorDiv', 'CeilDiv'].includes(k) && left[0] === 'Quantity' && right[0] === 'Quantity') {
      const direction = k === 'Mul' ? 1n : -1n, units = new Map<string, bigint>();
      for (const [name, power] of left[1]) units.set(name, BigInt(power));
      for (const [name, power] of right[1]) units.set(name, (units.get(name) ?? 0n) + direction * BigInt(power));
      const vector = [...units].filter(([, p]) => p !== 0n).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0).map(([name, power]) => [name, String(power)]);
      const result = ['Quantity', vector, String(BigInt(left[2]) + direction * BigInt(right[2]))]; resolve(result, this.schema, 'TYPE_QUANTITY_DOMAIN'); return result;
    }
    this.requireType(left, right);
    const scalar = ['UInt64', 'UInt128', 'UInt256', 'SInt128'].includes(left[0]);
    const indexedAdd = ['Add', 'Sub'].includes(k) && ['Amount', 'Shares', 'Rate', 'Quantity'].includes(left[0]);
    if (!scalar && !indexedAdd) fail('TYPE_MISMATCH'); return left;
  }
  reduce(n: CoreNode, path: number[]): any {
    return this.site(n, path, () => {
      if (this.remaining === 0) fail('WORK_EXHAUSTED'); this.remaining--;
      const k = n.constructor, o = n.operands, type = this.types.get(n)!;
      const children = this.children(n);
      if (k === 'And' || k === 'Or') {
        const left = this.reduce(children[0], [...path, 0]);
        return (k === 'And' ? !left : left) ? left : this.reduce(children[1], [...path, 1]);
      }
      if(k==='Select') {
        const selected=this.reduce(children[0],[...path,0])?1:2;
        return this.reduce(children[selected],[...path,selected]);
      }
      const v = children.map((child, i) => this.reduce(child, [...path, i]));
      let result: any;
      if (k.startsWith('Lit')) result = this.literal(n).value;
      else if(k==='ConstructAmount' || k==='ConstructShares' || k==='ConvertUInt') {
        result=v[0];if(!numericFits(type,result)) fail('ARITH_RANGE');
      } else if(k==='ConstructVariant') result={tag:o.tag,value:v[0]};
      else if(k==='ProjectVariant') {if(v[0].tag!==o.tag) fail('VARIANT_CASE');result=v[0].value;}
      else if(k==='ProjectSome') {if(v[0].length===0) fail('OPTION_NONE');result=v[0][0];}
      else if(k==='ScalarValue') {const scalar=BigInt(v[0]);result=o.component==='negative'?scalar<0n:o.component==='magnitude'?String(scalar<0n?-scalar:scalar):v[0];}
      else if (k === 'ReadLocal') result = this.locals.get(o.name);
      else if (k === 'ReadArg') result = this.args[o.name];
      else if (k === 'ReadObs') result = this.obs[o.name];
      else if (k === 'ReadPre') result = o.view === 'post' && has(this.writes, o.field) ? this.writes[o.field] : this.pre[o.field];
      else if (k === 'ProjectField' || k === 'AccessField') result = v[0][o.field];
      else if (k === 'ProjectIndex' || k === 'AccessIndex') {
        const index = BigInt(v[1]); if (index >= BigInt(v[0].length)) fail('INDEX_RANGE'); result = v[0][Number(index)];
      } else if (k === 'ConstructRecord') result = Object.fromEntries(o.fields.map((f: any, i: number) => [f.name, v[i]]));
      else if (k === 'ConstructEnum') result = o.member;
      else if (k === 'ConstructSome') result = [v[0]];
      else if (k === 'ConstructNone') result = [];
      else if (k === 'ConstructCollection') result = v;
      else if (['Add', 'Sub', 'Mul', 'FloorDiv', 'CeilDiv'].includes(k)) {
        const a = BigInt(v[0]), b = BigInt(v[1]); let n: bigint;
        if (k === 'Add') n = a + b;
        else if (k === 'Sub') n = a - b;
        else if (k === 'Mul') n = a * b;
        else {
          if (b <= 0n) fail('ARITH_DENOMINATOR');
          let q = a / b, r = a % b; if (r < 0n) { q--; r += b; }
          n = k === 'CeilDiv' && r !== 0n ? q + 1n : q;
        }
        result = String(n); if (!numericFits(type, result)) fail('ARITH_RANGE');
      } else if (k === 'Eq') result = canonical(v[0]) === canonical(v[1]);
      else if (['Lt', 'Lte', 'Gt', 'Gte'].includes(k)) {
        const a = BigInt(v[0]), b = BigInt(v[1]); result = k === 'Lt' ? a < b : k === 'Lte' ? a <= b : k === 'Gt' ? a > b : a >= b;
      } else if (k === 'Not') result = !v[0];
      else if (k === 'Require' || k === 'Ensure') { if (!v[0]) fail(k === 'Require' ? 'GUARD_FAILED' : 'ENSURES_FAILED'); return; }
      else if (k === 'Let') { this.locals.set(o.name, v[0]); return; }
      else if (k === 'NextWrite') { this.writes[o.field] = v[0]; return; }
      else if (k === 'Emit') {
        const descriptor = { operation: o.operation, fields: v[0] };
        valueBound(['Operation', o.operation], descriptor, this.schema, 'DESCRIPTOR_BOUND');
        this.descriptors.push(descriptor);
        let nodes = 1, depth = 1;
        for (const d of this.descriptors) { const z = valueSize(['Operation', d.operation], d, this.schema); nodes += z.nodes; depth = Math.max(depth, 1 + z.depth); }
        if (this.descriptors.length > 128 || bytes(canonical(this.descriptors)) > 65536 || nodes > 4096 || depth > 64) fail('DESCRIPTOR_BOUND');
        return;
      } else throw new Error('Unimplemented admitted constructor: ' + k);
      valueBound(type, result, this.schema, 'VALUE_BOUND'); return result;
    });
  }
  run(request: string, checkOnly = false): ExpressionResult | ExpressionCheckResult {
    try {
      // The largest legal separate components fit below this derived envelope
      // limit even with JSON string escaping. Each semantic component still has
      // its own independent65536-byte ceiling below.
      const r = parseCanonical(request, 2_000_000);
      closed(r, ['contract', 'source', 'core', 'Pre', 'Args', 'Obs', 'workInitial']);
      if (r.contract !== FINANCIAL_EXPRESSION_CONTRACT_V1 || typeof r.source !== 'string') fail('INPUT_SCHEMA');
      this.sourceBytes = bytes(r.source); if (this.sourceBytes > 65536) fail('INPUT_BOUND');
      decimal(r.workInitial); const work = BigInt(r.workInitial);
      if (work < 0n || work > 65536n) fail('INPUT_BOUND');
      this.initial = Number(work); this.remaining = this.initial;
      for (const value of [this.schema, r.core, r.Pre, r.Args, r.Obs]) if (bytes(canonical(value)) > 65536) fail('INPUT_BOUND');
      schemaShape(this.schema);
      const isAction = typeof r.core === 'object' && r.core !== null && has(r.core, 'statements');
      const count = { nodes: 0 };
      if (isAction) this.site(r.core, [], () => {
        closed(r.core, ['statements', 'span']); this.spanShape(r.core.span); array(r.core.statements);
        if (r.core.statements.length > 256) fail('INPUT_BOUND');
        r.core.statements.forEach((n: CoreNode, i: number) => this.shape(n, [i], 1, count));
      });
      else this.shape(r.core, [], 1, count);
      validateSchema(this.schema);
      if (isAction) {
        let suffix = false;
        r.core.statements.forEach((n: CoreNode, i: number) => this.site(n, [i], () => {
          if (n.constructor === 'Ensure') suffix = true;
          else if (suffix) fail('TYPE_STATEMENT_PLACEMENT');
          this.infer(n, [i], n.constructor === 'Ensure', true);
        }));
      } else this.infer(r.core, [], false);
      if (checkOnly) return { judgmentResult: 'ExpressionChecked' };
      const fieldTypes = Object.fromEntries(Object.entries(this.schema.fields).map(([k, f]) => [k, (f as any).type]));
      validateSnapshot(fieldTypes, r.Pre, this.schema); validateSnapshot(this.schema.args, r.Args, this.schema); validateSnapshot(this.schema.observations, r.Obs, this.schema);
      this.pre = r.Pre; this.args = r.Args; this.obs = r.Obs; this.reducing = true;
      if (!isAction) {
        const value = this.reduce(r.core, []); return { judgmentResult: 'ExpressionValue', type: this.types.get(r.core)!, value, workRemaining: String(this.remaining) };
      }
      r.core.statements.forEach((n: CoreNode, i: number) => this.reduce(n, [i]));
      const post = { ...this.pre, ...this.writes };
      this.site(r.core, [], () => aggregateBound(fieldTypes, post, this.schema, 'VALUE_BOUND'));
      return { status: 'ExpressionPrepared', post, descriptors: this.descriptors, workRemaining: String(this.remaining) };
    } catch (error) {
      if (!(error instanceof ExpressionFailure)) throw error;
      return { status: 'Rejected', code: error.code, span: error.span, nodePath: error.path.map(String), workUsed: String(this.reducing ? this.initial - this.remaining : 0) };
    }
  }
}

/** The trusted host binds Σ once. Untrusted evaluate() requests cannot supply or
 * reclassify it. This factory is not profile registration or financial authority. */
export function createFinancialExpressionContractV1(schemaCanonicalJSON: string): {
  evaluate(requestCanonicalJSON: string): ExpressionResult;
  check(requestCanonicalJSON: string): ExpressionCheckResult;
} {
  // Retain only the immutable text; every call gets a fresh owned schema tree.
  // check() performs structure/schema/whole-action typing only: it cannot certify
  // supplied snapshots, run guards, or publish a prepared state or descriptors.
  function run(requestCanonicalJSON: string, checkOnly: boolean): ExpressionResult | ExpressionCheckResult {
    let schema: Schema;
    try { schema = parseCanonical(schemaCanonicalJSON, 65536); }
    catch (error) {
      if (!(error instanceof ExpressionFailure)) throw error;
      return { status: 'Rejected', code: error.code, span: SYNTHETIC_SPAN, nodePath: [], workUsed: '0' };
    }
    return new Machine(schema).run(requestCanonicalJSON, checkOnly);
  }
  return Object.freeze({
    evaluate: (request: string) => run(request, false) as ExpressionResult,
    check: (request: string) => run(request, true) as ExpressionCheckResult,
  });
}
