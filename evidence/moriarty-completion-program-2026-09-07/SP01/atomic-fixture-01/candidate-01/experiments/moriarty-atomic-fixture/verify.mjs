import {createHash} from 'node:crypto';
import {readdirSync, readFileSync} from 'node:fs';
import {dirname, join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createSimulator} from '../moriarty-language/src/evaluate.ts';

const SCHEMA = 'moriarty-current-atomic-fixture/1';
const ROOT = ['commitments', 'genesis', 'manifest', 'program', 'schemaVersion', 'scope', 'sourcePins', 'states', 'steps'];
const MAX = '340282366920938463463374607431768211455';
const ACTORS = ['borrower', 'lender', 'pool', 'provider', 'trader'];
const CLAIMS = [
  'bounded_profile_safety_v1', 'loan_first_period_interest_floor_v1', 'loan_first_period_principal_v1',
  'loan_first_period_settlement_exact_v1', 'atomic_intent_refinement_v1', 'bounded_history_compliance_v1',
  'bounded_atomic_transition_v1'
];
const COUNTS = {PROGRAM: 1, GENESIS: 1, CLAIMS: 1, STATE: 3, ACTION: 2, OBSERVATIONS: 2, AUTHORITY: 2, SIGNED: 2, TRACE: 2, 'PROOF-CONTEXT': 2};
const ALLOWED_DOMAINS = new Set([
  'MORIARTY-PROGRAM-bounded-atomic/1', 'MORIARTY-GENESIS-bounded-atomic/1', 'MORIARTY-CLAIMS-bounded-atomic/1',
  'MORIARTY-STATE-bounded-atomic/1', 'MORIARTY-ACTION-bounded-atomic/1', 'MORIARTY-OBSERVATIONS-bounded-atomic/1',
  'MORIARTY-AUTHORITY-bounded-atomic/1', 'MORIARTY-OUTCOME-bounded-atomic/1', 'MORIARTY-TRACE-bounded-atomic/1',
  'MORIARTY-PROOF-CONTEXT-bounded-atomic/1'
]);
const INTEREST = (5000000000n * 8n * 31n) / (100n * 365n);
const PRINCIPAL = 500000000n;
const PAYMENT = PRINCIPAL + INTEREST;
const RESIDUAL = 4500000000n;
const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, '../..');
const language = resolve(here, '../moriarty-language');
const tracesPath = join(root, 'evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/traces.json');
const pin = (...parts) => ['experiments', 'moriarty-language', ...parts].join('/');
const domainOf = kind => kind === 'SIGNED' ? 'MORIARTY-OUTCOME-bounded-atomic/1' : `MORIARTY-${kind}-bounded-atomic/1`;
const named = (name, value) => ({name, value});
const text = value => ({tag: 'Text', value});
const uint = value => ({tag: 'UInt128', value: String(value)});
const amount = (value, unit) => ({tag: 'Amount', value: String(value), unit});
const amountOf = (values, name) => values.find(item => item.name === name).value.value;

function encode(value) {
  if (typeof value === 'string') return JSON.stringify(value);
  if (typeof value === 'boolean') return value ? 'true' : 'false';
  if (Array.isArray(value)) return `[${value.map(encode).join(',')}]`;
  if (value && typeof value === 'object') return `{${Object.keys(value).sort().map(key => `${JSON.stringify(key)}:${encode(value[key])}`).join(',')}}`;
  throw new Error('non-canonical value');
}
function hashCommit(domain, preimage) {
  const canonical = encode(preimage);
  return {
    canonicalHex: Buffer.from(canonical, 'utf8').toString('hex'),
    digest: createHash('sha256').update(domain, 'utf8').update('\0').update(canonical, 'utf8').digest('hex')
  };
}
function need(ok, message) { if (!ok) throw new Error(message); }
function same(actual, expected, message) { need(encode(actual) === encode(expected), message); }
function keys(value, expected, message) {
  need(value && typeof value === 'object' && !Array.isArray(value), message);
  same(Object.keys(value).sort(), [...expected].sort(), message);
}
function listed(fixture, kind) {
  return fixture.commitments.filter(item =>
    item.label === kind || String(item.label).startsWith(`${kind}-`) || String(item.label).endsWith(`-${kind}`) || item.domain === domainOf(kind));
}
function diskPins() {
  const src = readdirSync(join(language, 'src')).filter(name => name.endsWith('.ts')).map(name => pin('src', name));
  return [pin('spec', 'examples', 'loan.mori'), pin('spec', 'bounds.json'), ...src]
    .sort()
    .map(path => ({path, sha256: createHash('sha256').update(readFileSync(join(root, path))).digest('hex')}));
}

function evaluationInput(sim, genesis, state, name, args, actor = 'borrower') {
  const action = {schemaVersion: 'moriarty-action/1', name, arguments: [named('actor', text(actor)), ...args]};
  const statement = {
    allowedActions: sim.bound.manifest.core.actions.map(a => a.name),
    beforeStateHash: state.stateHash,
    domain: genesis.body.domain,
    genesisHash: genesis.genesisHash,
    grossDebitCaps: sim.bound.manifest.settlementBindings
      .map(b => ({actor, asset: b.asset, maximumLedgerAmount: MAX}))
      .sort((a, b) => a.asset.localeCompare(b.asset)),
    instanceId: 'instance',
    minimumNetCredits: [],
    mode: 'IntentRefinement',
    nonce: 'nonce' + state.body.revision,
    permittedCalls: [],
    permittedRecipients: [...ACTORS],
    predecessors: [state.stateHash],
    principal: actor,
    program: sim.program,
    requiredClaimRoot: genesis.body.requiredClaimRoot,
    requiredClaims: sim.bound.manifest.requiredClaims,
    schemaVersion: 'moriarty-outcome-intent/1',
    validity: {notAfterExclusive: '2000000000', notBefore: '0'}
  };
  return structuredClone({
    action,
    authority: {
      domain: 'MORIARTY-OUTCOME-bounded-atomic/1',
      schemaVersion: 'moriarty-authority/1',
      signature: {algorithm: 'simulation-only', bytes: '', keyId: actor},
      statement,
      tag: 'IntentRefinement'
    },
    checks: {
      authenticatedPrincipal: actor,
      genesisValid: true,
      nonceFresh: true,
      observationsAuthentic: true,
      predecessorSetValid: true,
      signatureValid: true,
      stateCurrentAndUnconsumed: true
    },
    genesis,
    observations: {
      observations: [{evidenceDigest: '0'.repeat(64), name: 'now', provider: 'clock', value: uint(1)}],
      schemaVersion: 'moriarty-observations/1'
    },
    program: sim.program,
    schemaVersion: 'moriarty-evaluation/1',
    state
  });
}

function reconstruct() {
  const loan = readFileSync(join(language, 'spec/examples/loan.mori'));
  const bounds = readFileSync(join(language, 'spec/bounds.json'));
  const sim = createSimulator(loan, bounds);
  const genesis = sim.makeGenesis({
    domain: {deployment: 'test', network: 'simulation'},
    instanceId: 'instance',
    observationBindings: [{authenticationPolicy: 'external', name: 'now', provider: 'clock'}],
    principalBindings: ACTORS.map(actor => ({actor, principal: actor}))
  });
  const initial = sim.initialState(genesis);
  const accrueInput = evaluationInput(sim, genesis, initial, 'accrue', []);
  const accrue = sim.simulate(accrueInput);
  need(accrue.kind === 'Simulation', 'accrue Simulation');
  const accrued = accrue.candidate.body.after;
  const settleInput = evaluationInput(sim, genesis, accrued, 'settle', [
    named('settlement_asset', text('USD_TEST_ASSET')),
    named('amount_due', amount(533972602, 'USD_micro'))
  ]);
  const settle = sim.simulate(settleInput);
  need(settle.kind === 'Simulation', 'settle Simulation');
  return {
    program: sim.program,
    manifest: sim.bound.manifest,
    genesis,
    states: [initial, accrued, settle.candidate.body.after],
    steps: [{input: accrueInput, result: accrue}, {input: settleInput, result: settle}]
  };
}

function link(fixture, kind, preimage, digest) {
  const matches = listed(fixture, kind).filter(item => encode(item.preimage) === encode(preimage) && item.digest === digest);
  need(matches.length >= 1, `missing ${kind} link`);
  need(matches[0].domain === domainOf(kind), `${kind} domain`);
  return matches[0];
}

function checkCommitments(fixture) {
  const labels = new Set();
  need(Array.isArray(fixture.commitments), 'commitments');
  need(fixture.commitments.length === 18, 'commitment count');
  for (const item of fixture.commitments) {
    keys(item, ['canonicalHex', 'digest', 'domain', 'label', 'preimage'], 'commitment fields');
    need(typeof item.label === 'string' && item.label.length > 0, 'label');
    need(!labels.has(item.label), `duplicate ${item.label}`);
    labels.add(item.label);
    need(ALLOWED_DOMAINS.has(item.domain), `domain ${item.domain}`);
    const got = hashCommit(item.domain, item.preimage);
    need(item.canonicalHex === got.canonicalHex, `${item.label} canonicalHex`);
    need(item.digest === got.digest, `${item.label} digest`);
    if (item.domain === domainOf('STATE')) need(item.preimage.stateHash === undefined, 'STATE own digest');
    if (item.domain === domainOf('TRACE')) need(item.preimage.traceHash === undefined, 'TRACE own digest');
    if (item.domain === domainOf('PROOF-CONTEXT')) {
      need(item.preimage.proofContextHash === undefined, 'PROOF-CONTEXT own digest');
      need(typeof item.preimage.traceHash === 'string', 'ProofContext.traceHash');
    }
  }
  for (const [kind, count] of Object.entries(COUNTS)) need(listed(fixture, kind).length === count, `${kind} count`);
  link(fixture, 'PROGRAM', fixture.manifest, fixture.program.programHash);
  link(fixture, 'GENESIS', fixture.genesis.body, fixture.genesis.genesisHash);
  link(fixture, 'CLAIMS', fixture.manifest.requiredClaims, fixture.genesis.body.requiredClaimRoot);
  fixture.states.forEach(state => link(fixture, 'STATE', state.body, state.stateHash));
  fixture.steps.forEach(step => {
    const body = step.result.candidate.body;
    link(fixture, 'ACTION', step.input.action, body.actionHash);
    link(fixture, 'OBSERVATIONS', step.input.observations, body.observationsHash);
    link(fixture, 'AUTHORITY', step.input.authority, step.result.context.authorityDigest);
    const signed = link(fixture, 'SIGNED', step.input.authority.statement, body.authorityConsumption.statementDigest);
    need(signed.domain === step.input.authority.domain, 'SIGNED uses authority.domain');
    link(fixture, 'TRACE', body, step.result.candidate.traceHash);
    need(step.result.context.traceHash === step.result.candidate.traceHash, 'context.traceHash');
    const proof = link(fixture, 'PROOF-CONTEXT', step.result.context, hashCommit(domainOf('PROOF-CONTEXT'), step.result.context).digest);
    need(proof.preimage.traceHash === step.result.candidate.traceHash, 'ProofContext binds candidate');
    need(body.authorityConsumption.statementDigest === signed.digest, 'statementDigest');
    need(step.result.context.actionHash === body.actionHash, 'context.actionHash');
    need(step.result.context.requiredClaimRoot === fixture.genesis.body.requiredClaimRoot, 'claim root');
    need(step.result.context.genesisHash === fixture.genesis.genesisHash, 'context genesis');
  });
}

function checkEconomics(fixture, traces) {
  need(INTEREST === 33972602n && PAYMENT === 533972602n && RESIDUAL === 4500000000n, 'derived amounts');
  const arith = traces.independentArithmetic.loan;
  need(arith.floorInterest === String(INTEREST) && arith.principalInstallment === String(PRINCIPAL), 'traces arithmetic');
  need(arith.totalDue === String(PAYMENT) && arith.remainingNotional === String(RESIDUAL), 'traces totals');
  const initial = traces.examples.loan.initialState;
  const [s0, s1, s2] = fixture.states;
  same(s0.body.values, initial.values, 'initial values');
  need(s0.body.revision === initial.revision && s0.body.remaining === initial.remaining, 'initial lifetime');
  need(s0.body.episodeStatus === initial.episodeStatus && s0.body.agreementStatus === initial.agreementStatus, 'initial status');
  same(s0.body.remainingNotional, initial.remainingNotional, 'initial remainingNotional');
  same(s0.body.obligations, initial.obligations, 'initial obligations');
  function row(body, complete, spec) {
    same(body.values, spec.afterValues, 'afterValues');
    need(body.revision === spec.revision && body.remaining === spec.remaining, 'lifetime');
    need(body.episodeStatus === spec.episodeStatus && body.agreementStatus === spec.agreementStatus, 'status');
    same(body.remainingNotional, spec.remainingNotional, 'remainingNotional');
    same(body.obligations, spec.retainedObligations, 'retainedObligations');
    same(complete.writes, spec.orderedWrites, 'orderedWrites');
    same(complete.effects, spec.orderedEffects, 'orderedEffects');
    same(complete.obligationDelta, spec.obligationDelta, 'obligationDelta');
  }
  row(s1.body, fixture.steps[0].result.candidate.body, traces.positiveRows['loan-accrue']);
  row(s2.body, fixture.steps[1].result.candidate.body, traces.positiveRows['loan-settle']);
  need(amountOf(s1.body.values, 'principal_due') === String(PRINCIPAL), 'principal_due');
  need(amountOf(s1.body.values, 'interest_due') === String(INTEREST), 'interest_due');
  need(amountOf(s2.body.values, 'notional') === String(RESIDUAL), 'terminal notional');
  need(amountOf(s2.body.values, 'lender_cash') === String(PAYMENT), 'lender_cash');
  need(amountOf(s2.body.values, 'interest_paid') === String(INTEREST), 'interest_paid');
  need(s2.body.remaining === '0' && s2.body.revision === '2', 'terminal lifetime');
  need(s2.body.episodeStatus === 'Closed' && s2.body.agreementStatus === 'Outstanding', 'terminal status');
  need(s2.body.remainingNotional.tag === 'Amount' && s2.body.remainingNotional.amount.value === String(RESIDUAL), 'residual notional');
  need(s2.body.obligations.length === 2 && s2.body.obligations.every(item => item.status === 'Settled'), 'Settled tombstones');
  const transfer = fixture.steps[1].result.candidate.body.effects.find(item => item.kind === 'Transfer');
  need(transfer && transfer.to === 'lender' && transfer.amount.value === String(PAYMENT), 'settlement transfer');
  same(fixture.manifest.requiredClaims.map(item => item.claimId), CLAIMS, 'claim order');
  same(fixture.manifest.requiredClaims, traces.examples.loan.requiredClaims, 'trace claims');
}

export function verifyFixture(fixture) {
  keys(fixture, ROOT, 'root fields');
  need(fixture.schemaVersion === SCHEMA, 'schemaVersion');
  need(fixture.scope === 'simulation-only', 'scope');
  need(Array.isArray(fixture.sourcePins) && Array.isArray(fixture.states) && Array.isArray(fixture.steps), 'arrays');
  need(fixture.states.length === 3 && fixture.steps.length === 2, 'state/step counts');
  same(fixture.sourcePins, diskPins(), 'sourcePins');
  const loanPin = fixture.sourcePins.find(item => item.path.endsWith('loan.mori'));
  need(loanPin && fixture.program.sourceHash === loanPin.sha256 && fixture.manifest.sourceHash === loanPin.sha256, 'sourceHash');
  const boundsBytes = readFileSync(join(language, 'spec/bounds.json'));
  const boundsHash = createHash('sha256').update('MORIARTY-BOUNDS-bounded-atomic/1', 'utf8').update('\0').update(boundsBytes).digest('hex');
  need(fixture.manifest.bounds.boundsHash === boundsHash, 'boundsHash');
  keys(fixture.program, ['bounds', 'coreVersion', 'profile', 'programHash', 'schemaVersion', 'sourceHash'], 'program');
  fixture.steps.forEach(step => {
    keys(step, ['input', 'result'], 'step fields');
    keys(step.input, ['action', 'authority', 'checks', 'genesis', 'observations', 'program', 'schemaVersion', 'state'], 'input fields');
    keys(step.result, ['candidate', 'context', 'kind'], 'result fields');
    need(step.result.kind === 'Simulation', 'Simulation only');
    need(step.result.candidate.body.outcome === 'Complete', 'Complete candidate');
    need(step.result.outcome === undefined && step.result.tag !== 'Accepted', 'no Accepted');
    need(step.input.authority.signature.algorithm === 'simulation-only' && step.input.authority.signature.bytes === '', 'synthetic signature');
    need(step.input.checks.authenticatedPrincipal === 'borrower', 'authenticatedPrincipal');
    for (const key of ['genesisValid', 'nonceFresh', 'observationsAuthentic', 'predecessorSetValid', 'signatureValid', 'stateCurrentAndUnconsumed']) {
      need(step.input.checks[key] === true, key);
    }
  });
  need(fixture.steps[0].input.action.name === 'accrue' && fixture.steps[1].input.action.name === 'settle', 'actions');
  checkCommitments(fixture);
  const traces = JSON.parse(readFileSync(tracesPath, 'utf8'));
  checkEconomics(fixture, traces);
  const expected = reconstruct();
  same(fixture.program, expected.program, 'program');
  same(fixture.manifest, expected.manifest, 'manifest');
  same(fixture.genesis, expected.genesis, 'genesis');
  same(fixture.states, expected.states, 'states');
  same(fixture.steps, expected.steps, 'steps');
  return true;
}

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  const path = process.argv[2];
  if (!path) throw new Error('usage: node verify.mjs <fixture.json>');
  const fixture = JSON.parse(readFileSync(path, 'utf8'));
  verifyFixture(fixture);
  process.stdout.write(JSON.stringify({ok: true, schemaVersion: fixture.schemaVersion, scope: fixture.scope}) + '\n');
}
