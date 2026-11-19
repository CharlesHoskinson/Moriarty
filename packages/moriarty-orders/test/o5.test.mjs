import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const source = new URL('../orders/o5.mori', import.meta.url);
const reportDir = new URL('../../../deliverables/competitor-report/IV.2/', import.meta.url);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const report = () => JSON.parse(readFileSync(new URL('result.json',reportDir)));

test('O5 binds the compiled source, feed and financial observation slots', () => {
  const mapping = lowerCompact(readFileSync(source),readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json',import.meta.url)));
  assert.deepEqual(mapping.metadata.observationFields.map(row=>row.name),['now','price_ab','sample_age']);
  assert.equal(mapping.metadata.programHash,'d218b0975575defffb1a8c3888aba4a26c3863636a6d6e7c87098f8e41ffb1b8');
  assert.equal(report().build.kernelSha256,hash(mapping.source));
});

test('the funded O5 native fixture delivers 100 A and at least 200 B at both inclusive age boundaries', () => {
  const evidence = report();
  assert.deepEqual(evidence.fills.map(row=>[row.age,row.delivery]),[[0,200],[30,201]]);
  for (const row of evidence.fills) {
    assert.deepEqual(row.balances,{owner:[0,0],executor:[100,1000000-row.delivery],recipient:[0,row.delivery],custody:[0,0]});
    assert.deepEqual(row.order,{revision:1,closed:1,remaining:127});
  }
});

test('refusals preserve the order and an earlier funded fill becomes invalid at inclusion', () => {
  const evidence = report();
  assert.deepEqual(evidence.refusals.map(row=>row.label).sort(),[
    'expiry','future','missing','stale','substitution','threshold equality','wrong controller','wrong owner','wrong provider'
  ].sort());
  assert.deepEqual(evidence.changed.before,evidence.changed.after);
  assert.deepEqual(evidence.changed.after,{owner:[0,0],executor:[0,1000000],recipient:[0,0],custody:[100,0]});
  const raw = readFileSync(new URL('receipts.json',reportDir));
  assert.equal(hash(raw),evidence.receiptsSha256);
  const receipts = JSON.parse(raw);
  const valid = receipts.find(row=>row.label==='baseline-prepared-fill');
  const changed = receipts.find(row=>row.label==='unchanged-fill-after-price-change');
  assert.equal(valid.status,'success');
  assert.notEqual(changed.status,'success');
  assert.match(JSON.stringify(changed.error),/mismatch.*read/i);
  assert.equal(valid.transaction.sha256,changed.transaction.sha256);
  for (const row of receipts) {
    assert.ok(Object.values(row.strict).every(Boolean));
    for (const ref of [row.beforeState,row.afterState,row.transaction]) {
      assert.equal(hash(readFileSync(new URL(ref.file,reportDir))),ref.sha256);
    }
  }
});
