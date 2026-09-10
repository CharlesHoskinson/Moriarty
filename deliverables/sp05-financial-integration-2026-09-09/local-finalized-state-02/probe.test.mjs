import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const api=await import('./probe.mjs').catch(e=>{if(e.code==='ERR_MODULE_NOT_FOUND')return {};throw e;});
const genesis='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';
const loan='ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',swap='8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261';
const states={
 [loan]:readFileSync(new URL('../local-recovery-03/indexed-initialize-state.bin',import.meta.url)).toString('hex'),
 [swap]:readFileSync(new URL('../swap-initialize-diagnosis-01/indexed-initialize-state.bin',import.meta.url)).toString('hex'),
};
const hash='0x'+'ab'.repeat(32);
function transport(change=()=>undefined){const calls=[];return {calls,fetchImpl:async(url,options)=>{
 assert.equal(url,'http://127.0.0.1:19944/');assert.equal(options.redirect,'error');const body=JSON.parse(options.body);calls.push(body);
 const changed=await change(body,calls);if(changed!==undefined)return changed;
 let result;if(body.method==='chain_getFinalizedHead'){assert.deepEqual(body.params,[]);result=hash;}
 else if(body.method==='chain_getHeader'){assert.deepEqual(body.params,[hash]);result={number:'0x5000'};}
 else if(body.method==='chain_getBlockHash'){assert.ok([0,20480].includes(body.params[0]));result=body.params[0]===0?'0x'+genesis:hash;}
 else if(body.method==='midnight_contractState'){assert.equal(body.params[1],hash);assert.ok(Object.hasOwn(states,body.params[0]));result=states[body.params[0]];}
 else throw Error('UNEXPECTED_RPC_METHOD');
 return Response.json({jsonrpc:'2.0',id:body.id,result});
 }};}
function run(options){assert.equal(typeof api.probeFinalizedLoanAndSwap,'function','missing bounded node probe');return api.probeFinalizedLoanAndSwap(options);}
test('controlled fetch drives actual RPC, native snapshot and both retained generated decoders',async t=>{
 t.mock.method(globalThis,'fetch',()=>{throw Error('LIVE_FETCH_FORBIDDEN');});
 const f=transport();const out=await run({fetchImpl:f.fetchImpl,deadlineMs:Date.now()+10000});
 assert.equal(out.status,'OBSERVED');assert.equal(out.methodSupport,'AVAILABLE');assert.equal(out.genesis,genesis);
 assert.deepEqual(out.snapshots.map(s=>s.contractAddress),[loan,swap]);assert.equal(out.snapshots[0].serializedStateHex,states[loan]);assert.equal(out.snapshots[1].serializedStateHex,states[swap]);
 assert.deepEqual(out.snapshots[1].balances,{'5475695f8ccb85c05055a4ffef06bbbff208c58b07cf5c61052d25143729d166':'1000000','e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4':'2000000'});
 assert.ok(out.snapshots.every(s=>Object.keys(s.state).length>10));assert.equal(out.loadersClosed,2);assert.equal(f.calls.length,12);
 assert.ok(Buffer.byteLength(api.serializePublicResult(out))<=65536);
});
test('wrong genesis stops before any contract-state request',async()=>{
 const f=transport(b=>b.method==='chain_getBlockHash'&&b.params[0]===0?Response.json({jsonrpc:'2.0',id:b.id,result:'0x'+'00'.repeat(32)}):undefined);
 const out=await run({fetchImpl:f.fetchImpl,deadlineMs:Date.now()+10000});assert.equal(out.status,'FAILED');assert.equal(out.failure.code,'NODE_GENESIS_MISMATCH');assert.equal(f.calls.length,2);assert.equal(out.loadersClosed,0);
});
test('method error is UNKNOWN support, one attempt, no swap or fallback',async()=>{
 const f=transport(b=>b.method==='midnight_contractState'?Response.json({jsonrpc:'2.0',id:b.id,error:{code:-32601,message:'Method not found'}}):undefined);
 const out=await run({fetchImpl:f.fetchImpl,deadlineMs:Date.now()+10000});assert.equal(out.status,'FAILED');assert.equal(out.methodSupport,'UNKNOWN');assert.equal(out.failure.code,'RPC_RESPONSE');assert.equal(out.loadersClosed,1);assert.equal(f.calls.filter(b=>b.method==='midnight_contractState').length,1);
});
test('malformed successful state response is fatal and is never retried',async()=>{
 const f=transport(b=>b.method==='midnight_contractState'?Response.json({jsonrpc:'2.0',id:b.id,result:'010203'}):undefined);
 const out=await run({fetchImpl:f.fetchImpl,deadlineMs:Date.now()+10000});assert.equal(out.failure.code,'FINALIZED_STATE_NATIVE');assert.equal(f.calls.filter(b=>b.method==='midnight_contractState').length,1);assert.equal(out.loadersClosed,1);
});
test('only connection-refused readiness is polled; after readiness no operation retry',async t=>{
 const started=Date.now();let clock=started;t.mock.method(Date,'now',()=>clock);
 const f=transport((_b,calls)=>{if(calls.length===1){clock=started+60000;throw new TypeError('fetch failed',{cause:Object.assign(Error('refused'),{code:'ECONNREFUSED'})});}});
 const out=await run({fetchImpl:f.fetchImpl,deadlineMs:started+89000});assert.equal(out.status,'FAILED');assert.equal(out.failure.code,'NODE_READINESS_DEADLINE');assert.equal(f.calls.length,1);assert.equal(out.loadersClosed,0);
});
test('connection refusal before readiness can recover without retrying either state operation',async()=>{
 const f=transport((_b,calls)=>{if(calls.length===1)throw new TypeError('fetch failed',{cause:Object.assign(Error('refused'),{code:'ECONNREFUSED'})});});
 const out=await run({fetchImpl:f.fetchImpl,deadlineMs:Date.now()+10000});assert.equal(out.status,'OBSERVED');assert.equal(f.calls.length,13);assert.equal(f.calls.filter(b=>b.method==='midnight_contractState').length,2);
});
test('actual TCP close error is retried only during readiness',async()=>{
 const {createServer}=await import('node:net');const server=createServer(s=>s.once('data',()=>s.destroy()));
 await new Promise(r=>server.listen(0,'127.0.0.1',r));let actual;
 try{await fetch(`http://127.0.0.1:${server.address().port}/`,{method:'POST',body:'{}',signal:AbortSignal.timeout(3000)});}catch(e){actual=e;}finally{await new Promise(r=>server.close(r));}
 assert.equal(actual?.cause?.code,'UND_ERR_SOCKET');
 const f=transport((_b,calls)=>{if(calls.length===1)throw actual;});
 const out=await run({fetchImpl:f.fetchImpl,deadlineMs:Date.now()+10000});assert.equal(out.status,'OBSERVED');assert.equal(f.calls.length,13);
 const after=transport(b=>{if(b.method==='chain_getBlockHash')throw actual;});
 const failed=await run({fetchImpl:after.fetchImpl,deadlineMs:Date.now()+10000});assert.equal(after.calls.length,2);assert.equal(failed.failure.causeCode,'UND_ERR_SOCKET');
});
test('unrecognized TypeError, HTTP and RPC errors are fatal before readiness',async()=>{
 for(const error of [new TypeError('fetch failed',{cause:{code:'EACCES'}}),new TypeError('arbitrary',{cause:{code:'UND_ERR_SOCKET'}}),Response.json({error:{code:-32601}}, {status:200}),new Response('',{status:503})]){
  const f=transport(()=>{if(error instanceof Error)throw error;return error;});const out=await run({fetchImpl:f.fetchImpl,deadlineMs:Date.now()+10000});assert.equal(out.status,'FAILED');assert.equal(f.calls.length,1);assert.equal(out.loadersClosed,0);
 }
});
