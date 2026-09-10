import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
import {PINNED_NM} from './providers.mjs';
const api=await import('./contract-balances.mjs').catch(()=>({}));
const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs')).href);
const transactionRuntime=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const D=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url);
const state=stage=>ledger.ContractState.deserialize(readFileSync(new URL(`swap-initialize-diagnosis-01/indexed-${stage}-state.bin`,D)));
const colorA='5475695f8ccb85c05055a4ffef06bbbff208c58b07cf5c61052d25143729d166',colorB='e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4';
const extract=options=>{assert.equal(typeof api.extractNativeContractBalances,'function');return options.ledger===fixtureLedger?api.projectUnshieldedContractBalanceMap(options.state.balance):api.extractNativeContractBalances({state:options.state});};
test('actual retained swap deployment has empty native contract balances',()=>{assert.deepEqual(extract({state:state('deploy'),ledger}),{});});
test('actual retained initialized swap native state contains both exact pool reserves',()=>{
 const s=state('initialize'),before=Buffer.from(s.serialize()),out=extract({state:s,ledger});assert.deepEqual(out,{[colorA]:'1000000',[colorB]:'2000000'});assert.ok(Object.isFrozen(out));assert.deepEqual(Buffer.from(s.serialize()),before);
});
test('actual retained initialized loan native state has no contract custody balance',()=>{const s=ledger.ContractState.deserialize(readFileSync(new URL('local-recovery-03/indexed-initialize-state.bin',D)));assert.deepEqual(extract({state:s,ledger}),{});});
// Controlled fixtures exercise the pure map projector for invalid values that
// native WASM setters normally reject. They do not bypass production state identity.
class FixtureState {constructor(balance){this.balance=balance;}}
const fixtureLedger={ContractState:FixtureState};
const token=()=>({tag:'unshielded',raw:colorA});
for(const [name,make] of [
 ['plain state',()=>({state:{balance:new Map()},ledger})],['missing state',()=>({ledger})],['wrong native class',()=>({state:new FixtureState(new Map()),ledger})],
 ['not a Map',()=>({state:new FixtureState([]),ledger:fixtureLedger})],['negative value',()=>({state:new FixtureState(new Map([[token(),-1n]])),ledger:fixtureLedger})],['overflow value',()=>({state:new FixtureState(new Map([[token(),1n<<128n]])),ledger:fixtureLedger})],['number value',()=>({state:new FixtureState(new Map([[token(),1]])),ledger:fixtureLedger})],['string value',()=>({state:new FixtureState(new Map([[token(),'1']])),ledger:fixtureLedger})],
 ['plain string token',()=>({state:new FixtureState(new Map([[colorA,1n]])),ledger:fixtureLedger})],['dust token',()=>({state:new FixtureState(new Map([[{tag:'dust'},1n]])),ledger:fixtureLedger})],['shielded token',()=>({state:new FixtureState(new Map([[{tag:'shielded',raw:colorA},1n]])),ledger:fixtureLedger})],['short token',()=>({state:new FixtureState(new Map([[{tag:'unshielded',raw:'aa'},1n]])),ledger:fixtureLedger})],['uppercase token',()=>({state:new FixtureState(new Map([[{tag:'unshielded',raw:colorA.toUpperCase()},1n]])),ledger:fixtureLedger})],['extra field',()=>({state:new FixtureState(new Map([[{...token(),extra:1},1n]])),ledger:fixtureLedger})],['duplicate token identity',()=>({state:new FixtureState(new Map([[token(),1n],[token(),2n]])),ledger:fixtureLedger})],['oversized map',()=>({state:new FixtureState(new Map(Array.from({length:4097},(_,i)=>[{tag:'unshielded',raw:i.toString(16).padStart(64,'0')},1n]))),ledger:fixtureLedger})]
])test(`native balance extraction rejects ${name}`,()=>{assert.throws(()=>extract(make()),/CONTRACT_BALANCE/);});
test('closed token fields reject getters, hidden fields, symbols and inherited shapes without getter access',()=>{
 let reads=0;const getter={tag:'unshielded',get raw(){reads++;return colorA;}};
 const hidden=Object.defineProperty(token(),'extra',{value:1}),symbol=Object.assign(token(),{[Symbol('extra')]:1}),inherited=Object.assign(Object.create({}),token());
 for(const t of [getter,hidden,symbol,inherited])assert.throws(()=>extract({state:new FixtureState(new Map([[t,1n]])),ledger:fixtureLedger}),/CONTRACT_BALANCE/);assert.equal(reads,0);
});
test('zero and maximum u128 are retained exactly in detached frozen decimal map',()=>{const map=new Map([[token(),0n],[{tag:'unshielded',raw:colorB},(1n<<128n)-1n]]),out=extract({state:new FixtureState(map),ledger:fixtureLedger});assert.deepEqual(out,{[colorA]:'0',[colorB]:'340282366920938463463374607431768211455'});map.clear();assert.equal(out[colorA],'0');assert.ok(Object.isFrozen(out));});

test('actual transaction-runtime ContractState cannot replace the SDK provider state class',()=>{assert.notEqual(ledger.ContractState,transactionRuntime.ContractState);const bytes=readFileSync(new URL('swap-initialize-diagnosis-01/indexed-initialize-state.bin',D));assert.throws(()=>api.extractNativeContractBalances({state:transactionRuntime.ContractState.deserialize(bytes)}),/CONTRACT_BALANCE_STATE/);assert.deepEqual(api.extractNativeContractBalances({state:ledger.ContractState.deserialize(bytes)}),{[colorA]:'1000000',[colorB]:'2000000'});});
