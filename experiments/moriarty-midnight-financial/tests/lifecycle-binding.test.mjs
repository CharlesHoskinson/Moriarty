import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {generateLifecycleWrapper} from '../custody/generate-lifecycle.mjs';
const source=readFileSync(new URL('../../moriarty-language/spec/successor/examples/loan-lifecycle-fee.mori',import.meta.url),'utf8');
const sha=x=>createHash('sha256').update(x).digest('hex');
test('new generated originate authenticates both roles before disbursement',()=>{
 const {source:wrapper}=generateLifecycleWrapper({source});
 const circuit=wrapper.slice(wrapper.indexOf('export circuit originate('));
 assert.match(circuit,/capabilityHash\(pad\(32, "moriarty:sp05:loan:lender"\), networkTag, programDigest, lenderSecret\) == lenderCapability, "LENDER_CAPABILITY"/);
 assert.match(circuit,/assert\(expectedActor == 5, "ACTOR_MAPPING"\)/);
 assert.match(circuit,/BORROWER_CAPABILITY/);assert.match(circuit,/assert\(expectedDebtor == 2, "ACTOR_MAPPING"\)/);
 assert.ok(circuit.indexOf('LENDER_CAPABILITY')<circuit.indexOf('receiveUnshielded'));
 assert.ok(circuit.indexOf('BORROWER_CAPABILITY')<circuit.indexOf('sendUnshielded'));
});
test('owned lifecycle wrapper equals generated output and hashes source and Core',()=>{
 const g=generateLifecycleWrapper({source});
 assert.equal(g.source,readFileSync(new URL('../custody/loan-lifecycle.compact',import.meta.url),'utf8'));
 assert.equal(g.binding.sourceDigest,sha(source));assert.ok(g.source.includes(g.binding.coreDigest));
});
test('retained loan and swap bytes remain pinned',()=>{
 assert.equal(sha(readFileSync(new URL('../custody/loan.compact',import.meta.url))),'c1485f2915cedf173b858dfdbfdb9cf135915e109dea18bf00c8d237f302759e');
 assert.equal(sha(readFileSync(new URL('../custody/swap.compact',import.meta.url))),'29b4be0dafcb6013577f67d0446622be7cf9b8ef0bb3fc1fb81a32801a879a56');
});
test('compiled source contains separate fee debit and checked net-goal assertions',()=>{
 const {source:wrapper}=generateLifecycleWrapper({source});
 assert.match(wrapper,/checkedAdd\(feeAmount, borrowerMinimumNetCredit\)/);
 assert.match(wrapper,/INTENT_NET_GOAL/);assert.match(wrapper,/INTENT_DEBIT_CAP/);
 assert.match(wrapper,/receiveUnshielded\(cashColor, feeAmount\)/);
});

import {canonical} from '../../moriarty-language/src/successor/expression-wire-v1.ts';
import {lowerLifecycleSourceBinding} from '../ledger/lifecycle-source-binding.mjs';
import {loadLifecycleCapabilityRuntime,lifecycleCapability,prepareLifecycleOriginationCall} from '../ledger/lifecycle-sdk-caller.mjs';
async function callerInput() {
 const runtime=await loadLifecycleCapabilityRuntime(),binding=lowerLifecycleSourceBinding(source);
 const b=label=>Uint8Array.from(createHash('sha256').update(label).digest());
 const networkTag=sha('offline test network'),roles={borrowerSecret:b('offline borrower secret'),lenderSecret:b('offline lender secret'),borrowerAddress:sha('offline borrower address'),lenderAddress:sha('offline lender address')};
 const authority={networkTag,programDigest:binding.programDigest,debtor:{party:'Borrower',capability:lifecycleCapability(runtime,'moriarty:sp05:loan:borrower',networkTag,binding.programDigest,roles.borrowerSecret)},lender:{party:'Lender',capability:lifecycleCapability(runtime,'moriarty:sp05:loan:lender',networkTag,binding.programDigest,roles.lenderSecret)},outcomeIntent:{grossCaps:[{actor:'Borrower',asset:'Cash',maximumLedgerAmount:'110'},{actor:'Lender',asset:'Cash',maximumLedgerAmount:'100'}],minimumNetCredits:[{actor:'Borrower',asset:'Cash',minimumLedgerAmount:'99'}]}};
 const state={...JSON.parse(readFileSync(new URL('../../moriarty-language/spec/successor/examples/loan-lifecycle.state.json',import.meta.url),'utf8')),schemaVersion:'moriarty-financial-lifecycle-state/2',authority};
 const expectedHead={contractAddress:sha('offline contract address'),revision:'0'};
 const acceptedRead={...expectedHead,sourceDigest:binding.sourceDigest,coreDigest:binding.coreDigest,programDigest:binding.programDigest,networkTag,borrowerCapability:authority.debtor.capability,lenderCapability:authority.lender.capability,borrowerAddress:roles.borrowerAddress,lenderAddress:roles.lenderAddress,cashColor:sha('offline Cash token'),originated:false,borrowerGrossCap:'110',lenderGrossCap:'100',borrowerHasNetGoal:true,lenderHasNetGoal:false,borrowerMinimumNetCredit:'99',lenderMinimumNetCredit:'0'};
 const snapshotJSON=canonical({Pre:{phase:'0',paid:'0'},Args:{transferId:'D1',originationId:'O1',feeId:'F1',feeAmount:'1'},Obs:{},workInitial:state.work.remaining});
 return {source,stateJSON:JSON.stringify(state),snapshotJSON,acceptedRead,expectedHead,roles,assetBinding:{sourceAsset:'Cash',ledgerColor:acceptedRead.cashColor,numerator:'1',denominator:'1'},now:'1700000000',runtime};
}
test('SDK preparation verifies real persistentHash and binds both private arguments',async()=>{
 const input=await callerInput(),prepared=await prepareLifecycleOriginationCall(input);
 let observed; const sentinel={};
 const r=await prepared.createUnprovenCall({sdk:{async createUnprovenCallTx(p,o){observed=o;return sentinel;}},providers:{},compiledContract:{}});
 assert.equal(r,sentinel);assert.equal(observed.circuitId,'originate');assert.equal(observed.args[5],5n);assert.equal(observed.args[6],2n);assert.equal(observed.args[7],1n);
 assert.deepEqual(observed.args[1],input.roles.lenderSecret);
 assert.equal(JSON.stringify(prepared.public).includes(Buffer.from(input.roles.lenderSecret).toString('hex')),false);
});
test('SDK refuses substituted lender secret',async()=>{
 const input=await callerInput();input.roles.lenderSecret=input.roles.borrowerSecret;
 await assert.rejects(()=>prepareLifecycleOriginationCall(input),/LENDER_CAPABILITY/);
});
test('SDK refuses stale revision and source substitution',async()=>{
 const input=await callerInput();input.acceptedRead.revision='1';
 await assert.rejects(()=>prepareLifecycleOriginationCall(input),/REVISION_MISMATCH/);
 input.acceptedRead.revision='0';input.acceptedRead.coreDigest=sha('different core');
 await assert.rejects(()=>prepareLifecycleOriginationCall(input),/PROGRAM_MISMATCH/);
});
test('SDK refuses fee-induced net failure before unproven-call creation',async()=>{
 const input=await callerInput(),snapshot=JSON.parse(input.snapshotJSON);snapshot.Args.feeAmount='2';input.snapshotJSON=canonical(snapshot);
 await assert.rejects(()=>prepareLifecycleOriginationCall(input),/INTENT_NET_GOAL/);
});
test('SDK refuses substituted intent and native asset conversion',async()=>{
 const input=await callerInput();input.acceptedRead.borrowerGrossCap='111';
 await assert.rejects(()=>prepareLifecycleOriginationCall(input),/INTENT_BINDING/);
 input.acceptedRead.borrowerGrossCap='110';input.assetBinding.numerator='2';
 await assert.rejects(()=>prepareLifecycleOriginationCall(input),/QUANTUM_UNSUPPORTED/);
});
test('specialized generator rejects an added source guard',()=>{
 assert.throws(()=>generateLifecycleWrapper({source:source.replace(/(action originate[^\n]*\{)/,'$1\n requires false;')}),/LIFECYCLE_SOURCE_PIN/);
});
test('SDK owns head and role inputs across asynchronous runtime loading',async()=>{
 const input=await callerInput();delete input.runtime;
 const expected=structuredClone(input.expectedHead),secret=new Uint8Array(input.roles.lenderSecret);
 const pending=prepareLifecycleOriginationCall(input);
 input.expectedHead.contractAddress=sha('substituted during await');input.expectedHead.revision='999';input.roles.lenderSecret.fill(0);
 const prepared=await pending;assert.equal(prepared.public.contractAddress,expected.contractAddress);assert.equal(prepared.public.expectedRevision,expected.revision);
 let observed;await prepared.createUnprovenCall({sdk:{async createUnprovenCallTx(p,o){observed=o;}},providers:{},compiledContract:{}});
 assert.deepEqual(observed.args[1],secret);assert.equal(observed.args[4],0n);
});
