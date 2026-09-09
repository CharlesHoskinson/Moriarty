import test from 'node:test';
import assert from 'node:assert/strict';
const module=await import('./run-local.mjs').catch(()=>({}));

for (const kind of ['loan','swap']) test(`${kind} driver calls SDK deployment and every financial stage with observation barriers`,async()=>{
  assert.equal(typeof module.runLocalFinancialCase,'function','actual financial driver must exist');
  const calls=[],address='ab'.repeat(32),txids=['deploy','initialize',...(kind==='loan'?['accrue','settle']:['swap','close'])];
  const sdk={deployContract:async(p,o)=>{calls.push('deploy');assert.equal(o.args.length,6);return {deployTxData:{public:{contractAddress:address,txId:'deploy'},private:{canary:'NEVER_PUBLISH'}}};},submitCallTx:async(p,o)=>{calls.push(o.circuitId);assert.equal(o.contractAddress,address);return {public:{txId:o.circuitId},private:{canary:'NEVER_PUBLISH'}};}};
  const providers={execute:async(label,fn)=>fn(),stop:()=>calls.push('stop'),cleanup:async()=>({walletStopped:true})};
  const settings={kind,network:'undeployed',compiledContract:{},roles:{firstSecret:new Uint8Array(32),secondSecret:new Uint8Array(32),firstAddress:'01'.repeat(32),secondAddress:'02'.repeat(32)},networkTag:'03'.repeat(32),now:()=>1700000000n};
  const out=await module.runLocalFinancialCase({...settings,sdk,providers,observe:async x=>{calls.push('observe:'+x.circuitId);return {receipt:{txId:x.txId},state:{observed:x.circuitId}};},verifyStage:(stage,x)=>{calls.push('compare:'+stage);assert.equal(x.state.observed,stage);}});
  assert.deepEqual(calls,txids.flatMap(id=>[id,'observe:'+id,'compare:'+id]));
  assert.equal(out.stages.length,4);assert.ok(!JSON.stringify(out).includes('NEVER_PUBLISH'));
});

test('driver stops immediately on observation or comparison failure and cleans up',async()=>{
  assert.equal(typeof module.runLocalFinancialCase,'function');
  let submitted=0,stopped=0,cleaned=0;
  await assert.rejects(module.runLocalFinancialCase({kind:'loan',network:'undeployed',compiledContract:{},roles:{firstSecret:new Uint8Array(32),secondSecret:new Uint8Array(32),firstAddress:'01'.repeat(32),secondAddress:'02'.repeat(32)},networkTag:'03'.repeat(32),now:()=>1700000000n,sdk:{deployContract:async()=>({deployTxData:{public:{contractAddress:'ab'.repeat(32),txId:'deploy'}}}),submitCallTx:async()=>{submitted++;}},providers:{execute:async(_,f)=>f(),stop:()=>stopped++,cleanup:async()=>{cleaned++;}},observe:async()=>{throw Error('NONCANONICAL_BLOCK');},verifyStage:()=>{}}),/NONCANONICAL_BLOCK/);
  assert.equal(submitted,0);assert.equal(stopped,1);assert.equal(cleaned,1);
});
