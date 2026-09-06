import test from 'node:test';
import assert from 'node:assert/strict';

const model = await import('../src/model.ts').catch(error => {
  if (error.code === 'ERR_MODULE_NOT_FOUND') return {};
  throw error;
});

test('scenario model is available', () => assert.equal(typeof model.evaluate, 'function'));

test('swap uses exact floor output and enforces the signed minimum', () => {
  assert.equal(typeof model.evaluate, 'function');
  const config = model.defaultConfig('swap');
  const result = model.evaluate(config);
  assert.equal(result.ok, true);
  assert.equal(new Map(result.metrics).get('Output B'), '19743');
  const rejected = model.evaluate({ ...config, minOut: '19744' });
  assert.equal(rejected.ok, false);
  assert.ok(rejected.diagnostics.some(d => d.code === 'SlippageExceeded'));
  assert.equal(model.evaluate({ ...config, fault: 'rounding-mutation' }).ok, false);
});

test('loan arithmetic preserves the rational and identifies it as an illustration', () => {
  assert.equal(typeof model.evaluate, 'function');
  const result = model.evaluate(model.defaultConfig('loan'));
  assert.equal(result.kind, 'illustration');
  assert.equal(new Map(result.metrics).get('Interest, exact USD'), '2480/73');
  assert.ok(result.diagnostics.some(d => d.code === 'NumericProfileUnresolved'));
  assert.ok(result.effects.some(e => /not paid/i.test(e)));
});

test('invalid, oversized and unbounded values reject rather than coerce', () => {
  assert.equal(typeof model.evaluate, 'function');
  for (const value of ['-1', '1e9', 'Infinity', '1'.repeat(1000), '', '0']) {
    assert.equal(model.evaluate({ ...model.defaultConfig('swap'), amountIn: value }).ok, false);
  }
  assert.equal(model.evaluate({ ...model.defaultConfig('loan'), days: '3661' }).ok, false);
});

test('demo workflow is ordered, has simulated evidence, and never real verification', () => {
  assert.equal(typeof model.advance, 'function');
  const config = model.defaultConfig('swap');
  let session = model.newSession();
  let consumed = [];
  for (let stage = 1; stage <= 5; stage++) {
    ({ session, consumed } = model.advance(session, config, consumed));
    assert.equal(session.stage, stage);
  }
  assert.equal(session.evidence.kind, 'SimulatedEvidence');
  assert.match(session.receipt, /simulated/);
  assert.equal(model.verifyRealProof(session.evidence).outcome, 'unavailable');
  assert.equal(model.verifyRealProof({ kind: 'VerificationCertificate' }).outcome, 'unavailable');
});

test('editing a prepared input invalidates authorization and proof state', () => {
  assert.equal(typeof model.advance, 'function');
  const config = model.defaultConfig('swap');
  const prepared = model.advance(model.newSession(), config, []);
  const changed = model.advance(prepared.session, { ...config, minOut: '1' }, []);
  assert.equal(changed.session.stage, 0);
  assert.equal(changed.session.evidence, null);
  assert.match(changed.message, /changed/i);
});

test('editing a prepared input invalidates its snapshot before reporting invalid input', () => {
  assert.equal(typeof model.advance, 'function');
  const config = model.defaultConfig('swap');
  let { session } = model.advance(model.newSession(), config, []);
  ({ session } = model.advance(session, config, []));
  assert.equal(session.stage, 2);

  const changed = model.advance(session, { ...config, amountIn: '' }, ['demo-loan@0']);
  assert.equal(changed.session.stage, 0);
  assert.equal(changed.session.statement, '');
  assert.equal(changed.session.evidence, null);
  assert.deepEqual(changed.consumed, ['demo-loan@0']);
  assert.match(changed.message, /changed/i);
});

test('proof failure and ledger conflict occur at separate demo stages', () => {
  assert.equal(typeof model.advance, 'function');
  for (const fault of ['missing-proof', 'altered-proof', 'wrong-domain', 'stale-predecessor', 'duplicate-consumption', 'cancelled']) {
    const config = { ...model.defaultConfig('swap'), fault };
    let session = model.newSession();
    let message;
    for (let n = 0; n < 5; n++) ({ session, message } = model.advance(session, config, []));
    assert.equal(session.stage, ['stale-predecessor', 'duplicate-consumption', 'cancelled'].includes(fault) ? 4 : 3);
    assert.equal(session.receipt, null);
    assert.ok(message.length > 0);
  }
});

test('repeated submission deduplicates even after restarting the demo workflow', () => {
  assert.equal(typeof model.advance, 'function');
  const config = model.defaultConfig('swap');
  let session = model.newSession(), consumed = [];
  for (let n = 0; n < 5; n++) ({ session, consumed } = model.advance(session, config, consumed));
  assert.equal(consumed.length, 1);
  session = model.newSession();
  for (let n = 0; n < 5; n++) ({ session, consumed } = model.advance(session, config, consumed));
  assert.equal(session.stage, 4);
  assert.equal(consumed.length, 1);
});

test('observation and authority scenarios reject without proposed committed effects', () => {
  assert.equal(typeof model.evaluate, 'function');
  for (const fault of ['missing-observation', 'stale-observation', 'wrong-authority']) {
    const result = model.evaluate({ ...model.defaultConfig('mandate'), fault });
    assert.equal(result.ok, false);
    assert.deepEqual(result.effects, []);
  }
});
