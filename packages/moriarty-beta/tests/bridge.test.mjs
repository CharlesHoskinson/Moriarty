import test from 'node:test';
import assert from 'node:assert/strict';
import { expand, simulate } from '../src/bridge.ts';
import { parseAndLowerSource6 } from '../../../experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified } from '../../../experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts';
import { transferSource,repaySource,transferScenario,repayScenario,explicitTransfer,explicitRepay } from './fixtures.mjs';
const json=JSON.stringify;
for(const [label,source,action,scenario,oracle] of [['transfer',transferSource,'pay',transferScenario,explicitTransfer],['repay',repaySource,'repay_loan',repayScenario,explicitRepay]]) {
 test(`${label}: independent explicit Source6 field and complete result correspondence`,()=>{
  const expanded=expand(source,action,json(scenario));
  assert.equal(expanded.status,'Expanded',json(expanded));
  assert.deepEqual(parseAndLowerSource6(expanded.source6),parseAndLowerSource6(oracle));
  const result=simulate(source,action,json(scenario));
  assert.equal(result.status,'PreparedUnqualified',json(result));
  assert.deepEqual(result.result,prepareSource6S0Unqualified(oracle));
  assert.ok(expanded.fieldMap.length>20);
  assert.ok(expanded.origins.some(x=>x.kind==='scenario'));
  assert.ok(expanded.origins.some(x=>x.kind==='derived'));
 });
}
test('transfer has all gross effects, exact accounting and external gates',()=>{
 const r=simulate(transferSource,'pay',json(transferScenario));
 assert.deepEqual(r.result.candidate.effects.map(e=>e.kind),['Debit','Credit','Credit','UseAllowance','UseReplay','AdvanceHead']);
 assert.deepEqual(r.result.candidate.candidatePost.balances.map(x=>x.amount),['8990','1000','10']);
 assert.deepEqual(r.result.candidate.candidatePost.allowances,[{owner:'Owner',remaining:'8990',spent:'1010'}]);
 assert.equal(r.result.candidate.candidatePost.workSpent,'1');
 assert.equal(r.result.candidate.candidatePost.workRemaining,'9');
 assert.deepEqual(r.result.candidate.candidatePost.consumedReplay,['["Midnight","Owner","n1"]']);
 assert.equal(r.result.candidate.requiredPremises.length,4);assert.equal(r.result.unverifiedBindings.length,4);
});
test('scenario closure rejects raw duplicates and unknown or missing fields',()=>{
 const text=json(transferScenario).replace('"head":"h0"','"head":"h0","\\u0068ead":"evil"');
 assert.equal(expand(transferSource,'pay',text).diagnostics[0].code,'BETA_JSON_DUPLICATE');
 for(const scenario of [{...transferScenario,intent:{}},{...transferScenario,signatureVerified:true},{...transferScenario,balances:[...transferScenario.balances,{account:'Other',amount:'0'}]},{...transferScenario,allowance:{owner:'Owner',remaining:'10000'}}]) assert.equal(expand(transferSource,'pay',json(scenario)).status,'FormationRejected');
});
function reject(source,action,scenario,judgment,code){const r=simulate(source,action,json(scenario));assert.equal(r.status,'CoreRejected',json(r));assert.equal(r.result.rejection.judgment,judgment);assert.equal(r.result.rejection.code,code);assert.equal(r.result.rejection.publishedEffects,null);return r;}
test('hostile candidate missing fee remains an Effect rejection',()=>{
 const candidate_effects=[{kind:'Debit',account:'Owner',asset:'A',amount:'1010'},{kind:'Credit',account:'Recipient',asset:'A',amount:'1000'},{kind:'UseAllowance',owner:'Owner',amount:'1010'},{kind:'UseReplay',key:'n1'},{kind:'AdvanceHead',predecessor:'h0',successor:'h1'}];
 reject(transferSource,'pay',{...transferScenario,candidate_effects,head:'stale'},'effect','S0_EFFECT_MISMATCH');
});
test('overpay remains Core Effect and expiry remains earlier Intent',()=>{
 const source=repaySource.replace('30.00 USD','0.31 USD');
 const scenario={...repayScenario,obligation:{...repayScenario.obligation,principal:'20',accrued:'10',outstanding:'30'}};
 reject(source,'repay_loan',scenario,'effect','S0_EFFECT_RANGE');
 reject(source,'repay_loan',{...scenario,round:'11'},'intent','S0_INTENT_SCOPE');
 reject(source,'repay_loan',{...scenario,head:'stale'},'effect','S0_EFFECT_RANGE');
});
test('receiver overflow, authority and replay use actual Core order',()=>{
 reject(transferSource,'pay',{...transferScenario,balances:[transferScenario.balances[0],{account:'Recipient',amount:((1n<<128n)-1n).toString()},transferScenario.balances[2]]},'effect','S0_EFFECT_RANGE');
 reject(transferSource,'pay',{...transferScenario,allowance:{owner:'Owner',remaining:'1009',spent:'0'}},'authority','S0_AUTH_SCOPE');
 reject(transferSource,'pay',{...transferScenario,work_remaining:'0'},'authority','S0_AUTH_SCOPE');
 reject(transferSource,'pay',{...transferScenario,head:'stale'},'history','S0_HISTORY_STALE');
 reject(transferSource,'pay',{...transferScenario,replay:'consumed'},'history','S0_HISTORY_REPLAY');
});
test('zero fee retains exact three snapshot cells but no fee effect',()=>{
 const source=transferSource.replace('const fee = 0.10 USD','const fee = 0.00 USD');
 const r=simulate(source,'pay',json(transferScenario));assert.equal(r.status,'PreparedUnqualified',json(r));
 assert.equal(r.result.candidate.effects.length,5);assert.equal(r.result.candidate.candidatePost.balances[2].amount,'0');
});
test('renaming declarations leaves economic Core identities unchanged',()=>{
 const original=simulate(transferSource,'pay',json(transferScenario));
 const renamed=simulate(transferSource.replaceAll('Buyer','Purchaser'),'pay',json(transferScenario));
 assert.deepEqual(renamed.result,original.result);
});
test('origin map identifies agreement, source reference uses and derived replay dependencies',()=>{
 const r=expand(transferSource,'pay',json(transferScenario));assert.equal(r.status,'Expanded');
 const agreement=r.fieldMap.find(x=>x.field==='agreement');const agreementOrigin=r.origins.find(x=>x.id===agreement.origins[0]);
 assert.equal(Buffer.from(transferSource).subarray(agreementOrigin.span.start,agreementOrigin.span.end).toString(),'Invoice');
 const resolved=r.origins.filter(x=>x.rule==='resolved-reference-use-and-definition');assert.ok(resolved.length>0);
 const spans=resolved.flatMap(x=>x.dependencies.map(id=>r.origins.find(o=>o.id===id))).filter(x=>x.kind==='source').map(x=>Buffer.from(transferSource).subarray(x.span.start,x.span.end).toString());
 assert.ok(spans.includes('price'));assert.ok(spans.includes('Buyer'));
 const effect=r.fieldMap.find(x=>x.field==='effects.4');assert.ok(effect);
 const derivation=r.origins.find(x=>x.id===effect.origins[0]);assert.ok(derivation.dependencies.length>=7);
});
