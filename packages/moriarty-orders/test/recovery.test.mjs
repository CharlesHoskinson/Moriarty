import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {test} from 'node:test';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';
import {checkRecoveryReadback} from '../src/recovery.mjs';

const mapping = file => lowerCompact(readFileSync(new URL('../orders/' + file, import.meta.url)),
  readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json', import.meta.url)));

// Independent native readbacks after the winning transaction. Each vector records
// owner A, other owner A, executor A, recipient B, first custody A, second custody A,
// tracked remainder, prior A spent, A refunded, closed flag, revision.
const expected = JSON.parse(readFileSync(new URL('./fixtures/recovery.json', import.meta.url)));
test('partial recovery uses a separate bounded program and leaves the one-fill O1 unchanged', () => {
  assert.notEqual(mapping('partial-recovery.mori').metadata.programHash, mapping('o1.mori').metadata.programHash);
  assert.deepEqual(mapping('partial-recovery.mori').metadata.textTable, mapping('o1.mori').metadata.textTable);
});
test('the compiled recovery policy separates fills, cancellations, surplus, revocation and late recovery', async t => {
  const {mkdtempSync,rmSync} = await import('node:fs');
  const {tmpdir} = await import('node:os');
  const {join} = await import('node:path');
  const {pathToFileURL} = await import('node:url');
  const {buildAcceptance} = await import('../src/build-acceptance.mjs');
  const {signOrder} = await import('../src/authorization.mjs');
  const out = mkdtempSync(join(tmpdir(),'moriarty-recovery-unit-'));
  t.after(()=>rmSync(out,{recursive:true,force:true}));
  const build = buildAcceptance(out,{skipZk:true,policy:'recovery'});
  const rt = await import(pathToFileURL(build.runtimePath));
  const compiled = await import(pathToFileURL(join(out,'contract/index.js')));
  const bytes = n=>new Uint8Array(32).fill(n), hex = b=>Buffer.from(b).toString('hex');
  const contract = new compiled.Contract({});
  // Two orders with different owners, accounts and contract addresses.
  async function open(account,secret) {
    const address = rt.sampleContractAddress();
    const terms = {account:{bytes:bytes(account)},recipient:{bytes:bytes(2)},executor:{bytes:bytes(3)},orderId:bytes(account),
      network:bytes(5),assetA:bytes(6),assetB:bytes(7),program:Buffer.from(build.programHash,'hex'),epoch:0n,input:100n,minimum:80n,fee:0n,expiry:1400n};
    const owner = rt.ecMulGenerator(secret);
    const deployed = (await contract.initialState(rt.createConstructorContext({},'00'.repeat(32)),terms,owner)).currentContractState;
    const order = {address,terms,secret,owner};
    order.ctx = (id,state,time,custody)=>{
      const c = rt.createCircuitContext({circuitId:id,contractAddress:address,coinPublicKeyOrZswapState:'00'.repeat(32),contractState:state,privateState:{},time});
      const q = c.callContext.currentQueryContext;
      q.block = {...q.block,secondsSinceEpoch:BigInt(time),lastBlockTime:BigInt(time),secondsSinceEpochErr:0,
        balance:new Map([[{tag:'unshielded',raw:hex(terms.assetA)},custody]])};
      return c;
    };
    order.sign = (action,revision,sk=secret)=>signOrder(rt,compiled.pureCircuits,{terms,contract:{bytes:rt.encodeContractAddress(address)},
      owner:rt.ecMulGenerator(sk),action,revision},sk);
    order.funded = after(await contract.provableCircuits.fund(order.ctx('fund',deployed,900,0n))).state;
    return order;
  }
  const after = r=>r.context.callContext.currentQueryContext;
  const ledger = state=>compiled.ledger(state.state ?? state);
  const paid = r=>[...after(r).effects.claimedUnshieldedSpends].map(([[tok,to],v])=>[Buffer.from(tok.raw,'hex')[0],Buffer.from(to.address,'hex')[0],Number(v)]).sort();
  const first = await open(1,11n), second = await open(4,12n);
  const fill = (o,state,time,revision=0n)=>contract.provableCircuits.fill(o.ctx('fill',state,time,100n),revision,40n,80n,0n,BigInt(time),o.sign(0n,revision));
  const cancel = (o,state,time,custody,revision=0n,sk)=>contract.provableCircuits.cancel(o.ctx('cancel',state,time,custody),revision,o.sign(1n,revision,sk));

  // Fill first: 40 A to the executor and 80 B to the recipient; the stale cancel loses; the owner recovers 60 A.
  const partial = await fill(first,first.funded,1000);
  assert.deepEqual(paid(partial),[[6,3,40],[7,2,80]]);
  const partialState = after(partial).state;
  await assert.rejects(()=>cancel(first,partialState,1001,60n,0n),/REVISION/);
  assert.deepEqual(paid(await cancel(first,partialState,1001,60n,1n)),[[6,1,60]]);
  // Cancel first: all 100 A return to the owner; the stale fill loses.
  const cancelled = await cancel(first,first.funded,1000,100n);
  assert.deepEqual(paid(cancelled),[[6,1,100]]);
  const revoked = after(cancelled).state;
  await assert.rejects(()=>fill(first,revoked,1001),/REVISION|CLOSED/);
  // Fresh funding cannot reactivate revoked authority; surplus is accepted but never refunded or filled.
  await assert.rejects(()=>contract.provableCircuits.fund(first.ctx('fund',revoked,1002,0n)),/ALREADY_FUNDED/);
  const surplus = after(await contract.provableCircuits.depositSurplus(first.ctx('depositSurplus',revoked,1002,0n),1n)).state;
  assert.equal(ledger(surplus).kernelState.f3,1n);
  await assert.rejects(()=>fill(first,surplus,1003,1n),/CLOSED/);
  // An unassigned deposit before cancellation does not raise the refund above the order's own remainder.
  const topped = after(await contract.provableCircuits.depositSurplus(first.ctx('depositSurplus',first.funded,1000,100n),7n)).state;
  assert.deepEqual(paid(await cancel(first,topped,1001,107n)),[[6,1,100]]);
  // The other order is isolated: its owner cannot cancel the first order, and its own state is untouched.
  await assert.rejects(()=>cancel(first,first.funded,1000,100n,0n,second.secret),/SIGNATURE/);
  assert.equal(ledger(second.funded).kernelState.f0,100n);
  assert.deepEqual(paid(await cancel(second,second.funded,1000,100n)),[[6,4,100]]);
  // Fills stop at expiry; owner recovery stays open long after the 2000-second horizon.
  await assert.rejects(()=>fill(first,first.funded,1400),/EXPIRED/);
  assert.deepEqual(paid(await cancel(first,first.funded,5000,100n)),[[6,1,100]]);
  assert.deepEqual(paid(await cancel(first,partialState,5000,60n,1n)),[[6,1,60]]);
});

test('refund reconciliation distinguishes spent principal, unassigned surplus and the other order', () => {
  for (const [i, value] of [[0, 100], [2, 0], [3, 0], [4, 0], [5, 40], [6, 60], [8, 100], [9, 0], [10, 1]]) {
    const changed = [...expected.partialRefund];
    changed[i] = value;
    assert.throws(() => checkRecoveryReadback(changed, expected.partialRefund));
  }
  // The frozen vectors agree with the trade arithmetic: 100 A funded with 7 A of unassigned surplus,
  // 40 A filled for 80 B, 60 A refunded to the owner, and the surplus left in custody.
  const [ownerA,,executorA,recipientB,custodyA,,remaining,spent,refunded,closed,revision] = expected.partialRefund;
  assert.deepEqual([executorA,recipientB,remaining,spent,refunded,closed,revision],[40,80,0,40,60,1,2]);
  assert.equal(ownerA - expected.funded[0], 60);
  assert.equal(expected.funded[4] - custodyA, 100);
});
