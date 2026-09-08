import {createHash} from 'node:crypto';
import {readdirSync, readFileSync} from 'node:fs';
import {dirname, join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {canonicalEncode} from '../moriarty-language/src/codec.ts';
import {createSimulator} from '../moriarty-language/src/evaluate.ts';

const MAX = '340282366920938463463374607431768211455';
const ACTORS = ['borrower', 'lender', 'pool', 'provider', 'trader'];
const DOMAIN = kind => `MORIARTY-${kind}-bounded-atomic/1`;
const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, '../..');
const language = resolve(here, '../moriarty-language');
const pin = (...parts) => ['experiments', 'moriarty-language', ...parts].join('/');
const named = (name, value) => ({name, value});
const text = value => ({tag: 'Text', value});
const uint = value => ({tag: 'UInt128', value: String(value)});
const amount = (value, unit) => ({tag: 'Amount', value: String(value), unit});

function sourcePins() {
  const src = readdirSync(join(language, 'src')).filter(name => name.endsWith('.ts')).map(name => pin('src', name));
  return [pin('spec', 'examples', 'loan.mori'), pin('spec', 'bounds.json'), ...src]
    .sort()
    .map(path => ({path, sha256: createHash('sha256').update(readFileSync(join(root, path))).digest('hex')}));
}

function commit(label, domain, preimage, expected) {
  const value = structuredClone(preimage);
  const canonical = canonicalEncode(value);
  const item = {
    label,
    domain,
    preimage: value,
    canonicalHex: Buffer.from(canonical, 'utf8').toString('hex'),
    digest: createHash('sha256').update(domain, 'utf8').update('\0').update(canonical, 'utf8').digest('hex')
  };
  if (expected !== undefined && item.digest !== expected) throw new Error(`${label} digest mismatch`);
  return item;
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

function simulate(sim, input) {
  const result = sim.simulate(input);
  if (result.kind !== 'Simulation') throw new Error(`${input.action.name} is not a Simulation`);
  return structuredClone(result);
}

export function buildFixture() {
  const sim = createSimulator(readFileSync(join(language, 'spec/examples/loan.mori')), readFileSync(join(language, 'spec/bounds.json')));
  const genesis = sim.makeGenesis({
    domain: {deployment: 'test', network: 'simulation'},
    instanceId: 'instance',
    observationBindings: [{authenticationPolicy: 'external', name: 'now', provider: 'clock'}],
    principalBindings: ACTORS.map(actor => ({actor, principal: actor}))
  });
  const initial = sim.initialState(genesis);
  const accrueInput = evaluationInput(sim, genesis, initial, 'accrue', []);
  const accrue = simulate(sim, accrueInput);
  const accrued = accrue.candidate.body.after;
  const settleInput = evaluationInput(sim, genesis, accrued, 'settle', [
    named('settlement_asset', text('USD_TEST_ASSET')),
    named('amount_due', amount(533972602, 'USD_micro'))
  ]);
  const settle = simulate(sim, settleInput);
  const states = [structuredClone(initial), structuredClone(accrued), structuredClone(settle.candidate.body.after)];
  const steps = [{input: accrueInput, result: accrue}, {input: settleInput, result: settle}];
  const manifest = structuredClone(sim.bound.manifest);
  const program = structuredClone(sim.program);
  const commitments = [
    commit('PROGRAM', DOMAIN('PROGRAM'), manifest, program.programHash),
    commit('GENESIS', DOMAIN('GENESIS'), genesis.body, genesis.genesisHash),
    commit('CLAIMS', DOMAIN('CLAIMS'), manifest.requiredClaims, genesis.body.requiredClaimRoot),
    commit('STATE-initial', DOMAIN('STATE'), states[0].body, states[0].stateHash),
    commit('STATE-accrued', DOMAIN('STATE'), states[1].body, states[1].stateHash),
    commit('STATE-settled', DOMAIN('STATE'), states[2].body, states[2].stateHash)
  ];
  for (const [suffix, step] of [['accrue', steps[0]], ['settle', steps[1]]]) {
    const body = step.result.candidate.body;
    commitments.push(
      commit(`ACTION-${suffix}`, DOMAIN('ACTION'), step.input.action, body.actionHash),
      commit(`OBSERVATIONS-${suffix}`, DOMAIN('OBSERVATIONS'), step.input.observations, body.observationsHash),
      commit(`AUTHORITY-${suffix}`, DOMAIN('AUTHORITY'), step.input.authority, step.result.context.authorityDigest),
      commit(`SIGNED-${suffix}`, step.input.authority.domain, step.input.authority.statement, body.authorityConsumption.statementDigest),
      commit(`TRACE-${suffix}`, DOMAIN('TRACE'), body, step.result.candidate.traceHash),
      commit(`PROOF-CONTEXT-${suffix}`, DOMAIN('PROOF-CONTEXT'), step.result.context)
    );
  }
  return {
    schemaVersion: 'moriarty-current-atomic-fixture/1',
    scope: 'simulation-only',
    sourcePins: sourcePins(),
    program,
    manifest,
    genesis: structuredClone(genesis),
    states,
    steps,
    commitments
  };
}

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  process.stdout.write(JSON.stringify(buildFixture()) + '\n');
}
