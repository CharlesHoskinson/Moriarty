// Explicit new call-only prove phase; original Source46 receipts stay held.
// No deployment prediction, local demo call, wallet construction or native launch at import.
import {join,dirname} from 'node:path';
import {execFile} from 'node:child_process';
import {promisify} from 'node:util';
import {createHash} from 'node:crypto';
import {openSync,fstatSync,readSync,writeSync,fsyncSync,closeSync,lstatSync,mkdirSync,readdirSync,realpathSync,constants as FS} from 'node:fs';
import {sealedCallFingerprint,assertSealedCallPreserved,isCallContext,normalizeContractAddress} from './sealed-call-policy.mjs';
import {ProducerCleanupError,withOwnedResources} from '../integration/owned-resources.mjs';
import {BOUNDS,VERIFY_OK,captureVerifyTuple,verifyAndPublishTuple} from './B3-verify-tuple.mjs';
import {makeCallProviderLink,LIMITATION,TRANSPORT_LIMITATION} from './B4-provider-link.mjs';
import {guardPreproofPublic} from './preproof-public-guard.mjs';
import {P6,PUBLIC_LIMIT,publicIdentity,validatePublic,publishValidated} from '../integration/capture-public.mjs';
const run=promisify(execFile),SHA=/^[0-9a-f]{64}$/,MIB=1<<20,NL=String.fromCharCode(10);
const SCOPE='Genuine call intent1 proved and sealed before disjoint DUST intent2; same raw native proof verified after merge and provider/ledger proof bytes linked; adapter vector only; real ledger validation remains required';
const SAFE_SCHEMA='genuine-bound-d-sealed-call-proof-safe/4',SAFE_STATUS='ACTUAL_GENUINE_SEALED_CALL_PRESERVED_AFTER_DUST_AND_NATIVE_VERIFY';
// Published product -> B3 member (keys sorted). safe-result.json is always written last.
const PRODUCTS={'batch-0.proof':'batch-0.proof','batch-0.public.json':'batch-0.public.json','batch-0.stats.json':'batch-0.stats.json','provenance.json':'provenance','tagged-decider.vk':'tagged-verifier','tagging.json':'tagging','wrapped-decider.proof':'wrapped-proof'};
// The before-proof PI record carries the frozen PROOF-EXPECTATIONS policy name; capture-metadata.json carries only the cleared sherlock-capture-public/1 canonical bytes.
const BEFORE_FILE='EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json';
const FILES=[BEFORE_FILE,...Object.keys(PRODUCTS),'capture-metadata.json','safe-result.json'].sort().join();
const CONTEXT=['schema','network','address','entry_point','deployment_record_sha256'],MODEL=['transaction_cost_model_sha256','parameters_sha256','parameters_block'];
// Native child env is fixed and minimal: nothing inherited, no LD_*.
const ENV=Object.freeze({PATH:'/usr/bin:/bin',LC_ALL:'C'});
// Exact product shapes ported from the Python inspector (J6) so they gate before the first FINAL write.
const CALL_KEY='sherlock/acceptValidity',ACCEPT='keys/acceptValidity.verifier',OFFICIAL20='1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1';
const FP=BigInt('0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001');
const fe=h=>typeof h==='string'&&SHA.test(h)&&BigInt('0x'+Buffer.from(h,'hex').reverse().toString('hex'))<FP;
const keysOf=o=>o&&typeof o==='object'&&!Array.isArray(o)?Object.keys(o).sort().join():null;
const STATS=['kind','k','keygen_pk_s','prove_s','verify_s','total_s','proof_bytes','vk_bytes','peak_rss_kb_after_prove','peak_rss_kb','vk_sha256','proof_sha256','w_vk_sha256','transcript','verify_params','srs','threads','negative_checks'].sort().join();
const TAGGING=['schema','raw_sha256','raw_bytes','tagged_sha256','tagged_bytes','checks','within_pinned_ledger_vk_limit','reader'].sort().join(),TAG_CHECKS=['raw_processed_roundtrip','tagged_suffix_is_raw','tagged_decode_reencode_equal','ledger_reader_init_reencode_equal'].sort().join();
const PUBLIC_KEYS=['public_inputs','binding_input','communication_commitment','stmt','stmt_slot','instance_id','batch_no','w_instance','layout','encoding'].sort().join();
const LINK_KEYS=['binding_le','callback_sha256','communication_commitment','stmt','instance_id','batch_no','verifier_sha256','verifier_key_sha256','deployment_record_sha256','transport_sha256','stored_proof_sha256','raw_proof_sha256','schema','key_location','address','entry_point','verifier_key_tag','stored_variant','transport_form','linked','limitation','transport_limitation','requires'].sort().join();
const SLOT={0:['binding_input','BindingInput'],1:['communication_commitment','CommunicationCommitment'],10:['stmt','Stmt']};
// Exact sherlock-provenance.rs inspect record (sherlock-decider-provenance/2): 22 members, none a path or a capture, config or preimage digest.
const PROV_KEYS=['schema','decider_vk_raw_sha256','decider_vk_raw_bytes','decider_vk_k','decider_relation_sha256','decider_relation_bytes','template_sha256','template_slots','stmt_index','parent_vk_sha256','w_vk_sha256','dom_stmt','stmt_fields','n_w_instance','nr_cols','comm_binding','comm_opening_constrained','s_g2','srs','checks','not_checked','p6'].sort().join(),PROV_CHECKS=['decider_vk_processed_roundtrip','decider_relation_roundtrip','template_equal','parent_vk_equal','comm_binding_bound_poseidon_stmt_v1','shape','s_g2_official_srs'].sort().join();
const PROV_NAMES=[...PROV_KEYS.split(','),...PROV_CHECKS.split(','),'k','official_pin_match','sha256'];
const NOT_CHECKED='decider.vk is the key of decider.relation: needs re-keygen at the decider degree under the pinned SRS';
// Fixed B4 observeSealed obligations, equal to the Python inspector constants; the two limitation strings come from B4 itself.
const B4_LINKED='PROVIDER_RETURN_EQ_ROUTE_CAPTURE; TRANSPORT_BODY_EQ_STORED_BODY; STORED_VARIANT_FROM_RESOLVED_VERIFIER_KEY_TAG; INSTALLED_CODEC_ROUNDTRIPS_STORED; SEALED_AND_FINALIZED_GETTERS_EQ_STORED',B4_REQUIRES=Object.freeze(['B3 native verify of raw_proof_hex (raw form, distinct from transport and stored forms) with the C-held native vector','real ledger validateTransaction','on-chain validation']);
const BARE=Buffer.from('midnight:proof[v5]:'),VERSIONED=Buffer.from('midnight:proof-versioned:'),VARIANTS=new Map([['verifier-key[v6]',['V2',1]],['verifier-key[v7]',['V3',2]]]);
const sha=b=>createHash('sha256').update(b).digest('hex');
const json=v=>Buffer.from(JSON.stringify(v)+NL);
const le=n=>{if(typeof n!=='bigint'||n<0n||n>=1n<<256n)throw Error('field element');return Buffer.from(n.toString(16).padStart(64,'0'),'hex').reverse().toString('hex');};
const deep=o=>{if(o&&typeof o==='object'){Object.values(o).forEach(deep);Object.freeze(o);}return o;};
const canon=p=>typeof p==='string'&&p.startsWith('/')&&realpathSync(p)===p;
// Capped stable read: canonical path, no final symlink, regular file, identity/size/times unchanged across the read.
function readStable(path,max) {
  if(!canon(path))throw Error('read path');
  const fd=openSync(path,FS.O_RDONLY|FS.O_NOFOLLOW);
  try {
    const a=fstatSync(fd,{bigint:true});
    if(!a.isFile()||a.size>BigInt(max))throw Error('read shape or bound');
    const b=Buffer.alloc(Number(a.size));
    for(let o=0;o<b.length;){const n=readSync(fd,b,o,b.length-o,o);if(!n)throw Error('read shrank');o+=n;}
    const z=fstatSync(fd,{bigint:true});
    if(readSync(fd,Buffer.alloc(1),0,1,b.length)||['dev','ino','size','mtimeNs','ctimeNs'].some(k=>z[k]!==a[k]))throw Error('read unstable');
    return b;
  } finally {closeSync(fd);}
}
const syncDir=d=>{const fd=openSync(d,FS.O_RDONLY|FS.O_DIRECTORY);try{fsyncSync(fd);}finally{closeSync(fd);}};
// Exclusive new file, never through a symlink; file then directory fsync before returning.
function writeNew(file,data,mode=0o444) {
  const b=Buffer.from(data),fd=openSync(file,FS.O_WRONLY|FS.O_CREAT|FS.O_EXCL|FS.O_NOFOLLOW,mode);
  try {for(let o=0;o<b.length;)o+=writeSync(fd,b,o,b.length-o);fsyncSync(fd);} finally {closeSync(fd);}
  syncDir(dirname(file));
}
function ownedDir(p,{fresh=false,empty=false}={}) {
  if(fresh){mkdirSync(p,{mode:0o700});syncDir(dirname(p));}
  const s=lstatSync(p);
  if(!canon(p)||!s.isDirectory()||s.uid!==process.getuid()||(s.mode&0o777)!==0o700||(empty&&readdirSync(p).length))throw Error('owned directory');
  return p;
}
// Frozen PROOF-EXPECTATIONS.BEFORE.json of this call phase; with it every input below has a byte pin.
const EXPECT_SHA='6937b585876e9f96df3f0401ae7bdd1fe4a1b173a73e6dcdcc09fed2328b1e0d';
// Input identity: each pinned input is hashed once, capped and streamed, against its pin under one fstat identity;
// each checkpoint requires the same canonical path and dev/ino/mode/uid/size/mtime/ctime. A write, truncate, chmod,
// chown or rename onto or of the file moves ctime or ino. This detects, it does not prevent: path consumers between
// checkpoints trust that no writer of these paths or their directories, host side included, swaps them transiently.
const ID=['dev','ino','mode','uid','size','mtimeNs','ctimeNs'],idOf=s=>ID.map(k=>String(s[k])).join(':');
function pinHash(path,max) {
  if(!canon(path))throw Error('read path');
  const fd=openSync(path,FS.O_RDONLY|FS.O_NOFOLLOW),h=createHash('sha256'),b=Buffer.alloc(MIB);
  try {
    const a=fstatSync(fd,{bigint:true}),n=Number(a.size);
    if(!a.isFile()||a.size>BigInt(max))throw Error('read shape or bound');
    let o=0;
    while(o<n){const k=readSync(fd,b,0,Math.min(MIB,n-o),o);if(!k)throw Error('read shrank');h.update(b.subarray(0,k));o+=k;}
    const z=fstatSync(fd,{bigint:true});
    if(readSync(fd,b,0,1,o)||ID.some(k=>z[k]!==a[k]))throw Error('read unstable');
    return {sha256:h.digest('hex'),id:idOf(a)};
  } finally {closeSync(fd);}
}
const INTENT=['schema','network','address','action','physical_calls','pre_state_sha256','deployment_record_sha256','instance_id','batch_no','tagged_verifier_sha256','template_sha256','prove_after_final_balance','balancing_scope'];
// Constructor refusals leave as fresh fixed-message Errors without cause; any other throw (Node path error, TypeError) is collapsed.
const CTOR_REFUSALS=new Set(['actual bound-D route required','bound-D wrapper options refused','actual submission intent required','submission intent differs from adopted call context','frozen actual inputs required','owned 0555 pinned bin copy required','frozen actual call differs','route material path record required','owned directory','read path','read shape or bound','read shrank','read unstable','field element','capture public expected required']);
export function wrapSealedCallBoundDRoute(route,options) {
  try {return wrapRoute(route,options);} catch(e) {let m=null;try{m=e instanceof Error&&CTOR_REFUSALS.has(e.message)?e.message:null;}catch{}throw Error(m??'bound-D wrapper construction refused');}
}
function wrapRoute(liveRoute,{config:liveConfig,publicOutputRoot,submissionIntent,proofExpectations,callContext,ledger,deadline,onFailure,runtimePorts:livePorts}={}) {
  // Detached snapshots, taken first: the route's own enumerable members (material copied; zkir and the rest spread through as
  // makeProvider and B4 consume them), exactly the five used ports, and the JSON config. Nothing below rereads the caller's objects.
  let route,ports,config;
  try {
    const m=liveRoute?.material;
    route=Object.freeze({...liveRoute,material:m&&typeof m==='object'?Object.freeze({...m}):m});
    const {callKey,handkeyBin,parseTemplate,checkCallPis,checkProvenance}=livePorts;
    ports=Object.freeze({callKey,handkeyBin,parseTemplate,checkCallPis,checkProvenance});
    config=deep(structuredClone(liveConfig));
  } catch {throw Error('bound-D wrapper options refused');}
  // Constructor guards: shapes and the adopted A context before any read, hash or launch.
  if(!route||typeof route.prove!=='function'||route.batchNo!==0n||typeof route.stmt!=='bigint'||typeof route.instanceId!=='bigint')throw Error('actual bound-D route required');
  if(typeof onFailure!=='function'||!Number.isFinite(deadline)||Date.now()>=deadline||typeof ledger?.Proof?.deserialize!=='function'||typeof ledger.ContractCall!=='function'||ports?.callKey!==CALL_KEY||['parseTemplate','checkCallPis','checkProvenance'].some(k=>typeof ports[k]!=='function')||!config||typeof config!=='object'||!SHA.test(config.verifier_sha256)||!SHA.test(config.handkey_sha256)||!fe(config.expected_dom_stmt)||typeof config.srs20_bound_path!=='string'||!config.srs20_bound_path.startsWith('/')||!/^[ -~]+$/.test(config.srs20_bound_path)||!isCallContext(callContext))throw Error('bound-D wrapper options refused');
  const cc=Object.freeze(Object.fromEntries(CONTEXT.map(k=>[k,callContext[k]]))),intent=deep(structuredClone(submissionIntent));
  if(!intent||Object.keys(intent).length!==INTENT.length||INTENT.some(k=>!(k in intent))||intent.schema!=='actual-sherlock-call-intent/2'||intent.network!=='stagenet'||typeof intent.address!=='string'||!intent.address||intent.action!=='sherlock/acceptValidity'||intent.physical_calls!==1||!SHA.test(intent.pre_state_sha256)||!SHA.test(intent.deployment_record_sha256)||intent.instance_id!==le(route.instanceId)||intent.batch_no!==le(route.batchNo)||intent.tagged_verifier_sha256!==config.verifier_sha256||intent.template_sha256!==config.template_sha256||intent.prove_after_final_balance!==false||JSON.stringify(intent.balancing_scope)!=='["dust"]')throw Error('actual submission intent required');
  if(intent.address!==cc.address||normalizeContractAddress(intent.address)!==cc.address||cc.network!==intent.network||cc.deployment_record_sha256!==intent.deployment_record_sha256||cc.entry_point!=='acceptValidity')throw Error('submission intent differs from adopted call context');
  // Capture-public expected: take one detached JSON snapshot; validate its closed shape with the module's own expectedOf (exact
  // key order, ASCII hex/decimal strings and ints only, so compact JSON+NL equals Python canonical_bytes); check its canonical
  // SHA against the opaque rc pin and its template/tagged/settle roles against config; only then freeze it. The caller's object
  // is never frozen or reread; the guard, the resolver check and publication use only this snapshot.
  let xp=null,pubId=null;
  try{xp=JSON.parse(JSON.stringify(config.capture_public_expected));pubId=publicIdentity(xp);}catch{xp=pubId=null;}
  if(!pubId||!SHA.test(config.capture_public_expected_sha256)||sha(json(xp))!==config.capture_public_expected_sha256||xp.templateSha256!==config.template_sha256||xp.templateSha256!==intent.template_sha256||xp.taggedVerifierSha256!==config.verifier_sha256||!SHA.test(config.settle_verifier_sha256)||xp.settleVerifierSha256!==config.settle_verifier_sha256||pubId.sha256===config.capture_sha256||new Set([xp.build[ACCEPT].sha256,xp.settleVerifierSha256,config.verifier_sha256,config.key_dir_sha256?.['decider.vk']]).size!==4)throw Error('capture public expected required');
  deep(xp);
  const inputs=[config.bin,join(config.key_dir,'decider.vk'),join(config.key_dir,'decider.relation'),config.verifier,config.template,config.tagging,config.provenance,config.capture,ports.handkeyBin,proofExpectations];
  const kd=config.key_dir_sha256&&typeof config.key_dir_sha256==='object'?config.key_dir_sha256:{},pins=[config.bin_sha256,kd['decider.vk'],kd['decider.relation'],config.verifier_sha256,config.template_sha256,config.tagging_sha256,config.provenance_sha256,config.capture_sha256,config.handkey_sha256,EXPECT_SHA];
  if(!inputs.every(canon)||!pins.every(h=>typeof h==='string'&&SHA.test(h))||config.bin_sha256!=='dc6e9ebedee878da4ccaf41b851c658d2fce480201a4c7681e2a505d2dae81a9')throw Error('frozen actual inputs required');
  const caps=[BOUNDS.bin,BOUNDS.key,BOUNDS.key,BOUNDS.key,BOUNDS.template,BOUNDS.record,BOUNDS.record,BOUNDS.capture,BOUNDS.bin,MIB];
  const ids=inputs.map((p,i)=>{const h=pinHash(p,caps[i]);if(h.sha256!==pins[i])throw Error('frozen actual inputs required');return h.id;});
  const assertInputs=()=>{if(inputs.some((p,i)=>!canon(p)||idOf(lstatSync(p,{bigint:true}))!==ids[i]))throw Error('input identity');};
  ownedDir(publicOutputRoot,{empty:true});
  // The handkey binary receives the private preimage, so it gets the same owned 0555 copy rule as the native bin.
  for(const p of [config.bin,ports.handkeyBin]){const bin=lstatSync(p);if(!bin.isFile()||bin.uid!==process.getuid()||(bin.mode&0o7777)!==0o555)throw Error('owned 0555 pinned bin copy required');}
  const eb=readStable(proofExpectations,MIB);
  if(sha(eb)!==EXPECT_SHA)throw Error('frozen actual call differs');
  const expect=JSON.parse(eb),w28=expect?.genuineParentW?.complete28;
  if(intent.instance_id!==expect.instance_id||!fe(expect.instance_id)||intent.batch_no!==expect.batch_no_le||expect.batch_no!=='0'||expect.status!=='FROZEN_BEFORE_GENUINE_CALL_AND_NATIVE_PROOF'||expect.n!==15||expect.stmt_index!==10||JSON.stringify(expect.freeSlots)!=='[0,1,10]'||config.template_sha256!==expect.templateSHA256||!SHA.test(config.parent_vk_sha256)||expect.genuineParentW?.vkSHA256!==config.parent_vk_sha256||!Array.isArray(w28)||w28.length!==28||!w28.every(fe))throw Error('frozen actual call differs');
  // route.material is the {prover,verifier,bzkir} path record makeProvider lookupKey reads, not bytes. The verifier
  // bytes B4 binds are resolved in composeProvider through the base provider's own lookupKey.
  const mat=route.material;
  if(!mat||typeof mat!=='object'||!['prover','verifier','bzkir'].every(k=>typeof mat[k]==='string'&&mat[k]))throw Error('route material path record required');
  const read=async(p,max=BOUNDS.capture)=>readStable(p,max);
  const s={invoked:false,merged:false,composed:false,published:false,first:null};
  let want=null,link=null,linked=null;
  // The first fixed failure is reported once, synchronously, before the SDK wraps the throw. The record carries fixed
  // codes and a boolean only; every throw is a fresh Error(code) with no cause. cleanup is computed in its own try (if a Proxy
  // trap throws, it reports true, conservatively), so onFailure still runs once. link_first is a one-shot snapshot; the driver
  // must re-read it through goGate/status.
  const late=new WeakSet();
  const fail=(code,e)=>{
    if(late.has(e))code='ERR_C1A_DEADLINE';
    if(!s.first){s.first=code;let lf=null;try{lf=link?.firstFailure();}catch{}lf=typeof lf==='string'&&/^ERR_B4_[A-Z0-9_]+$/.test(lf)?lf:null;let cl=true;try{const lc=link?link.cleanupFailure():null;cl=e instanceof ProducerCleanupError||/_FREE$/.test(lf??'')||lc==='ERR_B4_CODEC_FREE'||lc==='ERR_B4_SEALED_PROOF_FREE';}catch{}s.cl=cl;try{onFailure(Object.freeze({schema:'c1a-bound-d-failure/1',code,link_first:lf,cleanup:cl}));}catch{}}
    return Object.assign(Error(code),{code});
  };
  // Any record is terminal: every entry point, the callback return and publish refuse afterwards, so no success follows a failure.
  const terminal=()=>{if(s.first)throw fail('ERR_C1A_AFTER_FAILURE');};
  const left=()=>{const t=Math.floor(deadline-Date.now());if(!(t>0)){const e=Error('deadline');late.add(e);throw e;}return t;};
  // B4 go-gate: B4 records an async-entry refusal up to a microtask late, so after a full event-loop turn the link must still
  // be linked with no first failure and the wrapper must hold no record before native verify, publication or send.
  const goTick=()=>new Promise(r=>setImmediate(r));
  const gate=()=>{if(s.first||link?.status()!=='linked'||link.firstFailure()!==null||link.cleanupFailure()!==null)throw Error();};
  // Typed FINAL reread for the success return and goGate: capture-metadata.json on disk must validate as the snapshot's closed
  // public type with the construction identity, safe-result.json must be the bytes written, and membership must be exact.
  // The seven products must also rehash to the captured B3 digests, and the before-proof record to s.expSha.
  const finalTyped=()=>{const m=validatePublic(readStable(join(publicOutputRoot,'capture-metadata.json'),PUBLIC_LIMIT),xp),d=s.handle?.digests;if(m.bytes!==pubId.bytes||m.sha256!==pubId.sha256||!s.safe||!d||Object.entries(PRODUCTS).some(([p,n])=>{const b=readStable(join(publicOutputRoot,p),BOUNDS.key);return b.length!==d[n].bytes||sha(b)!==d[n].sha256;})||sha(readStable(join(publicOutputRoot,BEFORE_FILE),MIB))!==s.expSha||!readStable(join(publicOutputRoot,'safe-result.json'),MIB).equals(s.safe)||readdirSync(publicOutputRoot).sort().join()!==FILES)throw Error();};
  // Runs inside B4 route.prove, from the genuine call-only transaction.prove BEFORE DUST balancing.
  const callback=async args=>{
    terminal();
    if(s.invoked)throw fail('ERR_C1A_CALLBACK_REUSED');
    s.invoked=true;let stage='ERR_C1A_CALLBACK_ARGS';
    try {
      if(!SHA.test(args?.bindingLe)||typeof args.preimage!=='string')throw Error();
      ownedDir(args.dir);left();
      stage='ERR_C1A_INPUT_IDENTITY';assertInputs();
      stage='ERR_C1A_PROVENANCE';
      const prov=ports.checkProvenance(config);
      if(prov?.compiled_accept_verifier_sha256!==want.verifier_key_sha256||prov.tagged_verifier_sha256!==config.verifier_sha256)throw Error();
      // One stable owned copy of the SDK preimage feeds handkey pis and the route; B4 binds it to the SDK raw.
      stage='ERR_C1A_PREIMAGE';
      const work=ownedDir(join(args.dir,'c1a-owned'),{fresh:true}),pre=readStable(args.preimage,BOUNDS.capture),preimage=join(work,'preimage.bin');
      writeNew(preimage,pre,0o400);Object.assign(s,{work,preimage,preimageSha:sha(pre)});
      stage='ERR_C1A_HANDKEY_PIN';
      if(sha(readStable(ports.handkeyBin,BOUNDS.bin))!==config.handkey_sha256)throw Error();
      stage='ERR_C1A_TEMPLATE';
      const tb=readStable(config.template,BOUNDS.template);
      if(sha(tb)!==config.template_sha256)throw Error();
      const template=ports.parseTemplate(tb);
      if(template?.stmtIndex!==10)throw Error();
      // The pinned template bytes are also checked directly, not only through the port.
      const tj=JSON.parse(tb),slots=tj?.slots;
      if(!Array.isArray(slots)||slots.length!==15||tj.provisional!==false)throw Error();
      stage='ERR_C1A_PIS';
      assertInputs();const r=await run(ports.handkeyBin,['pis',preimage,args.bindingLe],{maxBuffer:MIB,env:ENV,timeout:left(),shell:false});
      const pis=JSON.parse(r.stdout).pis;
      if(!Array.isArray(pis)||!pis.every(h=>typeof h==='string'&&SHA.test(h))||pis[0]!==args.bindingLe)throw Error();
      ports.checkCallPis(template,pis,args.bindingLe,route.stmt);
      if(pis.length!==15||!pis.every(fe)||pis[10]!==want.stmt||slots.some((x,i)=>Object.hasOwn(SLOT,i)?JSON.stringify(x)!==JSON.stringify({kind:SLOT[i][0]}):keysOf(x)!=='kind,value'||x.kind!=='const'||!fe(x.value)||x.value!==pis[i]))throw Error();
      const labels=new Set([...slots.map((x,i)=>('slot'+i+':'+(Object.hasOwn(SLOT,i)?SLOT[i][1]:'Const(Fq(0x'+Buffer.from(x.value,'hex').reverse().toString('hex')+'))')).slice(0,60)),'short_vector','flipped_proof_byte','stmt_for_batch_no_plus_1']);
      // Pre-proof public guard: the 15-PI vector and the pinned source projection against the constructor's authenticated frozen snapshot of the
      // expected data, before bindCallback, the before-proof record and route.prove. Only the cleared public canonical bytes
      // are kept, privately, for later C; nothing new is published.
      stage='ERR_C1A_PUBLIC_GUARD';
      const pubSrc=readStable(inputs[7],BOUNDS.capture);
      if(sha(pubSrc)!==pins[7])throw Error();
      s.publicCapture=guardPreproofPublic({pis,bindingLe:args.bindingLe,projection:pubSrc,expected:xp});
      if(s.publicCapture.sha256!==pubId.sha256||s.publicCapture.canonical.length!==pubId.bytes||sha(s.publicCapture.canonical)!==pubId.sha256)throw Error();
      s.publicCaptureSourceSha=sha(pubSrc);
      const frozen=deep({schema:'genuine-sherlock-pi-before-proof/1',public_inputs:[...pis],binding_input:args.bindingLe,communication_commitment:pis[1],stmt:want.stmt,stmt_slot:10,instance_id:want.instance_id,batch_no:want.batch_no,w_instance:[...w28],verifier_sha256:config.verifier_sha256,submission_intent:intent});
      stage='ERR_C1A_BIND_CALLBACK';link.bindCallback(frozen);
      stage='ERR_C1A_EXPECTED_WRITE';
      ownedDir(publicOutputRoot,{empty:true});
      const expBytes=json(frozen);writeNew(join(publicOutputRoot,BEFORE_FILE),expBytes);
      stage='ERR_C1A_ROUTE_PROVE';left();
      const result=await route.prove({...args,preimage});
      if(result?.stats?.ok!==true||result.stats.verifies_adapter_vector_under_pinned_key!==true||!canon(result.proofFile))throw Error();
      left();
      stage='ERR_C1A_PREIMAGE_DRIFT';
      if(sha(readStable(preimage,BOUNDS.capture))!==s.preimageSha)throw Error();
      // Capture once after the genuine route proof and stats; every C semantic read below is bound to its digests.
      stage='ERR_C1A_CAPTURE';
      const handle=await captureVerifyTuple({config,dout:join(args.dir,'decider-out'),wrappedProofPath:result.proofFile,frozenPublic:frozen,read}),d=handle.digests;
      stage='ERR_C1A_BOUND_READ';
      const bound=n=>{const b=readStable(d[n].path,BOUNDS.key);if(b.length!==d[n].bytes||sha(b)!==d[n].sha256)throw Error();return b;};
      const proof=bound('batch-0.proof'),pubBytes=bound('batch-0.public.json'),stats=JSON.parse(bound('batch-0.stats.json')),wrapped=bound('wrapped-proof'),raw=bound('decider.vk'),tagged=bound('tagged-verifier'),tagging=JSON.parse(bound('tagging')),pub=JSON.parse(pubBytes),neg=stats.negative_checks,ptext=pubBytes.toString('latin1');
      stage='ERR_C1A_NATIVE_RECEIPT';
      if(ptext.split('"stmt_slot"').length!==2||!/"stmt_slot"[ ]*:[ ]*10[ ,}]/.test(ptext.replace(/[^ -~]/g,' '))||!Array.isArray(pub.w_instance)||!pub.w_instance.every(h=>SHA.test(h))||stats.k!==20||stats.proof_sha256!==d['batch-0.proof'].sha256||stats.vk_sha256!==d['decider.vk'].sha256||stats.w_vk_sha256!==config.parent_vk_sha256||proof.length<=100||tagged.length>50000||!neg||typeof neg!=='object'||Object.keys(neg).length!==18||!Object.values(neg).every(v=>v==='rejected'))throw Error();
      const srs=stats.srs,num=k=>typeof stats[k]==='number'&&Number.isFinite(stats[k])&&stats[k]>=0,pos=k=>Number.isSafeInteger(stats[k])&&stats[k]>0;
      if(keysOf(stats)!==STATS||stats.kind!=='recl9-decider-prove-call'||!['keygen_pk_s','prove_s','verify_s','total_s'].every(num)||!['proof_bytes','vk_bytes','peak_rss_kb_after_prove','peak_rss_kb'].every(pos)||stats.proof_bytes!==proof.length||stats.vk_bytes!==raw.length||stats.transcript!=='Blake2b (blake2b_simd::State)'||stats.verify_params!=="bls_midnight_2p14 verifier params (the ledger's embedded file)"||stats.threads!=='2'||keysOf(srs)!=='file,official_pin_match,sha256'||srs.sha256!==OFFICIAL20||srs.official_pin_match!==true||srs.file!==config.srs20_bound_path||labels.size!==18||!Object.keys(neg).every(k=>labels.has(k)))throw Error();
      if(keysOf(pub)!==PUBLIC_KEYS||pub.layout!=='[binding_input, communication_commitment, transcript field encodings] (ledger-9 ContractCall::public_inputs)'||pub.encoding!=='32-byte little-endian BLS12-381 scalar hex'||keysOf(tagging)!==TAGGING||tagging.schema!=='sherlock-vk-tagging/1'||tagging.raw_sha256!==d['decider.vk'].sha256||tagging.tagged_sha256!==config.verifier_sha256||sha(tagged)!==config.verifier_sha256||tagging.raw_bytes!==raw.length||tagging.tagged_bytes!==tagged.length||keysOf(tagging.checks)!==TAG_CHECKS||!Object.values(tagging.checks).every(v=>v===true)||tagging.within_pinned_ledger_vk_limit!==true||tagging.reader!=='midnight-transient-crypto =3.0.0 (midnight-ledger rev 9a8777c4), midnight-proofs =0.8.3')throw Error();
      // Provenance read the same pinned bytes B3 captured: both are checked against the same config pins.
      stage='ERR_C1A_PROVENANCE_BINDING';
      if(prov.capture_sha256!==d.capture.sha256||prov.template_sha256!==d.template.sha256||prov.provenance_sha256!==d.provenance.sha256||prov.tagging_sha256!==d.tagging.sha256||prov.decider_vk_raw_sha256!==d['decider.vk'].sha256||sha(raw)!==prov.decider_vk_raw_sha256||d.capture.sha256!==s.publicCaptureSourceSha)throw Error();
      // The published provenance.json itself, before any FINAL write: the exact 22-member producer record. JSON.parse keeps a later
      // duplicate, so with no escape in the pinned text every member name must occur exactly once, as Python unique() requires.
      const pvb=bound('provenance'),pvt=pvb.toString('latin1'),pv=JSON.parse(pvb),pvs=pv?.srs,pvc=pv?.checks,pn=(k,v)=>Number.isSafeInteger(pv[k])&&pv[k]===v;
      if(pvt.includes(String.fromCharCode(92))||PROV_NAMES.some(k=>pvt.split(JSON.stringify(k)).length!==2)||keysOf(pv)!==PROV_KEYS||pv.schema!=='sherlock-decider-provenance/2'||pv.decider_vk_raw_sha256!==d['decider.vk'].sha256||!pn('decider_vk_raw_bytes',raw.length)||pv.decider_relation_sha256!==d['decider.relation'].sha256||!pn('decider_relation_bytes',d['decider.relation'].bytes)||pv.template_sha256!==config.template_sha256||pv.parent_vk_sha256!==config.parent_vk_sha256||pv.w_vk_sha256!==config.parent_vk_sha256||!pn('n_w_instance',28)||!pn('nr_cols',4)||!pn('template_slots',15)||!pn('stmt_index',10)||!pn('decider_vk_k',20)||JSON.stringify(pv.stmt_fields)!==JSON.stringify([...Array(16).keys()])||pv.comm_binding!=='poseidon-rand-stmt-v1'||pv.comm_opening_constrained!==true||keysOf(pvs)!=='k,official_pin_match,sha256'||!Number.isSafeInteger(pvs.k)||pvs.k!==20||pvs.sha256!==OFFICIAL20||pvs.official_pin_match!==true||keysOf(pvc)!==PROV_CHECKS||!Object.values(pvc).every(v=>v===true)||typeof pv.s_g2!=='string'||!/^[0-9a-f]{192}$/.test(pv.s_g2)||pv.not_checked!==NOT_CHECKED||pv.p6!==P6||pv.dom_stmt!==config.expected_dom_stmt)throw Error();
      // Private cross-check only: the B3-captured raw config.capture bytes equal the pin the guard derived from. These bytes and
      // their identity are never published; capture-metadata.json is the cleared sherlock-capture-public/1 canonical bytes.
      const capture=readStable(d.capture.path,MIB);
      if(capture.length!==d.capture.bytes||sha(capture)!==d.capture.sha256||sha(capture)!==config.capture_sha256)throw Error();
      Object.assign(s,{frozen,handle,expSha:sha(expBytes),proof,wrapped,neg:deep(structuredClone(neg))});
      // Return the SAME genuine proof unless a record exists or the deadline passed; publication waits for post-merge link,
      // preservation and native verify.
      terminal();left();
      return result;
    } catch(e) {throw fail(stage,e);}
  };
  // Invoked once with A's exact {sealed,finalized,before,after,callContext,provingModel} after the DUST merge.
  const verifyAfterMerge=async({sealed,finalized,before,after,callContext:ctx,provingModel}={})=>{
    terminal();
    if(!s.handle||s.merged){s.merged=true;throw fail('ERR_C1A_POSTMERGE_ORDER');}
    s.merged=true;let stage='ERR_C1A_POSTMERGE_CONTEXT';
    try {
      left();
      const model=Object.freeze(Object.fromEntries(MODEL.map(k=>[k,provingModel?.[k]]))),block=model.parameters_block;
      if(ctx!==callContext||!isCallContext(ctx)||CONTEXT.some(k=>ctx[k]!==cc[k])||!sealed||!finalized||sealed===finalized||!SHA.test(model.transaction_cost_model_sha256)||!SHA.test(model.parameters_sha256)||!(Number.isSafeInteger(block)&&block>=0||typeof block==='string'&&/^(0|[1-9][0-9]*)$/.test(block)))throw Error();
      // B4 observes owned views of the one ContractCall in sealed and finalized before preservation, verify or publication.
      stage='ERR_C1A_PROVIDER_LINK';
      const receipt=withOwnedResources((own,ownCollection)=>{
        const callOf=(tx,segments)=>{
          const intents=ownCollection(tx.intents);let call;
          if(!(intents instanceof Map)||[...intents.keys()].sort().join()!==segments)throw Error();
          for(const [segment,intent] of intents){const actions=ownCollection(intent.actions);if(!Array.isArray(actions)||actions.length!==(segment===1?1:0))throw Error();if(segment===1)call=actions[0];}
          if(!(call instanceof ledger.ContractCall))throw Error();
          return call;
        };
        return link.observeSealed({sealedCall:callOf(sealed,'1'),finalizedCall:callOf(finalized,'1,2')});
      },{borrowed:[sealed,finalized]});
      stage='ERR_C1A_SEALED_PRESERVATION';
      assertSealedCallPreserved(before,sealedCallFingerprint(sealed,ledger,{complete:false}));
      const fin=sealedCallFingerprint(finalized,ledger,{complete:true});
      assertSealedCallPreserved(before,fin);
      for(const k of ['intent','proof','binding'])if(!(after?.[k] instanceof Uint8Array)||!(fin?.[k] instanceof Uint8Array)||Buffer.compare(after[k],fin[k]))throw Error();
      const sealedCall=Object.freeze({intent_sha256:sha(after.intent),proof_sha256:sha(after.proof),binding_sha256:sha(after.binding)});
      // Raw native proof, bare transport and stored versioned proof stay distinct and tie to the captured tuple.
      stage='ERR_C1A_TUPLE_LINK';
      const d=s.handle.digests;
      if(receipt?.schema!=='b4-call-provider-link/2'||receipt.raw_proof_sha256!==d['batch-0.proof'].sha256||!Buffer.from(receipt.raw_proof_hex,'hex').equals(s.proof)||receipt.transport_sha256!==d['wrapped-proof'].sha256||new Set([receipt.raw_proof_sha256,receipt.transport_sha256,receipt.stored_proof_sha256]).size!==3||receipt.preimage_sha256!==s.preimageSha||sha(readStable(s.preimage,BOUNDS.capture))!==s.preimageSha||receipt.callback_sha256!==sha(JSON.stringify(s.frozen))||receipt.key_location!==ports.callKey||Object.entries(want).some(([k,v])=>receipt[k]!==v)||['address','entry_point','deployment_record_sha256'].some(k=>receipt[k]!==cc[k]))throw Error();
      // The stored ProofVersioned form comes from the resolved verifier-key tag (Map lookup, no prototype keys) and is the sealed call's proof.
      const vt=VARIANTS.get(receipt.verifier_key_tag),stored=vt&&Buffer.concat([VERSIONED,Buffer.from([vt[1]]),s.wrapped.subarray(BARE.length,s.wrapped.length-s.proof.length),s.proof]);
      if(!vt||receipt.stored_variant!==vt[0]||receipt.transport_form!=='bare-proof[v5]'||!s.wrapped.subarray(0,BARE.length).equals(BARE)||sha(stored)!==receipt.stored_proof_sha256||sha(after.proof)!==receipt.stored_proof_sha256||receipt.key_location!==CALL_KEY)throw Error();
      stage='ERR_C1A_INPUT_IDENTITY';assertInputs();
      stage='ERR_C1A_LINK_GATE';await goTick();gate();
      stage='ERR_C1A_NATIVE_VERIFY';
      const bundleRoot=ownedDir(join(s.work,'bundle'),{fresh:true});
      for(const x of ['key','out'])ownedDir(join(bundleRoot,x),{fresh:true});
      let runs=0,safe=null;
      // The ONE native verify: promisified execFile of the pinned 0555 copy, shell false, utf8, fixed env, deadline.
      const native=async(file,argv,o)=>{if(runs++||file!==config.bin)throw Error('one native verify');return run(file,argv,{maxBuffer:o?.maxBuffer,env:ENV,timeout:left(),shell:false});};
      const publish=async(products,verified)=>{
        stage='ERR_C1A_PUBLISH';await goTick();gate();left();assertInputs();
        if(safe||runs!==1||verified?.schema!=='b3-verified-captured-tuple/1'||verified.status!=='CAPTURED_TUPLE_NATIVE_VERIFIED'||verified.native_verify_runs!==1||verified.digests!==d||!VERIFY_OK.test(verified.stdout)||Object.keys(products).sort().join()!==Object.keys(PRODUCTS).join()||Object.entries(PRODUCTS).some(([p,m])=>sha(products[p])!==d[m].sha256))throw Error();
        if(readdirSync(publicOutputRoot).join()!==BEFORE_FILE||sha(readStable(join(publicOutputRoot,BEFORE_FILE),MIB))!==s.expSha)throw Error();
        // Everything is validated and serialized before the first FINAL write: the cleared capture bytes against the snapshot,
        // the exact provider-link projection and the safe/4 record (capture_public identity; no config_sha256 or raw capture digest).
        const cid=validatePublic(s.publicCapture.canonical,xp);
        if(cid.bytes!==pubId.bytes||cid.sha256!==pubId.sha256)throw Error();
        const {raw_proof_hex,stored_proof_hex,preimage_sha256,...providerLink}=receipt,nativeTuple={schema:verified.schema,status:verified.status,stdout:verified.stdout,digests:Object.fromEntries(Object.entries(d).filter(([n])=>n!=='capture').map(([n,x])=>[n,{bytes:x.bytes,sha256:x.sha256}])),public_context_sha256:verified.public_context_sha256,bounds:verified.bounds,native_verify_runs:verified.native_verify_runs};
        safe=json({schema:SAFE_SCHEMA,status:SAFE_STATUS,public:s.frozen,proof:{bytes:s.proof.length,sha256:d['batch-0.proof'].sha256},wrapped_proof:{bytes:s.wrapped.length,sha256:d['wrapped-proof'].sha256},raw_decider_vk_sha256:d['decider.vk'].sha256,tagged_verifier_sha256:config.verifier_sha256,k:20,negative_checks:s.neg,private_opening_logged:false,independent_native_verify:true,production_adapter_verify:true,post_fee_merge_call_preserved:true,submission_intent:intent,scope:SCOPE,call_context:cc,proving_model:model,sealed_call:sealedCall,provider_link:providerLink,native_tuple:nativeTuple,capture_public:pubId});
        if(keysOf(providerLink)!==LINK_KEYS||providerLink.linked!==B4_LINKED||providerLink.limitation!==LIMITATION||providerLink.transport_limitation!==TRANSPORT_LIMITATION||JSON.stringify(providerLink.requires)!==JSON.stringify(B4_REQUIRES))throw Error();
        s.safe=safe;
        for(const p of Object.keys(PRODUCTS))writeNew(join(publicOutputRoot,p),products[p]);
        publishValidated(s.publicCapture.canonical,xp,b=>writeNew(join(publicOutputRoot,'capture-metadata.json'),b));
        writeNew(join(publicOutputRoot,'safe-result.json'),safe);
      };
      const final=await verifyAndPublishTuple(s.handle,{config,frozenPublic:s.frozen,bundleRoot,read,writeNew:async(p,b)=>writeNew(p,b),run:native,publish});
      stage='ERR_C1A_POST_VERIFY';
      if(final?.status!=='CAPTURED_TUPLE_NATIVE_VERIFIED_AND_PUBLISHED'||final.publishes!==1||final.native_verify_runs!==1||runs!==1||!safe||readdirSync(publicOutputRoot).sort().join()!==FILES||!readStable(join(publicOutputRoot,'safe-result.json'),MIB).equals(safe))throw Error();
      assertInputs();
      await goTick();gate();left();assertInputs();finalTyped();s.published=true;
      return Object.freeze({schema:SAFE_SCHEMA,status:SAFE_STATUS,safe_result_sha256:sha(safe)});
    } catch(e) {throw fail(stage,e);}
  };
  // Awaited once before transaction.prove. The call key's verifier bytes come from the base provider's own lookupKey and
  // must be the capture's compiled acceptValidity key, never the tagged D key (B4 verifier_sha256). Only then is the one
  // B4 link created; B4 re-hashes every later resolution against the same verifier_key_sha256.
  const composeProvider=async base=>{
    terminal();
    if(s.composed||s.invoked)throw fail('ERR_C1A_COMPOSE_ORDER');
    s.composed=true;let stage='ERR_C1A_COMPOSE_ARGS';
    try {
      if(!base||['prove','check','lookupKey'].some(k=>typeof base[k]!=='function'))throw Error();
      left();stage='ERR_C1A_INPUT_IDENTITY';assertInputs();
      stage='ERR_C1A_COMPOSE_LOOKUP';
      const vk=(await base.lookupKey(ports.callKey))?.verifierKey;
      if(!(vk instanceof Uint8Array)||!vk.length||vk.length>BOUNDS.key)throw Error();
      const vks=sha(Buffer.from(vk)),prov=ports.checkProvenance(config);
      if(vks===config.verifier_sha256||vk.length!==xp.build[ACCEPT].bytes||vks!==xp.build[ACCEPT].sha256||prov?.compiled_accept_verifier_sha256!==vks||prov.tagged_verifier_sha256!==config.verifier_sha256)throw Error();
      stage='ERR_C1A_COMPOSE_LINK';left();
      want=Object.freeze({stmt:le(route.stmt),instance_id:intent.instance_id,batch_no:intent.batch_no,verifier_sha256:config.verifier_sha256,verifier_key_sha256:vks});
      const l=makeCallProviderLink({callKey:ports.callKey,ledger,read,callContext,isCallContext,normalizeContractAddress,expect:want});
      if(['status','firstFailure','cleanupFailure'].some(k=>typeof l?.[k]!=='function'))throw Error();
      link=l;
      const p=link.compose(base),key=ports.callKey,guarded=(code,fn)=>(async()=>{terminal();return fn();})().catch(e=>{throw fail(code,e);});
      linked=link.route({...route,prove:callback});
      // Only the call key is caught to a fixed code, check included; DUST and other keys stay with B4/base unchanged.
      return Object.freeze({events:p.events,check:(raw,k)=>k===key?guarded('ERR_C1A_LINK_CHECK',()=>p.check(raw,k)):p.check(raw,k),lookupKey:k=>k===key?guarded('ERR_C1A_LINK_LOOKUP',()=>p.lookupKey(k)):p.lookupKey(k),prove:(raw,k,b)=>k===key?guarded('ERR_C1A_LINK_PROVIDER',()=>p.prove(raw,k,b)):p.prove(raw,k,b)});
    } catch(e) {throw fail(stage,e);}
  };
  // makeProvider keeps this as handRoutes[callKey] (material/zkir spread through) and calls prove later.
  const routeProve=async args=>{
    terminal();
    if(!linked)throw fail('ERR_C1A_ROUTE_BEFORE_COMPOSE');
    try {return await linked.prove(args);} catch(e) {throw fail('ERR_C1A_LINK_ROUTE',e);}
  };
  // Driver send gate, awaited immediately before any send: after a full event-loop turn the wrapper must have published, hold no
  // record, and B4 must still be linked with no first failure. The deadline, input identities and the typed reread of
  // capture-metadata.json/safe-result.json are enforced here too, so the deadline bounds send.
  const goGate=async()=>{await goTick();try{if(!s.published)throw Error();gate();left();assertInputs();finalTyped();}catch(e){throw fail('ERR_C1A_GO_GATE',e);}return true;};
  // Driver reread after any failure (onFailure is a one-shot snapshot): fixed codes and booleans only, never throws, never records.
  const failureStatus=()=>{let lf=null,lc=null,bad=false;try{lf=link?link.firstFailure():null;lc=link?link.cleanupFailure():null;}catch{bad=true;}lf=typeof lf==='string'&&/^ERR_B4_[A-Z0-9_]+$/.test(lf)?lf:null;if(lc!==null&&lc!=='ERR_B4_CODEC_FREE'&&lc!=='ERR_B4_SEALED_PROOF_FREE'){lc=null;bad=true;}return Object.freeze({schema:'c1a-bound-d-failure-status/1',code:s.first,link_first:lf,link_cleanup:lc,cleanup:bad||s.cl===true||lc!==null||/_FREE$/.test(lf??'')});};
  return Object.freeze({...route,prove:routeProve,composeProvider,verifyAfterMerge,goGate,failureStatus});
}

