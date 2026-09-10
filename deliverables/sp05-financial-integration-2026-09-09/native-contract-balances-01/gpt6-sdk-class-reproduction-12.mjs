// Independent source12 reproduction only: public retained bytes and SDK imports.
import fs from 'node:fs';
import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
const repo='/home/charl/Moriarty/.worktrees/sp05-deadline-review';
const root='/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/';
const ledger=await import(pathToFileURL(root+'midnight-js-protocol/dist/ledger.mjs'));
const producer=await import(pathToFileURL(root+'midnight-js-protocol/dist/compact-runtime.mjs'));
const compact=await import(pathToFileURL(root+'compact-runtime/dist/index.js'));
const raw=fs.readFileSync(repo+'/deliverables/sp05-financial-integration-2026-09-09/swap-initialize-diagnosis-01/indexed-initialize-state.bin');
const state=producer.ContractState.deserialize(raw);
assert.equal(producer.ContractState===ledger.ContractState,false);
assert.equal(producer.ContractState===compact.ContractState,true);
assert.equal(state instanceof ledger.ContractState,false);
// Exact source12 failing predicate, retained so later production fixes cannot
// turn this historic reproduction into a claim about the successor source.
const source12StatePredicate=typeof ledger?.ContractState==='function'&&state instanceof ledger.ContractState;
assert.equal(source12StatePredicate,false);
console.log(JSON.stringify({sourceCandidateSha256:'7dc65b008f67683377e2ff0eb621bf7082bd2378026fa73c830b87dec76ded4f',source12Error:'CONTRACT_BALANCE_STATE',producerEqualsIntegrationCompactRuntime:producer.ContractState===compact.ContractState,producerEqualsLedgerV8:producer.ContractState===ledger.ContractState,bytes:raw.length,nativeBalances:[...state.balance].map(([token,value])=>[token,value.toString()]),scope:'Offline public SDK/state only; no services/private/proofs or live finality.'}));
