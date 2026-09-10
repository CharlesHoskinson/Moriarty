import {readFileSync} from 'node:fs';
const root='/home/charl/Moriarty/.worktrees/sp05-deadline-review';
const {captureFinalizedFinancialState}=await import(root+'/experiments/moriarty-midnight-financial/ledger/finalized-financial-state.mjs');
const {loadProvenFinancialContract}=await import(root+'/experiments/moriarty-midnight-financial/ledger/proven-assets.mjs');
const loaded=await loadProvenFinancialContract({case:'swap',receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json',receiptSha256:'3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});
const raw=readFileSync(root+'/deliverables/sp05-financial-integration-2026-09-09/swap-initialize-diagnosis-01/indexed-initialize-state.bin').toString('hex');
const block='0x'+'ab'.repeat(32),deadlineMs=Date.now()+150;let canonicalCalls=0;
try{
 const result=await captureFinalizedFinancialState({contractAddress:'8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261',loadedContract:loaded,deadlineMs,rpc:async(method)=>{
  if(method==='chain_getFinalizedHead')return block;
  if(method==='chain_getHeader')return {number:'0x4fb0'};
  if(method==='midnight_contractState')return raw;
  if(method==='chain_getBlockHash'){
   if(++canonicalCalls===2){while(Date.now()<=deadlineMs+20){}}
   return block;
  }
 }});
 console.log(JSON.stringify({returnedAfterDeadline:Date.now()>deadlineMs,lateMs:Date.now()-deadlineMs,status:result.schema,canonicalCalls}));
} catch(e){console.log(JSON.stringify({error:e.message,lateMs:Date.now()-deadlineMs,canonicalCalls}));}
finally{loaded.cleanup();}
