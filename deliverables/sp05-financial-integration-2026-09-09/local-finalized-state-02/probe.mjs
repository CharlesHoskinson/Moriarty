/** One admitted local node-only observation; importing this file does not run it. */
import {openSync,writeFileSync,fsyncSync,closeSync,constants} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {resolve} from 'node:path';
import {createLocalRpc} from '../../../experiments/moriarty-midnight-financial/ledger/integrate-local.mjs';
import {captureFinalizedFinancialState} from '../../../experiments/moriarty-midnight-financial/ledger/finalized-financial-state.mjs';
import {loadProvenFinancialContract} from '../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs';
import {beforeDeadline} from '../../../experiments/moriarty-midnight-financial/ledger/receipt.mjs';
const GENESIS='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';
const SOURCE='a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6';
const CASES=[
 {case:'loan',address:'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',receiptSha256:'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7'},
 {case:'swap',address:'8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261',receiptSha256:'3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362'},
];
export const serializePublicResult=value=>JSON.stringify(value,(_key,v)=>typeof v==='bigint'?v.toString():v instanceof Uint8Array?{hex:Buffer.from(v).toString('hex')}:v,2)+'\n';
export async function probeFinalizedLoanAndSwap({fetchImpl=globalThis.fetch,deadlineMs=Date.now()+89000}={}){
 const started=Date.now();let phase='readiness';
 const result={schema:'moriarty.local-finalized-state-probe/1',status:'INCOMPLETE',methodSupport:'UNKNOWN',snapshots:[],loadersClosed:0,submissionsAllowed:0,proofsAllowed:0,scope:'Read-only node observations at explicit finalized anchors; no authenticated state proof or ledger-nonmutation claim'};
 const inTime=()=>{if(Date.now()>=deadlineMs)throw Error('PROBE_DEADLINE');};
 try{
  if(!Number.isSafeInteger(deadlineMs)||deadlineMs<=started||deadlineMs>started+89000)throw Error('PROBE_DEADLINE');
  const rpc=async(method,params,limit=deadlineMs)=>{
   inTime();const stop=Math.min(deadlineMs,limit,Date.now()+5000);
   const value=await createLocalRpc({node:'http://127.0.0.1:19944',deadlineMs:stop,fetchImpl})(method,params);inTime();return value;
  };
  const readyStop=Math.min(deadlineMs,started+60000);let ready=false;
  for(let sample=0;sample<60&&Date.now()<readyStop;sample++){
   try{
    const head=await rpc('chain_getFinalizedHead',[],readyStop);
    if(!/^0x[a-f0-9]{64}$/.test(head))throw Error('NODE_READINESS_HEAD');
    if(Date.now()>=readyStop)throw Error('NODE_READINESS_DEADLINE');ready=true;break;
   }catch(error){
    if(!(error instanceof TypeError&&error.message==='fetch failed'&&['ECONNREFUSED','UND_ERR_SOCKET'].includes(error.cause?.code)))throw error;
    if(Date.now()>=readyStop)break;
    await beforeDeadline(()=>new Promise(r=>setTimeout(r,Math.min(1000,readyStop-Date.now()-1))),readyStop);
   }
  }
  if(!ready)throw Error('NODE_READINESS_DEADLINE');
  phase='genesis';const genesis=await rpc('chain_getBlockHash',[0]);if(genesis!=='0x'+GENESIS)throw Error('NODE_GENESIS_MISMATCH');result.genesis=GENESIS;
  for(const c of CASES){
   phase=c.case+'-build';inTime();
   const loaded=await loadProvenFinancialContract({case:c.case,receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/'+c.case+'-output/build/build-receipt.json',receiptSha256:c.receiptSha256,sourceManifestHash:SOURCE});
   try{
    inTime();phase=c.case+'-snapshot';
    const snapshot=await captureFinalizedFinancialState({contractAddress:c.address,rpc,loadedContract:loaded,deadlineMs});
    inTime();result.snapshots.push(snapshot);result.methodSupport='AVAILABLE';
   }finally{if(loaded.cleanup()?.loaderHooksRemoved!==true)throw Error('PROBE_LOADER_CLEANUP');result.loadersClosed++;}
  }
  inTime();result.status='OBSERVED';
 }catch(error){
  result.status='FAILED';result.failure={phase,causeCode:['ECONNREFUSED','UND_ERR_SOCKET','ECONNRESET','ETIMEDOUT','EACCES','ENOTFOUND','UND_ERR_CONNECT_TIMEOUT'].includes(error?.cause?.code)?error.cause.code:'UNKNOWN',code:typeof error?.message==='string'&&/^[A-Z][A-Z0-9_]{0,95}$/.test(error.message)?error.message:'UNCLASSIFIED_PUBLIC_ERROR',errorClass:['Error','TypeError','RangeError','SyntaxError'].includes(error?.name)?error.name:'UNCLASSIFIED_ERROR',publicMessage:String(error?.message??'NO_ERROR_MESSAGE').replace(/[^\x20-\x7e]/g,' ').slice(0,2048)};
 }
 result.elapsedMs=Math.max(0,Date.now()-started);return result;
}
function retain(result){
 const raw=Buffer.from(serializePublicResult(result));if(raw.length>65536)throw Error('PROBE_OUTPUT_LIMIT');
 const output=new URL('./probe-result.json',import.meta.url),fd=openSync(output,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);
 try{writeFileSync(fd,raw);fsyncSync(fd);}finally{closeSync(fd);}
 const directory=openSync(new URL('.',import.meta.url),constants.O_RDONLY|constants.O_DIRECTORY);try{fsyncSync(directory);}finally{closeSync(directory);}
}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url){
 const timer=setTimeout(()=>{retain({schema:'moriarty.local-finalized-state-probe/1',status:'FAILED',methodSupport:'UNKNOWN',failure:{code:'PROBE_HARD_DEADLINE'},scope:'Timer expired; partial operation state unknown; no retry'});process.exit(2);},89000);
 try{const result=await probeFinalizedLoanAndSwap();clearTimeout(timer);retain(result);process.exitCode=result.status==='OBSERVED'?0:2;}
 catch{clearTimeout(timer);process.exitCode=3;}
}
