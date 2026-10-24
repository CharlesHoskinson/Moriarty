// Closed PUBLIC capture type sherlock-capture-public/1. Pure: no fs, SDK, wallet, network or native.
// The record is built ONLY from authenticated expected inputs and fixed constants; a private
// projection is compared to them and never copied. Byte-identical twin: capture_public.py.
import {createHash} from 'node:crypto';
export const PUBLIC_SCHEMA='sherlock-capture-public/1',EXPECTED_SCHEMA='sherlock-capture-public-expected/1',SOURCE_SCHEMA='sherlock-template-capture/1',CTOR_SCHEMA='sherlock-anchor-ctor/1';
export const PUBLIC_LIMIT=1<<16,SOURCE_LIMIT=1<<20,SMALL_FILE=16<<20,BUILD_FILE=256<<20;
export const BUILD_FILES=Object.freeze(['contract/index.js','zkir/acceptValidity.zkir','zkir/acceptValidity.bzkir','keys/acceptValidity.prover','keys/acceptValidity.verifier','zkir/settleValidity.zkir','zkir/settleValidity.bzkir','keys/settleValidity.prover','keys/settleValidity.verifier']);
export const CTOR_KEYS=Object.freeze(['id','keys','a102_lo','a102_hi','a80_lo','a80_hi','prog','r0','h0','b0']);
// Copies of sherlock-guards.mjs P6/EFFECTS_EMPTY and the pinned projector legend; the driver asserts equality at load.
export const P6='P6 NONCOMPOSABLE: under the hand-written Sherlock decider key acceptValidity is noncomposable. No contract may claim or rely on a call to it. Accepted relation: ops = the captured template constants with the stmt slot = the DeciderRelation stmt of a parent-verified W; the binding input is a public input; the communication commitment at slot 1 is constrained in circuit to Poseidon(comm_rand, stmt) for the compiled acceptValidity shape (one Native input, no outputs), where comm_rand is private witness only and is never a public input, template constant, logged value or record field. No equivalence to the compiled acceptValidity operation is claimed.';
export const EFFECTS_EMPTY='all nine Effects collections empty (native post-state and every partitioned transcript)';
export const SOURCE_LEGEND='Whitelist from actual complete quiescent original MP2 capture via pinned full oracle; ordered public build/source/constructor metadata retained; private capture/preimage/manifest products stripped.';
export const PUBLIC_LEGEND='Built only from authenticated expected ctor/source/build/guards/template inputs and fixed constants; no raw capture, preimage, manifest, hash, path or extra is an input.';
export const PUBLIC_FILES=Object.freeze(['EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json','batch-0.proof','batch-0.public.json','batch-0.stats.json','capture-metadata.json','provenance.json','safe-result.json','tagged-decider.vk','tagging.json','wrapped-decider.proof']);
export const CODES=Object.freeze(['EXPECTED','JSON','SHAPE','SCHEMA','TEMPLATE','CTOR','SOURCE','BUILD','GUARDS','KEYS','P6','EFFECTS','NONCANONICAL','SOURCE_FORM','MEMBERSHIP','UNCLASSIFIED'].map(c=>'ERR_CPUB_'+c));
const P=BigInt('0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001');
const HEX=/^[0-9a-f]{64}$/,DEC=/^(0|[1-9][0-9]{0,77})$/,TEXT=/^[\n -~]*$/,ID=['bytes','sha256'],ACCEPT='keys/acceptValidity.verifier',SETTLE='keys/settleValidity.verifier';
const EXPECTED_KEYS=['schema','templateSha256','ctor','source','guards','build','settleVerifierSha256','taggedVerifierSha256'];
const SECTIONS=[[['schema'],'SCHEMA'],[['route'],'SCHEMA'],[['status'],'SCHEMA'],[['projection'],'SCHEMA'],[['template'],'TEMPLATE'],[['ctor'],'CTOR'],[['frozen','source'],'SOURCE'],[['frozen','build'],'BUILD'],[['frozen','guards'],'GUARDS'],[['keys'],'KEYS'],[['p6'],'P6'],[['captures'],'EFFECTS']];
const sha=b=>createHash('sha256').update(b).digest('hex');
const req=(ok,c)=>{if(!ok)throw Error('ERR_CPUB_'+c);};
const isObj=v=>v!==null&&typeof v==='object'&&!Array.isArray(v);
const exact=(v,keys)=>isObj(v)&&JSON.stringify(Object.keys(v))===JSON.stringify(keys);
const same=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
const pick=(o,keys)=>isObj(o)?Object.fromEntries(keys.filter(k=>Object.hasOwn(o,k)).map(k=>[k,o[k]])):null;
const at=(o,path)=>path.reduce((x,k)=>isObj(x)&&Object.hasOwn(x,k)?x[k]:null,o);
const hex=v=>typeof v==='string'&&HEX.test(v),dec=v=>typeof v==='string'&&DEC.test(v)&&BigInt(v)<P;
const ident=(v,max)=>exact(v,ID)&&Number.isSafeInteger(v.bytes)&&v.bytes>0&&v.bytes<=max&&hex(v.sha256);
// Every refusal is a fresh Error(fixed code) without cause; foreign throws collapse to UNCLASSIFIED.
const fixed=fn=>(...a)=>{try{return fn(...a);}catch(e){let m=null;try{m=e instanceof Error?e.message:null;}catch{}throw Error(CODES.includes(m)?m:'ERR_CPUB_UNCLASSIFIED');}};
function parse(b,limit,code){
  req(b instanceof Uint8Array&&b.length>0&&b.length<=limit,code);
  let text=null,v;
  try{text=new TextDecoder('utf-8',{fatal:true}).decode(b);v=JSON.parse(text);}catch{text=null;}
  req(text!==null&&TEXT.test(text),code);
  return {text,v};
}
// Detached snapshot of the independently authenticated expected parameters, never the validated object.
function expectedOf(input){
  let e;try{e=JSON.parse(JSON.stringify(input));}catch{}
  req(exact(e,EXPECTED_KEYS)&&e.schema===EXPECTED_SCHEMA&&[e.templateSha256,e.settleVerifierSha256,e.taggedVerifierSha256].every(hex)&&exact(e.ctor,['schema','values'])&&e.ctor.schema===CTOR_SCHEMA&&exact(e.ctor.values,CTOR_KEYS)&&CTOR_KEYS.every(k=>dec(e.ctor.values[k]))&&ident(e.source,SMALL_FILE)&&ident(e.guards,SMALL_FILE)&&exact(e.build,BUILD_FILES)&&BUILD_FILES.every(f=>ident(e.build[f],BUILD_FILE)),'EXPECTED');
  req(e.build[SETTLE].sha256===e.settleVerifierSha256&&new Set([e.build[ACCEPT].sha256,e.build[SETTLE].sha256,e.taggedVerifierSha256]).size===3,'EXPECTED');
  return e;
}
function build(input){
  const e=expectedOf(input),id=r=>({bytes:r.bytes,sha256:r.sha256}),eff={effects:EFFECTS_EMPTY};
  return {schema:PUBLIC_SCHEMA,route:'sherlock/acceptValidity',status:'CAPTURED_CROSS_CHECKED',template:{sha256:e.templateSha256,provisional:false,n:15,stmt_index:10},ctor:{schema:CTOR_SCHEMA,values:pick(e.ctor.values,CTOR_KEYS)},frozen:{source:id(e.source),build:Object.fromEntries(BUILD_FILES.map(f=>[f,id(e.build[f])])),guards:id(e.guards)},keys:{compiled_acceptValidity_verifier:{sha256:e.build[ACCEPT].sha256},settleValidity_verifier:{sha256:e.build[SETTLE].sha256}},p6:P6,captures:[{...eff},{...eff}],projection:PUBLIC_LEGEND};
}
const canonical=r=>Buffer.from(JSON.stringify(r)+'\n');
const identity=b=>Object.freeze({schema:PUBLIC_SCHEMA,bytes:b.length,sha256:sha(b)});
export const buildPublic=fixed(build);
export const publicIdentity=fixed(input=>identity(canonical(build(input))));
// Private projection (pinned projector json.dump(indent=2)+newline) -> public bytes. Extras are dropped
// because nothing is copied: every whitelisted source field must equal the expected-built value.
export const derivePublic=fixed((projection,input)=>{
  const pub=build(input),{text,v:s}=parse(projection,SOURCE_LIMIT,'SOURCE_FORM');
  req(isObj(s)&&JSON.stringify(s,null,2)+'\n'===text,'SOURCE_FORM');
  req(s.schema===SOURCE_SCHEMA&&s.route===pub.route&&s.status===pub.status&&s.projection===SOURCE_LEGEND,'SCHEMA');
  req(same(pick(s.template,Object.keys(pub.template)),pub.template),'TEMPLATE');
  req(isObj(s.ctor)&&s.ctor.schema===CTOR_SCHEMA&&same(pick(s.ctor.values,CTOR_KEYS),pub.ctor.values),'CTOR');
  const f=isObj(s.frozen)?s.frozen:{},b=isObj(f.build)?f.build:{},k=isObj(s.keys)?s.keys:{};
  req(same(pick(f.source,ID),pub.frozen.source),'SOURCE');
  req(BUILD_FILES.every(n=>same(pick(b[n],ID),pub.frozen.build[n])),'BUILD');
  req(same(pick(f.guards,ID),pub.frozen.guards),'GUARDS');
  req(Object.keys(pub.keys).every(n=>same(pick(k[n],['sha256']),pub.keys[n])),'KEYS');
  req(s.p6===P6,'P6');
  req(Array.isArray(s.captures)&&s.captures.length===2&&s.captures.every(c=>isObj(c)&&c.effects===EFFECTS_EMPTY),'EFFECTS');
  const out=canonical(pub);return Object.freeze({schema:PUBLIC_SCHEMA,canonical:out,sha256:sha(out)});
});
// PUBLIC candidate: closed type, per-section fixed codes, then exact canonical bytes. Never drops anything.
function validate(candidate,input){
  const pub=build(input),want=canonical(pub),{v}=parse(candidate,PUBLIC_LIMIT,'JSON');
  req(exact(v,Object.keys(pub))&&exact(v.frozen,Object.keys(pub.frozen)),'SHAPE');
  for(const [path,code] of SECTIONS)req(same(at(v,path),at(pub,path)),code);
  req(Buffer.compare(Buffer.from(candidate),want)===0,'NONCANONICAL');
  return identity(want);
}
export const validatePublic=fixed(validate);
// The sink (the wrapper's exclusive writeNew) runs only after validation. The candidate is snapshotted
// once; only the snapshot is validated and the sink receives a distinct copy of it, so later candidate
// mutation (e.g. a SharedArrayBuffer view) cannot reach the sink. Non-Uint8Array candidates pass through
// and an uncopyable one becomes null, so parse refuses both with the unchanged JSON code.
const snapshot=c=>{try{return c instanceof Uint8Array?Buffer.from(c):c;}catch{return null;}};
export function publishValidated(candidate,input,sink){
  const b=snapshot(candidate),id=validatePublic(b,input);
  if(typeof sink!=='function')throw Error('ERR_CPUB_UNCLASSIFIED');
  sink(Buffer.from(b));return id;
}
export const assertPublicMembership=fixed(names=>{req(Array.isArray(names)&&names.every(n=>typeof n==='string')&&same([...names].sort(),PUBLIC_FILES),'MEMBERSHIP');return true;});
