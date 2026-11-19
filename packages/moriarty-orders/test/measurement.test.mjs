import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {test} from 'node:test';
import {measureTransaction, checkInventory} from '../../../scripts/qa/measure-order-lifecycle.mjs';
const recipe = JSON.parse(readFileSync(new URL('../../../deliverables/competitor-report/III.1/recipe.json',import.meta.url)));
const resource = {blockUsage:'0',bytesChurned:'8',bytesWritten:'4',readTime:'7',computeTime:'17'};
const strict = {verifyNativeProofs:true,verifyContractProofs:true,verifySignatures:true,enforceLimits:true,enforceBalancing:true};
const tx = {label:'fill',status:'success',time:12,transactionHash:'ab',strict,resources:resource,
  requiredFeeSpeck:'5',nativeFeeSpeck:'6',dustTimeBefore:12,dustTimeAfter:12,dustBefore:'21',dustAfter:'15',
  dustPayments:[{creationTime:12,accepted:true,segment:1,oldNullifier:'n',newCommitment:'c',vFee:'6'}],
  dustSpendEvents:[{source:{transactionHash:'ab',physicalSegment:1},content:{tag:'dustSpendProcessed',nullifier:'n',commitment:'c',vFee:'6'},serialized:[1]}]};
test('reject an incomplete inventory, wrong order or missing owner spend', () => {
  for (const spec of Object.values(recipe.operations)) {
    const steps = [...spec.setup,...spec.operation,...spec.recovery];
    checkInventory(spec,steps);
    for (const s of steps) assert.throws(()=>checkInventory(spec,steps.filter(x=>x!==s)));
    assert.throws(()=>checkInventory(spec,steps.toReversed()));
  }
});
test('count actual charged fees, all native resources and strict completion', () => {
  assert.equal(measureTransaction(tx).fee,6n);
  for(const edit of [r=>delete r.resources.readTime,r=>r.dustAfter='16',r=>r.dustTimeBefore++,
    r=>r.dustPayments[0].creationTime++,r=>r.dustSpendEvents[0].source.transactionHash='cd',
    r=>r.dustSpendEvents.push(r.dustSpendEvents[0]),r=>r.strict.verifyNativeProofs=false,
    r=>r.status='partialSuccess',r=>r.requiredFeeSpeck='7',r=>delete r.dustBefore]) {
    const changed=structuredClone(tx);edit(changed);assert.throws(()=>measureTransaction(changed));
  }
});
