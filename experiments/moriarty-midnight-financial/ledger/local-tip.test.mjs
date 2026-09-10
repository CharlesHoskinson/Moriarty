import test from 'node:test';
import assert from 'node:assert/strict';
const api=await import('./local-tip.mjs').catch(()=>({}));
const h='ab'.repeat(32),other='cd'.repeat(32);
function fixture(change={}){
 const calls=[];
 const block={height:12,hash:h,timestamp:new Date(Date.now()-5000).toISOString(),...change};
 const fetchImpl=async(url,options)=>{
  calls.push({url,...JSON.parse(options.body)});
  const {method,id}=JSON.parse(options.body);
  if(!method)return Response.json({data:{block}});
  return Response.json({jsonrpc:'2.0',id,result:method==='chain_getFinalizedHead'?'0x'+h:method==='chain_getHeader'?{number:'0xc'}:'0x'+h});
 };
 return {calls,block,options:{node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',deadlineMs:Date.now()+1000,fetchImpl}};
}
test('the observed 39-hour-old tip cannot reach wallet balancing',async()=>{
 assert.equal(typeof api.guardLocalWallet,'function');
 const f=fixture({timestamp:new Date(Date.now()-39*3600000).toISOString()});let balanced=0;
 const guarded=api.guardLocalWallet({wallet:{balanceUnboundTransaction(){balanced++;}},...f.options,deadlineMs:Date.now()+60});
 await assert.rejects(guarded.balanceUnboundTransaction(),/TIP/);assert.equal(balanced,0);
});
test('fresh indexed block is checked against finalized node chain before each balance',async()=>{
 assert.equal(typeof api.guardLocalWallet,'function');const f=fixture();let balanced=0,observed=[];const recipe={type:'UNBOUND_TRANSACTION',baseTransaction:{intents:new Map([[1,{dustActions:{ctime:new Date(),spends:[{}]}}]])}};
 const wallet={async balanceUnboundTransaction(...args){assert.equal(this,wallet);balanced++;assert.deepEqual(args,['tx','keys']);return recipe;},stop(){assert.equal(this,wallet);return 'stopped';}};
 const guarded=api.guardLocalWallet({wallet,...f.options,onTip:x=>observed.push(x)});
 assert.equal(await guarded.balanceUnboundTransaction('tx','keys'),recipe);assert.equal(guarded.stop(),'stopped');
 f.block.timestamp=new Date(Date.now()-39*3600000).toISOString();
 await assert.rejects(guarded.balanceUnboundTransaction(),/TIP/);assert.equal(balanced,1);assert.equal(observed.length,1);
 assert.equal(observed[0].hash,h);assert.equal(observed[0].height,12);assert.ok(f.calls.some(x=>x.method==='chain_getBlockHash'&&x.params[0]===12));
});
test('future, mismatched, far-behind and invalid block observations fail closed',async()=>{
 assert.equal(typeof api.waitForLocalTip,'function');
 for(const change of [{timestamp:new Date(Date.now()+60000).toISOString()},{hash:other},{height:999},{timestamp:null},{height:'12'}]){
  const f=fixture(change);await assert.rejects(api.waitForLocalTip({...f.options,deadlineMs:Date.now()+35}),/TIP/);
 }
});
test('readiness retries only reads until the indexed tip becomes fresh',async()=>{
 assert.equal(typeof api.waitForLocalTip,'function');const f=fixture();let reads=0;
 const original=f.options.fetchImpl;
 const fetchImpl=(url,o)=>{if(!JSON.parse(o.body).method){reads++;f.block.timestamp=new Date(Date.now()-(reads===1?39*3600000:5000)).toISOString();}return original(url,o);};
 const result=await api.waitForLocalTip({...f.options,fetchImpl,deadlineMs:Date.now()+2500});
 assert.equal(result.hash,h);assert.equal(reads,2);
});
test('hung and oversized indexer responses stop within the shared deadline',async()=>{
 assert.equal(typeof api.waitForLocalTip,'function');const f=fixture();let signal;
 await assert.rejects(api.waitForLocalTip({...f.options,deadlineMs:Date.now()+40,fetchImpl:(_u,o)=>{signal=o.signal;return new Promise(()=>{});}}),/TIP/);
 assert.equal(signal.aborted,true);
 await assert.rejects(api.waitForLocalTip({...f.options,deadlineMs:Date.now()+40,fetchImpl:async()=>new Response('x'.repeat(65537))}),/TIP/);
});
test('local readiness rejects remote endpoints before fetching',async()=>{
 assert.equal(typeof api.waitForLocalTip,'function');let calls=0;const f=fixture();
 await assert.rejects(api.waitForLocalTip({...f.options,indexer:'https://example.com/graphql',fetchImpl:()=>calls++}),/TIP/);assert.equal(calls,0);
});

test('SDK second-query regression is rejected before returning a recipe for signing',async()=>{
 assert.equal(typeof api.guardLocalWallet,'function');const f=fixture();let balanced=0;
 const wallet={async balanceUnboundTransaction(){balanced++;return {type:'UNBOUND_TRANSACTION',baseTransaction:{intents:new Map([[1,{dustActions:{ctime:new Date(Date.now()-39*3600000),spends:[{}]}}]])}};}};
 await assert.rejects(api.guardLocalWallet({wallet,...f.options}).balanceUnboundTransaction(),/TIP_DUST_TIME/);assert.equal(balanced,1);
});
test('actual rejected native transaction cannot reach another submission',async()=>{
 assert.equal(typeof api.guardLocalWallet,'function');
 const {readFileSync}=await import('node:fs'),{pathToFileURL}=await import('node:url'),{join}=await import('node:path');
 const {PINNED_NM}=await import('./providers.mjs');const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
 const tx=ledger.Transaction.deserialize('signature','proof','binding',readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-execution-02/run-public/public-transactions/09824832f0174603938ee7f51b368019ef1def3774adff746d2caa847f790864.bin',import.meta.url)));
 let submitted=0;const f=fixture();const wallet={submitTransaction(){submitted++;}};
 await assert.rejects(api.guardLocalWallet({wallet,...f.options}).submitTransaction(tx),/TIP_DUST_TIME/);assert.equal(submitted,0);
});
test('epoch millisecond timestamps use SDK units without accepting second-scale values',async()=>{
 const f=fixture({timestamp:Date.now()-5000});assert.equal((await api.waitForLocalTip(f.options)).hash,h);
 f.block.timestamp=Math.floor(Date.now()/1000);
 await assert.rejects(api.waitForLocalTip({...f.options,deadlineMs:Date.now()+35}),/TIP/);
});
test('slow proof and future native DUST time stop submission; recent native time passes',async()=>{
 const f=fixture();let submissions=0;const wallet={submitTransaction(tx){submissions++;assert.equal(this,wallet);return tx;}};
 const guarded=api.guardLocalWallet({wallet,...f.options});const dust={ctime:new Date(Date.now()-60001)};const tx={intents:new Map([[1,{dustActions:dust}]])};
 await assert.rejects(guarded.submitTransaction(tx),/TIP_DUST_TIME/);
 dust.ctime=new Date(Date.now()+10000);await assert.rejects(guarded.submitTransaction(tx),/TIP_DUST_TIME/);
 dust.ctime=new Date(Date.now()-1000);assert.equal(await guarded.submitTransaction(tx),tx);assert.equal(submissions,1);
});
test('readiness retention crossing the deadline cannot start wallet balancing',async()=>{
 const f=fixture();let balanced=0;const original=Date.now,now=original();
 const wallet={balanceUnboundTransaction(){balanced++;return {type:'UNBOUND_TRANSACTION',baseTransaction:{intents:new Map()}};}};
 try{
  Date.now=()=>now;
  const guarded=api.guardLocalWallet({wallet,...f.options,deadlineMs:now+1000,onTip:()=>{Date.now=()=>now+1001;}});
  await assert.rejects(guarded.balanceUnboundTransaction(),/TIP_DEADLINE/);assert.equal(balanced,0);
 }finally{Date.now=original;}
});
