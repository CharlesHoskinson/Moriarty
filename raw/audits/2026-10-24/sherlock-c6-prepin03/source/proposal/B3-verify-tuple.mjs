// B3 consumed native-verify tuple. When the genuine call callback completes, C
// captures every input the deferred native verify reads plus every product it
// later publishes. The ONE post-merge verify runs the pinned bin against an owned
// exclusive bundle written from those snapshot bytes, rechecks originals and the
// bundle before and after the awaited run, and publishes only the captured
// verified bytes. IO/run/publish are injected; nothing runs at import. Errors name
// members, never bytes. This binds verifier input identity only; B4 owns the
// provider/canonical-proof link.
import {createHash} from 'node:crypto';
import {join} from 'node:path';

export const BOUNDS=Object.freeze({bin:512<<20,key:4<<20,record:1<<20,capture:16<<20,template:1<<20,proof:1<<20,stdout:1<<20,members:12,bundle:4,runs:1,publishes:1});
// Exact positive line of recl9 verify_cmd; it exits 0 even when false.
export const VERIFY_OK=/^verify with ledger 2\^14 verifier params, stock MidnightVK::read: true in [0-9]+\.[0-9]{3}s\n$/;
const SHA=/^[0-9a-f]{64}$/;
const CFG_PATHS=['bin','key_dir','verifier','template','provenance','tagging','capture'];
const CFG_PINS=['bin_sha256','verifier_sha256','template_sha256','provenance_sha256','tagging_sha256','capture_sha256','parent_vk_sha256'];
const PUBLIC=['public_inputs','w_instance','instance_id','batch_no','binding_input','communication_commitment','stmt','stmt_slot'];
const WRAP=Buffer.from('midnight:proof[v5]:');
// tagged_serialize(&Proof(raw)) writes WRAP, then Vec<u8>: (len as u32) through
// midnight-serialize via_scale!(u32,4) -> ScaleBigInt (serialize/src/util.rs), which is
// the SCALE compact form, not a fixed LE32 word, then the raw bytes.
export function midnightVecLen(n){
  if(!Number.isInteger(n)||n<0||n>0xffffffff)throw Error('length is not a u32');
  if(n<1<<6)return Buffer.from([n<<2]);
  if(n<1<<14){const b=Buffer.alloc(2);b.writeUInt16LE(n<<2|1);return b;}
  if(n<2**30){const b=Buffer.alloc(4);b.writeUInt32LE(n*4+2);return b;}
  const b=Buffer.alloc(5);b[0]=3;b.writeUInt32LE(n,1);return b;
}
// [member, path(config, ctx), max bytes, config pin | null, published product | null]
const MEMBERS=[
  ['bin',c=>c.bin,BOUNDS.bin,c=>c.bin_sha256,null],
  ['decider.vk',c=>join(c.key_dir,'decider.vk'),BOUNDS.key,c=>c.key_dir_sha256['decider.vk'],null],
  ['decider.relation',c=>join(c.key_dir,'decider.relation'),BOUNDS.key,c=>c.key_dir_sha256['decider.relation'],null],
  ['tagged-verifier',c=>c.verifier,BOUNDS.key,c=>c.verifier_sha256,'tagged-decider.vk'],
  ['template',c=>c.template,BOUNDS.template,c=>c.template_sha256,null],
  ['tagging',c=>c.tagging,BOUNDS.record,c=>c.tagging_sha256,'tagging.json'],
  ['provenance',c=>c.provenance,BOUNDS.record,c=>c.provenance_sha256,'provenance.json'],
  ['capture',c=>c.capture,BOUNDS.capture,c=>c.capture_sha256,null],
  ['batch-0.proof',(c,x)=>join(x.dout,'batch-0.proof'),BOUNDS.proof,null,'batch-0.proof'],
  ['batch-0.public.json',(c,x)=>join(x.dout,'batch-0.public.json'),BOUNDS.proof,null,'batch-0.public.json'],
  ['batch-0.stats.json',(c,x)=>join(x.dout,'batch-0.stats.json'),BOUNDS.record,null,'batch-0.stats.json'],
  ['wrapped-proof',(c,x)=>x.wrappedProofPath,BOUNDS.proof,null,'wrapped-decider.proof']];
// recl9 verify_cmd reads key_dir/decider.vk, out/batch-0.proof, out/batch-0.public.json.
const BUNDLE=[['decider.vk','key'],['decider.relation','key'],['batch-0.proof','out'],['batch-0.public.json','out']];
const STATE=new WeakMap();
const sha=b=>createHash('sha256').update(b).digest('hex');
const absolute=p=>typeof p==='string'&&p.startsWith('/')&&!p.includes(String.fromCharCode(0))&&!p.split('/').some(s=>s==='.'||s==='..');
const deepFreeze=o=>{if(o&&typeof o==='object'){for(const v of Object.values(o))deepFreeze(v);Object.freeze(o);}return o;};

function pickConfig(config){
  if(!config||typeof config!=='object')throw Error('config required');
  const out={},kd=config.key_dir_sha256;
  for(const k of CFG_PATHS){if(!absolute(config[k]))throw Error(`config ${k} path`);out[k]=config[k];}
  for(const k of CFG_PINS){if(typeof config[k]!=='string'||!SHA.test(config[k]))throw Error(`config ${k} pin`);out[k]=config[k];}
  if(!kd||typeof kd!=='object'||Object.keys(kd).sort().join()!=='decider.relation,decider.vk'||!Object.values(kd).every(v=>typeof v==='string'&&SHA.test(v)))throw Error('config key_dir_sha256');
  out.key_dir_sha256={'decider.vk':kd['decider.vk'],'decider.relation':kd['decider.relation']};
  if(out.verifier_sha256===kd['decider.vk'])throw Error('config tagged verifier is the raw key');
  return deepFreeze(out);
}
async function readMember(read,name,path,max){
  const b=await read(path,max);
  if(!(b instanceof Uint8Array)||b.length>max)throw Error(`${name} read shape or bound`);
  return Buffer.from(b); // copy: later caller-buffer writes cannot reach the snapshot
}
function parseMember(name,bytes){try{return JSON.parse(bytes.toString('utf8'));}catch{throw Error(`${name} is not JSON`);}}
async function recheck(read,snap,phase){
  for(const [name,s] of Object.entries(snap)){
    const now=await readMember(read,name,s.path,s.max);
    if(now.length!==s.bytes.length||sha(now)!==s.sha256)throw Error(`${name} drift ${phase}`);
  }
}
const digestsOf=snap=>Object.freeze(Object.fromEntries(Object.entries(snap).map(([n,s])=>[n,Object.freeze({path:s.path,bytes:s.bytes.length,sha256:s.sha256})])));

export async function captureVerifyTuple({config,dout,wrappedProofPath,frozenPublic,read}){
  const cfg=pickConfig(config),ctx={dout,wrappedProofPath};
  if(!absolute(dout)||!absolute(wrappedProofPath))throw Error('dout or wrapped proof path');
  const context=deepFreeze(structuredClone(frozenPublic));
  if(!context||typeof context!=='object')throw Error('public context required');
  const snap={};
  for(const [name,pathOf,max,pinOf] of MEMBERS){
    const path=pathOf(cfg,ctx);if(!absolute(path))throw Error(`${name} path`);
    const bytes=await readMember(read,name,path,max),sha256=sha(bytes);
    if(pinOf&&sha256!==pinOf(cfg))throw Error(`${name} pin mismatch`);
    snap[name]=Object.freeze({path,max,bytes,sha256});
  }
  const proof=snap['batch-0.proof'].bytes,raw=snap['decider.vk'].bytes,tagged=snap['tagged-verifier'].bytes;
  if(!snap['wrapped-proof'].bytes.equals(Buffer.concat([WRAP,midnightVecLen(proof.length),proof])))throw Error('wrapped proof is not the raw proof');
  if(!(raw.length>0&&tagged.length>raw.length&&tagged.subarray(tagged.length-raw.length).equals(raw)))throw Error('tagged verifier does not carry raw key');
  const pub=parseMember('batch-0.public.json',snap['batch-0.public.json'].bytes),stats=parseMember('batch-0.stats.json',snap['batch-0.stats.json'].bytes);
  if(!pub||!Array.isArray(pub.public_inputs)||!pub.public_inputs.length||!pub.public_inputs.every(s=>typeof s==='string'&&SHA.test(s)))throw Error('public vector shape');
  for(const k of PUBLIC)if(!(k in pub)||!(k in context)||JSON.stringify(pub[k])!==JSON.stringify(context[k]))throw Error(`public ${k} differs from frozen context`);
  if(stats?.k!==20||stats.proof_sha256!==snap['batch-0.proof'].sha256||stats.vk_sha256!==snap['decider.vk'].sha256||stats.w_vk_sha256!==cfg.parent_vk_sha256)throw Error('stats do not name captured proof and key');
  const configJson=JSON.stringify(cfg),contextJson=JSON.stringify(context);
  const handle=Object.freeze({schema:'b3-consumed-native-verify-tuple/1',digests:digestsOf(snap),config_sha256:sha(configJson),public_context_sha256:sha(contextJson),bounds:BOUNDS});
  STATE.set(handle,{cfg,configJson,contextJson,snap,spent:false,failure:null});
  return handle;
}

export async function verifyAndPublishTuple(handle,{config,frozenPublic,bundleRoot,read,writeNew,run,publish,runOptions={}}){
  const st=STATE.get(handle);
  if(!st)throw Error('unknown verify tuple');
  if(st.spent)throw Error(`verify tuple already consumed${st.failure?`; first failure: ${st.failure}`:''}`);
  st.spent=true; // one use, including failed attempts
  try{
    if(JSON.stringify(pickConfig(config))!==st.configJson)throw Error('config drift');
    if(JSON.stringify(frozenPublic)!==st.contextJson)throw Error('public context drift');
    if(!absolute(bundleRoot))throw Error('bundle root path');
    await recheck(read,st.snap,'before verify');
    const dirs={key:join(bundleRoot,'key'),out:join(bundleRoot,'out')},bundle={};
    for(const [name,dir] of BUNDLE){
      const path=join(dirs[dir],name);
      await writeNew(path,Buffer.from(st.snap[name].bytes));
      bundle[`bundle/${dir}/${name}`]={...st.snap[name],path};
    }
    await recheck(read,bundle,'before verify');
    const r=await run(st.cfg.bin,['verify',dirs.key,dirs.out,'0'],{...runOptions,maxBuffer:BOUNDS.stdout});
    const stdout=typeof r?.stdout==='string'?r.stdout:'';
    if(!VERIFY_OK.test(stdout))throw Error('native verify did not print true');
    // A true verdict counts only if the consumed bundle and every original are unchanged.
    await recheck(read,bundle,'after verify');
    await recheck(read,st.snap,'after verify');
    const products={};
    for(const [name,,,,product] of MEMBERS)if(product)products[product]=Buffer.from(st.snap[name].bytes);
    const verified=Object.freeze({schema:'b3-verified-captured-tuple/1',status:'CAPTURED_TUPLE_NATIVE_VERIFIED',stdout,digests:handle.digests,bundle:Object.freeze(dirs),config_sha256:handle.config_sha256,public_context_sha256:handle.public_context_sha256,bounds:BOUNDS,native_verify_runs:1});
    await publish(Object.freeze(products),verified);
    return Object.freeze({...verified,status:'CAPTURED_TUPLE_NATIVE_VERIFIED_AND_PUBLISHED',publishes:1});
  }catch(e){st.failure??=String(e?.message??e);throw e;}
}
