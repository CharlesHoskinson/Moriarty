import assert from 'node:assert/strict';
import test from 'node:test';
const transport=await import('./financial-rpc.mjs').catch(()=>({}));
const node='https://rpc.preview.midnight.network',address='ab'.repeat(32),block='0x'+'cd'.repeat(32);
function fixture(response){
 assert.equal(typeof transport.createPreviewRpc,'function');
 const calls=[];const rpc=transport.createPreviewRpc({node,deadlineMs:Date.now()+1000,fetchImpl:async(url,options)=>{
  const request=JSON.parse(options.body);calls.push({url,options,request});
  return response?response(request,options):Response.json({jsonrpc:'2.0',id:request.id,result:'aabb'});
 }});return {rpc,calls};
}
test('Preview RPC sends exact pinned HTTPS endpoint and explicit read parameters',async()=>{
 const f=fixture();
 for(const [method,params] of [['chain_getFinalizedHead',[]],['chain_getHeader',[block]],['chain_getBlockHash',[0]],['midnight_contractState',[address,block]]])assert.equal(await f.rpc(method,params),'aabb');
 assert.equal(f.calls.length,4);
 f.calls.forEach((c,i)=>{assert.equal(c.url,node+'/');assert.equal(c.options.redirect,'error');assert.equal(c.options.method,'POST');assert.equal(c.request.id,i+1);assert.equal(c.options.signal.aborted,true);});
 assert.deepEqual(f.calls[3].request.params,[address,block]);
});
test('Preview RPC rejects every unapproved target before transport access',()=>{
 assert.equal(typeof transport.createPreviewRpc,'function');let calls=0;
 for(const target of ['http://rpc.preview.midnight.network',node+':443',node+':8443',node+'/other',node+'?x=1',node+'#x','https://user:pass@rpc.preview.midnight.network','https://rpc.preview.midnight.network.evil.test','https://127.0.0.1','https://RPC.PREVIEW.MIDNIGHT.NETWORK',null,{},node+'/../'])
  assert.throws(()=>transport.createPreviewRpc({node:target,deadlineMs:Date.now()+1000,fetchImpl:()=>calls++}),{message:'PREVIEW_RPC_ENDPOINT'});
 assert.equal(calls,0);
});
test('Preview RPC rejects writes and malformed anchored state parameters before fetch',async()=>{
 const f=fixture();await assert.rejects(f.rpc('author_submitExtrinsic',[]),{message:'RPC_METHOD'});
 for(const params of [[],[address],[address,null],['0x'+address,block],[address,'latest'],[address,block,1]])await assert.rejects(f.rpc('midnight_contractState',params),{message:'RPC_PARAMS'});
 assert.equal(f.calls.length,0);
});
test('Preview RPC rejects invalid deadlines without dispatch',()=>{
 assert.equal(typeof transport.createPreviewRpc,'function');
 for(const deadlineMs of [Infinity,NaN,String(Date.now()+1000),0,Date.now()-1])assert.throws(()=>transport.createPreviewRpc({node,deadlineMs,fetchImpl:()=>{throw Error('NO_FETCH');}}),{message:'INTEGRATION_DEADLINE'});
});
test('Preview RPC preserves unknown JSON-RPC failure and never retries or falls back',async()=>{
 const f=fixture(r=>Response.json({jsonrpc:'2.0',id:r.id,error:{code:-32601,message:'Method not found'}}));
 await assert.rejects(f.rpc('midnight_contractState',[address,block]),{message:'RPC_RESPONSE'});assert.equal(f.calls.length,1);
});
test('Preview RPC rejects malformed envelopes, HTTP errors and malformed JSON',async()=>{
 for(const response of [r=>Response.json({jsonrpc:'2.0',id:r.id+1,result:null}),r=>Response.json({jsonrpc:'2.0',id:r.id,result:null,error:{}}),()=>new Response('{}',{status:503}),()=>new Response('not-json')]){
  const f=fixture(response);await assert.rejects(f.rpc('chain_getFinalizedHead',[]));assert.equal(f.calls.length,1);assert.equal(f.calls[0].options.signal.aborted,true);
 }
});
test('Preview RPC rejects oversized response body and aborts',async()=>{
 const f=fixture(r=>Response.json({jsonrpc:'2.0',id:r.id,result:'a'.repeat(65536)}));
 await assert.rejects(f.rpc('chain_getFinalizedHead',[]),{message:'RPC_BODY_LIMIT'});assert.equal(f.calls.length,1);assert.equal(f.calls[0].options.signal.aborted,true);
});
test('Preview RPC rejects late fetch completion even when the timer cannot run',async t=>{
 let clock=Date.now();const stop=clock+1000;t.mock.method(Date,'now',()=>clock);
 const f=fixture(r=>{clock=stop+1;return Response.json({jsonrpc:'2.0',id:r.id,result:null});});
 await assert.rejects(f.rpc('chain_getFinalizedHead',[]),{message:'RPC_DEADLINE'});assert.equal(f.calls[0].options.signal.aborted,true);
});
test('Preview RPC rejects late body completion and aborts',async t=>{
 let clock=Date.now();const stop=clock+1000;t.mock.method(Date,'now',()=>clock);
 const f=fixture(r=>({ok:true,body:{getReader:()=>({read:async()=>{clock=stop+1;return {done:true};},releaseLock(){}})}}));
 await assert.rejects(f.rpc('chain_getFinalizedHead',[]),{message:'RPC_DEADLINE'});assert.equal(f.calls[0].options.signal.aborted,true);
});
test('Preview RPC deadline bounds a transport that never completes',async()=>{
 assert.equal(typeof transport.createPreviewRpc,'function');let signal,calls=0;
 const rpc=transport.createPreviewRpc({node,deadlineMs:Date.now()+30,fetchImpl:async(_url,o)=>{calls++;signal=o.signal;return new Promise(()=>{});}});
 await assert.rejects(rpc('chain_getFinalizedHead',[]),{message:'RPC_DEADLINE'});assert.equal(calls,1);assert.equal(signal.aborted,true);
});
