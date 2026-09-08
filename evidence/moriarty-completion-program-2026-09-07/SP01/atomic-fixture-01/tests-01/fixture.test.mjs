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
    assert.equal(item.preimage.stateHash, undefined);
    assert.equal(item.preimage.traceHash, undefined);
    assert.equal(item.preimage.proofContextHash, undefined);
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
  const body = fixture.states[2].body;
  body.agreementStatus = 'NoOutstanding';
  body.remainingNotional = {tag: 'NotApplicable'};
  fixture.steps[1].result.candidate.body.after.body.agreementStatus = 'NoOutstanding';
  fixture.steps[1].result.candidate.body.after.body.remainingNotional = {tag: 'NotApplicable'};
  const state = named(fixture, 'STATE').find(item => item.preimage && item.preimage.revision === '2');
  assert.ok(state, 'STATE commitment for settled body');
  state.preimage = body;
  rehash(state);
  const trace = named(fixture, 'TRACE').find(item => item.preimage?.after?.body?.revision === '2');
  assert.ok(trace, 'TRACE commitment for settle result');
  trace.preimage = fixture.steps[1].result.candidate.body;
  rehash(trace);
  reject(verifyFixture, fixture);
});
