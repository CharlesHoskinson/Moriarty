import assert from 'node:assert/strict';
import {test} from 'node:test';
import {readFileSync} from 'node:fs';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';
import {checkO9Balances} from '../src/continuation.mjs';

// Each row is [A,B,C] in native unshielded quanta. The executor starts with 1M B and 1M C.
const o9Balances = JSON.parse(readFileSync(new URL('./fixtures/o9.json', import.meta.url)));
test('every frozen O9 stage follows from the funded stage by the order transfers', () => {
  // [A,B,C] per holder. Derive each stage independently and compare it with the frozen fixture.
  const move = (state, from, to, asset, amount) => {
    const next = structuredClone(state);
    next[from][asset] -= amount; next[to][asset] += amount;
    return next;
  };
  const [A,B,C] = [0,1,2];
  const waiting = move(move(o9Balances.funded,'custody','executor',A,100),'executor','custody',B,200);
  const derived = {
    waiting,
    resumed: move(move(waiting,'custody','executor',B,200),'executor','owner',C,300),
    refunded: move(waiting,'custody','owner',B,200),
    withdrawn: move(o9Balances.funded,'custody','owner',A,100),
  };
  derived.spent = move(derived.refunded,'owner','recipient',B,200);
  derived.withdrawnSpent = move(derived.withdrawn,'owner','recipient',A,100);
  for (const [stage, balances] of Object.entries(derived)) checkO9Balances(o9Balances[stage], balances);
  for (const stage of Object.values(o9Balances)) {
    for (const asset of [A,B,C]) assert.equal(Object.values(stage).reduce((n,row)=>n+row[asset],0),
      Object.values(o9Balances.funded).reduce((n,row)=>n+row[asset],0));
  }
  const substituted = structuredClone(o9Balances.refunded);
  substituted.owner[1]++; substituted.custody[1]--;
  assert.throws(() => checkO9Balances(substituted, derived.refunded));
});

test('bounded-atomic O9 lowers all four financial actions into separate transitions', () => {
  const source = readFileSync(new URL('../orders/o9.mori', import.meta.url));
  const bounds = readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json', import.meta.url));
  const {metadata} = lowerCompact(source,bounds);
  assert.deepEqual(metadata.actions.map(a => [a.name,a.circuit,a.effects.length]), [
    ['first','transition0',2], ['resume','transition1',2], ['refund','transition2',1], ['withdraw','transition3',1],
  ]);
});

test('the compiled source rejects false conditions, sub-minimum terms, expiry and second execution', async t => {
  const {mkdtempSync,rmSync} = await import('node:fs');
  const {tmpdir} = await import('node:os');
  const {join} = await import('node:path');
  const {pathToFileURL} = await import('node:url');
  const {buildAcceptance} = await import('../src/build-acceptance.mjs');
  const dir = mkdtempSync(join(tmpdir(),'moriarty-o9-kernel-'));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  buildAcceptance(dir,{skipZk:true,policy:'staged'});
  const {pureCircuits:k} = await import(pathToFileURL(join(dir,'contract/index.js')));
  const start = {f0:100n,f1:0n,f2:0n}, hints = {unused:0n};
  const first = (b,time) => k.transition0(start,{a0:3n,a1:b},{o0:time},3n,0n,hints);
  assert.throws(()=>first(199n,1199n),/first minimum/);
  assert.throws(()=>first(200n,1200n),/first expired/);
  const waiting = first(207n,1199n);
  assert.equal(waiting.effect1.v3,207n);
  const resume = (state,condition,c,time) => k.transition1(state.after,{a0:3n,a1:condition,a2:c},
    {o0:time},state.remaining,state.revision,hints);
  const refund = (state,time) => k.transition2(state.after,{a0:3n},{o0:time},state.remaining,state.revision,hints);
  assert.throws(()=>resume(waiting,0n,300n,1399n),/condition false/);
  assert.throws(()=>resume(waiting,1n,299n,1399n),/second minimum/);
  assert.throws(()=>resume(waiting,1n,300n,1400n),/second expired/);
  assert.throws(()=>refund(waiting,1399n),/not expired/);
  const sold = resume(waiting,1n,304n,1399n), recovered = refund(waiting,1400n);
  assert.deepEqual([sold.effect0.v3,sold.effect1.v3,recovered.effect0.v3],[207n,304n,207n]);
  for (const terminal of [sold,recovered]) {
    assert.deepEqual(terminal.after,{f0:0n,f1:0n,f2:2n});
    assert.throws(()=>resume(terminal,1n,300n,1399n),/consumed/);
    assert.throws(()=>refund(terminal,1400n),/consumed/);
  }
  // Owner exit when the first trade never happened: only after its deadline, and only once.
  const withdraw = (state,time,remaining=3n,revision=0n) => k.transition3(state,{a0:3n},{o0:time},remaining,revision,hints);
  assert.throws(()=>withdraw(start,1199n),/first open/);
  const exit = withdraw(start,1200n);
  assert.deepEqual([exit.effect0.v0,exit.effect0.v2,exit.effect0.v3,exit.after.f2],[0n,3n,100n,2n]);
  assert.throws(()=>k.transition0(exit.after,{a0:3n,a1:200n},{o0:1199n},exit.remaining,exit.revision,hints),/stage/);
  assert.throws(()=>withdraw(exit.after,1300n,exit.remaining,exit.revision),/consumed/);
  assert.throws(()=>withdraw(waiting.after,1300n,waiting.remaining,waiting.revision),/consumed/);
});

test('the staged custody admits one terminal action in either order, also from a restarted client state', async t => {
  const {mkdtempSync,rmSync} = await import('node:fs');
  const {tmpdir} = await import('node:os');
  const {join} = await import('node:path');
  const {pathToFileURL} = await import('node:url');
  const {buildAcceptance} = await import('../src/build-acceptance.mjs');
  const {signOrder} = await import('../src/authorization.mjs');
  const dir = mkdtempSync(join(tmpdir(),'moriarty-o9-custody-'));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  const build = buildAcceptance(dir,{skipZk:true,policy:'staged'});
  const rt = await import(pathToFileURL(build.runtimePath));
  const compiled = await import(pathToFileURL(join(dir,'contract/index.js')));
  const bytes = n=>new Uint8Array(32).fill(n), hex = b=>Buffer.from(b).toString('hex');
  const address = rt.sampleContractAddress(), secret = 123456789n, key = rt.ecMulGenerator(secret);
  const terms = {account:{bytes:bytes(1)},executor:{bytes:bytes(3)},network:bytes(5),orderId:bytes(4),program:Buffer.from(build.programHash,'hex'),
    assetA:bytes(6),assetB:bytes(7),assetC:bytes(8),eventId:bytes(9),epoch:0n};
  const contract = new compiled.Contract({});
  const ctx = (id,state,time,held)=>{
    const c = rt.createCircuitContext({circuitId:id,contractAddress:address,coinPublicKeyOrZswapState:'00'.repeat(32),contractState:state,privateState:{},time});
    const q = c.callContext.currentQueryContext;
    q.block = {...q.block,secondsSinceEpoch:BigInt(time),lastBlockTime:BigInt(time),secondsSinceEpochErr:0,
      balance:new Map(Object.entries(held).map(([asset,amount])=>[{tag:'unshielded',raw:hex(terms[asset])},amount]))};
    return c;
  };
  const state = r=>r.context.callContext.currentQueryContext.state;
  const head = s=>compiled.ledger(s.state ?? s);
  const sign = (s,action)=>signOrder(rt,compiled.pureCircuits,{terms,contract:{bytes:rt.encodeContractAddress(address)},owner:key,action,revision:head(s).revision},secret);
  const deployed = (await contract.initialState(rt.createConstructorContext({},'00'.repeat(32)),terms,key)).currentContractState;
  const funded = state(await contract.provableCircuits.fund(ctx('fund',deployed,900,{})));
  const call = (name,s,time,held,args)=>contract.provableCircuits[name](ctx(name,s,time,held),...args);
  const waiting = state(await call('first',funded,1000,{assetA:100n},[200n,1000n,sign(funded,0n)]));
  const ticket = head(waiting).pending;
  await assert.rejects(()=>call('resume',waiting,1100,{assetB:200n},[ticket,300n,1100n,sign(waiting,2n)]),/condition false/);
  const enabled = state(await call('enable',waiting,1100,{},[terms.eventId,sign(waiting,1n)]));
  const resumeOn = (s,time)=>call('resume',s,time,{assetB:200n},[ticket,300n,BigInt(time),sign(enabled,2n)]);
  const refundOn = (s,time)=>call('refund',s,time,{assetB:200n},[ticket,sign(enabled,3n)]);
  // A restarted client holds only the serialized accepted state and its earlier grant.
  const restart = s=>{
    const saved = new rt.ContractState();
    saved.data = s;
    return rt.ContractState.deserialize(saved.serialize());
  };
  const resumed = state(await resumeOn(enabled,1300));
  await assert.rejects(()=>refundOn(restart(resumed),1400),/CONTINUATION_CONSUMED/);
  await assert.rejects(()=>resumeOn(restart(resumed),1301),/CONTINUATION_CONSUMED/);
  const refunded = state(await refundOn(enabled,1400));
  // Consumption alone refuses: the replayed resume runs before the second deadline.
  await assert.rejects(()=>resumeOn(restart(refunded),1300),/CONTINUATION_CONSUMED/);
  await assert.rejects(()=>refundOn(restart(refunded),1500),/CONTINUATION_CONSUMED/);
  assert.deepEqual([head(resumed).terminal,head(refunded).terminal],[1n,2n]);
  // No first trade before 1200: the owner withdraws the 100 A once; a late first trade cannot follow.
  await assert.rejects(()=>call('withdraw',funded,1199,{assetA:100n},[sign(funded,4n)]),/FIRST_OPEN/);
  const exit = await call('withdraw',funded,1200,{assetA:100n},[sign(funded,4n)]);
  assert.deepEqual([...exit.context.callContext.currentQueryContext.effects.claimedUnshieldedSpends]
    .map(([[tok,to],v])=>[Buffer.from(tok.raw,'hex')[0],Buffer.from(to.address,'hex')[0],v]),[[6,1,100n]]);
  const withdrawn = state(exit);
  assert.equal(head(withdrawn).terminal,3n);
  await assert.rejects(()=>call('first',restart(withdrawn),1100,{assetA:100n},[200n,1100n,sign(funded,0n)]),/STAGE_FIRST/);
  await assert.rejects(()=>call('withdraw',restart(withdrawn),1300,{},[sign(withdrawn,4n)]),/STAGE_WITHDRAW/);
});
