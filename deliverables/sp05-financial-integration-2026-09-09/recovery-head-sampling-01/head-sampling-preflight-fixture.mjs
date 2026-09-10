// Synthetic transport only: actual preflight wrapper and pinned SDK/build inputs.
import {createServer} from 'node:http';
import fs from 'node:fs';
import {createHash} from 'node:crypto';
import {syncBuiltinESMExports} from 'node:module';
import {join} from 'node:path';
import {pathToFileURL,fileURLToPath} from 'node:url';
import {PINNED_NM} from '../../../experiments/moriarty-midnight-financial/ledger/providers.mjs';
import {EXISTING_LOAN} from '../../../experiments/moriarty-midnight-financial/ledger/recover-deployment.mjs';
import {preflightLocalRecovery} from '../../../experiments/moriarty-midnight-financial/ledger/integrate-local.mjs';
import {validateLocalLaunchPlan} from '../../../experiments/moriarty-midnight-financial/ledger/launch-local.mjs';
const started=performance.now(),requests=[],privateAccess=[],originalFs=new Map();
const oldHeight=20323,oldHash='ab'.repeat(32),newHeight=20324,newHash='cd'.repeat(32);
let currentHeight=oldHeight,currentHash=oldHash,finalizedReads=0;const samplingEvents=[];
const planBytes=fs.readFileSync(new URL('../local-recovery-diagnostic-01/public-plan.json',import.meta.url)),plan=JSON.parse(planBytes);
const privateRoots=[plan.wallet.seedFile,plan.wallet.stateDirectory,plan.roles.secretsFile,plan.existingDeployment.sourcePrivateStateDirectory,plan.existingDeployment.inspectionDirectory,plan.privateState.directory,plan.privateState.passwordFile,plan.outputDirectory];
for(const name of ['readFileSync','openSync','lstatSync','writeFileSync','mkdirSync']){
 const operation=fs[name];originalFs.set(name,operation);
 fs[name]=function(path,...args){const p=path instanceof URL?fileURLToPath(path):typeof path==='string'?path:'';
  if(privateRoots.some(root=>p===root||p.startsWith(root+'/'))){privateAccess.push(name);throw Error('OFFLINE_PRIVATE_ACCESS_FORBIDDEN');}
  return operation.call(this,path,...args);
 };
}
syncBuiltinESMExports();
const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const raw=fs.readFileSync(plan.existingDeployment.transactionFile),tx=ledger.Transaction.deserialize('signature','proof','binding',raw),state=[...tx.intents.values()].flatMap(i=>i.actions)[0].initialState;
const record={__typename:'RegularTransaction',id:1,raw:raw.toString('hex'),hash:EXISTING_LOAN.transactionHash,identifiers:[...EXISTING_LOAN.identifiers],protocolVersion:1000000,unshieldedCreatedOutputs:[],unshieldedSpentOutputs:[],block:{height:EXISTING_LOAN.blockHeight,hash:EXISTING_LOAN.blockHash,author:null,timestamp:Date.now()},transactionResult:{status:'SUCCESS',segments:[]},fees:{estimatedFees:'0',paidFees:'0'}};
const server=createServer(async(req,res)=>{
 let body='';for await(const chunk of req)body+=chunk;
 try{const q=JSON.parse(body);requests.push(q);let response;
  if(q.jsonrpc==='2.0'){
   if(!['chain_getBlockHash','chain_getHeader','chain_getFinalizedHead'].includes(q.method))throw Error('OFFLINE_UNEXPECTED_RPC');
   if(q.method==='chain_getFinalizedHead'&&++finalizedReads===4){currentHeight=newHeight;currentHash=newHash;samplingEvents.push({event:'advance-after-indexed-tip',request:requests.length,height:currentHeight,hash:currentHash});}
   const result=q.method==='chain_getHeader'?{number:'0x'+(q.params[0]==='0x'+oldHash?oldHeight:currentHeight).toString(16)}:'0x'+(q.method==='chain_getBlockHash'&&q.params[0]===0?plan.networkTag:q.method==='chain_getBlockHash'&&q.params[0]===EXISTING_LOAN.blockHeight?EXISTING_LOAN.blockHash:q.method==='chain_getBlockHash'&&q.params[0]===oldHeight?oldHash:currentHash);
   samplingEvents.push({request:requests.length,method:q.method,params:q.params,result});
   response={jsonrpc:'2.0',id:q.id,result};
  }else{
   let data;
   if(q.operationName==='TX_ID_QUERY')data={transactions:[record]};
   else if(q.operationName==='CONTRACT_STATE_QUERY')data={contractAction:q.variables.offset?.blockOffset?.hash===currentHash?null:{__typename:'ContractDeploy',state:Buffer.from(state.serialize()).toString('hex')}};
   else if(q.operationName==='QUERY_UNSHIELDED_BALANCES_WITH_OFFSET')data={contractAction:{__typename:'ContractDeploy',unshieldedBalances:[]}};
   else if(q.query==='query($offset: BlockOffset) { block(offset: $offset) { height hash timestamp } }')data={block:{height:currentHeight,hash:currentHash,timestamp:new Date().toISOString()}};
   else throw Error('OFFLINE_UNEXPECTED_GRAPHQL');
   if(data.block)samplingEvents.push({request:requests.length,event:'indexed-tip',height:data.block.height,hash:data.block.hash});
   response={data};
  }
  res.writeHead(200,{'content-type':'application/json'});res.end(JSON.stringify(response));
 }catch(e){res.writeHead(400,{'content-type':'application/json'});res.end(JSON.stringify({errors:[{message:e.message}]}));}
});
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
let outcome;
try{
 const endpoint='http://127.0.0.1:'+server.address().port;
 // Only transport endpoints and the fixture deadline differ from reviewed plan.
 plan.networkConfig={...plan.networkConfig,node:endpoint,indexer:endpoint+'/graphql',indexerWS:endpoint.replace('http:','ws:')+'/ws',proofServer:endpoint+'/proof-forbidden'};
 plan.limits.deadlineMs=Date.now()+6000;
 const validated=validateLocalLaunchPlan(plan);
 const result=await preflightLocalRecovery(validated);
 outcome={status:result.status,stateBlock:result.stateBlock,transactionHash:result.binding.transactionHash};
}catch(e){outcome={status:'FAILED',name:e.name,message:e.message};process.exitCode=1;}
finally{
 server.closeAllConnections();await new Promise(resolve=>server.close(resolve));
 for(const [name,operation] of originalFs)fs[name]=operation;syncBuiltinESMExports();
}
console.log(JSON.stringify({schema:'moriarty.head-sampling-preflight-reproducer/1',...outcome,elapsedMs:performance.now()-started,publicPlanSha256:createHash('sha256').update(planBytes).digest('hex'),requests,samplingEvents,privateAccess,fixtureClosed:!server.listening,scope:'Actual preflightLocalRecovery with real pinned build/native inputs and synthetic loopback RPC/GraphQL; no actual blockchain outcome or failure cause established'},null,2));
