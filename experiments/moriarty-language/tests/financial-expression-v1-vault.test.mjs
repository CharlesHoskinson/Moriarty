import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {N,U,Bin,Act,Req,Let,Arg,run} from './financial-expression-v1-support.mjs';
const D=new URL('../../../deliverables/sp02-financial-pure-expression-2026-09-10/inputs/',import.meta.url);
const rule=JSON.parse(readFileSync(new URL('reference-vault-rule.json',D))).definition;
const fixtures=JSON.parse(readFileSync(new URL('independent-vault-arithmetic-01.json',D))).cases;
const sourceSchema=JSON.parse(readFileSync(new URL('financial-schema.json',D))).sigma;
function input(row){
 const schema=structuredClone(sourceSchema);schema.args=structuredClone(schema.recordTypes.VaultConversionInput);
 const financial=Object.fromEntries(Object.keys(schema.recordTypes.FinancialState).map(k=>[k,[]]));
 const method=row.operation[0].toUpperCase()+row.operation.slice(1);
 return {schema,Pre:{financial},Obs:{},Args:{method,assets:['deposit','withdraw'].includes(row.operation)?row.quantity:'0',shares:['mint','redeem'].includes(row.operation)?row.quantity:'0',supply:row.supply,valuationAssets:row.valuation}};
}
function count(n){return 1+Object.values(n.operands).reduce((sum,v)=>sum+(v?.constructor&&typeof v.constructor==='string'?count(v):Array.isArray(v)?v.reduce((s,e)=>s+(e?.value?.constructor?count(e.value):e?.constructor&&typeof e.constructor==='string'?count(e):0),0):0),0);}
test('retained exact vault body has56 nodes and full reference Sigma is admitted',()=>{
 assert.equal(count(rule.body),56);const r=run(rule.body,input(fixtures[0]));assert.equal(r.judgmentResult,'ExpressionValue');assert.deepEqual(r.value,{assetFee:'0',assets:'4',shares:'1'});
});
for(const row of fixtures)test('independent vault oracle: '+row.id,()=>{
 const p=input(row);
 // These are explicit pure Requires for the published initialized subrule domain,
 // not a hidden host branch or an alteration of the56-node body.
 const guarded=Act(Req(Bin('Gt',Arg('supply'),U(0))),Req(Bin('Gt',Arg('valuationAssets'),U(0))),Let('vaultResult',structuredClone(rule.body)));
 const actual=run(guarded,p,1000);
 if(row.expect.rejection){
  assert.equal(actual.status,'Rejected');assert.equal(actual.code,{'RESULT_UINT128_BOUND':'ARITH_RANGE','INPUT_UINT128_BOUND':'INPUT_VALUE','OUTSIDE_INITIALIZED_SUBRULE':'GUARD_FAILED'}[row.expect.rejection]);
  assert.equal('post' in actual,false);
 }else{
  assert.equal(actual.status,'ExpressionPrepared');const bare=run(rule.body,p,1000);assert.equal(bare.value[row.expect.resultUnit],row.expect.output);
  assert.equal(bare.value.assetFee,'0');assert.equal(Number(bare.workRemaining)-Number(actual.workRemaining),9);
 }
});
test('unprotected body itself does not claim initialized-domain rejection',()=>{
 for(const row of fixtures.filter(r=>r.expect.rejection==='OUTSIDE_INITIALIZED_SUBRULE'))assert.equal(run(rule.body,input(row)).judgmentResult,'ExpressionValue');
});
