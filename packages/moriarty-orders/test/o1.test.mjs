import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';

const frozen = JSON.parse(readFileSync(new URL('./fixtures/o1.json',import.meta.url)));
test('both complete O1 deltas reconcile custody, executor and recipient', async()=>{
  const {checkO1Accounts} = await import('../native/o1-accounting.mjs');
  const post = [[0,0],[100,999800],[0,200],[0,0]];
  checkO1Accounts(frozen,'exact',frozen.funded,post);
  checkO1Accounts(frozen,'improved',frozen.funded,[[0,0],[100,999799],[0,201],[0,0]]);
  assert.throws(()=>checkO1Accounts(frozen,'exact',frozen.funded,[[0,0],[99,999800],[0,200],[1,0]]));
  assert.throws(()=>checkO1Accounts(frozen,'exact',frozen.funded,[[0,200],[100,999800],[0,0],[0,0]]));
  assert.throws(()=>checkO1Accounts(frozen,'exact',frozen.funded,[[0,0],[100,999800],[0,199],[0,0]]));
  assert.throws(()=>checkO1Accounts(frozen,'rejected',post,[[0,0],[100,999600],[0,400],[0,0]]));
});
