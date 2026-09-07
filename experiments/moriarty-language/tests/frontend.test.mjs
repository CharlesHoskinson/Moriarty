import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { parseSource, compile, canonicalEncode, canonicalDecode } from '../src/frontend.ts';
import {checkAndLower} from '../src/checker.ts';
import {validateSource} from '../src/validate.ts';
import {normalizeError,throwDiagnostic} from '../src/diagnostics.ts';
// INTERNAL NONADMITTED configurations isolate limit checks. No public API accepts these.
function lowerNonAdmittedTestConfiguration(input,boundsInput,validateManifest=true){
  try{const source=parseSource(input);const configured=JSON.parse(boundsInput);const error=validateSource(source,configured);if(error)throwDiagnostic(error);return {source,...checkAndLower(source,new TextEncoder().encode(boundsInput),validateManifest)};}catch(e){throwDiagnostic(normalizeError(e));}
}
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
    const reduced=JSON.parse(bounds);reduced[group][key]=1;assert.throws(()=>lowerNonAdmittedTestConfiguration(loan,JSON.stringify(reduced)),e=>{assert.equal(e.code,group==='astEncoding'?'AST_BOUNDS':group==='programManifestEncoding'?'PROGRAM_ENCODING':'PROGRAM_BOUNDS');return true;},`${group}.${key}`);
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

test('profile-valid redundant parentheses exceed128 without consuming semantic depth',()=>{
  const source=minimal(`let x = ${'('.repeat(12000)}uint(1)${')'.repeat(12000)};`);
  const output=compile(source,bounds);
  assert.equal(output.bound.manifest.core.actions[0].resourceCounts.expressionDepth,'1');
  const expr=output.source.declarations.at(-1).statements[0].expression;
  assert.equal(Number(expr.span.endByte)-Number(expr.span.startByte),24007);
  assert.equal(Number(expr.literal.span.endByte)-Number(expr.literal.span.startByte),7);
  assert.throws(()=>compile(minimal(`let x = ${'floor_div(uint(1),'.repeat(1500)}uint(1)${')'.repeat(1500)};`),bounds),e=>e.name==='FrontendError'&&e.code==='AST_BOUNDS');
});

test('public frontend returns one closed diagnostic at earliest stage with exact fields',async()=>{
  const api=await import('../src/frontend.ts');
  assert.equal(typeof api.parse,'function');assert.equal(typeof api.check,'function');assert.equal(typeof api.elaborate,'function');
  const lexical=api.parse(minimal('let x = uint(01);'),bounds);
  assert.deepEqual(Object.keys(lexical).sort(),['code','message','primarySpan','relatedSpans','stage']);
  assert.equal(lexical.code,'LEXICAL_TOKEN');assert.equal(lexical.stage,'2');assert.equal(lexical.message,lexical.code);assert.deepEqual(lexical.relatedSpans,[]);
  const duplicate=api.check(minimal('guard uint(1), "bad type"; set closed = uint(0); set closed = uint(1);'),bounds);
  assert.equal(duplicate.code,'DUPLICATE_NAME');assert.equal(duplicate.stage,'4');
  const premature=api.check(minimal('guard uint(1), "bad type"; let x = missing;'),bounds);
  assert.equal(premature.code,'NAME_RESOLUTION');assert.equal(premature.stage,'5');
  const range=api.check(minimal('let x = uint(340282366920938463463374607431768211456);'),bounds);
  assert.equal(range.code,'UINT_RANGE');assert.equal(range.stage,'6');
  const lexicalBeforeRange=api.parse(minimal('let x = uint(340282366920938463463374607431768211456); /'),bounds);
  assert.equal(lexicalBeforeRange.code,'LEXICAL_TOKEN');
  const good=api.elaborate(minimal(''),bounds);assert.equal(good.schemaVersion,'moriarty-program/1');
});

test('independent stage6 errors select smallest source span, including earlier policies',async()=>{
  const {check}=await import('../src/frontend.ts');
  assert.equal(typeof check,'function');
  const loan=readFileSync(new URL('../spec/examples/loan.moriarty',import.meta.url),'utf8');
  const bad=loan.replace('effect(accrue, 1, amount)','effect(accrue, 99, amount)').replace('action settle(actor: Text,','action settle(actor: UInt128,');
  const result=check(bad,bounds);assert.equal(result.code,'POLICY_TARGET');
  const missingStatus=minimal('guard uint(1), "x";').replace('status agreement no_remaining_notional;','');
  assert.equal(check(missingStatus,bounds).code,'STATUS_RULE');
});

test('validated canonical record decoding requires a schema validator',async()=>{
  const {decodeCanonicalRecord}=await import('../src/frontend.ts');
  assert.throws(()=>decodeCanonicalRecord('{"extra":true}'));
  const validate=value=>{if(typeof value!=='object'||value===null||Object.keys(value).join(',')!=='accepted'||typeof value.accepted!=='boolean')throw new Error('closed schema');};
  assert.throws(()=>decodeCanonicalRecord('{"accepted":true,"extra":true}',validate));
  assert.deepEqual(decodeCanonicalRecord('{"accepted":true}',validate),{accepted:true});
});

test('semantic depth boundary16 accepts and17 rejects with PROGRAM_BOUNDS',async()=>{
  const {check}=await import('../src/frontend.ts');
  const chain=n=>Array.from({length:n},()=> 'uint(1)').join(' + ');
  assert.equal(compile(minimal(`let x = ${chain(16)};`),bounds).bound.manifest.core.actions[0].resourceCounts.expressionDepth,'16');
  const tooDeep=check(minimal(`let x = ${chain(17)};`),bounds);
  assert.equal(tooDeep.code,'PROGRAM_BOUNDS');assert.equal(tooDeep.stage,'7');
  const reduced=JSON.parse(bounds);reduced.typedProgramEncoding.utf8Bytes=1;
  assert.throws(()=>lowerNonAdmittedTestConfiguration(minimal(`let x = ${chain(17)};`),JSON.stringify(reduced)),e=>{assert.equal(e.code,'PROGRAM_BOUNDS');assert.deepEqual(e.primarySpan,{startByte:'0',endByte:'0'});return true;});
});

test('both example hashes and canonical full records retain audited materialization bytes',()=>{
  for(const name of ['loan','swap']){
    const source=readFileSync(new URL(`../spec/examples/${name}.moriarty`,import.meta.url));
    const result=compile(source,bounds);
    const root=new URL(`../../../evidence/moriarty-completion-program-2026-09-07/MC01/profile-04/materialized/${name}/`,import.meta.url);
    for(const [file,value] of [['source-ast',result.source],['typed-program',result.typed],['bound-program',result.bound]])assert.equal(canonicalEncode(value),readFileSync(new URL(`${file}.json`,root),'utf8'));
  }
});

test('internal nonadmitted configuration isolates typed versus manifest bounds; public APIs reject overrides',async()=>{
  const {check,elaborate}=await import('../src/frontend.ts');
  const reduced=JSON.parse(bounds);reduced.programManifestEncoding.utf8Bytes=1;
  assert.equal(lowerNonAdmittedTestConfiguration(minimal(''),JSON.stringify(reduced),false).typed.schemaVersion,'moriarty-typed-program/1');
  assert.throws(()=>lowerNonAdmittedTestConfiguration(minimal(''),JSON.stringify(reduced)),e=>e.code==='PROGRAM_ENCODING');
  assert.equal(check(minimal(''),JSON.stringify(reduced)).code,'PROGRAM_ENCODING');
  assert.equal(elaborate(minimal(''),JSON.stringify(reduced)).code,'PROGRAM_ENCODING');
});

test('malformed settlement literal kinds are stage4 declaration errors after complete syntax',async()=>{
  const {parse,check,elaborate}=await import('../src/frontend.ts');
  const source=minimal('').replace('state closed','settlement malformed asset uint(1) quantum text("bad"); state closed');
  for(const api of [parse,check,elaborate]){
    const result=api(source,bounds);
    assert.equal(result.code,'DECLARATION_SCHEMA');assert.equal(result.stage,'4');
    assert.equal(Buffer.from(source).subarray(Number(result.primarySpan.startByte),Number(result.primarySpan.endByte)).toString(),'settlement malformed asset uint(1) quantum text("bad");');
    assert.deepEqual(Object.keys(result).sort(),['code','message','primarySpan','relatedSpans','stage']);
  }
  assert.throws(()=>parseSource(source),e=>e.code==='DECLARATION_SCHEMA');
  const trailingSyntax=check(source+' agreement',bounds);
  assert.equal(trailingSyntax.code,'PARSE_ERROR');assert.equal(trailingSyntax.stage,'3');
  const earlierDuplicate=source.replace('settlement malformed','unit Duplicate; unit Duplicate; settlement malformed');
  assert.equal(check(earlierDuplicate,bounds).code,'DUPLICATE_NAME');
  const laterDuplicate=source.replace(' state closed',' unit Duplicate; unit Duplicate; state closed');
  assert.equal(check(laterDuplicate,bounds).code,'DECLARATION_SCHEMA');
});

test('settlement wrong shape wins before semantic errors while valid shapes reach stage6',async()=>{
  const {check}=await import('../src/frontend.ts');
  const insert=declaration=>minimal('guard uint(1), "wrong guard type";').replace('state closed',`${declaration} state closed`);
  for(const declaration of ['settlement malformed asset false quantum amount(0, Missing);','settlement malformed asset text("") quantum uint(0);']){
    const d=check(insert(declaration),bounds);assert.equal(d.code,'DECLARATION_SCHEMA');assert.equal(d.stage,'4');
  }
  const semantic=insert('unit U; settlement bad asset text("") quantum amount(0, U);');
  const d=check(semantic,bounds);assert.equal(d.code,'SETTLEMENT_DECLARATION');assert.equal(d.stage,'6');
});

test('keyword misuse is syntax stage3, reserved tokens are lexical stage2, with numeric stage priority',async()=>{
  const {parse}=await import('../src/frontend.ts');
  for(const body of ['let state = uint(1);','let x = ;','let x = state.amount;','let x = uint(1); let action = uint(2);']){
    const d=parse(minimal(body),bounds);assert.equal(d.code,'PARSE_ERROR',body);assert.equal(d.stage,'3',body);
  }
  const keywordAndSlash=minimal('let state = uint(1); /');
  const lexical=parse(keywordAndSlash,bounds);assert.equal(lexical.code,'LEXICAL_TOKEN');assert.equal(lexical.stage,'2');
  assert.equal(Number(lexical.primarySpan.startByte),Buffer.byteLength(keywordAndSlash.slice(0,keywordAndSlash.lastIndexOf('/'))));
  const earlyReserved=minimal('guard text("é😀") == text("é😀"), "ok"; let constructor = uint(1); /');
  const reserved=parse(earlyReserved,bounds);assert.equal(reserved.code,'LEXICAL_TOKEN');
  assert.equal(Number(reserved.primarySpan.startByte),Buffer.byteLength(earlyReserved.slice(0,earlyReserved.indexOf('constructor'))));
  const earlySlash=minimal('/ let constructor = uint(1);');
  assert.equal(Number(parse(earlySlash,bounds).primarySpan.startByte),Buffer.byteLength(earlySlash.slice(0,earlySlash.lastIndexOf('/'))));
  const twoKeywords=minimal('let state = uint(1); let action = uint(2);');
  const first=parse(twoKeywords,bounds);assert.equal(first.code,'PARSE_ERROR');
  assert.equal(Number(first.primarySpan.startByte),Buffer.byteLength(twoKeywords.slice(0,twoKeywords.indexOf('state ='))));
});

test('Const State Episode and guard literals preserve schema-versus-typing distinctions',async()=>{
  const {parse,check}=await import('../src/frontend.ts');
  for(const declaration of ['const wrong: UInt128 = true;','state wrong: UInt128 = text("1");']){
    const source=minimal('').replace('state closed',`${declaration} state closed`);
    assert.equal(parse(source,bounds).schemaVersion,'moriarty-ast/1');
    assert.equal(check(source,bounds).code,'TYPE_MISMATCH');assert.equal(check(source,bounds).stage,'6');
    assert.equal(check(source.replace('action run','unit Duplicate; unit Duplicate; action run'),bounds).code,'DUPLICATE_NAME');
  }
  const status=minimal('').replace('closed == uint(1)','closed == false');
  assert.equal(parse(status,bounds).schemaVersion,'moriarty-ast/1');assert.equal(check(status,bounds).code,'STATUS_RULE');
  assert.equal(check(minimal('guard uint(1), "wrong";'),bounds).code,'TYPE_MISMATCH');
  assert.equal(check(minimal('guard true, "ok";'),bounds).schemaVersion,'moriarty-typed-program/1');
  assert.equal(parse(minimal('').replace('state closed: UInt128','state closed: Amount'),bounds).code,'PARSE_ERROR');
  assert.equal(parse(minimal('').replace('state closed: UInt128','state closed: Bool'),bounds).code,'PARSE_ERROR');
  const wrongEffect=minimal('').replace('action run','unit U; effect Transfer { asset: Text; from: Text; to: Text; amount: Amount<U>; } action run');
  assert.equal(check(wrongEffect,bounds).code,'DECLARATION_SCHEMA');
});
