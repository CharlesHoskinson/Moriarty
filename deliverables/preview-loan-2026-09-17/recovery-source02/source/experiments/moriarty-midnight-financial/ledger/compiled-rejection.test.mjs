import test from 'node:test';
import assert from 'node:assert/strict';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const receiptPath=process.env.MORIARTY_SWAP_BUILD_RECEIPT;
assert.ok(receiptPath,'MORIARTY_SWAP_BUILD_RECEIPT must name the retained full swap build; no install/rebuild/skip');
const helper=await import('./compiled-rejection.mjs').catch(e=>{if(e.code==='ERR_MODULE_NOT_FOUND')return {};throw e;});
const sdkUrl=pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-contracts/dist/index.mjs')).href;
const loaderUrl=new URL('./proven-assets.mjs',import.meta.url).href;
const realLoader=await import(loaderUrl);
function required(){assert.equal(typeof helper.checkCompiledSwapRejection,'function','missing compiled rejection checker');return helper.checkCompiledSwapRejection;}
for(const mutation of ['revision','program','network'])test('actual generated close rejects '+mutation+' without changing serialized inputs',async t=>{
 let loaded;
 t.mock.module(loaderUrl,{namedExports:{loadProvenFinancialContract:async options=>{loaded=await realLoader.loadProvenFinancialContract(options);return loaded;}}});
 const result=await required()({receiptPath,mutation});
 assert.equal(result.status,'REJECTED_BEFORE_TRANSACTION');assert.equal(result.code,mutation.toUpperCase()+'_MISMATCH');
 assert.equal(result.stateUnchanged,true);assert.equal(result.privateStateUnchanged,true);assert.equal(result.loaderClosed,true);
 assert.equal(result.zkConfigCalls,0);assert.equal(result.boundary,'createUnprovenCallTxFromInitialStates');
 assert.equal(result.currentLedgerStateChecked,false);
 assert.throws(()=>loaded.assertFresh(),/financial assets loader closed/);
 console.log(JSON.stringify(result));
});
function expected(){return new Error('failed assert: REVISION_MISMATCH',{cause:Object.assign(new Error("Error executing circuit 'close'",{cause:Object.assign(new Error('failed assert: REVISION_MISMATCH'),{name:'CompactError'})}),{name:'ContractRuntimeError',_tag:'ContractRuntimeError'})});}
for(const [name,run,code] of [
 ['unexpected success',()=>({private:{unprovenTx:{}}}),'COMPILED_REJECTION_UNEXPECTED_SUCCESS'],
 ['unrelated error',()=>{throw Error('not the expected circuit error');},'COMPILED_REJECTION_UNEXPECTED_ERROR'],
 ['matching text without runtime cause',()=>{throw Error('failed assert: REVISION_MISMATCH');},'COMPILED_REJECTION_UNEXPECTED_ERROR'],
 ['public input mutation',(_zk,o)=>{o.initialContractState.balance=new Map();throw expected();},'COMPILED_REJECTION_STATE_MUTATED'],
 ['private input mutation',(_zk,o)=>{o.initialPrivateState.changed=1;throw expected();},'COMPILED_REJECTION_PRIVATE_STATE_MUTATED'],
 ['connected ZK lookup',async(zk)=>{await zk.getVerifierKey('close');},'COMPILED_REJECTION_ZK_CONFIG_ACCESSED'],
 ])test(name+' fails closed and cleans the connected loader',async t=>{
 let closes=0,calls=0;
 t.mock.module(loaderUrl,{namedExports:{loadProvenFinancialContract:async()=>({compiledContract:{},cleanup(){closes++;return {loaderHooksRemoved:true};}})}});
 t.mock.module(sdkUrl,{namedExports:{createUnprovenCallTxFromInitialStates:async(...args)=>{calls++;return run(...args);}}});
 await assert.rejects(required()({receiptPath,mutation:'revision'}),e=>e.message===code);
 assert.equal(calls,1);assert.equal(closes,1);
});
test('cleanup failure cannot produce an accepted rejection result',async t=>{
 t.mock.module(loaderUrl,{namedExports:{loadProvenFinancialContract:async()=>({compiledContract:{},cleanup(){return {loaderHooksRemoved:false};}})}});
 t.mock.module(sdkUrl,{namedExports:{createUnprovenCallTxFromInitialStates:async()=>{throw expected();}}});
 await assert.rejects(required()({receiptPath,mutation:'revision'}),/COMPILED_REJECTION_CLEANUP/);
});
test('unknown fields and SDK overrides reject before loading',async()=>{
 await assert.rejects(required()({receiptPath,mutation:'revision',sdk:{}}),/COMPILED_REJECTION_OPTIONS/);
 await assert.rejects(required()({receiptPath,mutation:'anything'}),/COMPILED_REJECTION_OPTIONS/);
});
