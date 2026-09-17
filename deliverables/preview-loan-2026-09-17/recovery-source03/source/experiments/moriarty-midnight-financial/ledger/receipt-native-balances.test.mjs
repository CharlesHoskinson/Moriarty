/** Retained actual swap bytes/state, controlled transport/finality context.
 * These regressions establish the observer's balance source, not live finality. */
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import {observeFinalizedStage,decodeNativeFinancialTransaction} from './receipt.mjs';
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs'));
const stateRuntime=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs'));
assert.notEqual(stateRuntime.ContractState,ledger.ContractState,'SDK state producer and transaction runtime are distinct classes');
const codec=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/wallet-sdk-address-format/dist/index.js'));
const root=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url);
function fixture(){
 const raw=readFileSync(new URL('local-swap-01/run-public/public-transactions/3fec717c8d31e6da9b9ace76c9f209bdbda11fe9d27f5d870c6edf428d2be7b4.bin',root));
 const tx=ledger.Transaction.deserialize('signature','proof','binding',raw),native=decodeNativeFinancialTransaction(raw,ledger);
 const state=stateRuntime.ContractState.deserialize(readFileSync(new URL('swap-initialize-diagnosis-01/indexed-initialize-state.bin',root)));
 const hash='f268da29bb3e6e81f8a97bae0eb53938f3a146fce677deb5c7ae18b8c5335386';
 const data={tx,txId:native.identifiers.at(-1),identifiers:native.identifiers,txHash:native.transactionHash,status:'SucceedEntirely',blockHash:hash,blockHeight:20393,protocolVersion:1000000,fees:{paidFees:'1',estimatedFees:'1'},unshielded:{spent:[],created:native.outputs.map(o=>({owner:codec.MidnightBech32m.encode('undeployed',new codec.UnshieldedAddress(Buffer.from(o.owner,'hex'))).toString(),tokenType:o.type,value:BigInt(o.value),intentHash:o.intentHash}))}};
 const queries=[];let separateReads=0;
 const provider={watchForTxData:async()=>data,queryContractState:async(a,c)=>{queries.push([a,c]);return state;},queryUnshieldedBalances:async()=>{separateReads++;return [];}};
 const args={provider,ledger,txId:data.txId,contractAddress:native.actions[0].address,circuitId:'initialize',decodeState:x=>x,deadlineMs:Date.now()+10000,expectedProtocolVersion:1000000,rpc:async name=>name==='chain_getHeader'?{number:'0x4fa9'}:'0x'+hash};
 return {args,state,queries,hash,separateReads:()=>separateReads};
}
const expected={'5475695f8ccb85c05055a4ffef06bbbff208c58b07cf5c61052d25143729d166':'1000000','e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4':'2000000'};
test('observer reads actual initialized reserves from its exact-block native state, not empty separate index',async()=>{
 const f=fixture(),before=f.state.serialize(),observed=await observeFinalizedStage(f.args);
 assert.deepEqual(observed.receipt.contractBalances,expected);assert.equal(f.separateReads(),0);
 assert.deepEqual(f.queries,[[f.args.contractAddress,{type:'blockHash',blockHash:f.hash}]]);assert.deepEqual(f.state.serialize(),before);
});
test('an unavailable separate balance projection cannot suppress complete native balances',async()=>{
 const f=fixture();f.args.provider.queryUnshieldedBalances=async()=>{throw Error('SEPARATE_PROJECTION_MUST_NOT_BE_READ');};
 const observed=await observeFinalizedStage(f.args);assert.deepEqual(observed.receipt.contractBalances,expected);
});
test('observer rejects a plain object in place of native exact-block contract state',async()=>{
 const f=fixture();f.args.provider.queryContractState=async()=>({data:f.state.data,balance:f.state.balance});
 await assert.rejects(observeFinalizedStage(f.args),/CONTRACT_BALANCE_STATE/);
});
