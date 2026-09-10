import test from 'node:test';
import assert from 'node:assert/strict';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';
import { parseExpressionSource, formatExpressionSource } from '../src/successor/expression-source-frontend.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const empty = () => ({ units: [], assets: [], vaults: [], parties: [], recordTypes: {}, enumTypes: {}, fields: {}, args: {}, observations: {}, operations: {} });
const source = body => `profile "moriarty-expression-source/1"; agreement Demo { action step() { ${body} } }`;
const invocation = (Pre = {}, Args = {}, Obs = {}, workInitial = '1000') => canonical({ Pre, Args, Obs, workInitial });
function expression(expr, type, expected, patch = {}, pre = {}) {
  const schema = { ...empty(), ...patch, fields: { out: { type, writeClass: 'ordinary' }, ...(patch.fields ?? {}) } };
  const runtime = createExpressionSourceV1(canonical(schema));
  const input = source(`next.out = ${expr};`);
  const result = runtime.evaluate(input, invocation({ out: expected, ...pre }));
  assert.equal(result.status, 'ExpressionPrepared', JSON.stringify(result));
  assert.deepEqual(result.post.out, expected);
  return runtime.elaborate(input);
}

test('source scalar types, signed extremes and named rounding keep exact values', () => {
  expression('u64(7)', ['UInt64'], '7');
  expression('1 + 2 * 3', ['UInt128'], '7');
  expression('i128(5) + -7', ['SInt128'], '-2');
  expression('floor_div(-5, i128(2))', ['SInt128'], '-3');
  expression('ceil_div(-5, i128(2))', ['SInt128'], '-2');
  expression('"é😀"', ['Text'], 'é😀');
  expression('not false and true or false', ['Bool'], true);
});
test('source financial literals retain every exact type index', () => {
  const patch = { assets: ['A', 'B'], vaults: ['Vault'], parties: ['Holder'], units: ['USD'] };
  expression('amount(5,A)', ['Amount', 'A'], '5', patch);
  expression('shares(5,Vault,Holder)', ['Shares', 'Vault', 'Holder'], '5', patch);
  expression('rate(-5,2)', ['Rate', '2'], '-5', patch);
  expression('price(5,A,B,2)', ['Price', 'A', 'B', '2'], '5', patch);
  expression('quantity<Units<USD,1>,2>(-5)', ['Quantity', [['USD', '1']], '2'], '-5', patch);
});
test('source record, enum, option, collection and both access aliases', () => {
  const patch = { recordTypes: { Row: { n: ['UInt64'] } }, enumTypes: { Mode: ['Closed', 'Open'] } };
  expression('record<Row> {n: u64(5)}', ['Record', 'Row'], { n: '5' }, patch);
  expression('Mode.Open', ['Enum', 'Mode'], 'Open', patch);
  expression('some<UInt64>(u64(5))', ['Option', ['UInt64']], ['5']);
  expression('none<UInt64>()', ['Option', ['UInt64']], []);
  expression('collection<UInt64,2>(u64(5))', ['Collection', ['UInt64'], '2'], ['5']);
  expression('(record<Row> { n: u64(5) }).n', ['UInt64'], '5', patch);
  expression('access_field(record<Row> {n: u64(5)}, "n")', ['UInt64'], '5', patch);
  expression('collection<UInt64,2>(u64(5))[u64(0)]', ['UInt64'], '5');
  expression('access_index(collection<UInt64,2>(u64(5)),u64(0))', ['UInt64'], '5');
});
test('source check rejects a dead branch wrong type before runtime guard', () => {
  const runtime = createExpressionSourceV1(canonical(empty()));
  const input = source('requires false; requires true or 1;');
  const result = runtime.evaluate(input, invocation());
  assert.equal(result.code, 'TYPE_MISMATCH');
  assert.equal(result.workUsed, '0');
  assert.equal(runtime.check(input).code, 'TYPE_MISMATCH');
});
test('source formatting preserves complete expression meaning and is idempotent', () => {
  const input = source('let xs=collection<UInt64,2>(u64(5));requires xs[u64(0)]==u64(5);');
  const formatted = formatExpressionSource(input);
  assert.equal(formatExpressionSource(formatted), formatted);
  const runtime = createExpressionSourceV1(canonical(empty()));
  assert.deepEqual(runtime.evaluate(input, invocation()), runtime.evaluate(formatted, invocation()));
  assert.equal(parseExpressionSource(formatted).profile.value, 'moriarty-expression-source/1');
});

test('one real source program reaches all forty reviewed constructors', () => {
  const schema = { ...empty(), assets: ['A', 'B'], parties: ['Holder'], vaults: ['Vault'], units: ['USD'],
    recordTypes: { Row: { n: ['UInt64'] } }, enumTypes: { Mode: ['Closed', 'Open'] },
    args: { input: ['UInt64'], row: ['Record', 'Row'] }, observations: { ready: ['Bool'] },
    fields: { counter: { type: ['UInt64'], writeClass: 'ordinary' } }, operations: { Notice: 'Row' } };
  const input = `profile "moriarty-expression-source/1";
  agreement AllExpressions {
    action step(input: UInt64, row: Record<Row>) {
      let number = u64(7);
      let signed = i128(-5);
      let truth = true;
      let label = "hello";
      let cash = amount(5,A);
      let quantityValue = quantity<Units<USD,1>,2>(-5);
      let sharesValue = shares(5,Vault,Holder);
      let rateValue = rate(-5,2);
      let priceValue = price(5,A,B,2);
      let observed = obs.ready;
      let before = pre.counter;
      let recordValue = record<Row> { n: number };
      let member = Mode.Open;
      let present = some<UInt64>(number);
      let absent = none<UInt64>();
      let items = collection<UInt64,2>(number);
      let projectedField = recordValue.n;
      let accessedField = access_field(recordValue,"n");
      let projectedIndex = items[u64(0)];
      let accessedIndex = access_index(items,u64(0));
      let sum = number + input;
      let difference = number - input;
      let product = number * input;
      let floorValue = floor_div(signed,i128(2));
      let ceilValue = ceil_div(signed,i128(2));
      requires number == u64(7);
      requires number < u64(8);
      requires number <= u64(7);
      requires number > u64(6);
      requires number >= u64(7);
      requires not false and observed or false;
      requires floorValue == i128(-3) and ceilValue == i128(-2);
      next.counter = before + input;
      emit Notice row;
      ensures post.counter == pre.counter + input;
    }
  }`;
  const runtime = createExpressionSourceV1(canonical(schema));
  const elaboration = runtime.elaborate(input);
  assert.equal(elaboration.judgmentResult, 'SourceElaborated', JSON.stringify(elaboration));
  const seen = new Set();
  const visit = value => {
    if (!value || typeof value !== 'object') return;
    if (Object.hasOwn(value, 'constructor')) seen.add(value.constructor);
    Object.values(value).forEach(visit);
  };
  visit(elaboration.core);
  assert.deepEqual([...seen].sort(), ['LitUInt','LitSInt','LitBool','LitText','LitAmount','LitQuantity','LitShares','LitRate','LitPrice',
    'ReadLocal','ReadArg','ReadObs','ReadPre','ProjectField','AccessField','ProjectIndex','AccessIndex',
    'ConstructRecord','ConstructEnum','ConstructSome','ConstructNone','ConstructCollection',
    'Add','Sub','Mul','FloorDiv','CeilDiv','Eq','Lt','Lte','Gt','Gte','Not','And','Or','Require','Let','NextWrite','Ensure','Emit'].sort());
  const result = runtime.evaluate(input, invocation({ counter: '10' }, { input: '2', row: { n: '42' } }, { ready: true }));
  assert.deepEqual(result.post, { counter: '12' });
  assert.deepEqual(result.descriptors, [{ operation: 'Notice', fields: { n: '42' } }]);
});

test('a direct source emission preserves its record operand and exact work', () => {
  const schema = { ...empty(), recordTypes: { Row: { n: ['UInt64'] } }, args: { row: ['Record','Row'] }, operations: { Notice: 'Row' } };
  const input = 'profile "moriarty-expression-source/1"; agreement Demo { action step(row: Record<Row>) { emit Notice row; } }';
  const runtime = createExpressionSourceV1(canonical(schema));
  const result = runtime.evaluate(input, invocation({}, { row: { n: '5' } }, {}, '2'));
  assert.equal(result.status, 'ExpressionPrepared');
  assert.equal(result.workRemaining, '0');
  assert.equal(runtime.elaborate(input).staticWorkBound, '2');
});

test('source boundaries admit128 collection items and65 parameters without widening old grammar', () => {
  const items = Array.from({ length: 128 }, (_, i) => `u64(${i})`).join(',');
  expression(`collection<UInt64,128>(${items})`, ['Collection',['UInt64'],'128'], Array.from({ length: 128 }, (_, i) => String(i)));
  const args = Object.fromEntries(Array.from({ length: 65 }, (_, i) => ['arg'+i, ['UInt64']]));
  const input = `profile "moriarty-expression-source/1"; agreement Demo { action step(${Object.keys(args).map(name => `${name}:UInt64`).join(',')}) { requires arg64 == u64(64); } }`;
  const runtime = createExpressionSourceV1(canonical({ ...empty(), args }));
  assert.equal(runtime.evaluate(input, invocation({}, Object.fromEntries(Object.keys(args).map((name, i) => [name, String(i)])))).status, 'ExpressionPrepared');
});

test('source metadata, equality and namespace boundaries reject deterministically', () => {
  const runtime = createExpressionSourceV1(canonical({ ...empty(), assets: ['A','B'], units: ['EUR','USD'] }));
  const failures = [
    ['let x = -0;', 'SOURCE_LITERAL_SHAPE'],
    ['let x = i128(1 + 2);', 'SOURCE_LITERAL_SHAPE'],
    ['let x = u64(-1);', 'TYPE_LITERAL'],
    ['let x = amount(-1,A);', 'SOURCE_LITERAL_SHAPE'],
    ['let x = amount(1,A) + amount(1,B);', 'TYPE_MISMATCH'],
    ['let x = u64(1) + 1;', 'TYPE_MISMATCH'],
    ['let x = price(1,A,A,2);', 'TYPE_LITERAL'],
    ['let x = rate(1,19);', 'TYPE_LITERAL'],
    ['let x = quantity<Units<USD,1,EUR,-1>,2>(1);', 'TYPE_LITERAL'],
    ['let x = quantity<Units<USD>,2>(1);', 'SOURCE_TYPE_SHAPE'],
    ['let x = none<Units>();', 'SOURCE_TYPE_SHAPE'],
    ['let x = none<Unit>();', 'TYPE_NAME'],
    ['let x = collection<UInt64,0>(u64(1));', 'TYPE_COLLECTION_BOUND'],
    ['let x = collection<UInt64,1>() == collection<UInt64,2>();', 'TYPE_MISMATCH'],
    ['let x = access_field(1,"not an identifier");', 'SOURCE_LITERAL_SHAPE'],
    ['let amount = 1;', 'SOURCE_RESERVED_NAME'],
    ['let x = unknown_function(1);', 'SOURCE_CALL'],
  ];
  for (const [body, code] of failures) {
    const result = runtime.evaluate(source(body), invocation());
    assert.equal(result.code, code, body + ': ' + JSON.stringify(result));
    assert.equal(result.workUsed, '0');
    assert.equal(Object.hasOwn(result, 'post'), false);
  }
});

test('source stage rollback includes descriptors and rejects direct financial writes', () => {
  const schema = { ...empty(), recordTypes: { Row: { n: ['UInt64'] } }, operations: { Notice: 'Row' },
    fields: { counter: { type: ['UInt64'], writeClass: 'ordinary' }, debt: { type: ['UInt64'], writeClass: 'financial' } } };
  const runtime = createExpressionSourceV1(canonical(schema));
  const snapshots = invocation({ counter: '10', debt: '20' });
  const input = source('next.counter = u64(11); emit Notice {n:u64(5)}; ensures false;');
  const result = runtime.evaluate(input, snapshots);
  assert.equal(result.code, 'ENSURES_FAILED');
  assert.equal(Object.hasOwn(result, 'post'), false);
  assert.equal(Object.hasOwn(result, 'descriptors'), false);
  assert.equal(runtime.evaluate(source('next.debt = u64(0);'), snapshots).code, 'TYPE_FINANCIAL_WRITE');
  const valid = runtime.evaluate(source('requires pre.counter == u64(10);'), snapshots);
  valid.post.counter = '999';
  assert.equal(runtime.evaluate(source('requires pre.counter == u64(10);'), snapshots).post.counter, '10');
});

test('source byte spans, lowered paths and short-circuit work survive formatting-independent text', () => {
  const runtime = createExpressionSourceV1(canonical(empty()));
  const skipped = source('requires true or floor_div(1,0) == 0;');
  assert.equal(runtime.evaluate(skipped, invocation({}, {}, {}, '3')).status, 'ExpressionPrepared');
  assert.ok(BigInt(runtime.elaborate(skipped).staticWorkBound) > 3n);
  const input = source('let label = "é😀"; requires false or floor_div(1,0) == 0;');
  const result = runtime.evaluate(input, invocation());
  assert.equal(result.code, 'ARITH_DENOMINATOR');
  assert.equal(Buffer.from(input).subarray(Number(result.span.start), Number(result.span.end)).toString(), 'floor_div(1,0)');
  assert.deepEqual(result.nodePath, ['1','0','1','0']);
  const exhausted = runtime.evaluate(source('requires true;'), invocation({}, {}, {}, '1'));
  assert.equal(exhausted.code, 'WORK_EXHAUSTED');
  assert.equal(exhausted.workUsed, '1');
  assert.deepEqual(exhausted.nodePath, ['0','0']);
});

test('source API rejects hostile transport, schema overrides and unsupported declaration semantics', () => {
  let touched = false;
  const hostile = { get source() { touched = true; throw new Error('must not access'); } };
  const runtime = createExpressionSourceV1(canonical(empty()));
  assert.equal(runtime.evaluate(hostile, invocation()).status, 'Rejected');
  assert.equal(touched, false);
  assert.equal(runtime.evaluate(source(''), canonical({ Pre: {}, Args: {}, Obs: {}, workInitial: '10', schema: empty() })).code, 'INPUT_SCHEMA');
  const program = 'profile "moriarty-expression-source/1"; agreement Demo { state x: UInt64 = u64(0); action step() {} }';
  assert.equal(runtime.check(program).code, 'SOURCE_DECLARATION');
  const multiple = 'profile "moriarty-expression-source/1"; agreement Demo { action first() {} action second() {} }';
  assert.equal(runtime.check(multiple).code, 'SOURCE_ACTION_COUNT');
  const keywordSchema = { ...empty(), fields: { state: { type: ['Bool'], writeClass: 'ordinary' } } };
  assert.equal(createExpressionSourceV1(canonical(keywordSchema)).check(source('')).code, 'SOURCE_SCHEMA_NAME');
});

test('formatter retains lowered Core structure, types and evaluation order', () => {
  const schema = { ...empty(), units: ['EUR','USD'], recordTypes: { Row: { text: ['Text'], n: ['UInt64'] } } };
  const input = source('let q=quantity<Units<EUR,-1,USD,1>,2>(- 5);let r=record<Row>{text:"é",n:u64(1)};requires r.n != u64(2);');
  const runtime = createExpressionSourceV1(canonical(schema));
  const strip = value => {
    if (Array.isArray(value)) return value.map(strip);
    if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value).filter(([k]) => k !== 'span').map(([k,v]) => [k,strip(v)]));
    return value;
  };
  const formatted = formatExpressionSource(input);
  assert.deepEqual(strip(runtime.elaborate(formatted).core), strip(runtime.elaborate(input).core));
  assert.equal(formatExpressionSource(formatted), formatted);
  assert.deepEqual(runtime.evaluate(formatted, invocation()), runtime.evaluate(input, invocation()));
});

test('D5 outer transport and component limits precede source parsing and typed admission', () => {
  const runtime = createExpressionSourceV1(canonical(empty()));
  assert.equal(runtime.evaluate('invalid source', ' '.repeat(2_000_001)).code, 'INPUT_BOUND');
  // The outer transport is not mistakenly capped at one component's 65536 bytes.
  const parts = canonical({ Pre: 'x'.repeat(30000), Args: 'x'.repeat(30000), Obs: 'x'.repeat(30000), workInitial: '1' });
  assert.ok(Buffer.byteLength(parts) > 65536);
  assert.equal(runtime.evaluate(source(''), parts).code, 'INPUT_SCHEMA');
  assert.equal(runtime.evaluate('invalid source', canonical({ Pre: 'x'.repeat(65535), Args: {}, Obs: {}, workInitial: '1' })).code, 'INPUT_BOUND');
  assert.equal(runtime.evaluate(source('requires false;'), invocation({}, {}, {}, '65537')).code, 'INPUT_BOUND');
});

test('source final aggregate bound rejects at the action after both ordinary writes', () => {
  const type = ['Collection', ['Collection', ['Bool'], '128'], '128'];
  const schema = { ...empty(), args: { payload: type }, fields: { a: { type, writeClass: 'ordinary' }, b: { type, writeClass: 'ordinary' } } };
  const input = 'profile "moriarty-expression-source/1"; agreement Demo { action step(payload: Collection<Collection<Bool,128>,128>) { next.a = payload; next.b = payload; } }';
  const runtime = createExpressionSourceV1(canonical(schema));
  const result = runtime.evaluate(input, invocation({ a: [], b: [] }, { payload: Array.from({ length: 16 }, () => Array(128).fill(false)) }));
  assert.equal(result.code, 'VALUE_BOUND');
  assert.equal(result.workUsed, '4');
  assert.deepEqual(result.nodePath, []);
  assert.match(Buffer.from(input).subarray(Number(result.span.start), Number(result.span.end)).toString(), /^action step/);
  assert.equal(Object.hasOwn(result, 'post'), false);
});
test('source descriptor128 limit rolls back the129th emission', () => {
  const runtime = createExpressionSourceV1(canonical({ ...empty(), recordTypes: { Empty: {} }, operations: { Notice: 'Empty' } }));
  const result = runtime.evaluate(source('emit Notice {};'.repeat(129)), invocation());
  assert.equal(result.code, 'DESCRIPTOR_BOUND');
  assert.deepEqual(result.nodePath, ['128']);
  assert.equal(result.workUsed, '258');
  assert.equal(Object.hasOwn(result, 'descriptors'), false);
});
test('new parameter256 and collection128 ceilings have explicit next-value rejection', () => {
  const args = Object.fromEntries(Array.from({ length: 256 }, (_, i) => ['p'+i, ['Bool']]));
  const declaration = count => `profile "moriarty-expression-source/1"; agreement Demo { action step(${Array.from({length:count},(_,i)=>`p${i}:Bool`).join(',')}) {} }`;
  const runtime = createExpressionSourceV1(canonical({ ...empty(), args }));
  assert.equal(runtime.check(declaration(256)).judgmentResult, 'SourceChecked');
  assert.equal(runtime.check(declaration(257)).code, 'ARITY_BOUND');
  const language = createExpressionSourceV1(canonical(empty()));
  assert.equal(language.check(source(`let items = collection<Bool,128>(${Array(129).fill('true').join(',')});`)).code, 'ARITY_BOUND');
  assert.equal(language.check(source('let items = collection<Bool,129>();')).code, 'TYPE_COLLECTION_BOUND');
});
