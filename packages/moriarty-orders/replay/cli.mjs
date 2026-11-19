#!/usr/bin/env node
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {fileURLToPath} from 'node:url';
import * as ledger from '@midnightntwrk/ledger-v9';
import * as runtime from '@midnight-ntwrk/compact-runtime';
import * as compiled from '../generated/index.js';
import {readAcceptedOrder,verifyEnrollment} from '../src/lifecycle.mjs';
import {replayNativeReceipt} from '../native/replay.mjs';

const root = fileURLToPath(new URL('../',import.meta.url));
const validator = join(root,'native/validate_call');
const manifest = JSON.parse(readFileSync(join(root,'replay/manifest.json')));
const [command,operation,...extra] = process.argv.slice(2);
assert.ok(command==='replay' && Object.hasOwn(manifest.samples,operation) && !extra.length,
  'usage: moriarty-orders replay o1-fill | moriarty-orders replay cancel-unfilled');
const sample = manifest.samples[operation];
const directory = join(root,'replay',operation);
const read = file=>readFileSync(join(directory,file));
const sha256 = bytes=>createHash('sha256').update(bytes).digest('hex');
for (const [file,expected] of Object.entries(sample.sha256)) assert.equal(sha256(read(file)),expected,file);
const checks = JSON.parse(read('checks.json'));
const records = JSON.parse(read('receipts.json'));
assert.equal(checks.executionId,sample.executionId);
assert.equal(checks.deployment.network,manifest.network);
assert.equal(checks.deployment.address,sample.deployment);
assert.equal(checks.deployment.programHash,manifest.program);
assert.deepEqual(checks.deployment.verifiers,manifest.verifiers);
assert.deepEqual(records.map(r=>r.label),sample.labels);
function balances(artifact,revision) {
  const state = ledger.LedgerState.deserialize(read(artifact.file));
  const head = readAcceptedOrder({network:manifest.network,index:address=>state.index(address)},runtime,compiled,checks.deployment);
  verifyEnrollment(ledger,runtime,checks.deployment,head,checks.deployment.enrollment);
  assert.equal(head.revision,BigInt(revision));
  assert.equal(head.kernelState.f3,BigInt(revision));
  const roles = [head.terms.account,head.terms.executor,head.terms.recipient,null];
  return roles.map(role=>[head.terms.assetA,head.terms.assetB].map(asset=>{
    const color = Buffer.from(asset).toString('hex');
    if (!role) return Number(head.custody.get(color)??0n);
    const coins = [...state.utxo.filter(runtime.decodeUserAddress(role.bytes))];
    return Number(coins.filter(coin=>coin.type===color).reduce((sum,coin)=>sum+coin.value,0n));
  }));
}
const receipts = records.map((row,index)=>{
  if (index) assert.equal(row.beforeState.sha256,records[index-1].afterBlockStateSha256);
  const result = replayNativeReceipt(validator,directory,row);
  const expected = row.label.startsWith('control-')?'failure':'success';
  assert.equal(result.outcome,expected);
  for (const call of result.contractCalls) {
    assert.equal(call.proofVersion,'V3');
    if (call.address===sample.deployment) assert.equal(call.verifierKeySHA256,manifest.verifiers[call.entryPoint]);
  }
  return result;
});
const funded = records.find(row=>row.label==='fund-order');
const initialBalances = balances(funded.afterState,0);
const finalBalances = balances(records.at(-1).afterState,1);
assert.deepEqual(initialBalances,sample.initialBalances);
assert.deepEqual(finalBalances,sample.finalBalances);
if (operation==='cancel-unfilled') {
  const spend = records.find(row=>row.label==='spend-refund');
  assert.deepEqual(balances(spend.beforeState,1),sample.finalBalances);
  assert.deepEqual(balances(spend.afterState,1),sample.finalBalances);
  const before = ledger.LedgerState.deserialize(read(spend.beforeState.file));
  const order = readAcceptedOrder({network:manifest.network,index:a=>before.index(a)},runtime,compiled,checks.deployment);
  const owner = runtime.decodeUserAddress(order.terms.account.bytes);
  const color = Buffer.from(order.terms.assetA).toString('hex');
  const tx = ledger.Transaction.deserialize('signature','proof','binding',read(spend.transaction.file));
  const inputs = [...tx.intents.values()].flatMap(i=>[...(i.guaranteedUnshieldedOffer?.inputs??[]),...(i.fallibleUnshieldedOffer?.inputs??[])]);
  const returned = inputs.filter(c=>c.type===color);
  assert.ok(returned.length && returned.every(c=>ledger.addressFromKey(c.owner)===owner));
  assert.equal(returned.reduce((sum,c)=>sum+c.value,0n),100n);
}
console.log(JSON.stringify({schema:'moriarty-orders/installed-replay/1',operation,address:sample.deployment,initialBalances,finalBalances,receipts},null,2));
