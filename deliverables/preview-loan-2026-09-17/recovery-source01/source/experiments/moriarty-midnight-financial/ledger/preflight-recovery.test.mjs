import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {syncBuiltinESMExports} from 'node:module';
import {fileURLToPath} from 'node:url';
import {join} from 'node:path';
import {preflightLocalRecovery} from './integrate-local.mjs';
import {EXISTING_LOAN} from './recover-deployment.mjs';

test('actual public preflight rejects wrong genesis after retained native and build checks without private access',async t=>{
 const original='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03',destination='/inert/preflight-recovery-destination',inspection='/inert/preflight-recovery-inspection';
 const source=fileURLToPath(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-execution-04/',import.meta.url));
 const networkTag='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';
 const plan={kind:'loan',limits:{submissions:3,allocationId:'synthetic-genesis-gate',deadlineMs:Date.now()+60000},build:{receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:EXISTING_LOAN.buildReceiptSha256,sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'},networkTag,expectedProtocolVersion:1000000,roles:{secretsFile:original+'/roles.json'},privateState:{directory:destination,passwordFile:'/inert/preflight-password'},outputDirectory:'/inert/preflight-output',networkConfig:{networkId:'undeployed',node:'http://127.0.0.1:9944',indexer:'http://127.0.0.1:8088/graphql',indexerWS:'ws://127.0.0.1:8088/ws'}};
 plan.existingDeployment={schema:'moriarty.existing-local-loan/1',transactionFile:join(source,'run-public/public-transactions',EXISTING_LOAN.transactionHash+'.bin'),transactionHash:EXISTING_LOAN.transactionHash,identifiers:[...EXISTING_LOAN.identifiers],txId:EXISTING_LOAN.txId,contractAddress:EXISTING_LOAN.contractAddress,buildReceiptSha256:EXISTING_LOAN.buildReceiptSha256,networkTag,expectedProtocolVersion:1000000,sourceAllocationId:EXISTING_LOAN.allocationId,sourceResultFile:join(source,'attempt-result.json'),sourceResultSha256:'bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54',sourcePrivateStateDirectory:original+'/contract-state',inspectionDirectory:inspection,destinationDirectory:destination};
 const requests=[],privateAccess=[];
 t.mock.method(globalThis,'fetch',async(url,options)=>{
  const body=JSON.parse(options.body);requests.push({url,body});
  assert.equal(url,'http://127.0.0.1:9944/');assert.equal(body.method,'chain_getBlockHash');assert.deepEqual(body.params,[0]);assert.equal(options.method,'POST');assert.equal(options.redirect,'error');
  return new Response(JSON.stringify({jsonrpc:'2.0',id:body.id,result:'0x'+'00'.repeat(32)}));
 });
 for(const name of ['readFileSync','openSync','lstatSync','writeFileSync','mkdirSync']){
  const operation=fs[name];t.mock.method(fs,name,function(path,...args){
   const value=path instanceof URL?fileURLToPath(path):typeof path==='string'?path:'';
   if([original,destination,inspection,plan.privateState.passwordFile].some(root=>value===root||value.startsWith(root+'/'))){privateAccess.push(name);throw Error('PRIVATE_ACCESS_FORBIDDEN');}
   return operation.call(this,path,...args);
  });
 }
 syncBuiltinESMExports();
 try{await assert.rejects(preflightLocalRecovery(plan),e=>{assert.equal(e.message,'RECOVERY_PUBLIC_GENESIS');return true;});assert.equal(requests.length,1);assert.deepEqual(privateAccess,[]);}
 finally{t.mock.restoreAll();syncBuiltinESMExports();}
});
