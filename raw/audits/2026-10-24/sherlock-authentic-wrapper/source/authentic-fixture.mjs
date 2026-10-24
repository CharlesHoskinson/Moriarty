// SOURCE ONLY. A separately reviewed invocation may read authentic fixture operands.
// No SDK import, source-launched child, SRS read, wallet, proof production or proof replay.
// The selected node --test CLI starts one Node test child; effects are declared separately.
import {createHash} from 'node:crypto';
import {openSync,closeSync,fstatSync,readSync,realpathSync,constants as FS,mkdirSync,copyFileSync,chmodSync,rmSync,readdirSync} from 'node:fs';
import {join,dirname} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';

export const ROOT='/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-recursion-restart-20261022';
export const SUBJECT=ROOT+'/wallet/contract-call-fee-producer02/R3-C4-H-driver01';
export const CALL='sherlock/acceptValidity';
const C5=ROOT+'/native/source46-C1-C5-expected02';
const HELPER='/home/charl/projectfenrir-sherlock-public-observations-20261020/recursion/stagenet-strawman/chain/harness/sherlock-guards.mjs';
export const sha=b=>createHash('sha256').update(b).digest('hex');
const fromLe=h=>BigInt('0x'+Buffer.from(h,'hex').reverse().toString('hex'));
const le=n=>Buffer.from(n.toString(16).padStart(64,'0'),'hex').reverse().toString('hex');
const ID=['dev','ino','size','mode','uid','mtimeNs','ctimeNs'];

// C6_FIXTURE_* errors use fixed messages. Node and JSON.parse can emit their own diagnostics.
function readPinned(p,bytes,digest) {
  if(typeof p!=='string'||realpathSync(p)!==p)throw Error('C6_FIXTURE_PATH');
  const fd=openSync(p,FS.O_RDONLY|FS.O_NOFOLLOW);
  try {
    const a=fstatSync(fd,{bigint:true});
    if(!a.isFile()||a.size!==BigInt(bytes)||bytes>64*1024*1024)throw Error('C6_FIXTURE_SIZE');
    const out=Buffer.alloc(bytes);let o=0;
    while(o<bytes){const n=readSync(fd,out,o,bytes-o,o);if(!n)throw Error('C6_FIXTURE_SHORT');o+=n;}
    const z=fstatSync(fd,{bigint:true});
    if(readSync(fd,Buffer.alloc(1),0,1,bytes)||ID.some(k=>z[k]!==a[k])||sha(out)!==digest)throw Error('C6_FIXTURE_PIN');
    return out;
  } finally {closeSync(fd);}
}
const record=row=>JSON.parse(readPinned(row.path,row.bytes,row.sha256));
const publicRow=(path,bytes,sha256)=>({path,bytes,sha256});
const ordinaryPins=[
  ['proposal/bound-d-call-phase.mjs',41571,'845aee76bbeea3b0b83e8f440277812b49a57f9e1bdc4f48f7e7d3dec7428553'],
  ['proposal/sealed-call-policy.mjs',8455,'edd309f39f01d046c76c2df5c88ff9decb6be90a0e6b1655933755e4f374ba36'],
  ['integration/owned-resources.mjs',2093,'3e97c40a94629b027ce3a46ac27f7835476d313a210a043d5fe45c944c44e735'],
  ['integration/producer-policy.mjs',2188,'e0d436f77c33089a7385b434361badc6fd4e5d388d69cc8dbfda05be874b0ac8'],
  ['proposal/preproof-public-guard.mjs',2396,'a7f7e2db73b59c1f65162ffb6688777ef68f519dc056ff3a29314dfaa06916cf'],
  ['integration/capture-public.mjs',9271,'732da1b84ce86c5a33c2b9b7c0cae05963c39547c4c826a286b24f93a58925f8'],
  ['proposal/B3-verify-tuple.mjs',9659,'ad892d43bfe742499f7e27f94645a9623d8eb81aff9feff2788149535a8845a0'],
  ['proposal/B4-provider-link.mjs',15949,'14c7b754d392cc1b82fdc403aa26975538c6a19985308681c94919169cde7b4e'],
];

export async function loadSubject() {
  // Only public source and metadata are read here. Private fixture bodies are read below,
  // in explicit test construction/compose, after review and execution selection.
  for(const [p,n,h] of ordinaryPins)readPinned(join(SUBJECT,p),n,h);
  readPinned(HELPER,16625,'854de0ad4c6944999c16101837649c3c65fc0e797622712171d998d69c969e63');
  const wrapper=await import(pathToFileURL(join(SUBJECT,'proposal/bound-d-call-phase.mjs')));
  const policy=await import(pathToFileURL(join(SUBJECT,'proposal/sealed-call-policy.mjs')));
  const b4=await import(pathToFileURL(join(SUBJECT,'proposal/B4-provider-link.mjs')));
  const guards=await import(pathToFileURL(HELPER));
  return {wrapper,policy,b4,guards};
}

export async function loadOldB4Subject(root) {
  // Compatibility fixture: the exact current public wrapper and its exact ordinary
  // dependencies, paired with an authentic historical B4. No bytes are rewritten.
  // This isolated module graph tests the production COMPOSE_LINK version refusal;
  // it is never a substitute for a current-B4 positive or cleanup observation.
  const graph=join(root.dir,'old-b4-graph');mkdirSync(graph,{mode:0o700});
  for(const d of ['proposal','integration'])mkdirSync(join(graph,d),{mode:0o700});
  const old=ROOT+'/wallet/contract-call-fee-producer02/B4-materialized03/proposal/B4-provider-link.mjs';
  for(const [relative,n,h] of ordinaryPins){
    const isOld=relative==='proposal/B4-provider-link.mjs',source=isOld?old:join(SUBJECT,relative);
    const bytes=isOld?15148:n,digest=isOld?'086f1a2e77fb15b55487764ab2550ab26b649efcc716dce6580c1b2b433970b8':h;
    readPinned(source,bytes,digest);
    const destination=join(graph,relative);copyFileSync(source,destination,FS.COPYFILE_EXCL);chmodSync(destination,0o444);
    readPinned(destination,bytes,digest);
  }
  return {wrapper:await import(pathToFileURL(join(graph,'proposal/bound-d-call-phase.mjs'))),
    policy:await import(pathToFileURL(join(graph,'proposal/sealed-call-policy.mjs'))),
    b4:await import(pathToFileURL(join(graph,'proposal/B4-provider-link.mjs'))),guards:await import(pathToFileURL(HELPER))};
}

export function prepareFixtureRoot() {
  const control=record(publicRow(C5+'/EXACT-CONTROL-AND-OPERATIONS.HELD.json',251739,'a721e8752905c36904c63b9149ca30baaae7b5beda9388a12a97a065b1ebd79e'));
  const actual=record(publicRow(C5+'/ACTUAL-CALL-BINDING.json',4956,'77409f932c73d7ca183e3108fbfe5d2a813ab84f63feb34ef88030e1879293de'));
  const elf=record(publicRow(C5+'/ACTUAL-ELF-BINDING.json',1436,'340024f60a401bf138e8be99cadfba1c81562d0524bee9d58566a5a6fadcd75b'));
  if(actual.status!=='BOUND_ACTUAL_ADOPTED_STAGENET_DEPLOYMENT_AND_GENUINE_D_CALL_INPUTS'||elf.status!=='BOUND_ACTUAL_ADOPTED_SOURCE46_NORMAL_ELF')throw Error('C6_FIXTURE_BINDING');
  const expected=record(actual.capturePublicExpected),expect=record(control.proofExpectations);
  // Only authentic normal ELF bytes are copied. Copy destinations are private and exclusive;
  // the wrapper itself enforces real identity/mode/pins again. No binary is run.
  const dir=join(dirname(fileURLToPath(import.meta.url)),'execution01');
  mkdirSync(dir,{mode:0o700});
  const bins=join(dir,'bin');mkdirSync(bins,{mode:0o700});
  function copyExecutable(row,name){
    readPinned(row.path,row.bytes,row.sha256);
    const target=join(bins,name);copyFileSync(row.path,target,FS.COPYFILE_EXCL);chmodSync(target,0o555);
    readPinned(target,row.bytes,row.sha256);return target;
  }
  const bin=copyExecutable(elf.retainedELF,'straw-recl9');
  const handkey=copyExecutable(control.helper.actualRetained,'straw-handkey');
  const compiledPath=join(control.buildDirectory,'keys/acceptValidity.verifier');
  let sequence=0;
  return {dir,control,actual,expected,expect,bin,handkey,compiledPath,
    output(){const p=join(dir,'case-'+String(++sequence).padStart(2,'0'));mkdirSync(p,{mode:0o700});return p;},
    release(){rmSync(dir,{recursive:true});}};
}

// All public fixture descriptors come from the pinned Root materializers. No synthetic
// file is assigned an authentic digest. The private files are passed unchanged to the
// actual wrapper's constructor, whose native/policy pins remain fully enforced.
export function construct(subject,root,{onFailure,provenanceError}={}) {
  const {actual:a,control:c,expected:x,expect:e}=root,events=[],calls=[];
  const adoptedConfig=record(a.originalDeciderConfiguration);
  const forbidden=name=>(()=>{calls.push(name);throw Error('C6_UNEXPECTED_PROOF_OPERATION');});
  const config={bin:root.bin,bin_sha256:'dc6e9ebedee878da4ccaf41b851c658d2fce480201a4c7681e2a505d2dae81a9',
    key_dir:dirname(a.rawDeciderVK.path),key_dir_sha256:{'decider.vk':a.rawDeciderVK.sha256,'decider.relation':a.deciderRelation.sha256},
    verifier:a.taggedVerifier.path,verifier_sha256:a.taggedVerifier.sha256,template:c.publicTemplate.path,template_sha256:c.publicTemplate.sha256,
    tagging:a.tagging.path,tagging_sha256:a.tagging.sha256,provenance:a.provenance.path,provenance_sha256:a.provenance.sha256,
    capture:a.captureProjection.path,capture_sha256:a.captureProjection.sha256,parent_vk_sha256:c.parentVK_SHA256,
    handkey_sha256:c.helper.actualRetained.sha256,expected_dom_stmt:c.expectedDomain,srs20_bound_path:c.srs20.boundPath,s_g2:adoptedConfig.s_g2,
    settle_verifier_sha256:x.settleVerifierSha256,capture_public_expected:x,capture_public_expected_sha256:a.capturePublicExpected.sha256};
  const context=subject.policy.freezeCallContext({network:'stagenet',address:a.address,entryPoint:'acceptValidity',deploymentRecordSha256:a.deploymentRecord.sha256});
  // stmt and pre_state are constructor-shape specimens only. No callback/proof is
  // allowed to consume them and no actual current state/statement claim is made.
  const route={batchNo:0n,instanceId:fromLe(e.instance_id),stmt:0n,material:{prover:'UNUSED',verifier:root.compiledPath,bzkir:'UNUSED'},prove:forbidden('route.prove')};
  const intent={schema:'actual-sherlock-call-intent/2',network:'stagenet',address:a.address,action:CALL,physical_calls:1,
    pre_state_sha256:a.deploymentRecord.sha256,deployment_record_sha256:a.deploymentRecord.sha256,
    instance_id:le(route.instanceId),batch_no:le(0n),tagged_verifier_sha256:config.verifier_sha256,template_sha256:config.template_sha256,
    prove_after_final_balance:false,balancing_scope:['dust']};
  const compiled=()=>readPinned(root.compiledPath,x.build['keys/acceptValidity.verifier'].bytes,x.build['keys/acceptValidity.verifier'].sha256);
  const checkProvenance=()=>{
    calls.push('checkProvenance');if(provenanceError)throw provenanceError;
    // This is the existing public runtimePorts seam. Validate authentic records with
    // the same production pure guards; do not import the SDK-bearing route module.
    const g=subject.guards,kd=config.key_dir_sha256;
    g.assertKeyDirEntries(readdirSync(config.key_dir),'C6 fixture key directory');
    const raw=readPinned(a.rawDeciderVK.path,a.rawDeciderVK.bytes,a.rawDeciderVK.sha256);
    readPinned(a.deciderRelation.path,a.deciderRelation.bytes,a.deciderRelation.sha256);
    const tagged=readPinned(a.taggedVerifier.path,a.taggedVerifier.bytes,a.taggedVerifier.sha256);
    g.assertTaggedCarriesRaw(tagged,raw);
    g.checkTaggingRecord(record(a.tagging),{raw_sha256:kd['decider.vk'],raw_bytes:raw.length,tagged_sha256:config.verifier_sha256,tagged_bytes:tagged.length});
    const p=record(a.provenance);
    g.checkProvenanceRecord(p,{decider_vk_sha256:kd['decider.vk'],decider_relation_sha256:kd['decider.relation'],template_sha256:config.template_sha256,
      parent_vk_sha256:config.parent_vk_sha256,s_g2:config.s_g2,template_slots:15,stmt_index:10});
    const cap=g.checkCaptureRecord(record(a.captureProjection),config.template_sha256);
    if(cap.settle_verifier_sha256!==config.settle_verifier_sha256)throw Error('C6_FIXTURE_SETTLE');
    g.assertNotCompiledAccept({tagged_sha256:config.verifier_sha256,compiled_accept_sha256:cap.compiled_accept_verifier_sha256},compiled(),raw);
    return {...cap,tagged_verifier_sha256:config.verifier_sha256};
  };
  const ledger={Proof:{deserialize:forbidden('Proof.deserialize')},ContractCall:class UnusedContractCall{}};
  const publicOutputRoot=root.output();
  const wrapped=subject.wrapper.wrapSealedCallBoundDRoute(route,{config,callContext:context,submissionIntent:intent,ledger,
    proofExpectations:c.proofExpectations.path,publicOutputRoot,deadline:Date.now()+60_000,
    onFailure:record=>{events.push(record);onFailure?.(record);},
    runtimePorts:{callKey:CALL,handkeyBin:root.handkey,parseTemplate:forbidden('parseTemplate'),checkCallPis:forbidden('checkCallPis'),checkProvenance}});
  const counts={check:0,prove:0,lookup:0};
  const base={events:[],async check(){counts.check++;return [];},async prove(){counts.prove++;throw Error('C6_UNEXPECTED_BASE_PROVE');},
    async lookupKey(){counts.lookup++;return {verifierKey:new Uint8Array(compiled())};}};
  return {wrapped,base,counts,calls,events,publicOutputRoot};
}
