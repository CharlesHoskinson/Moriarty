import test from 'node:test';
import assert from 'node:assert/strict';
import {createLocalRpc} from './integrate-local.mjs';

const address='33'.repeat(32), block='0x'+'ab'.repeat(32);
function fixture(result='aabb') {
  const requests=[];
  const rpc=createLocalRpc({node:'http://127.0.0.1:9944',deadlineMs:Date.now()+1000,
    fetchImpl:async(url,options)=>{
      const body=JSON.parse(options.body);requests.push({url,options,body});
      return Response.json({jsonrpc:'2.0',id:body.id,result});
    }});
  return {rpc,requests};
}
test('local state RPC sends exact contract and explicit block in one read request',async()=>{
  const f=fixture();assert.equal(await f.rpc('midnight_contractState',[address,block]),'aabb');
  assert.equal(f.requests.length,1);
  assert.deepEqual(f.requests[0].body,{jsonrpc:'2.0',id:1,method:'midnight_contractState',params:[address,block]});
  assert.equal(f.requests[0].options.redirect,'error');assert.equal(f.requests[0].options.method,'POST');
  assert.equal(f.requests[0].options.signal.aborted,true);
});
test('local state RPC rejects absent block, malformed identity and extra arguments before fetch',async()=>{
  const f=fixture();
  for(const params of [[],[address],[address,null],[address,'latest'],[address,block,0],['0x'+address,block],[address.toUpperCase().replace('3','G'),block],['ab'.repeat(31),block],[address,'ab'.repeat(32)]]) {
    await assert.rejects(f.rpc('midnight_contractState',params),/RPC_PARAMS/);
  }
  assert.equal(f.requests.length,0);
});
test('state query capability does not admit transaction writes',async()=>{
  const f=fixture();await assert.rejects(f.rpc('author_submitExtrinsic',[address]),/RPC_METHOD/);assert.equal(f.requests.length,0);
});
test('state query retains the response size limit and performs no retry',async()=>{
  const f=fixture('ab'.repeat(32768));await assert.rejects(f.rpc('midnight_contractState',[address,block]),/RPC_BODY_LIMIT/);assert.equal(f.requests.length,1);
});
test('unsupported state method error propagates without an unpinned fallback',async()=>{
  let calls=0;
  const rpc=createLocalRpc({node:'http://127.0.0.1:9944',deadlineMs:Date.now()+1000,fetchImpl:async(_url,options)=>{
    calls++;const {id}=JSON.parse(options.body);return Response.json({jsonrpc:'2.0',id,error:{code:-32601,message:'Method not found'}});
  }});
  await assert.rejects(rpc('midnight_contractState',[address,block]),/RPC_RESPONSE/);assert.equal(calls,1);
});
test('RPC rejects a valid response completed after its absolute deadline before its timer fires',async t=>{
  const start=Date.now(),deadlineMs=start+1000;let clock=start,calls=0,signal;
  t.mock.method(Date,'now',()=>clock);
  const rpc=createLocalRpc({node:'http://127.0.0.1:9944',deadlineMs,fetchImpl:async(_url,options)=>{
    calls++;signal=options.signal;const {id}=JSON.parse(options.body);
    clock=deadlineMs+1;
    return Response.json({jsonrpc:'2.0',id,result:'aabb'});
  }});
  await assert.rejects(rpc('midnight_contractState',[address,block]),/RPC_DEADLINE/);
  assert.equal(calls,1);assert.equal(signal.aborted,true);
});
