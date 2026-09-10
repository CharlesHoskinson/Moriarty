import test,{before,after} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import {loadProvenFinancialContract} from './proven-assets.mjs';
const api=await import('./finalized-financial-state.mjs').catch(e=>{if(e.code==='ERR_MODULE_NOT_FOUND')return {};throw e;});
const runtime=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs').href);
const raw=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/swap-initialize-diagnosis-01/indexed-initialize-state.bin',import.meta.url));
const address='8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261',hash='0x'+'ab'.repeat(32),height=20400;
let loadedContract;
before(async()=>{assert.ok(process.env.MORIARTY_SWAP_BUILD_RECEIPT,'MORIARTY_SWAP_BUILD_RECEIPT requires retained full artifacts, never rebuild');loadedContract=await loadProvenFinancialContract({case:'swap',receiptPath:process.env.MORIARTY_SWAP_BUILD_RECEIPT,receiptSha256:'3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'});});
after(()=>loadedContract?.cleanup());
function fixture(override=()=>undefined){
 const calls=[];return {calls,options:{contractAddress:address,loadedContract,deadlineMs:Date.now()+10000,rpc:async(method,params)=>{
  calls.push({method,params});const changed=await override(method,params,calls);if(changed!==undefined)return changed;
  if(method==='chain_getFinalizedHead'){assert.deepEqual(params,[]);return hash;}
  if(method==='chain_getHeader'){assert.deepEqual(params,[hash]);return {number:'0x'+height.toString(16)};}
  if(method==='chain_getBlockHash'){assert.deepEqual(params,[height]);return hash;}
  if(method==='midnight_contractState'){assert.deepEqual(params,[address,hash]);return raw.toString('hex');}
  throw Error('UNEXPECTED_RPC');
 }}};
}
function capture(o){assert.equal(typeof api.captureFinalizedFinancialState,'function','missing finalized node-state reader');return api.captureFinalizedFinancialState(o);}
test('actual full native state and generated decoder captured at explicit canonical finalized anchor',async()=>{
 const f=fixture(),result=await capture(f.options);
 assert.equal(result.contractAddress,address);assert.equal(result.blockHash,hash);assert.equal(result.blockHeight,height);
 assert.equal(result.stateSha256,'5a1dcdb726f25a724f89d0185af0fb8bb863b37e094fce8fc7dc1e75caf2b92c');
 assert.equal(result.serializedStateHex,raw.toString('hex'));
 assert.deepEqual(result.balances,{'5475695f8ccb85c05055a4ffef06bbbff208c58b07cf5c61052d25143729d166':'1000000','e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4':'2000000'});
 const actual=loadedContract.decodeState(runtime.ContractState.deserialize(raw).data);
 assert.deepEqual(result.state,structuredClone(actual));assert.ok(Object.keys(result.state).length>10);
 assert.equal(result.authenticatedStateProof,false);assert.equal(result.noInterveningActionsAfterAnchorEstablished,false);
 assert.deepEqual(f.calls.map(c=>c.method),['chain_getFinalizedHead','chain_getHeader','chain_getBlockHash','midnight_contractState','chain_getBlockHash']);
 assert.equal(loadedContract.assertFresh(),true);
});
for(const [name,override,code] of [
 ['malformed finalized head',m=>m==='chain_getFinalizedHead'?'bad':undefined,'FINALIZED_STATE_HEAD'],
 ['malformed header',m=>m==='chain_getHeader'?{number:'7'}:undefined,'FINALIZED_STATE_HEADER'],
 ['unsafe height',m=>m==='chain_getHeader'?{number:'0x20000000000000'}:undefined,'FINALIZED_STATE_HEIGHT'],
 ['canonical mismatch',m=>m==='chain_getBlockHash'?'0x'+'cd'.repeat(32):undefined,'FINALIZED_STATE_NONCANONICAL'],
 ['canonical changes during state query',(m,_p,c)=>m==='chain_getBlockHash'&&c.length===5?'0x'+'cd'.repeat(32):undefined,'FINALIZED_STATE_NONCANONICAL'],
 ['absent contract',m=>m==='midnight_contractState'?null:undefined,'FINALIZED_STATE_ENCODING'],
 ['unknown RPC envelope',m=>m==='midnight_contractState'?{result:raw.toString('hex')}:undefined,'FINALIZED_STATE_ENCODING'],
 ['non-native bytes',m=>m==='midnight_contractState'?'010203':undefined,'FINALIZED_STATE_NATIVE'],
 ['noncanonical trailing bytes',m=>m==='midnight_contractState'?raw.toString('hex')+'00':undefined,'FINALIZED_STATE_NATIVE'],
 ['oversized response',m=>m==='midnight_contractState'?'00'.repeat(1048577):undefined,'FINALIZED_STATE_ENCODING'],
 ])test(name+' rejects connected RPC evidence',async()=>{await assert.rejects(capture(fixture(override).options),e=>e.message===code);});
test('wrong address and expired deadline stop before RPC',async()=>{
 const f=fixture();await assert.rejects(capture({...f.options,contractAddress:'0x'+address}),/FINALIZED_STATE_ADDRESS/);
 await assert.rejects(capture({...f.options,deadlineMs:Date.now()-1}),/DEADLINE_EXPIRED/);assert.equal(f.calls.length,0);
});
test('unsupported node method propagates without latest or indexer fallback',async()=>{
 const f=fixture(m=>{if(m==='midnight_contractState')throw Error('METHOD_NOT_FOUND');});
 await assert.rejects(capture(f.options),/METHOD_NOT_FOUND/);assert.equal(f.calls.length,4);
});
test('hanging RPC is unknown under the shared deadline',async()=>{
 const f=fixture(m=>m==='midnight_contractState'?new Promise(()=>{}):undefined);
 await assert.rejects(capture({...f.options,deadlineMs:Date.now()+40}),/OBSERVATION_TIMEOUT_UNKNOWN/);
});
test('invalid full decoded state is rejected',async()=>{
 const f=fixture();await assert.rejects(capture({...f.options,loadedContract:{decodeState:()=>null}}),/FINALIZED_STATE_DECODE/);
});
test('native unsupported shielded contract balance is rejected',async()=>{
 const state=runtime.ContractState.deserialize(raw);state.balance=new Map([[{tag:'shielded',raw:'11'.repeat(32)},1n]]);
 await assert.rejects(capture(fixture(m=>m==='midnight_contractState'?Buffer.from(state.serialize()).toString('hex'):undefined).options),/CONTRACT_BALANCE_TOKEN/);
});

test('later best/finalized head does not change the explicit as-of anchor',async()=>{
 let moved=false;const f=fixture(m=>{if(m==='midnight_contractState')moved=true;if(m==='chain_getFinalizedHead'&&moved)return '0x'+'cd'.repeat(32);});
 const result=await capture(f.options);assert.equal(result.blockHash,hash);assert.equal(moved,true);
 assert.equal(f.calls.filter(c=>c.method==='chain_getFinalizedHead').length,1);
 assert.equal(result.noInterveningActionsAfterAnchorEstablished,false);
});
test('nonfinite and coerced original deadlines reject before any RPC',async()=>{
 for(const deadlineMs of [Infinity,NaN,String(Date.now()+10000),Date.now()+10000.5]){
  const f=fixture();await assert.rejects(capture({...f.options,deadlineMs}),/DEADLINE_EXPIRED/);assert.equal(f.calls.length,0);
 }
});
