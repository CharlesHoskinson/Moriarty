import test from 'node:test';
import assert from 'node:assert/strict';
import {createExpressionContractV1} from '../src/successor/expression-v1.ts';
import {J,inputs,N,P} from './expression-v1-support.mjs';
const schema=J(inputs().schema);
// Assemble exact wire strings independently of the runtime serializer or host
// recursive stringify. Type metadata does not contribute to value/Core depth.
const option=(n,bottom='["Bool"]')=>'["Option",'.repeat(n)+bottom+']'.repeat(n);
const coreFor=t=>J(N('ConstructNone',{elementType:'TYPE_MARKER'})).replace('"TYPE_MARKER"',t);
const request=(core,work='1')=>J({contract:'moriarty-expression-contract/1',source:'',core:'CORE_MARKER',Pre:{},Args:{},Obs:{},workInitial:work}).replace('"CORE_MARKER"',core);
const evaluate=(type,work='1')=>createExpressionContractV1(schema).evaluate(request(coreFor(type),work));
function rejected(result,code,work='0') {assert.deepEqual(result,{status:'Rejected',code,span:P,nodePath:[],workUsed:work});}
function checkNone(result,n) {
 assert.equal(result.judgmentResult,'ExpressionValue');assert.deepEqual(result.value,[]);assert.equal(result.workRemaining,'0');
 let t=result.type;for(let i=0;i<n+1;i++){assert.equal(t.length,2);assert.equal(t[0],'Option');t=t[1];}assert.deepEqual(t,['Bool']);
}
for(const n of [508,509,600,3000])test(`None with Option metadata depth ${n}`,()=>checkNone(evaluate(option(n)),n));
const maxDepth=Math.floor((65536-Buffer.byteLength(coreFor(option(0))))/11);
test('largest Option chain fitting Core byte bound succeeds',()=>{
 assert.ok(maxDepth>5900);assert.ok(Buffer.byteLength(coreFor(option(maxDepth)))<=65536);
 checkNone(evaluate(option(maxDepth)),maxDepth);
});
test('next Option chain exceeds Core bytes and rejects before work',()=>{
 assert.ok(Buffer.byteLength(coreFor(option(maxDepth+1)))>65536);rejected(evaluate(option(maxDepth+1)),'INPUT_BOUND');
});
test('deep metadata still needs an evaluation work unit',()=>rejected(evaluate(option(maxDepth),'0'),'WORK_EXHAUSTED'));
test('deep type shape error is INPUT_SCHEMA',()=>rejected(evaluate(option(3000,'["Unknown"]')),'INPUT_SCHEMA'));
test('deep unresolved nominal type is TYPE_NAME',()=>rejected(evaluate(option(3000,'["Record","Missing"]')),'TYPE_NAME'));
test('deep Unit data type remains disallowed',()=>rejected(evaluate(option(3000,'["Unit"]')),'TYPE_NAME'));
test('deep Collection capacity checks preserve inner type error order',()=>{
 rejected(evaluate('["Collection",'+option(3000,'["Record","Missing"]')+',"129"]'),'TYPE_NAME');
 rejected(evaluate('["Collection",'+option(3000)+',"129"]'),'TYPE_COLLECTION_BOUND');
});
test('deep Collection metadata accepts valid finite capacities',()=>{
 const n=2000,t='["Collection",'.repeat(n)+'["Bool"]'+',"0"]'.repeat(n);
 const result=evaluate(t);assert.equal(result.judgmentResult,'ExpressionValue');assert.deepEqual(result.value,[]);assert.equal(result.workRemaining,'0');
 let inner=result.type[1];for(let i=0;i<n;i++){assert.equal(inner[0],'Collection');assert.equal(inner[2],'0');inner=inner[1];}assert.deepEqual(inner,['Bool']);
});
test('malformed canonical encoding remains rejected at deep metadata',()=>{
 rejected(createExpressionContractV1(schema).evaluate(request(coreFor(option(3000)).replace('["Bool"]','[ "Bool"]'))),'INPUT_SCHEMA');
});
test('deep schema JSON is rejected by normative schema depth bound',()=>{
 const s=schema.replace('"args":{}','"args":{"x":'+option(3000)+'}');
 rejected(createExpressionContractV1(s).evaluate(request(coreFor('["Bool"]'))),'INPUT_BOUND');
});
test('deep types compare structurally during ConstructSome typing',()=>{
 const n=2500;
 const core=J(N('ConstructSome',{elementType:'TYPE_MARKER',value:'VALUE_MARKER'}))
  .replace('"TYPE_MARKER"',option(n)).replace('"VALUE_MARKER"',coreFor(option(n-1)));
 const result=createExpressionContractV1(schema).evaluate(request(core,'2'));
 assert.equal(result.judgmentResult,'ExpressionValue');assert.deepEqual(result.value,[[]]);assert.equal(result.workRemaining,'0');
 let t=result.type;for(let i=0;i<n+1;i++){assert.equal(t[0],'Option');t=t[1];}assert.deepEqual(t,['Bool']);
});
test('deep mismatched types reject statically before work',()=>{
 const core=J(N('ConstructSome',{elementType:'TYPE_MARKER',value:'VALUE_MARKER'}))
  .replace('"TYPE_MARKER"',option(2500)).replace('"VALUE_MARKER"',coreFor(option(2499,'["Text"]')));
 rejected(createExpressionContractV1(schema).evaluate(request(core,'0')),'TYPE_MISMATCH');
});
test('deep metadata types in unselected branches remain statically checked',()=>{
 const condition=N('LitBool',{value:false});
 const core=J(N('And',{left:condition,right:N('Eq',{left:'LEFT_MARKER',right:'RIGHT_MARKER'})}))
  .replace('"LEFT_MARKER"',coreFor(option(2500))).replace('"RIGHT_MARKER"',coreFor(option(2500,'["Record","Missing"]')));
 assert.deepEqual(createExpressionContractV1(schema).evaluate(request(core,'2')),
  {status:'Rejected',code:'TYPE_NAME',span:P,nodePath:['1','1'],workUsed:'0'});
});
