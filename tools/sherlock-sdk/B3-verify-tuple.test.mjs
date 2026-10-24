import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {captureVerifyTuple,verifyAndPublishTuple,BOUNDS,midnightVecLen} from './B3-verify-tuple.mjs';

// Pure DI tests: in-memory files, fake run/publish. No SDK, native code or fixtures.
const h=b=>createHash('sha256').update(b).digest('hex');
const hex=n=>n.toString(16).padStart(2,'0').repeat(32);
const OK='verify with ledger 2^14 verifier params, stock MidnightVK::read: true in 12.345s\n';
const FALSE='verify with ledger 2^14 verifier params, stock MidnightVK::read: false in 12.345s\n';
const BUNDLE='/own/bundle01',DOUT='/w/decider-out',WRAPPED='/w/wrapped.proof',PARENT=hex(9);
const P={bin:'/r/bin/straw-recl9',vk:'/r/keys/decider.vk',rel:'/r/keys/decider.relation',tagged:'/r/tagged.vk',template:'/r/template.json',tagging:'/r/tagging.json',provenance:'/r/provenance.json',capture:'/r/capture.json',proof:DOUT+'/batch-0.proof',pub:DOUT+'/batch-0.public.json',stats:DOUT+'/batch-0.stats.json',wrapped:WRAPPED};
const ORIG=Object.values(P);
const PRODUCTS={'batch-0.proof':P.proof,'batch-0.public.json':P.pub,'batch-0.stats.json':P.stats,'wrapped-decider.proof':P.wrapped,'tagged-decider.vk':P.tagged,'tagging.json':P.tagging,'provenance.json':P.provenance};

function world(len=150,prefix=[0x59,0x02]){
  const raw=Buffer.from('raw-decider-vk-bytes'),proof=Buffer.alloc(len,7);
  const context={schema:'genuine-sherlock-pi-before-proof/1',public_inputs:[hex(1),hex(2),hex(3)],binding_input:hex(1),communication_commitment:hex(2),stmt:hex(4),stmt_slot:10,instance_id:hex(5),batch_no:hex(0),w_instance:[hex(6),hex(7)],verifier_sha256:null,submission_intent:{schema:'actual-sherlock-call-intent/2'}};
  const {schema,verifier_sha256,submission_intent,...vector}=context;
  const F={[P.bin]:Buffer.from('ELF-straw-recl9'),[P.vk]:raw,[P.rel]:Buffer.from('decider-relation'),[P.tagged]:Buffer.concat([Buffer.from('TAG:'),raw]),
    [P.template]:Buffer.from('template'),[P.tagging]:Buffer.from('tagging'),[P.provenance]:Buffer.from('provenance'),[P.capture]:Buffer.from('capture'),
    [P.proof]:proof,[P.pub]:Buffer.from(JSON.stringify({...vector,layout:'L',encoding:'E'})),
    [P.stats]:Buffer.from(JSON.stringify({k:20,proof_sha256:h(proof),vk_sha256:h(raw),w_vk_sha256:PARENT,negative_checks:{}})),
    [P.wrapped]:Buffer.concat([Buffer.from('midnight:proof[v5]:'),Buffer.from(prefix),proof])};
  context.verifier_sha256=h(F[P.tagged]);
  const config={kind:'sherlock',bin:P.bin,key_dir:'/r/keys',verifier:P.tagged,template:P.template,provenance:P.provenance,tagging:P.tagging,capture:P.capture,w_dir:'/w/{batch}',
    bin_sha256:h(F[P.bin]),handkey_sha256:hex(8),verifier_sha256:h(F[P.tagged]),template_sha256:h(F[P.template]),provenance_sha256:h(F[P.provenance]),tagging_sha256:h(F[P.tagging]),capture_sha256:h(F[P.capture]),parent_vk_sha256:PARENT,
    key_dir_sha256:{'decider.vk':h(raw),'decider.relation':h(F[P.rel])}};
  const fs=new Map(ORIG.map(p=>[p,Buffer.from(F[p])]));
  const w={F,fs,config,context,stdout:OK,during:null,runThrow:false,publishFail:false,runs:[],publishes:[]};
  w.io={
    read:p=>{const b=fs.get(p);if(!b)throw Error(`ENOENT ${p}`);return b;}, // same buffer, no bound: helper must copy and bound
    writeNew:(p,b)=>{if(fs.has(p))throw Error(`EEXIST ${p}`);fs.set(p,Buffer.from(b));},
    run:async(file,args,opts)=>{
      w.runs.push({file,args,opts,seen:['decider.vk','decider.relation'].map(n=>Buffer.from(fs.get(`${args[1]}/${n}`))).concat(['batch-0.proof','batch-0.public.json'].map(n=>Buffer.from(fs.get(`${args[2]}/${n}`))))});
      if(w.during)w.during();if(w.runThrow)throw Error('spawn failed');return {stdout:w.stdout,stderr:''};},
    publish:async(products,receipt)=>{w.publishes.push({products,receipt});if(w.publishFail)throw Error('disk full');}};
  return w;
}
const capture=w=>captureVerifyTuple({config:w.config,dout:DOUT,wrappedProofPath:WRAPPED,frozenPublic:w.context,read:w.io.read});
const verify=(w,t,over={})=>verifyAndPublishTuple(t,{config:w.config,frozenPublic:w.context,bundleRoot:BUNDLE,runOptions:{timeout:5000},...w.io,...over});
const flip=(w,p)=>{const b=Buffer.from(w.fs.get(p));b[0]^=1;w.fs.set(p,b);};
const none=w=>{assert.equal(w.runs.length,0);assert.equal(w.publishes.length,0);};

test('one native verify on the captured bundle and one publish of exactly the captured bytes',async()=>{
  const w=world(),cfg0=JSON.stringify(w.config),ctx0=JSON.stringify(w.context);
  const t=await capture(w),r=await verify(w,t);
  assert.equal(w.runs.length,1);assert.equal(w.publishes.length,1);
  const [c]=w.runs;
  assert.equal(c.file,P.bin);assert.deepEqual(c.args,['verify',`${BUNDLE}/key`,`${BUNDLE}/out`,'0']);
  assert.equal(c.opts.maxBuffer,BOUNDS.stdout);assert.equal(c.opts.timeout,5000);
  [P.vk,P.rel,P.proof,P.pub].forEach((p,i)=>assert.ok(c.seen[i].equals(w.F[p]),p));
  const {products,receipt}=w.publishes[0];
  assert.deepEqual(Object.keys(products).sort(),Object.keys(PRODUCTS).sort());
  for(const [n,p] of Object.entries(PRODUCTS))assert.ok(products[n].equals(w.F[p]),n);
  assert.equal(receipt.stdout,OK);assert.equal(r.status,'CAPTURED_TUPLE_NATIVE_VERIFIED_AND_PUBLISHED');
  for(const [n,p] of [['bin',P.bin],['decider.relation',P.rel],['template',P.template],['capture',P.capture],['batch-0.proof',P.proof]])assert.equal(r.digests[n].sha256,h(w.F[p]),n);
  assert.equal(JSON.stringify(w.config),cfg0);assert.equal(JSON.stringify(w.context),ctx0);assert.equal(Object.isFrozen(w.config),false);
});

for(const path of ORIG)test(`drift of ${path} after capture refuses before verify`,async()=>{
  const w=world(),t=await capture(w);flip(w,path);
  await assert.rejects(verify(w,t),/drift before verify/);none(w);
});

for(const path of [...ORIG,`${BUNDLE}/key/decider.vk`,`${BUNDLE}/key/decider.relation`,`${BUNDLE}/out/batch-0.proof`,`${BUNDLE}/out/batch-0.public.json`])
  test(`drift of ${path} during awaited verify discards a true verdict`,async()=>{
    const w=world(),t=await capture(w);w.during=()=>flip(w,path);
    await assert.rejects(verify(w,t),/drift after verify/);
    assert.equal(w.runs.length,1);assert.equal(w.publishes.length,0);
  });

for(const [k,v] of [['bin','/r/bin/other'],['key_dir','/r/keys2'],['verifier','/r/tagged2.vk'],['template','/r/t2'],['tagging','/r/g2'],['provenance','/r/p2'],['capture','/r/c2'],['template_sha256',hex(10)],['parent_vk_sha256',hex(11)],['key_dir_sha256',{'decider.vk':hex(12),'decider.relation':hex(13)}]])
  test(`config ${k} drift refuses before verify even with identical target bytes`,async()=>{
    const w=world(),t=await capture(w);
    if(w.F[w.config[k]])w.fs.set(v,Buffer.from(w.F[w.config[k]]));
    if(k==='key_dir')for(const n of ['decider.vk','decider.relation'])w.fs.set(`${v}/${n}`,Buffer.from(w.F[`/r/keys/${n}`]));
    await assert.rejects(verify(w,t,{config:{...structuredClone(w.config),[k]:v}}),/config drift/);none(w);
  });

test('complete public context drift refuses before verify',async()=>{
  const w=world(),t=await capture(w);w.context.public_inputs[0]=hex(14);
  await assert.rejects(verify(w,t),/public context drift/);none(w);
});

for(const [label,out] of [['false',FALSE],['trailing line',OK+'extra\n'],['missing newline',OK.trim()],['empty',''],['absent',undefined]])
  test(`native verify stdout ${label} cannot become success`,async()=>{
    const w=world(),t=await capture(w);w.stdout=out;
    await assert.rejects(verify(w,t),/did not print true/);
    assert.equal(w.runs.length,1);assert.equal(w.publishes.length,0);
  });

test('verifier throw cannot become success',async()=>{
  const w=world(),t=await capture(w);w.runThrow=true;
  await assert.rejects(verify(w,t),/spawn failed/);assert.equal(w.publishes.length,0);
});

test('publish failure is not success and the tuple stays spent with its first failure',async()=>{
  const w=world(),t=await capture(w);w.publishFail=true;
  await assert.rejects(verify(w,t),/disk full/);w.publishFail=false;
  await assert.rejects(verify(w,t),/already consumed; first failure: disk full/);
  assert.equal(w.runs.length,1);assert.equal(w.publishes.length,1);
});

test('a successful tuple cannot be verified or published again',async()=>{
  const w=world(),t=await capture(w);await verify(w,t);
  await assert.rejects(verify(w,t),/already consumed/);
  assert.equal(w.runs.length,1);assert.equal(w.publishes.length,1);
});

test('first drift failure survives restore and the tuple keeps original digests',async()=>{
  const w=world(),t=await capture(w);flip(w,P.proof);
  await assert.rejects(verify(w,t),/batch-0.proof drift before verify/);
  w.fs.set(P.proof,Buffer.from(w.F[P.proof]));
  await assert.rejects(verify(w,t),/first failure: batch-0.proof drift before verify/);
  assert.equal(t.digests['batch-0.proof'].sha256,h(w.F[P.proof]));none(w);
});

test('a pre-existing bundle member refuses before verify',async()=>{
  const w=world(),t=await capture(w);w.fs.set(`${BUNDLE}/out/batch-0.proof`,Buffer.from(w.F[P.proof]));
  await assert.rejects(verify(w,t),/EEXIST/);none(w);
});

test('mutating caller buffers, config and context after capture cannot change the snapshot',async()=>{
  const w=world(),cfg0=structuredClone(w.config),ctx0=structuredClone(w.context),t=await capture(w);
  for(const p of ORIG){w.fs.get(p).fill(0);w.fs.set(p,Buffer.from(w.F[p]));}
  w.config.bin='/evil';w.config.key_dir_sha256['decider.vk']=hex(15);w.context.stmt_slot=99;
  await verify(w,t,{config:cfg0,frozenPublic:ctx0});
  [P.vk,P.rel,P.proof,P.pub].forEach((p,i)=>assert.ok(w.runs[0].seen[i].equals(w.F[p]),p));
  for(const [n,p] of Object.entries(PRODUCTS))assert.ok(w.publishes[0].products[n].equals(w.F[p]),n);
});

for(const [why,edit,re] of [
  ['wrapped proof is not the raw proof',w=>flip(w,P.wrapped),/wrapped proof/],
  ['stats name another proof',w=>w.fs.set(P.stats,Buffer.from(JSON.stringify({k:20,proof_sha256:hex(13),vk_sha256:h(w.F[P.vk]),w_vk_sha256:PARENT,negative_checks:{}}))),/stats/],
  ['final public differs from frozen context',w=>{w.context.stmt_slot=11;},/public stmt_slot/],
  ['pinned tagged verifier differs',w=>flip(w,P.tagged),/tagged-verifier pin/],
  ['relative config path',w=>{w.config.template='rel/template.json';},/config template path/],
  ['oversize proof',w=>w.fs.set(P.proof,Buffer.alloc(BOUNDS.proof+1)),/batch-0.proof read shape or bound/]])
  test(`capture refuses: ${why}`,async()=>{const w=world();edit(w);await assert.rejects(capture(w),re);none(w);});

// Worked out by hand from serialize/src/util.rs ScaleBigInt::serialize for u32 lengths; not computed by the helper.
const LEN_VECTORS=[[0,[0x00]],[1,[0x04]],[63,[0xfc]],[64,[0x01,0x01]],[150,[0x59,0x02]],[16383,[0xfd,0xff]],[16384,[0x02,0x00,0x01,0x00]],[1048576,[0x02,0x00,0x40,0x00]],[1073741823,[0xfe,0xff,0xff,0xff]],[1073741824,[0x03,0x00,0x00,0x00,0x40]],[4294967295,[0x03,0xff,0xff,0xff,0xff]]];
for(const [n,bytes] of LEN_VECTORS)test(`Midnight Vec<u8> length ${n} encodes as the ScaleBigInt bytes`,()=>{assert.deepEqual([...midnightVecLen(n)],bytes);});
test('non-u32 lengths are refused',()=>{for(const n of [-1,1.5,2**32,NaN])assert.throws(()=>midnightVecLen(n),/u32/);});
for(const [n,prefix] of [[63,[0xfc]],[64,[0x01,0x01]],[16384,[0x02,0x00,0x01,0x00]]])
  test(`capture accepts the tagged Proof length prefix of a ${n}-byte proof`,async()=>{const w=world(n,prefix),t=await capture(w);assert.equal(t.digests['wrapped-proof'].bytes,19+prefix.length+n);none(w);});
for(const [why,prefix] of [['fixed LE32 length',[150,0,0,0]],['non-minimal four-byte compact length',[0x5a,0x02,0x00,0x00]],['wrong length',[0x5d,0x02]]])
  test(`capture refuses wrapped proof with ${why}`,async()=>{const w=world(150,prefix);await assert.rejects(capture(w),/wrapped proof is not the raw proof/);none(w);});
