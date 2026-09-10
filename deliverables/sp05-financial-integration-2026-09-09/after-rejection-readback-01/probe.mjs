/** Fixed read-only readback. Import is inert; tests inject a controlled transport. */
import {readFileSync,openSync,writeFileSync,fsyncSync,closeSync,constants} from 'node:fs';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {pathToFileURL} from 'node:url';
import {resolve} from 'node:path';
import {createLocalRpc} from '../../../experiments/moriarty-midnight-financial/ledger/integrate-local.mjs';
import {captureFinalizedFinancialState} from '../../../experiments/moriarty-midnight-financial/ledger/finalized-financial-state.mjs';
import {loadProvenFinancialContract} from '../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs';
import {beforeDeadline} from '../../../experiments/moriarty-midnight-financial/ledger/receipt.mjs';
import {serializePublicResult} from '../local-finalized-state-02/probe.mjs';
export {serializePublicResult};
const D='deliverables/sp05-financial-integration-2026-09-09/',S=D+'local-stale-loan-01/';
const GENESIS='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846',ADDRESS='ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713';
const CANDIDATE='29612e0e094e7a6e5dfc9b714e5b3f66fd9a15dbbb43b46c7906c584a011bad0',STATE='552d58ff2918332665b179b37a70907c4492a02b5c56d7db05dce5e427e573e0';
const digest=raw=>createHash('sha256').update(raw).digest('hex');
const check=(ok,code)=>{if(!ok)throw Error(code);};
export function validateEvidence({preflight,prior,candidate,candidateRaw,outcome,containment}){
 check(preflight.status==='SETTLED_LOAN_PUBLIC_VERIFIED'&&preflight.current.blockHeight===20422&&preflight.current.blockHash==='0xff3d377f98bda1e372aa83c91c60addda6fd279251e3e10c2c824c51c0634d5c'&&preflight.current.stateSha256===STATE,'BEFORE_BINDING');
 const known=prior.snapshots.find(s=>s.contractAddress===ADDRESS);check(prior.status==='OBSERVED'&&known?.stateSha256===STATE&&digest(Buffer.from(known.serializedStateHex,'hex'))===STATE,'KNOWN_NATIVE_BINDING');
 check(digest(candidateRaw)===CANDIDATE&&candidate.rawSha256===CANDIDATE&&candidate.transactionHash===CANDIDATE&&candidate.identifiers.length===2&&candidate.identifiers.every(v=>/^00[a-f0-9]{64}$/.test(v)),'CANDIDATE_BINDING');
 check(outcome.candidateRawSha256===CANDIDATE&&outcome.status==='NODE_REJECTED'&&outcome.phase==='submit'&&outcome.code==='NODE_RPC_INVALID_TRANSACTION'&&outcome.rpcCode===1010&&outcome.nodeRejectionEstablished===true&&outcome.stalePredicateEstablished===false,'OUTCOME_BINDING');
 check(containment.status==='COMPLETE'&&containment.units.length===2&&containment.units.every(u=>u.properties.MainPID==='0'&&u.cgroupExists===false&&u.cgroupProcsEmpty===true)&&containment.containers.length===3&&containment.containers.every(c=>c.state.Status==='exited'&&c.state.Pid===0),'PRIOR_CONTAINMENT');
 check(Number.isFinite(Date.parse(containment.observedAt))&&Date.now()>Date.parse(containment.observedAt),'POST_TERMINAL_ORDER');
 return {anchor:preflight.current,known,candidateIds:candidate.identifiers,containmentObservedAt:containment.observedAt};
}
function evidence(){
 const pins=JSON.parse(readFileSync(new URL('./inputs.json',import.meta.url))),records={};
 for(const [path,pin] of Object.entries(pins.files)){
  check(path.startsWith(D)&&!path.split('/').includes('..'),'INPUT_PATH');const raw=readFileSync(new URL('../../../'+path,import.meta.url));check(digest(raw)===pin,'INPUT_HASH');records[path]=path.endsWith('.bin')?raw:JSON.parse(raw);
 }
 return validateEvidence({preflight:records[S+'public-preflight-result.json'],prior:records[D+'local-finalized-state-02/probe-result.json'],candidate:records[S+'run-public/adverse-candidate.json'],candidateRaw:records[S+'run-public/adverse-candidate.bin'],outcome:records[S+'run-public/adverse-submission-outcome.json'],containment:records[S+'terminal-containment.json']});
}
export async function readbackAfterRejection({fetchImpl=globalThis.fetch,deadlineMs=Date.now()+89000,loadContract=loadProvenFinancialContract}={}){
 const started=Date.now();let phase='retained-evidence';const result={schema:'moriarty.after-rejection-readback/1',status:'INCOMPLETE',submissionsAllowed:0,proofsAllowed:0,loadersClosed:0,scope:'Trusted-node financial state nonmutation after retained pool rejection; no stale-specific reason, included rollback, authenticated proof or Preview claim'};
 const inTime=()=>check(Date.now()<deadlineMs,'READBACK_DEADLINE');
 try{
  check(Number.isSafeInteger(deadlineMs)&&deadlineMs>started&&deadlineMs<=started+89000,'READBACK_DEADLINE');const old=evidence();result.candidateRawSha256=CANDIDATE;result.candidateIds=old.candidateIds;result.beforeAnchor=old.anchor;result.beforeObservationKind='historical-reobservation-at-exact-preflight-anchor';result.priorContainmentObservedAt=old.containmentObservedAt;
  const rpc=async(method,params,limit=deadlineMs)=>{inTime();const value=await createLocalRpc({node:'http://127.0.0.1:19944',deadlineMs:Math.min(deadlineMs,limit,Date.now()+5000),fetchImpl})(method,params);inTime();return value;};
  phase='readiness';const readyStop=Math.min(deadlineMs,started+60000);let ready=false;
  for(let n=0;n<60&&Date.now()<readyStop;n++){
   try{const h=await rpc('chain_getFinalizedHead',[],readyStop);check(/^0x[a-f0-9]{64}$/.test(h),'READINESS_HEAD');ready=true;break;}
   catch(e){if(!(e instanceof TypeError&&e.message==='fetch failed'&&['ECONNREFUSED','UND_ERR_SOCKET'].includes(e.cause?.code)))throw e;if(Date.now()>=readyStop)break;await beforeDeadline(()=>new Promise(r=>setTimeout(r,Math.min(1000,readyStop-Date.now()-1))),readyStop);}
  }
  check(ready,'READINESS_DEADLINE');phase='genesis';check(await rpc('chain_getBlockHash',[0])==='0x'+GENESIS,'GENESIS_MISMATCH');
  const anchor=async()=>{const blockHash=await rpc('chain_getFinalizedHead',[]);check(/^0x[a-f0-9]{64}$/.test(blockHash),'BARRIER_HASH');const h=await rpc('chain_getHeader',[blockHash]);check(typeof h?.number==='string'&&/^0x[a-f0-9]+$/i.test(h.number)&&h.number.length<=18,'BARRIER_HEADER');const blockHeight=Number(BigInt(h.number));check(Number.isSafeInteger(blockHeight)&&blockHeight>=old.anchor.blockHeight,'BARRIER_HEIGHT');check(await rpc('chain_getBlockHash',[blockHeight])===blockHash,'BARRIER_CANONICAL');return {blockHash,blockHeight};};
  phase='post-terminal-barrier';result.barrier={...await anchor(),observedAt:new Date().toISOString(),afterPriorTerminalObservation:true};
  phase='load-reviewed-decoder';const loaded=await loadContract({case:'loan',receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});
  try{
   phase='revalidate-before-anchor';const before=await captureFinalizedFinancialState({contractAddress:ADDRESS,rpc:(m,p,l)=>m==='chain_getFinalizedHead'?Promise.resolve(old.anchor.blockHash):rpc(m,p,l),loadedContract:loaded,deadlineMs});
   check(before.blockHeight===old.anchor.blockHeight&&before.stateSha256===STATE&&before.serializedStateHex===old.known.serializedStateHex,'BEFORE_NATIVE_MISMATCH');result.before=before;
   phase='strictly-later-anchor';let later=false;for(let n=0;n<60;n++){const now=await anchor();if(now.blockHeight>result.barrier.blockHeight){later=true;break;}await beforeDeadline(()=>new Promise(r=>setTimeout(r,1000)),deadlineMs);}check(later,'NO_LATER_ANCHOR');
   phase='after-snapshot';const after=await captureFinalizedFinancialState({contractAddress:ADDRESS,rpc,loadedContract:loaded,deadlineMs});result.after=after;
   check(after.blockHeight>result.barrier.blockHeight&&after.blockHeight>before.blockHeight,'AFTER_NOT_STRICTLY_LATER');
   check(await rpc('chain_getBlockHash',[before.blockHeight])===before.blockHash&&await rpc('chain_getBlockHash',[result.barrier.blockHeight])===result.barrier.blockHash&&await rpc('chain_getBlockHash',[after.blockHeight])===after.blockHash,'ANCHOR_REORG');
   result.comparison={allNativeBytesEqual:after.serializedStateHex===before.serializedStateHex,stateSha256Equal:after.stateSha256===before.stateSha256,allDecodedFieldsEqual:isDeepStrictEqual(after.state,before.state),allBalancesEqual:isDeepStrictEqual(after.balances,before.balances)};
   check(Object.values(result.comparison).every(v=>v===true),'FINANCIAL_NONMUTATION_FAILED');inTime();result.status='NONMUTATION_OBSERVED';
  }finally{check(loaded.cleanup()?.loaderHooksRemoved===true,'LOADER_CLEANUP');result.loadersClosed++;}
  inTime();
 }catch(e){result.status='UNKNOWN';result.failure={phase,code:typeof e?.message==='string'&&/^[A-Z][A-Z0-9_]{0,95}$/.test(e.message)?e.message:'PUBLIC_READBACK_ERROR'};}
 result.elapsedMs=Math.max(0,Date.now()-started);return result;
}
function retain(value){const raw=Buffer.from(serializePublicResult(value));check(raw.length<=65536,'OUTPUT_LIMIT');const fd=openSync(new URL('./probe-result.json',import.meta.url),constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,raw);fsyncSync(fd);}finally{closeSync(fd);}const dir=openSync(new URL('.',import.meta.url),constants.O_RDONLY|constants.O_DIRECTORY);try{fsyncSync(dir);}finally{closeSync(dir);}}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url){const timer=setTimeout(()=>{retain({schema:'moriarty.after-rejection-readback/1',status:'UNKNOWN',failure:{code:'READBACK_HARD_DEADLINE'},submissionsAllowed:0});process.exit(2);},89000);try{const result=await readbackAfterRejection();clearTimeout(timer);retain(result);process.exitCode=result.status==='NONMUTATION_OBSERVED'?0:2;}catch{clearTimeout(timer);process.exitCode=3;}}
