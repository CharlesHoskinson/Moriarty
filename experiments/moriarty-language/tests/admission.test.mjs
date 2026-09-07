import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {compile,parse,check,elaborate,parseSource,FrontendError} from '../src/frontend.ts';
import {checkAndLower} from '../src/checker.ts';
import {derive,evaluate,createSimulator} from '../src/evaluate.ts';
const bounds=readFileSync(new URL('../spec/bounds.json',import.meta.url));
const source=readFileSync(new URL('../spec/examples/loan.mori',import.meta.url));
const registeredHash='ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c';
const malformed=()=>[
  undefined,null,{},[],false,1,'','null','{',
  JSON.stringify({schemaVersion:'moriarty-bounds/1',semanticProfile:'moriarty-bounded-atomic/1'}),
  JSON.stringify({...JSON.parse(bounds),domainRegistry:{invalid:'unregistered'}}),
  JSON.stringify({...JSON.parse(bounds),astEncoding:{...JSON.parse(bounds).astEncoding,utf8Bytes:999999999}}),
  JSON.stringify({...JSON.parse(bounds),programShape:{...JSON.parse(bounds).programShape,expressionDepthRootOne:99999}}),
  JSON.stringify({...JSON.parse(bounds),astEncoding:null}),
  JSON.stringify(JSON.parse(bounds)),
  bounds.toString()+'\n',
  Uint8Array.from([0xff,0xfe]),
];
const closed=d=>{assert.deepEqual(Object.keys(d).sort(),['code','message','primarySpan','relatedSpans','stage']);assert.equal(d.code,'PROGRAM_ENCODING');assert.equal(d.stage,'8');assert.equal(d.message,'PROGRAM_ENCODING');assert.deepEqual(d.relatedSpans,[]);assert.deepEqual(d.primarySpan,{startByte:'0',endByte:'0'});};

test('exact registered bytes independently reproduce the pinned bounds hash and accepted program',()=>{
  assert.equal(createHash('sha256').update('MORIARTY-BOUNDS-bounded-atomic/1').update(Uint8Array.of(0)).update(bounds).digest('hex'),registeredHash);
  for(const bytes of [bounds,new Uint8Array(bounds),bounds.toString()])assert.equal(compile(source,bytes).bound.manifest.bounds.boundsHash,registeredHash);
});

test('public frontend rejects all modified and malformed bounds without leaked host exceptions',()=>{
  for(const bad of malformed()){
    for(const api of [parse,check,elaborate]){
      let result;assert.doesNotThrow(()=>{result=api(source,bad);},`${api.name}: ${String(bad).slice(0,80)}`);closed(result);
    }
    assert.throws(()=>compile(source,bad),e=>{assert.ok(e instanceof FrontendError);closed(e.diagnostic);return true;});
    assert.throws(()=>createSimulator(source,bad),e=>e instanceof FrontendError&&e.code==='PROGRAM_ENCODING');
  }
});

test('earlier source diagnostics retain stage priority over malformed registry bytes',()=>{
  const valid=source.toString();
  for(const [broken,code,stage] of [
    [valid+' /','LEXICAL_TOKEN','2'],
    [valid+' agreement','PARSE_ERROR','3'],
    [valid.replace('unit USD_micro;','unit USD_micro; unit USD_micro;'),'DUPLICATE_NAME','4'],
    [valid.replace('let interest_numerator = state.notional','let interest_numerator = state.missing'),'NAME_RESOLUTION','5'],
    [valid.replace('action accrue(actor: Text)','action accrue(actor: UInt128)'),'ACTOR_PARAMETER','6'],
    [valid.replace('let interest_numerator = state.notional','let interest_numerator = '+Array(18).fill('uint(1)').join(' + ')+' + state.notional'),'TYPE_MISMATCH','6'],
  ]){
    for(const api of [check,elaborate]){const d=api(broken,'{}');assert.equal(d.code,code);assert.equal(d.stage,stage);}
  }
  assert.equal(parse(valid.replace('unit USD_micro;','unit USD_micro; unit USD_micro;'),'{}').code,'DUPLICATE_NAME');
});

test('derive rechecks exact registry even when a forged full program matches altered bounds',()=>{
  const changed=JSON.stringify({...JSON.parse(bounds),domainRegistry:{invalid:'unregistered'}});
  // Deliberate attack fixture from INTERNAL, NONADMITTED lowering. Never a public bypass.
  const forged=checkAndLower(parseSource(source),new TextEncoder().encode(changed)).bound;
  assert.notEqual(forged.manifest.bounds.boundsHash,registeredHash);
  const result=derive(forged,{}, {source,bounds:changed});
  assert.equal(result.outcome,'Rejected');closed(result.diagnostics[0]);
  for(const bad of malformed()){
    const rejected=derive(compile(source,bounds).bound,{}, {source,bounds:bad});
    assert.equal(rejected.outcome,'Rejected');closed(rejected.diagnostics[0]);
  }
});

test('backend registry admission precedes authentication or commit, including absent binding bytes',async()=>{
  const bound=compile(source,bounds).bound;
  for(const bad of malformed()){
    let calls=0;
    const backend={source,bounds:bad,authenticate:async()=>{calls++;return {};},verifyAndCommit:async()=>{calls++;return {};}};
    const result=await evaluate(bound,{},backend);assert.equal(result.outcome,'Rejected');closed(result.diagnostics[0]);assert.equal(calls,0);
  }
  const absent=await evaluate(bound,{});assert.equal(absent.diagnostics[0].code,'PROOF_INVALID');
});

test('detached or disguised byte views fail closed; caller mutation cannot alter the trusted registry',async()=>{
  const detached=new Uint8Array(bounds);structuredClone(detached.buffer,{transfer:[detached.buffer]});
  const disguised=new Proxy(new Uint8Array(bounds),{});
  for(const invalid of [detached,disguised,new DataView(new ArrayBuffer(4)),new Uint16Array(4),new ArrayBuffer(4)]){
    for(const api of [parse,check,elaborate]){let result;assert.doesNotThrow(()=>{result=api(source,invalid);});closed(result);}
  }
  const {registeredBounds}=await import('../src/registered-bounds.ts');
  const copy=registeredBounds();copy.bytes.fill(0);copy.bounds.astEncoding.utf8Bytes=1;
  assert.equal(compile(source,bounds).bound.manifest.bounds.boundsHash,registeredHash);
});

test('a source stage7 failure is retained ahead of a bounds stage8 mismatch',()=>{
  const tooDeep=`agreement Scalar profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 100; state closed: UInt128 = uint(0); observation now: UInt128; status episode closed_when closed == uint(1); status agreement no_remaining_notional; action run(actor: Text) { let x = ${Array(17).fill('uint(1)').join(' + ')}; } }`;
  for(const api of [check,elaborate]){const result=api(tooDeep,'{}');assert.equal(result.code,'PROGRAM_BOUNDS');assert.equal(result.stage,'7');}
});

test('oversized bounds and decorated typed-array length cannot bypass bounded admission',()=>{
  const hugeString=' '.repeat(bounds.length+1);
  const hugeBytes=new Uint8Array(bounds.length+1);
  class DisguisedLength extends Uint8Array {get byteLength(){throw new Error('user byteLength getter must not run');}}
  const disguised=new DisguisedLength(bounds.length+1);
  for(const bytes of [hugeString,hugeBytes,disguised,'é'.repeat(bounds.length)])for(const api of [parse,check,elaborate]){let result;assert.doesNotThrow(()=>{result=api(source,bytes);});closed(result);}
});

test('oversized bounds are rejected before encoder or digest work on their contents',async()=>{
  const {admitRegisteredBounds}=await import('../src/registered-bounds.ts');
  const oversized=' '.repeat(bounds.length+1),bytes=new Uint8Array(bounds.length+1);
  const encoder=TextEncoder.prototype.encode,hashPrototype=Object.getPrototypeOf(createHash('sha256')),update=hashPrototype.update;
  TextEncoder.prototype.encode=function(value){if(value===oversized)throw new Error('oversized string reached encoder');return encoder.call(this,value);};
  hashPrototype.update=function(value,...rest){if(value instanceof Uint8Array&&value.byteLength===bytes.byteLength)throw new Error('oversized buffer reached hash');return update.call(this,value,...rest);};
  try{for(const value of [oversized,bytes])assert.throws(()=>admitRegisteredBounds(value),e=>e instanceof FrontendError&&e.code==='PROGRAM_ENCODING');}
  finally{TextEncoder.prototype.encode=encoder;hashPrototype.update=update;}
});

test('missing local binding and invalid simulator byte containers reject at program admission',()=>{
  const bound=compile(source,bounds).bound;
  for(const binding of [undefined,null,{},[],{bounds},{source:undefined,bounds}]){const result=derive(bound,{},binding);assert.equal(result.outcome,'Rejected');closed(result.diagnostics[0]);}
  const disguised=new Proxy(new Uint8Array(bounds),{});
  for(const bad of [disguised,{length:2**40}])assert.throws(()=>createSimulator(source,bad),e=>e instanceof FrontendError&&e.code==='PROGRAM_ENCODING');
});

test('closed runtime schema admission precedes authentication even with registered source and bounds',async()=>{
 const bound=compile(source,bounds).bound;let authentication=0,commits=0;
 const backend={source,bounds,authenticate:async()=>{authentication++;return {};},verifyAndCommit:async()=>{commits++;return {};}};
 for(const malformed of [{},{schemaVersion:'moriarty-evaluation/1',extra:true}]){
  const result=await evaluate(bound,malformed,backend);assert.equal(result.diagnostics[0].code,'INPUT_SCHEMA');assert.equal(authentication,0);assert.equal(commits,0);
 }
});
test('Core source-map edits have SOURCE_MAP while non-Core manifest edits have PROGRAM_ENCODING',async()=>{
 const original=compile(source,bounds).bound;
 for(const [edit,expected] of [[b=>b.manifest.core.actions[0].instructions[0].sourceRef.spans[0].endByte='1','SOURCE_MAP'],[b=>b.manifest.name='Changed','PROGRAM_ENCODING']]){
  const bound=structuredClone(original);edit(bound);bound.programHash=createHash('sha256').update('MORIARTY-PROGRAM-bounded-atomic/1').update(Uint8Array.of(0)).update((await import('../src/codec.ts')).canonicalEncode(bound.manifest)).digest('hex');
  const result=derive(bound,{}, {source,bounds});assert.equal(result.diagnostics[0].code,expected);assert.equal(result.diagnostics[0].stage,'8');
  let auth=0;const backend={source,bounds,authenticate:async()=>{auth++;return {};},verifyAndCommit:async()=>({})};const accepted=await evaluate(bound,{},backend);assert.equal(accepted.diagnostics[0].code,expected);assert.equal(auth,0);
 }
});
