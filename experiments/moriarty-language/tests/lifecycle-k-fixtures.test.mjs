import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createFinancialExpressionContractV4 } from '../src/successor/financial-expression-v1.ts';
import { compareLifecycleResult } from '../formal/k/lifecycle-corpus.mjs';

const fixture=JSON.parse(readFileSync(new URL('../formal/k/fixtures/lifecycle-v1.json',import.meta.url),'utf8'));
test('lifecycle fixtures retain38distinct requirement IDs and separate external controls',()=>{
  assert.equal(fixture.coverage.length,38);
  assert.equal(new Set(fixture.coverage.map(x=>x.id)).size,38);
  assert.equal(new Set(fixture.cases.map(x=>x.id)).size,fixture.cases.length);
  assert.equal(fixture.coverage.find(x=>x.id==='metadata-5947').status,'external-control');
  assert.equal(fixture.coverage.find(x=>x.id==='comparator-corruption').status,'external-control');
  for(const row of fixture.cases){
    assert.deepEqual(Object.keys(row.packet).sort(),['financialPreState','request','schema']);
    assert.ok(Object.values(row.packet).every(v=>typeof v==='string'));
    assert.equal(row.expectedMatchesCore,true,row.id);
    assert.notEqual(row.expectedMatchesSource,false,row.id);
    assert.ok(row.derivation.length>20,row.id);
  }
});
test('every concrete packet agrees with its full independent expected result',()=>{
  for(const row of fixture.cases){
    const {schema,request,financialPreState}=row.packet;
    const result=createFinancialExpressionContractV4(schema,financialPreState).evaluate(request);
    assert.deepEqual(compareLifecycleResult(result,row.expected),{ok:true,differences:[]},row.id);
    if(result.status==='Rejected')for(const key of ['post','financialPost','effects','descriptors','workRemaining'])assert.equal(Object.hasOwn(result,key),false,`${row.id}/${key}`);
  }
});
test('four frozen lifecycle packets chain complete independently expected predecessors',()=>{
  const chain=fixture.cases.filter(x=>x.chain?.id==='loan-lifecycle');assert.equal(chain.length,4);
  for(let i=1;i<chain.length;i++){
    const request=JSON.parse(chain[i].packet.request),previous=chain[i-1].expected;
    assert.equal(chain[i].chain.previous,chain[i-1].id);
    assert.deepEqual(JSON.parse(chain[i].packet.financialPreState),previous.financialPost);
    assert.deepEqual(request.Pre,previous.post);assert.equal(request.workInitial,previous.workRemaining);
  }
});
