import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { parseSource, compile, canonicalEncode, canonicalDecode } from '../src/frontend.ts';
const bounds = readFileSync(new URL('../spec/bounds.json', import.meta.url));
const prefix = 'agreement Generic profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000;';
const minimal = body => `${prefix} state closed: UInt128 = uint(0); observation now: UInt128; status episode closed_when closed == uint(1); status agreement no_remaining_notional; action run(actor: Text) { ${body} } }`;

test('complete parser rejects comments, plain division, chained comparison, trailing input and unknown syntax', () => {
  for (const body of ['//comment\nguard true, "x";', 'let x = uint(8) / uint(2);', 'guard uint(1) < uint(2) < uint(3), "x";', 'while true {}', 'let constructor = uint(1);', 'let x = uint(01);', 'let x = uint(-1);', 'let x = uint(1e3);']) {
    assert.throws(() => parseSource(minimal(body)), undefined, body);
  }
  assert.throws(() => parseSource(minimal('') + ' garbage'));
  assert.throws(() => parseSource(Uint8Array.from([0xc0,0xaf])));
});

test('canonical codec uses ASCII key order, preserves scalar text, and rejects alternative bytes', () => {
  assert.equal(canonicalEncode({z:'é😀\u0000\n',a:true}), '{"a":true,"z":"é😀\\u0000\\n"}');
  assert.deepEqual(canonicalDecode('{"a":true}'), {a:true});
  for (const value of ['{"a":true,"a":false}', '{ "a":true}', '{"z":false,"a":true}', '{"a":1}', 'null', '"\\ud800"', '"\\u00e9"']) assert.throws(() => canonicalDecode(value), undefined, value);
  for (const value of [1,null,undefined,{a:undefined},'\ud800']) assert.throws(() => canonicalEncode(value));
});

test('UTF8 spans and erased parentheses retain independent literal span', () => {
  const source = minimal('guard text("é😀") == (text("é😀")), "yes";');
  const ast = parseSource(source);
  const action = ast.declarations.find(d => d.tag === 'ActionDecl');
  const expression = action.statements[0].condition.right;
  const bytes = Buffer.from(source);
  const slice = span => bytes.subarray(Number(span.startByte), Number(span.endByte)).toString();
  assert.equal(slice(expression.span), '(text("é😀"))');
  assert.equal(slice(expression.literal.span), 'text("é😀")');
  assert.equal(expression.literal.token, '"é😀"');
  const output = compile(source, bounds);
  assert.equal(output.typed.annotations[0].nodeId, 'a_0_s_0_e_0');
  assert.equal(output.typed.annotations[0].type.tag, 'Bool');
  assert.deepEqual(output.typed.annotations[0].unitVector, []);
  assert.deepEqual(output.typed.annotations[1].unitVector, []);
});

test('typing rejects forward values, duplicate state writes, wrong actors and nominal units', () => {
  for (const body of ['let x = y; let y = uint(1);', 'set closed = uint(1); set closed = uint(0);', 'guard uint(1) == text("1"), "x";', 'let x = not uint(1);', 'guard uint(1), "x";']) assert.throws(() => compile(minimal(body), bounds), undefined, body);
  assert.throws(() => compile(minimal('').replace('actor: Text', 'actor: UInt128'), bounds));
  assert.throws(() => compile(minimal('let x = amount(1,A) + amount(2,B);').replace('state closed', 'unit A; unit B; state closed'), bounds));
  assert.throws(() => compile(minimal('let x = const.later;').replace(/ } }$/, ' } const later: UInt128 = uint(1); }'), bounds));
});

test('both complete supplied programs produce deterministic full AST, typed annotations, Core and hashes', () => {
  for (const name of ['loan','swap']) {
    const source = readFileSync(new URL(`../spec/examples/${name}.moriarty`, import.meta.url));
    const result = compile(source, bounds);
    assert.ok(result.source.declarations.length > 20);
    assert.ok(result.typed.annotations.length > 90);
    assert.equal(result.bound.manifest.core.actions.length, 2);
    assert.deepEqual(result, compile(source, bounds));
    assert.equal(result.bound.programHash.length, 64);
    assert.equal(result.bound.manifest.requiredClaims.length, name === 'loan' ? 7 : 8);
    assert.ok(result.bound.manifest.initialState.filter(x => x.value.tag === 'Amount').every(x => x.value.unit));
    assert.equal(result.typed.annotations.length, result.bound.manifest.core.actions.reduce((n,a) => n + Number(a.resourceCounts.expressionNodes),0));
  }
});

test('precedence, left association, floor numerator preorder and per-statement IDs are exact', () => {
  const out=compile(minimal('let x = uint(8) - uint(3) - uint(2); let q = floor_div(uint(5) + uint(1), uint(2)); guard true or false and not false, "ok";'),bounds);
  const instructions=out.bound.manifest.core.actions[0].instructions;
  assert.equal(instructions[0].expression.tag,'Sub');
  assert.equal(instructions[0].expression.left.tag,'Sub');
  assert.equal(instructions[1].expression.nodeId,'a_0_s_1_e_0');
  assert.equal(instructions[1].expression.numerator.nodeId,'a_0_s_1_e_1');
  assert.equal(instructions[1].expression.denominator.nodeId,'a_0_s_1_e_4');
  assert.equal(instructions[2].condition.right.tag,'And');
});

test('unit algebra produces ordered Quantity vectors and cancels exactly without erasing nominal amounts', () => {
  const source=minimal('let product = amount(2,B) * amount(3,A); let quotient = floor_div(product, amount(2,B)); let scalar = floor_div(quotient, amount(1,A));').replace('state closed','unit B; unit A; state closed');
  const instructions=compile(source,bounds).bound.manifest.core.actions[0].instructions;
  assert.deepEqual(instructions[0].type,{tag:'Quantity',unitVector:[{exponent:'1',unit:'A'},{exponent:'1',unit:'B'}]});
  assert.deepEqual(instructions[1].type,{tag:'Amount',unit:'A'});
  assert.deepEqual(instructions[2].type,{tag:'UInt128'});
});

test('policies reject missing, duplicate, wrong-unit, future and nonfinancial bindings',()=>{
  const loan=readFileSync(new URL('../spec/examples/loan.moriarty',import.meta.url),'utf8');
  for(const mutated of [
    loan.replace('write(accrue, interest_due), ',''),
    loan.replace('write(accrue, interest_due)', 'write(accrue, interest_due), write(accrue, interest_due)'),
    loan.replace('unit USD_micro;', 'unit USD_micro; unit Wrong;').replace('    unit USD_micro;', '    unit Wrong;'),
    loan.replace('floor(accrue, interest_calculated)', 'floor(accrue, interest_numerator)'),
    loan.replace('write(accrue, interest_due)', 'write(accrue, cursor)'),
    loan.replace('effect(accrue, 1, amount)', 'effect(accrue, 99, amount)'),
  ])assert.throws(()=>compile(mutated,bounds));
});

test('explicit reserve rules are generic, hash-bound, and require the exact structural guard',()=>{
  const swap=readFileSync(new URL('../spec/examples/swap.moriarty',import.meta.url),'utf8');
  const good=compile(swap,bounds);
  assert.deepEqual(good.bound.manifest.reserveRules,[{action:'swap',closure:'close'}]);
  for(const mutated of [swap.replace('remaining > uint(1)','remaining >= uint(2)'),swap.replace('reserve swap for close;','reserve swap for swap;'),swap.replace('reserve swap for close;','reserve swap for missing;'),swap.replace('reserve swap for close;','reserve swap for close; reserve close for swap;')])assert.throws(()=>compile(mutated,bounds));
  const renamed=swap.replace(/\bswap\b/g,'exchange').replace(/\bclose\b/g,'finish');
  assert.equal(compile(renamed,bounds).bound.manifest.reserveRules[0].action,'exchange');
  assert.notEqual(compile(swap.replace('reserve swap for close;',''),bounds).bound.programHash,good.bound.programHash);
});

test('settlements, statuses and exact effect schemas reject malformed records',()=>{
  const loan=readFileSync(new URL('../spec/examples/loan.moriarty',import.meta.url),'utf8');
  for(const mutated of [loan.replace('quantum amount(1, USD_micro)','quantum amount(0, USD_micro)'),loan.replace('status agreement remaining_notional notional;','status agreement remaining_notional cursor;'),loan.replace('    from: Text;\n    to: Text;','    to: Text;\n    from: Text;'),loan.replace('observation now: UInt128;','observation now: Text;'),loan.replace('state cursor: UInt128 = uint(0);','state cursor: UInt128 = true;')])assert.throws(()=>compile(mutated,bounds));
});

test('every aggregate encoding and action shape limit is enforced jointly',()=>{
  const loan=readFileSync(new URL('../spec/examples/loan.moriarty',import.meta.url),'utf8');
  for(const [group,key] of [['astEncoding','utf8Bytes'],['astEncoding','decodedNodes'],['typedProgramEncoding','utf8Bytes'],['programManifestEncoding','utf8Bytes'],['programManifestEncoding','decodedDepthRootZero'],['programShape','expressionNodesPerEntrypoint'],['programShape','instructionsPerEntrypoint']]) {
    const reduced=JSON.parse(bounds);reduced[group][key]=1;assert.throws(()=>compile(loan,JSON.stringify(reduced)),undefined,`${group}.${key}`);
  }
  assert.throws(()=>compile(minimal('let huge = uint(340282366920938463463374607431768211456);'),bounds));
  assert.throws(()=>parseSource(minimal(`guard true, "${'é'.repeat(129)}";`)));
});

test('canonical codec rejects sparse, decorated and accessor containers',()=>{
  const sparse=new Array(1);sparse.foo='x';
  const symbol=[];symbol[Symbol('x')]=true;
  const hidden=[];Object.defineProperty(hidden,'hidden',{value:true});
  const getter=[];Object.defineProperty(getter,'0',{get(){throw new Error('getter must not execute');},enumerable:true});
  const record={};Object.defineProperty(record,'x',{get(){throw new Error('getter must not execute');},enumerable:true});
  for(const value of [sparse,symbol,hidden,getter,record])assert.throws(()=>canonicalEncode(value),e=>e.code==='NON_CANONICAL_VALUE');
});
