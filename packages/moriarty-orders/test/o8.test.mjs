import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {fundExchangeInputs,prepareExchange} from '../src/atomic-exchange.mjs';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const fixture=JSON.parse(readFileSync(new URL('./fixtures/o8.json',import.meta.url)));
const directory=new URL('../../../deliverables/competitor-report/IV.3/',import.meta.url);
const read=name=>JSON.parse(readFileSync(new URL(name,directory)));
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');

test('the O8 source lowers to three named transfers and the archived kernel',()=>{
  const mapping=lowerCompact(readFileSync(new URL('../orders/o8.mori',import.meta.url)),readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json',import.meta.url)));
  assert.equal(mapping.metadata.actions[0].effects.length,3);
  assert.deepEqual(mapping.metadata.textTable.map(x=>x.text),['ASSET_A','ASSET_B','ASSET_C',...fixture.owners]);
  assert.equal(mapping.metadata.programHash,'388882b7f449190fe54528955b3934f80d6a8ee885faff800e475adf7c61ce07');
  assert.equal(digest(mapping.source),read('result.json').build.kernelSha256);
});

test('three independently funded owner wallets exchange their exact approved legs in one transaction',()=>{
  const r=read('result.json');
  assert.deepEqual(r.funded,fixture.funded);
  assert.deepEqual(r.settled,fixture.settled);
  assert.deepEqual(r.inputAmounts,fixture.amounts);
  assert.equal(new Set(r.fundingWallets).size,3);
  assert.equal(new Set(r.authorizations.map(g=>g.ownerKey.x)).size,3);
  assert.equal(r.outcome,'success');
  assert.equal(r.orderRevision,1);
  const matched=read('receipts.json').filter(row=>row.label==='atomic-three-owner-settlement');
  assert.equal(matched.length,1);
  assert.equal(matched[0].transactionHash,r.settlementTx);
});

test('all seven missing, changed and underfunded cases move zero assets',()=>{
  const r=read('result.json');
  assert.deepEqual(r.refusals.map(x=>x.label).sort(),[...fixture.refusals].sort());
  for(const row of r.refusals){
    assert.equal(row.before,row.after);
    assert.deepEqual(row.balances,row.label==='unfunded third leg'?fixture.unfunded:fixture.funded);
  }
});

test('the wallet adapter rejects an incomplete funding group and a substituted contract before signing',()=>{
  assert.throws(()=>fundExchangeInputs(undefined,undefined,undefined,[]),/three distinct wallets/);
  assert.throws(()=>prepareExchange({network:'undeployed'},{address:'substituted'},{address:'pinned',network:'undeployed'},{}),/deployment mismatch/);
});
