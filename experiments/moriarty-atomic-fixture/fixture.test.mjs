import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';

const SCHEMA = 'moriarty-current-atomic-fixture/1';
const ROOT = ['commitments', 'genesis', 'manifest', 'program', 'schemaVersion', 'scope', 'sourcePins', 'states', 'steps'];
const INTEREST = (5000000000n * 8n * 31n) / (100n * 365n);
const PRINCIPAL = 500000000n;
const PAYMENT = PRINCIPAL + INTEREST;
const RESIDUAL = 4500000000n;
const CLAIMS = [
  'bounded_profile_safety_v1', 'loan_first_period_interest_floor_v1', 'loan_first_period_principal_v1',
  'loan_first_period_settlement_exact_v1', 'atomic_intent_refinement_v1', 'bounded_history_compliance_v1',
  'bounded_atomic_transition_v1',
];

async function load(spec) {
  try { return await import(spec); } catch (error) { return {error}; }
}
async function apis() {
  const generate = await load('./generate.mjs');
  const verify = await load('./verify.mjs');
  assert.equal(typeof generate.buildFixture, 'function',
    generate.error ? `missing generate.mjs export buildFixture (${generate.error.code || generate.error.message})` : 'buildFixture is not a function');
  assert.equal(typeof verify.verifyFixture, 'function',
    verify.error ? `missing verify.mjs export verifyFixture (${verify.error.code || verify.error.message})` : 'verifyFixture is not a function');
  return {buildFixture: generate.buildFixture, verifyFixture: verify.verifyFixture};
}
const clone = value => structuredClone(value);
const reject = (verifyFixture, fixture) => assert.throws(() => verifyFixture(fixture));
const amountOf = (values, name) => values.find(item => item.name === name).value.value;
const domainOf = kind => kind === 'SIGNED' ? 'MORIARTY-OUTCOME-bounded-atomic/1' : `MORIARTY-${kind}-bounded-atomic/1`;
const named = (fixture, kind) => fixture.commitments.filter(item =>
  item.label === kind || String(item.label).startsWith(`${kind}-`) || String(item.label).endsWith(`-${kind}`) || item.domain === domainOf(kind));
function encode(value) {
  if (typeof value === 'string') return JSON.stringify(value);
  if (typeof value === 'boolean') return value ? 'true' : 'false';
  if (Array.isArray(value)) return `[${value.map(encode).join(',')}]`;
  if (value && typeof value === 'object') return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${encode(value[key])}`).join(',')}}`;
  throw new Error('non-canonical value');
}
function rehash(commitment) {
  const canonical = encode(commitment.preimage);
  commitment.canonicalHex = Buffer.from(canonical, 'utf8').toString('hex');
  commitment.digest = createHash('sha256').update(commitment.domain, 'utf8').update('\0').update(canonical, 'utf8').digest('hex');
}
function assertEntryHash(item) {
  const canonical = encode(item.preimage);
  assert.equal(item.canonicalHex, Buffer.from(canonical, 'utf8').toString('hex'));
  assert.equal(item.digest, createHash('sha256').update(item.domain, 'utf8').update('\0').update(canonical, 'utf8').digest('hex'));
}
function labeled(fixture, label) {
  const item = fixture.commitments.find(entry => entry.label === label);
  assert.ok(item, label);
  return item;
}
function assertCommitmentsHold(fixture) {
  const labels = new Set();
  for (const item of fixture.commitments) {
    assert.deepEqual(Object.keys(item).sort(), ['canonicalHex', 'digest', 'domain', 'label', 'preimage']);
    assert.equal(labels.has(item.label), false, item.label);
    labels.add(item.label);
    const canonical = encode(item.preimage);
    assert.equal(item.canonicalHex, Buffer.from(canonical, 'utf8').toString('hex'));
    assert.equal(item.digest, createHash('sha256').update(item.domain, 'utf8').update('\0').update(canonical, 'utf8').digest('hex'));
  }
  const link = (kind, preimage, digest) => {
    const matches = named(fixture, kind).filter(item => encode(item.preimage) === encode(preimage) && item.digest === digest);
    assert.ok(matches.length >= 1, kind);
    return matches[0];
  };
  const program = link('PROGRAM', fixture.manifest, fixture.program.programHash);
  assert.equal(program.domain, domainOf('PROGRAM'));
  const genesis = link('GENESIS', fixture.genesis.body, fixture.genesis.genesisHash);
  assert.equal(genesis.domain, domainOf('GENESIS'));
  const claims = link('CLAIMS', fixture.manifest.requiredClaims, fixture.genesis.body.requiredClaimRoot);
  assert.equal(claims.domain, domainOf('CLAIMS'));
  fixture.states.forEach(state => {
    const item = link('STATE', state.body, state.stateHash);
    assert.equal(item.domain, domainOf('STATE'));
    assert.equal(item.preimage.stateHash, undefined);
  });
  fixture.steps.forEach(step => {
    const action = link('ACTION', step.input.action, step.result.candidate.body.actionHash);
    assert.equal(action.domain, domainOf('ACTION'));
    const observations = link('OBSERVATIONS', step.input.observations, step.result.candidate.body.observationsHash);
    assert.equal(observations.domain, domainOf('OBSERVATIONS'));
    const authority = link('AUTHORITY', step.input.authority, step.result.context.authorityDigest);
    assert.equal(authority.domain, domainOf('AUTHORITY'));
    const signed = link('SIGNED', step.input.authority.statement, step.result.candidate.body.authorityConsumption.statementDigest);
    assert.equal(signed.domain, step.input.authority.domain);
    const trace = link('TRACE', step.result.candidate.body, step.result.candidate.traceHash);
    assert.equal(trace.domain, domainOf('TRACE'));
    assert.equal(trace.preimage.traceHash, undefined);
    assert.equal(step.result.context.traceHash, step.result.candidate.traceHash);
    const proof = link('PROOF-CONTEXT', step.result.context, named(fixture, 'PROOF-CONTEXT').find(item => encode(item.preimage) === encode(step.result.context)).digest);
    assert.equal(proof.domain, domainOf('PROOF-CONTEXT'));
    assert.equal(proof.preimage.proofContextHash, undefined);
    assert.equal(proof.preimage.traceHash, step.result.candidate.traceHash);
  });
}

test('buildFixture and verifyFixture exports exist', async () => { await apis(); });

test('valid fixture is deterministic, simulation-only, and has three states plus two steps', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const first = buildFixture();
  assert.equal(JSON.stringify(first), JSON.stringify(buildFixture()));
  assert.equal(verifyFixture(first), true);
  assert.equal(first.schemaVersion, SCHEMA);
  assert.equal(first.scope, 'simulation-only');
  assert.deepEqual(Object.keys(first).sort(), ROOT);
  assert.equal(first.states.length, 3);
  assert.equal(first.steps.length, 2);
  assert.equal(first.steps[0].input.action.name, 'accrue');
  assert.equal(first.steps[1].input.action.name, 'settle');
  for (const step of first.steps) {
    assert.equal(step.result.kind, 'Simulation');
    assert.equal(step.result.candidate.body.outcome, 'Complete');
    assert.ok(named(first, 'SIGNED').some(item => item.domain === step.input.authority.domain));
  }
  assert.equal(named(first, 'PROGRAM').length, 1);
  assert.equal(named(first, 'GENESIS').length, 1);
  assert.equal(named(first, 'CLAIMS').length, 1);
  assert.equal(named(first, 'STATE').length, 3);
  for (const kind of ['ACTION', 'OBSERVATIONS', 'AUTHORITY', 'SIGNED', 'TRACE', 'PROOF-CONTEXT']) assert.equal(named(first, kind).length, 2, kind);
  for (const item of first.commitments) {
    assert.deepEqual(Object.keys(item).sort(), ['canonicalHex', 'digest', 'domain', 'label', 'preimage']);
    if (item.domain === domainOf('STATE')) assert.equal(item.preimage.stateHash, undefined);
    if (item.domain === domainOf('TRACE')) assert.equal(item.preimage.traceHash, undefined);
    if (item.domain === domainOf('PROOF-CONTEXT')) {
      assert.equal(item.preimage.proofContextHash, undefined);
      assert.match(item.preimage.traceHash, /^[0-9a-f]{64}$/);
    }
  }
  for (const step of first.steps) {
    assert.equal(step.result.context.traceHash, step.result.candidate.traceHash);
    const proof = named(first, 'PROOF-CONTEXT').find(item => item.preimage && item.preimage.traceHash === step.result.candidate.traceHash);
    assert.ok(proof, 'PROOF-CONTEXT binds candidate.traceHash');
    assert.equal(proof.preimage.traceHash, step.result.candidate.traceHash);
  }
});

test('fixture matches independent accrual, payment, residual notional, and Settled tombstones', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = buildFixture();
  assert.equal(verifyFixture(fixture), true);
  assert.equal(INTEREST, 33972602n);
  assert.equal(PAYMENT, 533972602n);
  const [initial, accrued, settled] = fixture.states;
  assert.equal(initial.body.revision, '0');
  assert.equal(initial.body.remaining, '2');
  assert.equal(accrued.body.revision, '1');
  assert.equal(accrued.body.remaining, '1');
  assert.equal(amountOf(accrued.body.values, 'principal_due'), String(PRINCIPAL));
  assert.equal(amountOf(accrued.body.values, 'interest_due'), String(INTEREST));
  assert.deepEqual(accrued.body.obligations.map(item => [item.dueId, item.amount.value, item.status]), [
    ['lam01:period1:PR', String(PRINCIPAL), 'Outstanding'], ['lam01:period1:IP', String(INTEREST), 'Outstanding'],
  ]);
  assert.equal(settled.body.remaining, '0');
  assert.equal(settled.body.revision, '2');
  assert.equal(settled.body.episodeStatus, 'Closed');
  assert.equal(settled.body.agreementStatus, 'Outstanding');
  assert.equal(settled.body.remainingNotional.amount.value, String(RESIDUAL));
  assert.equal(amountOf(settled.body.values, 'notional'), String(RESIDUAL));
  assert.equal(amountOf(settled.body.values, 'lender_cash'), String(PAYMENT));
  assert.equal(amountOf(settled.body.values, 'interest_paid'), String(INTEREST));
  assert.deepEqual(settled.body.obligations.map(item => [item.dueId, item.status]), [
    ['lam01:period1:PR', 'Settled'], ['lam01:period1:IP', 'Settled'],
  ]);
  const transfer = fixture.steps[1].result.candidate.body.effects.find(item => item.kind === 'Transfer');
  assert.equal(transfer.to, 'lender');
  assert.equal(transfer.amount.value, String(PAYMENT));
  assert.deepEqual(fixture.manifest.requiredClaims.map(item => item.claimId), CLAIMS);
});

test('rejects a changed source pin', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = clone(buildFixture());
  const pin = fixture.sourcePins.find(item => String(item.path).endsWith('loan.mori')) || fixture.sourcePins[0];
  pin.sha256 = '00'.repeat(32);
  reject(verifyFixture, fixture);
});

test('rejects a changed settlement amount or recipient', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const amount = clone(buildFixture());
  amount.steps[1].input.action.arguments.find(item => item.name === 'amount_due').value.value = '1';
  amount.steps[1].result.candidate.body.effects.find(item => item.kind === 'Transfer').amount.value = '1';
  reject(verifyFixture, amount);
  const recipient = clone(buildFixture());
  recipient.steps[1].result.candidate.body.effects.find(item => item.kind === 'Transfer').to = 'trader';
  recipient.steps[1].input.authority.statement.permittedRecipients =
    recipient.steps[1].input.authority.statement.permittedRecipients.filter(item => item !== 'lender');
  reject(verifyFixture, recipient);
});

test('rejects a missing obligation tombstone', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = clone(buildFixture());
  fixture.states[2].body.obligations.pop();
  fixture.steps[1].result.candidate.body.after.body.obligations.pop();
  reject(verifyFixture, fixture);
});

test('rejects terminal residual-debt erasure', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = clone(buildFixture());
  fixture.states[2].body.agreementStatus = 'NoOutstanding';
  fixture.states[2].body.remainingNotional = {tag: 'NotApplicable'};
  fixture.states[2].body.values.find(item => item.name === 'notional').value.value = '0';
  fixture.steps[1].result.candidate.body.after.body.agreementStatus = 'NoOutstanding';
  fixture.steps[1].result.candidate.body.after.body.remainingNotional = {tag: 'NotApplicable'};
  reject(verifyFixture, fixture);
});

test('rejects genesis, authority, or predecessor context tampering', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const genesis = clone(buildFixture()); genesis.genesis.body.domain.network = 'mainnet'; reject(verifyFixture, genesis);
  const authority = clone(buildFixture());
  authority.steps[1].input.authority.statement.beforeStateHash = '00'.repeat(32); reject(verifyFixture, authority);
  const predecessor = clone(buildFixture());
  predecessor.steps[1].input.authority.statement.predecessors = ['ff'.repeat(32)];
  predecessor.steps[1].result.context.predecessors = ['ff'.repeat(32)];
  reject(verifyFixture, predecessor);
});

test('rejects a missing or duplicate commitment', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const missing = clone(buildFixture()); missing.commitments = missing.commitments.slice(1); reject(verifyFixture, missing);
  const duplicate = clone(buildFixture());
  duplicate.commitments = [...duplicate.commitments, clone(duplicate.commitments[0])];
  reject(verifyFixture, duplicate);
});

test('rejects canonicalHex, domain, or digest tampering', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const canonical = clone(buildFixture()); canonical.commitments[0].canonicalHex = '00'; reject(verifyFixture, canonical);
  const domain = clone(buildFixture()); domain.commitments[0].domain = 'MORIARTY-PROGRAM-bounded-atomic/0'; reject(verifyFixture, domain);
  const digest = clone(buildFixture()); digest.commitments[0].digest = 'ff'.repeat(32); reject(verifyFixture, digest);
});

test('rejects extra or missing root, step, or result fields', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const extraRoot = clone(buildFixture()); extraRoot.extraField = true; reject(verifyFixture, extraRoot);
  const missingRoot = clone(buildFixture()); delete missingRoot.manifest; reject(verifyFixture, missingRoot);
  const extraStep = clone(buildFixture()); extraStep.steps[0].extraField = true; reject(verifyFixture, extraStep);
  const missingStep = clone(buildFixture()); delete missingStep.steps[0].input; reject(verifyFixture, missingStep);
  const extraResult = clone(buildFixture()); extraResult.steps[1].result.extraField = true; reject(verifyFixture, extraResult);
  const missingResult = clone(buildFixture()); delete missingResult.steps[1].result.context; reject(verifyFixture, missingResult);
});

test('rejects self-consistent residual erasure after independent commitment rehash', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = clone(buildFixture());
  const erase = body => {
    body.agreementStatus = 'NoOutstanding';
    body.remainingNotional = {tag: 'NotApplicable'};
    body.values.find(item => item.name === 'notional').value.value = '0';
  };
  erase(fixture.states[2].body);
  erase(fixture.steps[1].result.candidate.body.after.body);
  const state = named(fixture, 'STATE').find(item => item.preimage && item.preimage.revision === '2');
  assert.ok(state, 'STATE commitment for settled body');
  state.preimage = clone(fixture.states[2].body);
  rehash(state);
  fixture.states[2].stateHash = state.digest;
  fixture.steps[1].result.candidate.body.after.stateHash = state.digest;
  fixture.steps[1].result.candidate.body.after.body = clone(fixture.states[2].body);
  const trace = named(fixture, 'TRACE').find(item => item.preimage?.after?.body?.revision === '2');
  assert.ok(trace, 'TRACE commitment for settle result');
  trace.preimage = clone(fixture.steps[1].result.candidate.body);
  rehash(trace);
  fixture.steps[1].result.candidate.traceHash = trace.digest;
  fixture.steps[1].result.context.traceHash = trace.digest;
  const proof = named(fixture, 'PROOF-CONTEXT').find(item =>
    item.preimage && item.preimage.beforeStateHash === fixture.steps[1].input.state.stateHash);
  assert.ok(proof, 'PROOF-CONTEXT commitment for settle result');
  proof.preimage = clone(fixture.steps[1].result.context);
  rehash(proof);
  assertCommitmentsHold(fixture);
  reject(verifyFixture, fixture);
});

test('rejects a fully rehashed unrelated OBSERVATIONS-settle entry', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = clone(buildFixture());
  const item = labeled(fixture, 'OBSERVATIONS-settle');
  item.preimage = {unrelated: 'not an observations bundle'};
  rehash(item);
  assertEntryHash(item);
  reject(verifyFixture, fixture);
});

test('rejects swapped ACTION-accrue and ACTION-settle labels', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = clone(buildFixture());
  const accrue = labeled(fixture, 'ACTION-accrue');
  const settle = labeled(fixture, 'ACTION-settle');
  accrue.label = 'ACTION-settle';
  settle.label = 'ACTION-accrue';
  assertEntryHash(accrue);
  assertEntryHash(settle);
  reject(verifyFixture, fixture);
});

test('rejects globally relabeled unique commitment entries', async () => {
  const {buildFixture, verifyFixture} = await apis();
  const fixture = clone(buildFixture());
  assert.equal(fixture.commitments.length, 18);
  fixture.commitments.forEach((item, index) => {
    item.label = `unrelated${index}`;
    assertEntryHash(item);
  });
  reject(verifyFixture, fixture);
});
