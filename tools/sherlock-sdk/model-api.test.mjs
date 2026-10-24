import test from 'node:test';
import assert from 'node:assert/strict';
import {assertRecipeParameters} from './producer-policy.mjs';
import {ProducerCleanupError,producerFailure} from './owned-resources.mjs';

function fixture({mismatch=false,fault=null,freeFault=null,alias=false}={}) {
 const records=new Map(),primary=Error('PRIVATE_MODEL_API_DETAIL');let runtimeGets=0,shared;
 const handle=(name,extra={})=>{
  const r={name,calls:0,lookups:0},object={...extra};records.set(object,r);
  Object.defineProperty(object,'free',{get(){r.lookups++;return ()=>{r.calls++;if(freeFault===name)throw Error('PRIVATE_FREE_'+name);};}});return object;
 };
 // Faithful CostModel API: toString only, no serialize. The helper must not
 // acquire a runtime wrapper at all when comparing its serialized parent.
 const runtime=Object.freeze({toString:()=>'<same public runtime model>',free(){throw Error('unacquired runtime freed');}});
 assert.equal(typeof runtime.serialize,'undefined');
 const parameter=side=>{
  const params=handle(side+'Params');
  Object.defineProperty(params,'transactionCostModel',{get(){
   if(fault===side+'Getter')throw primary;
   if(alias&&shared)return shared;
   const model=handle(side+'Model',{serialize(){if(fault===side+'Serialize')throw primary;return Uint8Array.from(side==='expected'&&mismatch?[1,2,4]:[1,2,3]);}});
   Object.defineProperty(model,'runtimeCostModel',{get(){runtimeGets++;throw Error('runtime getter must not be used');}});
   if(alias)shared=model;return model;
  }});return params;
 };
 const actual=parameter('actual'),expected=parameter('expected'),recipe={blockData:{ledgerParameters:actual}};
 const check=()=>{
  for(const [object,r] of records){const borrowed=object===actual||object===expected;assert.equal(r.calls,borrowed?0:1,r.name);assert.equal(r.lookups,borrowed?0:1,r.name+' lookup');}
  assert.equal(runtimeGets,0);
 };
 return {actual,expected,recipe,primary,records,check,run:()=>assertRecipeParameters(recipe,expected)};
}

test('supported full model serialization matches without any runtime serialize API or getter access',()=>{
 const f=fixture();f.run();assert.equal(f.records.size,4);f.check();
});
test('different full model bytes refuse even though runtime text projection could match',()=>{
 const f=fixture({mismatch:true});assert.throws(f.run,error=>error.code==='ERR_ASSERTION'&&error.message.includes('ERR_PRODUCER_TRANSACTION_COST_MODEL_CHANGED'));f.check();
});
for(const fault of ['actualGetter','expectedGetter','actualSerialize','expectedSerialize']) {
 test('partial full-model getter or serializer failure drains acquired views: '+fault,()=>{
  const f=fixture({fault});assert.throws(f.run,error=>error===f.primary);f.check();
 });
}
test('full-model cleanup failure refuses equality while draining both views',()=>{
 const f=fixture({freeFault:'expectedModel'});assert.throws(f.run,error=>error instanceof ProducerCleanupError&&!error.primaryPresent);f.check();
});
test('model mismatch plus free failure preserves assertion primary and drains sibling',()=>{
 const f=fixture({mismatch:true,freeFault:'expectedModel'});assert.throws(f.run,error=>error instanceof ProducerCleanupError&&error.cause.code==='ERR_ASSERTION'&&producerFailure(error).primary===error.cause);f.check();
});
test('serializer plus free failure preserves original primary without abandoning sibling',()=>{
 const f=fixture({fault:'actualSerialize',freeFault:'expectedModel'});assert.throws(f.run,error=>error instanceof ProducerCleanupError&&producerFailure(error).primary===f.primary);f.check();
});
test('aliased owned model view is freed once and borrowed parameters remain live',()=>{
 const f=fixture({alias:true});f.run();assert.equal(f.records.size,3);f.check();
});
test('same borrowed parameter can provide distinct owned model getter views',()=>{
 let freed=0,gets=0;const params={get transactionCostModel(){gets++;return {serialize:()=>Uint8Array.of(7),free(){freed++;},get runtimeCostModel(){throw Error('unexpected runtime access');}};},free(){throw Error('borrowed parameters freed');}};
 assertRecipeParameters({blockData:{ledgerParameters:params}},params);assert.equal(gets,2);assert.equal(freed,2);
});
test('missing recipe parameters refuse before acquiring any proving model view',()=>{
 let reads=0;const params={get transactionCostModel(){reads++;throw Error('should not read');}};
 assert.throws(()=>assertRecipeParameters({},params));assert.equal(reads,0);
});

test('truthy malformed recipe parameters get a fixed assertion refusal and drain acquired proving model',()=>{
 let freed=0;const expected={get transactionCostModel(){return {serialize:()=>Uint8Array.of(1),free(){freed++;}};},free(){throw Error('borrowed parameters freed');}};
 assert.throws(()=>assertRecipeParameters({blockData:{ledgerParameters:{}}},expected),error=>error.code==='ERR_ASSERTION'&&error.message.includes('ERR_PRODUCER_TRANSACTION_COST_MODEL_API'));assert.equal(freed,1);
});
