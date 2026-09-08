You are the user-selected Grok 4.6 high implementation worker. Write the concrete edits immediately using Edit/Write tools. All required code and current fragments follow. Do not begin another planning or permission cycle. Parent completed the required source/design review, worktree, and resource admission. This is Moriarty, not bridge/Goldbach. Latest user routing supersedes older Fable audit requirements. No agents or provider calls. No shell commands, tests, dependency installs, network/web, native proofs, wallet access, or Git writes. Parent runs tests and independent GPT6 review. Worktree: /home/charl/Moriarty/.worktrees/sp01-grok-high. Input commit fd241f897001f74dac23a4ddf18729ed63270e81.

OBJECTIVE: Add exactly the planned SP01 decoder regression tests. This is the tests-only phase. Do not modify implementation.
FILES: Own only experiments/moriarty-language/tests/semantics.test.mjs plus FOREMAN_REPORT.md and FOREMAN_REPORT.json. Preserve all existing tests.
INTERFACES: Existing setup/input/amount helpers supply fixtures. Current opening lines:
```js
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createSimulator, evaluate, derive, hash, sealState} from '../src/evaluate.ts';
const bounds=readFileSync(new URL('../spec/bounds.json',import.meta.url));
const source=name=>readFileSync(new URL(`../spec/examples/${name}.mori`,import.meta.url));
const text=value=>({tag:'Text',value});
const uint=value=>({tag:'UInt128',value:String(value)});
const amount=(value,unit)=>({tag:'Amount',value:String(value),unit});
```
Add decodeEvaluation to the existing evaluate import and add these codec/registry imports (avoid duplicate bindings):
```js
import {createSimulator, evaluate, derive, hash, sealState, decodeEvaluation} from '../src/evaluate.ts';
import {canonicalEncode} from '../src/codec.ts';
import {registeredBounds} from '../src/registered-bounds.ts';

```
Append this exact test block:
```js
function decoderFixture() { return input(setup('loan'),'accrue',[]); }
function decoderReject(carrier,code='INPUT_SCHEMA') {
  assert.throws(()=>decodeEvaluation(carrier),error=>{
    assert.equal(error.code,code);
    assert.equal(error.stage,9);
    return true;
  });
}
function decoderNodeCount(v) {
  if(Array.isArray(v))return 1+v.reduce((sum,x)=>sum+decoderNodeCount(x),0);
  if(v!==null&&typeof v==='object')return 1+Object.values(v).reduce((sum,x)=>sum+decoderNodeCount(x),0);
  return 1;
}
function decoderNodeTree(nodes) {
  if(nodes===1)return true;
  const children=[];
  for(let remaining=nodes-1;remaining>0;){
    const count=Math.min(remaining,129);
    children.push(count===1?true:Array(count-1).fill(true));
    remaining-=count;
  }
  assert.ok(children.length<=128);
  return children;
}
function decoderWireAtBytes(target) {
  const i=decoderFixture();
  i.state.body.obligations=Array.from({length:64},()=>({
    amount:amount(1,'USD_micro'),creditor:'',debtor:'',denomination:'',dueId:'',status:'Outstanding'
  }));
  let remaining=target-Buffer.byteLength(canonicalEncode(i),'utf8');
  assert.ok(remaining>=0);
  for(const obligation of i.state.body.obligations){
    for(const field of ['creditor','debtor','denomination','dueId']){
      const count=Math.min(remaining,256);
      obligation[field]='x'.repeat(count);
      remaining-=count;
    }
  }
  assert.equal(remaining,0,'the closed-schema fixture has sufficient padding capacity');
  const wire=canonicalEncode(i);
  assert.equal(Buffer.byteLength(wire,'utf8'),target);
  return {i,wire};
}

test('SP01 decoder accepts canonical string, Buffer and Uint8Array as frozen inputs',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  for(const carrier of [wire,Buffer.from(wire),new TextEncoder().encode(wire)]){
    const decoded=decodeEvaluation(carrier);
    assert.deepEqual(decoded,i);
    assert.ok(Object.isFrozen(decoded));
    assert.ok(Object.isFrozen(decoded.authority.statement));
    assert.ok(Object.isFrozen(decoded.action.arguments[0].value));
    assert.throws(()=>{decoded.action.name='settle';},TypeError);
  }
});

test('SP01 decoder ignores hostile shadow properties on genuine byte views',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  for(const bytes of [Buffer.from(wire),new TextEncoder().encode(wire)]){
    let calls=0;
    for(const key of ['length','byteLength','byteOffset','buffer','constructor',Symbol.iterator]){
      Object.defineProperty(bytes,key,{configurable:true,get(){calls++;throw Error('caller getter');}});
    }
    assert.deepEqual(decodeEvaluation(bytes),i);
    assert.equal(calls,0);
  }
});

test('SP01 decoder rejects proxies and non-byte impostors without caller code',()=>{
  let calls=0;
  const trap=()=>{calls++;throw Error('caller trap');};
  const traps={get:trap,getPrototypeOf:trap,ownKeys:trap,getOwnPropertyDescriptor:trap};
  const getterObject={get length(){return trap();},get byteLength(){return trap();},[Symbol.iterator]:trap};
  const revoked=Proxy.revocable(new Uint8Array(0),traps);revoked.revoke();
  for(const carrier of [
    getterObject,new Proxy(new Uint8Array(0),traps),revoked.proxy,
    new Proxy({},traps),Object.create(Uint8Array.prototype),
    new Uint8ClampedArray(0),new Uint16Array(0),new DataView(new ArrayBuffer(0)),
    new ArrayBuffer(0),[],null,undefined,42,true,new String('{}')
  ]){
    decoderReject(carrier);
    assert.equal(calls,0);
  }
});

test('SP01 decoder rejects detached and oversized byte views by intrinsic size',()=>{
  const detached=new Uint8Array(8);
  structuredClone(detached.buffer,{transfer:[detached.buffer]});
  decoderReject(detached);
  const oversized=new Uint8Array(65537);let calls=0;
  Object.defineProperty(oversized,'length',{get(){calls++;return 0;}});
  decoderReject(oversized,'INPUT_BOUNDS');
  assert.equal(calls,0);
  decoderReject(new Uint8Array(0));
});

test('SP01 decoder snapshots precisely the byte view before downstream decoding',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  const storage=Buffer.from('!'+wire+'!');
  const bytes=new Uint8Array(storage.buffer,storage.byteOffset+1,Buffer.byteLength(wire));
  registeredBounds();
  const original=TextDecoder.prototype.decode;let observed=0;
  TextDecoder.prototype.decode=function(snapshot,options){
    observed++;
    assert.notEqual(snapshot,bytes);
    assert.notEqual(snapshot.buffer,bytes.buffer);
    assert.equal(snapshot.byteLength,Buffer.byteLength(wire));
    bytes.fill(0);
    return original.call(this,snapshot,options);
  };
  try{assert.deepEqual(decodeEvaluation(bytes),i);}finally{TextDecoder.prototype.decode=original;}
  assert.equal(observed,1);
});

test('SP01 decoder bounds primitive strings before any UTF8 encoding allocation',()=>{
  registeredBounds();
  const ascii='x'.repeat(65537),multibyte='\u0800'.repeat(21846),supplementary='\u{1f600}'.repeat(16385);
  assert.equal(multibyte.length,21846);
  assert.equal(Buffer.byteLength(multibyte),65538);
  assert.equal(supplementary.length,32770);
  assert.equal(Buffer.byteLength(supplementary),65540);
  const encode=TextEncoder.prototype.encode,encodeInto=TextEncoder.prototype.encodeInto;
  let conversions=0;
  TextEncoder.prototype.encode=function(value){
    if(value===ascii||value===multibyte||value===supplementary){conversions++;throw Error('unexpected input encode');}
    return encode.call(this,value);
  };
  TextEncoder.prototype.encodeInto=function(value,destination){
    if(value===ascii||value===multibyte||value===supplementary){conversions++;throw Error('unexpected input encodeInto');}
    return encodeInto.call(this,value,destination);
  };
  try{
    decoderReject(ascii,'INPUT_BOUNDS');
    decoderReject(multibyte,'INPUT_BOUNDS');
    decoderReject(supplementary,'INPUT_BOUNDS');
  }finally{
    TextEncoder.prototype.encode=encode;
    TextEncoder.prototype.encodeInto=encodeInto;
  }
  assert.equal(conversions,0);
});

test('SP01 decoder accepts 4096 hex signature characters and rejects 4098',()=>{
  const i=decoderFixture();i.authority.signature.bytes='ab'.repeat(2048);
  assert.deepEqual(decodeEvaluation(canonicalEncode(i)),i);
  i.authority.signature.bytes+='ab';
  assert.equal(i.authority.signature.bytes.length,4098);
  decoderReject(canonicalEncode(i),'INPUT_BOUNDS');
  for(const hex of ['f','zz','AB']){
    i.authority.signature.bytes=hex;
    decoderReject(canonicalEncode(i));
  }
});

test('SP01 decoder applies array length 128 and 129 to a schema-valid statement',()=>{
  const i=decoderFixture();i.authority.statement.permittedRecipients=Array(128).fill('party');
  assert.deepEqual(decodeEvaluation(canonicalEncode(i)),i);
  i.authority.statement.permittedRecipients.push('party');
  decoderReject(canonicalEncode(i),'INPUT_BOUNDS');
});

test('SP01 decoder admits exact 65536 input bytes and rejects the next byte',()=>{
  const accepted=decoderWireAtBytes(65536),rejected=decoderWireAtBytes(65537);
  for(const carrier of [accepted.wire,Buffer.from(accepted.wire),new TextEncoder().encode(accepted.wire)]){
    assert.deepEqual(decodeEvaluation(carrier),accepted.i);
  }
  for(const carrier of [rejected.wire,Buffer.from(rejected.wire),new TextEncoder().encode(rejected.wire)]){
    decoderReject(carrier,'INPUT_BOUNDS');
  }
});

test('SP01 decoder enforces depth, node, record-key and UTF8 text admission before schema',()=>{
  // The at-limit variants deliberately have one unknown outer member: their
  // expected INPUT_SCHEMA proves only generic encoding admission, not validity.
  for(const [depth,code] of [[16,'INPUT_SCHEMA'],[17,'INPUT_BOUNDS']]){
    const i=decoderFixture();let nested=true;
    for(let n=1;n<depth;n++)nested=[nested];
    i.extra=nested;decoderReject(canonicalEncode(i),code);
  }
  for(const [nodes,code] of [[8192,'INPUT_SCHEMA'],[8193,'INPUT_BOUNDS']]){
    const i=decoderFixture(),base=decoderNodeCount(i);
    i.extra=decoderNodeTree(nodes-base);
    assert.equal(decoderNodeCount(i),nodes);
    const wire=canonicalEncode(i);assert.ok(Buffer.byteLength(wire)<=65536);
    decoderReject(wire,code);
  }
  for(const [keys,code] of [[64,'INPUT_SCHEMA'],[65,'INPUT_BOUNDS']]){
    const i=decoderFixture();i.extra=Object.fromEntries(Array.from({length:keys},(_,n)=>['k'+n,true]));
    decoderReject(canonicalEncode(i),code);
  }
  for(const [count,code] of [[2048,'INPUT_SCHEMA'],[2049,'INPUT_BOUNDS']]){
    const i=decoderFixture();i.extra='é'.repeat(count);
    assert.equal(Buffer.byteLength(i.extra),count*2);
    decoderReject(canonicalEncode(i),code);
  }
});

test('SP01 decoder rejects noncanonical UTF8, JSON, schema and Unicode inputs',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  const duplicate=wire.slice(0,-1)+',"schemaVersion":"moriarty-evaluation/1"}';
  for(const carrier of [
    ' '+wire,wire+'\n',duplicate,'{', 'null', '42', '[]',
    new Uint8Array([0xc3,0x28]),new Uint8Array([0xef,0xbb,0xbf,...Buffer.from(wire)]),
    wire.replace('"nonce0"','"\ud800"'),wire.replace('"nonce0"','"\\ud800"')
  ])decoderReject(carrier);
  const wrong=structuredClone(i);wrong.extra=true;decoderReject(canonicalEncode(wrong));
  const profile=structuredClone(i);profile.program.profile='moriarty-bounded-atomic/2';decoderReject(canonicalEncode(profile));
  const unicode=structuredClone(i);unicode.authority.statement.nonce='é'.repeat(128);
  assert.deepEqual(decodeEvaluation(canonicalEncode(unicode)),unicode);
  unicode.authority.statement.nonce+='é';decoderReject(canonicalEncode(unicode));
  unicode.authority.statement.nonce='\u{1f600}'.repeat(64);
  assert.deepEqual(decodeEvaluation(canonicalEncode(unicode)),unicode);
  unicode.authority.statement.nonce+='\u{1f600}';decoderReject(canonicalEncode(unicode));
});

```
CONSTRAINTS: Only this test edit and concise reports. Do not touch other files or run tests. Do not weaken expected results. VERIFICATION: Parent will execute the focused ^SP01 decoder tests against unchanged runtime, inspect actual failure causes, and then freeze tests before source edits. Finish by writing FOREMAN_REPORT.md and .json with changed files, no tests run, and any deviations. You have at most 590 seconds. Return a concise summary after writes.
