import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {mkdtempSync, readFileSync, rmSync, symlinkSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {test} from 'node:test';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {createSimulator} from '../../../experiments/moriarty-language/src/evaluate.ts';
import {lowerCompact, prepareCompactCall} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const language = fileURLToPath(new URL('../../../experiments/moriarty-language/', import.meta.url));
const bounds = readFileSync(join(language, 'spec/bounds.json'));
const source = readFileSync(join(language, 'spec/examples/swap.mori'));
const mapping = lowerCompact(source, bounds);
const compactDir = join(language, 'compact');
const kernel = join(compactDir, 'generated/swap/kernel.compact');

const named = (name, value) => ({name, value});
const text = value => ({tag: 'Text', value});
const uint = value => ({tag: 'UInt128', value: String(value)});
const amount = (value, unit) => ({tag: 'Amount', value: String(value), unit});
const swapArgs = [
  named('actor', text('trader')), named('recipient', text('trader')),
  named('asset_in', text('ASSET_A')), named('asset_out', text('ASSET_B')),
  named('amount_in', amount(10_000, 'AssetA_quantum')), named('min_out', amount(19_743, 'AssetB_quantum')),
];
const now = [named('now', uint(1))];

test('bound program and compiled accounting ABI match the proof target', () => {
  const digest = bytes => createHash('sha256').update(bytes).digest('hex');
  assert.equal(mapping.metadata.programHash, 'b00a55b8da611d23c08461a11853151aa50ec7ee8150f9d16afbc66d62e018fa');
  assert.equal(digest(source), '0c2217365f2e518ec70835cf05a09150253d2df334e7dcf0504fd8c8d887051e');
  assert.equal(digest(mapping.source), '090b9e2ecf5a0850bee0d46570ca9edb3ac374743e2ed29235913c6e1bb58741');
  assert.equal(digest(readFileSync(join(compactDir, 'arithmetic.compact'))),
    'b2969d0bc346fa93d88546578c9f9d29f1b74d7bc3f88f2e04709fa0932d9415');
  assert.deepEqual(mapping.metadata, JSON.parse(readFileSync(join(compactDir, 'generated/swap/metadata.json'))));
  assert.deepEqual(mapping.bound, JSON.parse(readFileSync(join(compactDir, 'generated/swap/bound-program.json'))));
});

function swapInput(sim) {
  const genesis = sim.makeGenesis({
    domain: {network: 'simulation', deployment: 'ii6'}, instanceId: 'swap1',
    principalBindings: ['pool', 'provider', 'trader'].map(actor => ({actor, principal: actor})),
    observationBindings: [{name: 'now', provider: 'clock', authenticationPolicy: 'external'}],
  });
  const state = sim.initialState(genesis);
  const statement = {
    schemaVersion: 'moriarty-outcome-intent/1', mode: 'IntentRefinement', principal: 'trader',
    allowedActions: ['swap'], permittedRecipients: ['pool', 'trader'], permittedCalls: [],
    grossDebitCaps: [{actor: 'trader', asset: 'ASSET_A', maximumLedgerAmount: '10000'}],
    minimumNetCredits: [], program: sim.program, domain: genesis.body.domain, instanceId: 'swap1',
    genesisHash: genesis.genesisHash, requiredClaimRoot: genesis.body.requiredClaimRoot,
    requiredClaims: sim.bound.manifest.requiredClaims, beforeStateHash: state.stateHash,
    predecessors: [state.stateHash], nonce: 'nonce0', validity: {notBefore: '0', notAfterExclusive: '2000000000'},
  };
  return {
    schemaVersion: 'moriarty-evaluation/1', genesis, program: sim.program, state,
    action: {schemaVersion: 'moriarty-action/1', name: 'swap', arguments: structuredClone(swapArgs)},
    authority: {schemaVersion: 'moriarty-authority/1', tag: 'IntentRefinement', statement,
      domain: 'MORIARTY-OUTCOME-bounded-atomic/1', signature: {algorithm: 'simulation-only', keyId: 'trader', bytes: ''}},
    observations: {schemaVersion: 'moriarty-observations/1', observations: [
      {name: 'now', provider: 'clock', evidenceDigest: '0'.repeat(64), value: uint(1)},
    ]},
    checks: {authenticatedPrincipal: 'trader', signatureValid: true, nonceFresh: true, genesisValid: true,
      observationsAuthentic: true, predecessorSetValid: true, stateCurrentAndUnconsumed: true},
  };
}

test('source simulator admits the exact cap and rejects each insufficient authority premise independently', () => {
  const sim = createSimulator(source, bounds);
  const input = swapInput(sim);
  const ok = sim.simulate(input);
  assert.equal(ok.kind, 'Simulation', JSON.stringify(ok));
  assert.deepEqual(ok.candidate.body.effects.map(e => [e.from, e.to, e.asset, e.settlement.ledgerAmount]), [
    ['trader', 'pool', 'ASSET_A', '10000'], ['pool', 'trader', 'ASSET_B', '19743'],
  ]);
  for (const [code, change] of [
    ['INTENT_DEBIT_CAP', i => { i.authority.statement.grossDebitCaps[0].maximumLedgerAmount = '9999'; }],
    ['INTENT_RECIPIENT_FORBIDDEN', i => { i.authority.statement.permittedRecipients = ['pool']; }],
    ['INTENT_RECIPIENT_FORBIDDEN', i => { i.authority.statement.permittedRecipients = ['trader']; }],
    ['PRINCIPAL_BINDING', i => { i.checks.authenticatedPrincipal = 'pool'; }],
    ['SIGNATURE_INVALID', i => { i.checks.signatureValid = false; }],
  ]) {
    const changed = structuredClone(input);
    change(changed);
    const result = sim.simulate(changed);
    assert.equal(result.outcome, 'Rejected');
    assert.equal(result.diagnostics[0].code, code);
  }
});

test('compiled kernel conserves both assets, debits only available A, and fixes both swap and close destinations', async t => {
  assert.equal(mapping.source, readFileSync(kernel, 'utf8'));
  // Freeze the external IDs independently of the mapper and runtime decoder.
  assert.deepEqual(mapping.metadata.textTable.map(x => [x.id, x.text]), [
    ['0', 'ASSET_A'], ['1', 'ASSET_B'], ['2', 'pool'], ['3', 'provider'], ['4', 'trader'],
  ]);
  const build = mkdtempSync(join(tmpdir(), 'moriarty-ii6-'));
  t.after(() => rmSync(build, {recursive: true, force: true}));
  execFileSync('compact', ['compile', '--skip-zk', '--compact-path', compactDir, kernel, build]);
  symlinkSync(join(language, 'node_modules'), join(build, 'node_modules'), 'dir');
  const {pureCircuits} = await import(pathToFileURL(join(build, 'contract/index.js')).href);
  const {arguments: args} = prepareCompactCall(mapping, 'swap', mapping.bound.manifest.initialState,
    swapArgs, now, '8', '0');
  // Numerals below are direct raw ABI expectations for the pinned program.
  const before = {...args[0], f2: 10_000n}; // Exact debit boundary in a hypothetical kernel input.
  const swap = pureCircuits.transition0(before, ...args.slice(1));
  const swapped = {f0: 1_010_000n, f1: 1_980_257n, f2: 0n, f3: 19_743n, f4: 0n, f5: 0n, f6: 0n};
  assert.deepEqual(swap.after, swapped);
  assert.deepEqual(swap.effect0, {v0: 0n, v1: 4n, v2: 2n, v3: 10_000n});
  assert.deepEqual(swap.effect1, {v0: 1n, v1: 2n, v2: 4n, v3: 19_743n});
  assert.throws(() => pureCircuits.transition0({...before, f2: 9_999n}, ...args.slice(1)), /insufficient synthetic balance/);
  assert.throws(() => pureCircuits.transition0(before, {...args[1], a0: 3n}, ...args.slice(2)), /trader authority required/);
  assert.throws(() => pureCircuits.transition0(before, {...args[1], a1: 3n}, ...args.slice(2)), /recipient must be trader/);
  const closeArgs = [swap.after, {a0: 3n}, args[2], swap.remaining, swap.revision, {unused: 0n}];
  const close = pureCircuits.transition1(...closeArgs);
  assert.deepEqual(close.after, {f0: 0n, f1: 0n, f2: 0n, f3: 19_743n, f4: 1_010_000n, f5: 1_980_257n, f6: 1n});
  assert.deepEqual(close.effect0, {v0: 0n, v1: 2n, v2: 3n, v3: 1_010_000n});
  assert.deepEqual(close.effect1, {v0: 1n, v1: 2n, v2: 3n, v3: 1_980_257n});
  assert.throws(() => pureCircuits.transition1(swap.after, {a0: 4n}, ...closeArgs.slice(2)), /provider authority required/);
  for (const state of [before, swap.after, close.after]) {
    assert.equal(state.f0 + state.f2 + state.f4, 1_010_000n);
    assert.equal(state.f1 + state.f3 + state.f5, 2_000_000n);
  }
});
