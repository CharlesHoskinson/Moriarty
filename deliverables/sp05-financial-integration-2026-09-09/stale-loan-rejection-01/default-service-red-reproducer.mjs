import test from 'node:test';import assert from 'node:assert/strict';import {readFileSync} from 'node:fs';import {pathToFileURL} from 'node:url';
const root='/home/charl/Moriarty/.worktrees/sp05-deadline-review',nm='/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';
const load=p=>import(pathToFileURL(nm+'/'+p).href);
const {Effect}=await load('effect/dist/esm/index.js');const {PolkadotNodeClient,DEFAULT_CONFIG}=await load('@midnight-ntwrk/wallet-sdk-node-client/dist/effect/PolkadotNodeClient.js');const serviceApi=await load('@midnight-ntwrk/wallet-sdk-capabilities/dist/submission/submissionService.js');
const ledger=await load('@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs');const helper=await import(pathToFileURL(root+'/experiments/moriarty-midnight-financial/ledger/stale-loan-rejection.mjs').href);
test('actual default service and node-client invalid callback reaches closed classifier',async t=>{
 const raw=readFileSync(root+'/deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/public-transactions/473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480.bin');let sends=0,disconnects=0,constructs=0;
 const api={isConnected:true,async disconnect(){disconnects++;},tx:{midnight:{sendMnTransaction(hex){assert.equal(hex,'0x'+raw.toString('hex'));return {send(callback){sends++;queueMicrotask(()=>callback({status:{isInvalid:true}}));return Promise.resolve(()=>{});}};}}}};
 t.mock.method(PolkadotNodeClient,'make',()=>{constructs++;return Effect.succeed(new PolkadotNodeClient(DEFAULT_CONFIG,api));});
 const service=serviceApi.makeDefaultSubmissionService({relayURL:new URL('ws://127.0.0.1:1')});let failure;
 try{await service.submitTransaction(ledger.Transaction.deserialize('signature','proof','binding',raw),'Finalized');}catch(e){failure=e;}finally{await service.close();}
 console.log({constructs,sends,disconnects,error:failure?.message});assert.equal(constructs,1);assert.equal(sends,1);assert.equal(disconnects,1);assert.ok(failure);
 const actual=helper.classifyStaleLoanFailure(failure,'submit',raw);console.log(JSON.stringify({boundary:'Actual default submission service and actual PolkadotNodeClient with controlled API callback; no live network',classification:actual}));assert.equal(actual.nodeRejectionEstablished,true);
});
