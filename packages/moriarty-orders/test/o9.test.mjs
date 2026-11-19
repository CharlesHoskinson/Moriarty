import assert from 'node:assert/strict';
import {test} from 'node:test';
import {readFileSync} from 'node:fs';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';
import {checkO9Balances} from '../src/continuation.mjs';

// Each row is [A,B,C] in native unshielded quanta. The executor starts with 1M B and 1M C.
const o9Balances = JSON.parse(readFileSync(new URL('./fixtures/o9.json', import.meta.url)));
test('every O9 readback binds each owner and asset after the first trade', () => {
  for (const expected of Object.values(o9Balances)) {
    checkO9Balances(expected, expected);
    const substituted = structuredClone(expected);
    substituted.owner[1]++;
    substituted.custody[1]--;
    assert.throws(() => checkO9Balances(substituted, expected));
  }
});

test('bounded-atomic O9 lowers all three financial actions into separate transitions', () => {
  const source = readFileSync(new URL('../orders/o9.mori', import.meta.url));
  const bounds = readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json', import.meta.url));
  const {metadata} = lowerCompact(source,bounds);
  assert.deepEqual(metadata.actions.map(a => [a.name,a.circuit,a.effects.length]), [
    ['first','transition0',2], ['resume','transition1',2], ['refund','transition2',1],
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
});
