import assert from 'node:assert/strict';
import {test} from 'node:test';
import {checkContinuation} from '../src/continuation.mjs';

const bytes = n => new Uint8Array(32).fill(n);
const frozen = {
  predecessor:bytes(1), eventId:bytes(2), custody:{bytes:bytes(3)}, orderId:bytes(4),
  program:bytes(5), asset:bytes(6), amount:200n, budget:1n,
};
test('a durable capture refuses each independent predecessor, eventId, custody and cap substitution', () => {
  checkContinuation(frozen,structuredClone(frozen));
  for (const key of Object.keys(frozen)) {
    const changed = structuredClone(frozen);
    changed[key] = typeof frozen[key] === 'bigint' ? frozen[key]+1n : key === 'custody' ? {bytes:bytes(99)} : bytes(99);
    assert.throws(() => checkContinuation(frozen,changed), /continuation/i, key);
  }
  const missing = {...frozen};
  delete missing.budget;
  assert.throws(() => checkContinuation(frozen,missing), /continuation/i);
});

test('both terminal orders route through the installed head and refuse reuse after the client restarts', async () => {
  const {createHash} = await import('node:crypto');
  const {prepareTerminal} = await import('../src/continuation.mjs');
  const digest = bytes => createHash('sha256').update(bytes).digest();
  const installed = bytes(6), address = 'staged-escrow';
  const deployment = {address,network:'local',programHash:Buffer.from(frozen.program).toString('hex'),
    verifiers:Object.fromEntries(['fund','first','enable','resume','refund'].map(k=>[k,digest(installed).toString('hex')]))};
  const contract = {operation:()=>({verifierKey:installed}),serialize:()=>bytes(1)};
  const terms = {program:frozen.program,network:digest('local')};
  const client = head => ({instance:{address,contract:{generated:{ledger:()=>head}}},ledger:{
    state:{index:()=>contract},network:'local',runtime:{ContractState:{deserialize:()=>({data:{}})}},
    prepareCall:(instance,action,args)=>({action,args})}});
  for (const [winner,terminal] of [['resume',1n],['refund',2n]]) {
    const grant = {action:winner,capture:frozen,delivery:300n,now:1201n,signature:{}};
    const pending = client({terms,current:{f2:1n},terminal:0n,pending:frozen});
    const prepared = prepareTerminal(pending.ledger,pending.instance,deployment,grant);
    assert.equal(prepared.action,winner);
    assert.deepEqual(prepared.args,winner==='resume'?[frozen,300n,1201n,{}]:[frozen,{}]);
    for (const action of ['resume','refund']) {
      const restarted = client({terms,current:{f2:2n},terminal,pending:frozen});
      assert.throws(()=>prepareTerminal(restarted.ledger,restarted.instance,deployment,{...grant,action}),/consumed/);
    }
  }
});
