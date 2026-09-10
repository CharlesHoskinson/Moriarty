// Offline independent check of retained public bytes; no transport or wallet.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {loadProvenFinancialContract} from '../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs';
const root='/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';
const {ContractState}=await import(pathToFileURL(root+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs').href);
const read=p=>JSON.parse(readFileSync(new URL(p,import.meta.url)));
const observed=read('./probe-result.json'),prior=read('../local-finalized-state-02/probe-result.json').snapshots.find(x=>x.contractAddress===observed.before.contractAddress);
const normalized=value=>JSON.parse(JSON.stringify(value,(_key,v)=>typeof v==='bigint'?v.toString():v instanceof Uint8Array?{hex:Buffer.from(v).toString('hex')}:v));
const loaded=await loadProvenFinancialContract({case:'loan',receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});
const results=[];let cleanup;
try{
 for(const name of ['before','after']){
  const snapshot=observed[name],raw=Buffer.from(snapshot.serializedStateHex,'hex'),state=ContractState.deserialize(raw);
  try{
   assert.equal(raw.length,8055);assert.equal(raw.toString('hex'),snapshot.serializedStateHex);
   assert.deepEqual(Buffer.from(state.serialize()),raw);
   assert.equal(createHash('sha256').update(raw).digest('hex'),snapshot.stateSha256);
   const decoded=normalized(loaded.decodeState(state.data));assert.deepEqual(decoded,snapshot.state);
   const balances={};for(const [token,value] of state.balance){assert.equal(token.tag,'unshielded');assert.ok(!Object.hasOwn(balances,token.raw));balances[token.raw]=String(value);}
   assert.deepEqual(balances,snapshot.balances);
   assert.deepEqual(balances,{'e92df6339320f55209d4586ce039b6ef05a7be960999fca913006146cb72cdef':'0'});
   assert.equal(decoded.initialized,true);assert.equal(decoded.revision,'2');assert.equal(decoded.remaining,'0');
   results.push({anchor:name,blockHeight:snapshot.blockHeight,blockHash:snapshot.blockHash,bytes:raw.length,stateSha256:snapshot.stateSha256,canonicalNativeRoundtrip:true,allGeneratedFieldsMatch:true,completeNativeBalancesMatch:true,balances});
  }finally{state.free();}
 }
 assert.equal(observed.before.serializedStateHex,observed.after.serializedStateHex);assert.deepEqual(observed.before.state,observed.after.state);assert.deepEqual(observed.before.balances,observed.after.balances);
 assert.equal(observed.before.serializedStateHex,prior.serializedStateHex);assert.deepEqual(observed.before.state,prior.state);assert.deepEqual(observed.before.balances,prior.balances);
 assert.equal(observed.before.blockHeight,20422);assert.equal(observed.barrier.blockHeight,20425);assert.equal(observed.after.blockHeight,20426);
 assert.ok(Date.parse(observed.barrier.observedAt)>Date.parse(observed.priorContainmentObservedAt));
 assert.equal(observed.before.authenticatedStateProof,false);assert.equal(observed.after.noInterveningActionsAfterAnchorEstablished,false);
}finally{cleanup=loaded.cleanup();assert.equal(cleanup.loaderHooksRemoved,true);}
console.log(JSON.stringify({schema:'moriarty.independent-native-readback-check/1',status:'PASS_SCOPED',results,completeEquality:true,knownHistoricalNativeStateMatches:true,loadedDecoderCleanup:cleanup,networkOperations:0,scope:'Independent native decoding of retained actual observations. Canonical chain authority remains the trusted-node observation; no new node query, authenticated state proof or transient-state exclusion.'},null,2));
