import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {runLifecycleCorpus,compareLifecycleResult} from '/home/charl/Moriarty/experiments/moriarty-language/formal/k/lifecycle-corpus.mjs';
import {createFinancialAgreementSourceV5} from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts';
const report=await runLifecycleCorpus(); assert.equal(report.ok,true);
const retained=JSON.parse(readFileSync('/home/charl/Moriarty/deliverables/lifecycle-corpus-2026-09-17/corpus-result.json','utf8')); assert.deepEqual(report,retained);
let mutations=0;
for(const stage of report.stages){
 function walk(v,parts=[]){
  if(v!==null&&typeof v==='object'){for(const k of Object.keys(v))walk(v[k],[...parts,k]);return;}
  const changed=structuredClone(stage.expected); let n=changed; for(const k of parts.slice(0,-1))n=n[k]; n[parts.at(-1)]=v===null?'unexpected':typeof v==='string'?v+'!':!v;
  assert.equal(compareLifecycleResult(changed,stage.expected).ok,false,parts.join('/')); mutations++;
 }
 walk(stage.expected);
}
let first=true;
const divergent=await runLifecycleCorpus({sourceFactory(){const real=createFinancialAgreementSourceV5();return {...real,evaluate(...args){const out=real.evaluate(...args);if(first){first=false;out.post.phase='77';}return out;}};}});
assert.equal(divergent.ok,false);assert.equal(divergent.stages[1].source.input.snapshot.Pre.phase,'77');assert.equal(divergent.stages[1].core.input.snapshot.Pre.phase,'1');
for(const stage of report.stages){const f=stage.source.result.financialPost;assert.equal(f.balances.filter(b=>b.asset==='Cash').reduce((s,b)=>s+BigInt(b.amount),0n),110n);assert.equal(BigInt(f.work.remaining)+BigInt(f.work.spent),529n);assert.equal(f.allowances[0].spent,'100');}
console.log(JSON.stringify({passed:true,mutations,retainedObservationExactMatch:true,sourcePredecessorIsolation:true,cashConservation:true,grossLenderDebitPreserved:true}));
