// Known-public synthetic contract tests; expected results are explicit literals, never module output.
import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import * as C from './capture-public.mjs';
const h=c=>c.repeat(64),sha=b=>createHash('sha256').update(b).digest('hex'),SENT='PRIVATE-SENTINEL-preimage';
const P=BigInt('0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001');
const P6T='P6 NONCOMPOSABLE: under the hand-written Sherlock decider key acceptValidity is noncomposable. No contract may claim or rely on a call to it. Accepted relation: ops = the captured template constants with the stmt slot = the DeciderRelation stmt of a parent-verified W; the binding input is a public input; the communication commitment at slot 1 is constrained in circuit to Poseidon(comm_rand, stmt) for the compiled acceptValidity shape (one Native input, no outputs), where comm_rand is private witness only and is never a public input, template constant, logged value or record field. No equivalence to the compiled acceptValidity operation is claimed.';
const EFF='all nine Effects collections empty (native post-state and every partitioned transcript)';
const SRC_LEGEND='Whitelist from actual complete quiescent original MP2 capture via pinned full oracle; ordered public build/source/constructor metadata retained; private capture/preimage/manifest products stripped.';
const PUB_LEGEND='Built only from authenticated expected ctor/source/build/guards/template inputs and fixed constants; no raw capture, preimage, manifest, hash, path or extra is an input.';
const FILES9=['contract/index.js','zkir/acceptValidity.zkir','zkir/acceptValidity.bzkir','keys/acceptValidity.prover','keys/acceptValidity.verifier','zkir/settleValidity.zkir','zkir/settleValidity.bzkir','keys/settleValidity.prover','keys/settleValidity.verifier'];
const CK=['id','keys','a102_lo','a102_hi','a80_lo','a80_hi','prog','r0','h0','b0'];
const FILES=['EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json','batch-0.proof','batch-0.public.json','batch-0.stats.json','capture-metadata.json','provenance.json','safe-result.json','tagged-decider.vk','tagging.json','wrapped-decider.proof'];
const put=(v,keys,x)=>{let o=v;for(const k of keys.slice(0,-1))o=o[k];o[keys.at(-1)]=x;return v;};
const values=()=>Object.fromEntries(CK.map((k,i)=>[k,String(i+1)]));
const build=()=>Object.fromEntries(FILES9.map((f,i)=>[f,{bytes:100+i,sha256:h('456789abc'[i])}]));
const expected=()=>({schema:'sherlock-capture-public-expected/1',templateSha256:h('3'),ctor:{schema:'sherlock-anchor-ctor/1',values:values()},source:{bytes:11,sha256:h('1')},guards:{bytes:22,sha256:h('2')},build:build(),settleVerifierSha256:h('c'),taggedVerifierSha256:h('d')});
const source=(extras=true)=>{
  const s={schema:'sherlock-template-capture/1',route:'sherlock/acceptValidity',status:'CAPTURED_CROSS_CHECKED',template:{sha256:h('3'),provisional:false,n:15,stmt_index:10},ctor:{schema:'sherlock-anchor-ctor/1',values:values()},frozen:{source:{bytes:11,sha256:h('1')},build:build(),guards:{bytes:22,sha256:h('2')}},keys:{compiled_acceptValidity_verifier:{sha256:h('8')},settleValidity_verifier:{sha256:h('c')}},p6:P6T,captures:[{effects:EFF},{effects:EFF}],projection:SRC_LEGEND};
  if(extras){for(const k of [['template','path'],['ctor','values','private_rand'],['frozen','source','path'],['frozen','build',FILES9[0],'path'],['keys','compiled_acceptValidity_verifier','path']])put(s,k,SENT);put(s,['captures',0,'preimage_sha256'],h('e'));s.rawCapture={path:SENT,sha256:h('f')};}
  return s;
};
const PUBLIC=()=>({...source(false),schema:'sherlock-capture-public/1',projection:PUB_LEGEND});
const pub=v=>Buffer.from(JSON.stringify(v)+'\n'),src=v=>Buffer.from(JSON.stringify(v,null,2)+'\n');
const refuse=(code,fn)=>assert.throws(fn,e=>e instanceof Error&&e.message===code&&e.cause===undefined&&C.CODES.includes(code)&&!String(e.stack).includes(SENT));
const refusePublish=(code,candidate,exp=expected())=>{const calls=[];refuse(code,()=>C.publishValidated(candidate,exp,b=>calls.push(b)));assert.equal(calls.length,0);};
const PRIVATE=[[['rawCapturePath'],'SHAPE'],[['frozen','manifest'],'SHAPE'],[['template','path'],'TEMPLATE'],[['ctor','values','private_rand'],'CTOR'],[['frozen','source','path'],'SOURCE'],[['frozen','build',FILES9[0],'path'],'BUILD'],[['frozen','guards','path'],'GUARDS'],[['keys','compiled_acceptValidity_verifier','path'],'KEYS'],[['captures',0,'preimage_sha256'],'EFFECTS']];
const MISMATCH=[[['schema'],'x','SCHEMA'],[['projection'],'x','SCHEMA'],[['template','stmt_index'],11,'TEMPLATE'],[['template','n'],true,'TEMPLATE'],[['template','provisional'],true,'TEMPLATE'],[['ctor','values','id'],'999','CTOR'],[['frozen','source','sha256'],h('0'),'SOURCE'],[['frozen','build',FILES9[2],'bytes'],1,'BUILD'],[['frozen','guards','bytes'],23,'GUARDS'],[['keys','settleValidity_verifier','sha256'],h('b'),'KEYS'],[['p6'],P6T.slice(0,17),'P6'],[['captures',1,'effects'],'x','EFFECTS']];
const BAD_EXPECTED=[[['extra'],1],[['ctor','values','id'],'01'],[['ctor','values','id'],P.toString()],[['ctor','values','id'],'-1'],[['settleVerifierSha256'],h('b')],[['taggedVerifierSha256'],h('8')],[['source','bytes'],0],[['source','bytes'],true],[['build',FILES9[0],'bytes'],(256<<20)+1],[['templateSha256'],h('A')]];
test('derive drops private projection extras and equals the independent public literal',()=>{
  const r=C.derivePublic(src(source()),expected()),want=pub(PUBLIC());
  assert.equal(r.schema,'sherlock-capture-public/1');assert.ok(r.canonical.equals(want));assert.equal(r.sha256,sha(want));
  for(const s of [SENT,h('e'),h('f'),'private_rand'])assert.ok(!r.canonical.includes(s));
});
test('publish validates before one exact sink write and identity roundtrips',()=>{
  const want=pub(PUBLIC()),calls=[],id=C.publishValidated(want,expected(),b=>calls.push(b));
  assert.deepEqual(id,{schema:'sherlock-capture-public/1',bytes:want.length,sha256:sha(want)});assert.equal(calls.length,1);assert.ok(calls[0].equals(want));
  assert.deepEqual(C.publicIdentity(expected()),id);assert.deepEqual(C.validatePublic(C.derivePublic(src(source()),expected()).canonical,expected()),id);
});
test('old raw-publication bug class: private projection bytes are refused as PUBLIC',()=>{
  refusePublish('ERR_CPUB_SCHEMA',pub(source(false)));refusePublish('ERR_CPUB_SCHEMA',src(source(false)));refusePublish('ERR_CPUB_SHAPE',pub(source()));
});
test('PUBLIC candidate top-level and nested private keys rejected before the sink',()=>{
  for(const [k,c] of PRIVATE)refusePublish('ERR_CPUB_'+c,pub(put(PUBLIC(),k,SENT)));
});
test('ctor/source/build/guards/keys/P6/template/effects mismatches have fixed codes',()=>{
  for(const [k,x,c] of MISMATCH){refusePublish('ERR_CPUB_'+c,pub(put(PUBLIC(),k,x)));refuse('ERR_CPUB_'+c,()=>C.derivePublic(src(put(source(),k,x)),expected()));}
  refuse('ERR_CPUB_EFFECTS',()=>C.derivePublic(src(put(source(),['captures'],[{effects:EFF},{effects:EFF},{effects:EFF}])),expected()));
  refuse('ERR_CPUB_BUILD',()=>C.derivePublic(src(put(source(),['frozen','build',FILES9[8]],null)),expected()));
});
test('expected parameters are a closed authenticated schema with canonical bounded values',()=>{
  for(const [k,x] of BAD_EXPECTED)refusePublish('ERR_CPUB_EXPECTED',pub(PUBLIC()),put(expected(),k,x));
  const e=expected();e.build=Object.fromEntries(Object.entries(e.build).reverse());refusePublish('ERR_CPUB_EXPECTED',pub(PUBLIC()),e);
  refuse('ERR_CPUB_EXPECTED',()=>C.publicIdentity(new Proxy({},{ownKeys(){throw Error(SENT);}})));
  const top=(P-1n).toString();assert.equal(C.validatePublic(pub(put(PUBLIC(),['ctor','values','b0'],top)),put(expected(),['ctor','values','b0'],top)).schema,'sherlock-capture-public/1');
});
test('JSON duplicate, nonfinite, noncanonical and bounded inputs are refused',()=>{
  const want=pub(PUBLIC()),t=want.toString();
  for(const [b,c] of [[Buffer.from(t.slice(0,-2)+',"p6":'+JSON.stringify(P6T)+'}\n'),'NONCANONICAL'],[Buffer.from(t.replace('"n":15','"n":NaN')),'JSON'],[Buffer.from(t.replace('"n":15','"n":1e999')),'TEMPLATE'],[Buffer.from(t.replace('"n":15','"n":15.0')),'NONCANONICAL'],[src(PUBLIC()),'NONCANONICAL'],[want.subarray(0,-1),'NONCANONICAL'],[Buffer.from(t.replace('NONCOMPOSABLE','NONCOMPOSABL\u00c9')),'JSON'],[Buffer.alloc((1<<16)+1,32),'JSON'],[Buffer.alloc(0),'JSON'],[t,'JSON']])refusePublish('ERR_CPUB_'+c,b);
  refuse('ERR_CPUB_SOURCE_FORM',()=>C.derivePublic(pub(source()),expected()));
  const s=src(source()).toString();refuse('ERR_CPUB_SOURCE_FORM',()=>C.derivePublic(Buffer.from(s.slice(0,-3)+',\n  "p6": "x"\n}\n'),expected()));
});
test('exact ten-artifact interface with uppercase before-proof filename',()=>{
  assert.deepEqual([...C.PUBLIC_FILES],FILES);assert.equal(C.assertPublicMembership([...FILES].reverse()),true);
  for(const bad of [FILES.map(n=>n===FILES[0]?'expected-before-proof.json':n),[...FILES,'capture.json'],FILES.slice(1),[...FILES.slice(1),FILES[1]],null])refuse('ERR_CPUB_MEMBERSHIP',()=>C.assertPublicMembership(bad));
});
test('shared Node/Python golden length and sha256 for normal and P-1 b0 fixtures',()=>{
  const top='52435875175126190479447740508185965837690552500527637822603658699938581184512';assert.equal((P-1n).toString(),top);
  for(const [b0,bytes,digest] of [['10',2974,'48b10f20716b13b64235793fbb33dcc1013a0e3e2b6662db474f6c29b9035791'],[top,3049,'6bf33a4bf374b221b525b7ac6c1f17a52cf042c94e03564acccb409588909cd6']]){
    const e=put(expected(),['ctor','values','b0'],b0),r=C.derivePublic(src(put(source(),['ctor','values','b0'],b0)),e),fixture=pub(put(PUBLIC(),['ctor','values','b0'],b0));
    assert.deepEqual([r.canonical.length,sha(r.canonical),r.sha256],[bytes,digest,digest]);
    assert.deepEqual(C.publicIdentity(e),{schema:'sherlock-capture-public/1',bytes,sha256:digest});
    assert.deepEqual([fixture.length,sha(fixture)],[bytes,digest]);
  }
});
test('publish snapshots the candidate once and the sink copy is isolated from candidate mutation',()=>{
  const want=pub(PUBLIC()),mem=new Uint8Array(new SharedArrayBuffer(want.length+8)),cand=mem.subarray(4,4+want.length);cand.set(want);
  const calls=[],id=C.publishValidated(cand,expected(),b=>{calls.push(b);cand.fill(32);});
  assert.deepEqual(id,{schema:'sherlock-capture-public/1',bytes:want.length,sha256:sha(want)});
  assert.equal(calls.length,1);assert.ok(Buffer.isBuffer(calls[0]));assert.notEqual(calls[0],cand);assert.notEqual(calls[0].buffer,mem.buffer);assert.ok(calls[0].equals(want));
  mem.fill(0);assert.ok(calls[0].equals(want));refusePublish('ERR_CPUB_JSON',cand);
  const plain=Buffer.from(want),seen=[];C.publishValidated(plain,expected(),b=>seen.push(b));
  assert.equal(seen.length,1);assert.notEqual(seen[0],plain);plain.fill(32);assert.ok(seen[0].equals(want));
});
test('embedded constants equal independent literal copies',()=>{
  assert.deepEqual([C.P6,C.EFFECTS_EMPTY,C.SOURCE_LEGEND,C.PUBLIC_LEGEND,[...C.BUILD_FILES],[...C.CTOR_KEYS]],[P6T,EFF,SRC_LEGEND,PUB_LEGEND,FILES9,CK]);
});
