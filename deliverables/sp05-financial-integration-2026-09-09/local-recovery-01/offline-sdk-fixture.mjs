// Offline diagnostic: loopback HTTP only, actual pinned SDK, retained public bytes.
import {createServer} from 'node:http';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from '../../../experiments/moriarty-midnight-financial/ledger/providers.mjs';
import {EXISTING_LOAN,verifyExistingLoanPublic} from '../../../experiments/moriarty-midnight-financial/ledger/recover-deployment.mjs';
const start=performance.now(),requests=[],rpcRequests=[],mode=process.argv[2]??'compatible';
if(!['compatible','schema-reject'].includes(mode))throw Error('UNKNOWN_OFFLINE_MODE');
const sdkPath=join(PINNED_NM,'@midnight-ntwrk/midnight-js-indexer-public-data-provider/dist/index.mjs');
const {indexerPublicDataProvider}=await import(pathToFileURL(sdkPath).href);
const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const raw=readFileSync(new URL('../local-execution-04/run-public/public-transactions/'+EXISTING_LOAN.transactionHash+'.bin',import.meta.url));
const tx=ledger.Transaction.deserialize('signature','proof','binding',raw),state=[...tx.intents.values()].flatMap(i=>i.actions)[0].initialState;
const record={__typename:'RegularTransaction',id:1,raw:raw.toString('hex'),hash:EXISTING_LOAN.transactionHash,identifiers:[...EXISTING_LOAN.identifiers],protocolVersion:1000000,unshieldedCreatedOutputs:[],unshieldedSpentOutputs:[],block:{height:EXISTING_LOAN.blockHeight,hash:EXISTING_LOAN.blockHash,author:null,timestamp:Date.now()},transactionResult:{status:'SUCCESS',segments:[]},fees:{estimatedFees:'0',paidFees:'0'}};
const server=createServer(async(req,res)=>{
 let body='';for await(const chunk of req)body+=chunk;
 try{
  const q=JSON.parse(body);requests.push({operationName:q.operationName,variables:q.variables,query:q.query});let data;
  if(mode==='schema-reject'){res.writeHead(200,{'content-type':'application/json'});res.end(JSON.stringify({errors:[{message:'SYNTHETIC schema diagnostic: requested field unavailable'}]}));return;}
  if(q.operationName==='TX_ID_QUERY')data={transactions:[record]};
  else if(q.operationName==='CONTRACT_STATE_QUERY')data={contractAction:{__typename:'ContractDeploy',state:Buffer.from(state.serialize()).toString('hex')}};
  else if(q.operationName==='QUERY_UNSHIELDED_BALANCES_WITH_OFFSET')data={contractAction:{__typename:'ContractDeploy',unshieldedBalances:[]}};
  else throw Error('UNEXPECTED_FIXTURE_QUERY');
  res.writeHead(200,{'content-type':'application/json'});res.end(JSON.stringify({data}));
 }catch(e){res.writeHead(400,{'content-type':'application/json'});res.end(JSON.stringify({errors:[{message:e.message}]}));}
});
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
let outcome;
try{
 const endpoint='http://127.0.0.1:'+server.address().port;
 const provider=indexerPublicDataProvider(endpoint,endpoint.replace('http:','ws:'));
 const rpc=async(method,params)=>{rpcRequests.push({method,params});return method==='chain_getHeader'?{number:'0x'+EXISTING_LOAN.blockHeight.toString(16)}:'0x'+EXISTING_LOAN.blockHash;};
 const result=await verifyExistingLoanPublic({raw,ledger,provider,rpc,decodeState:()=>({scope:'public-state-native-equality-only'}),deadlineMs:Date.now()+5000,expectedProtocolVersion:1000000,tip:{status:'READY',hash:EXISTING_LOAN.blockHash,finalizedHash:'0x'+EXISTING_LOAN.blockHash,height:EXISTING_LOAN.blockHeight,finalizedHeight:EXISTING_LOAN.blockHeight,timestampMs:Date.now()}});
 outcome={status:result.status,stateBlock:result.stateBlock};
}catch(e){outcome={status:'FAILED',name:e.name,message:e.message};process.exitCode=1;}
finally{server.closeAllConnections();await new Promise(resolve=>server.close(resolve));}
console.log(JSON.stringify({schema:'moriarty.offline-sdk-diagnostic/1',mode,...outcome,elapsedMs:performance.now()-start,sdkEntrySha256:createHash('sha256').update(readFileSync(sdkPath)).digest('hex'),requests,rpcRequests,fixtureClosed:!server.listening,scope:'Synthetic GraphQL with retained native transaction/initial state; no actual chain cause established; no wallet/private material accessed'},null,2));
