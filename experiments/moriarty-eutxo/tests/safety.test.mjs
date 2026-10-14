import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdirSync, existsSync, rmSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { compilePublic, runAikenViaScript, parseAikenCheckJson } from '../src/compile-public.mjs';
import { admitPublicAtomic } from '../src/admit.mjs';
import { compile } from '../../moriarty-language/src/frontend.ts';
import { createSimulator } from '../../moriarty-language/src/evaluate.ts';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, '..');
const bounds = readFileSync(new URL('../../moriarty-language/spec/bounds.json', import.meta.url), 'utf8');
const manifest = JSON.parse(readFileSync(join(root, 'examples/revision-step.manifest.json'), 'utf8'));
const designers = '/home/charl/moriarty-eutxo-validation-2026-10-12/designers';

test('script -e propagates child exit status', () => {
  const wrapper = spawnSync('script', ['-q', '-c', 'exit 1', '/dev/null'], { encoding: 'utf8' });
  const corrected = spawnSync('script', ['-q', '-e', '-c', 'exit 1', '/dev/null'], { encoding: 'utf8' });
  assert.equal(wrapper.status, 0);
  assert.equal(corrected.status, 1);
  const via = runAikenViaScript(['--version']);
  assert.equal(via.status, 0);
});

test('missing Aiken test titles are not treated as success', () => {
  const parsed = parseAikenCheckJson(JSON.stringify({
    summary: { total: 1, passed: 1, failed: 0 },
    modules: [{ name: 'atomic', test: [{ kind: 'unit', title: 'unrelated', status: 'success' }] }],
  }), '');
  assert.equal(parsed.ok, true);
  assert.equal(parsed.tests.some(t => t.title === 'positive_step'), false);
});

test('failed Aiken JSON status cannot be reported as success', () => {
  const parsed = parseAikenCheckJson(JSON.stringify({
    summary: { total: 1, passed: 0, failed: 1 },
    modules: [{ name: 'atomic', test: [{ kind: 'unit', title: 'positive_step', status: 'failure' }] }],
  }), '');
  assert.equal(parsed.tests[0].status, 'failure');
  assert.notEqual(parsed.tests[0].status, 'success');
});

test('failed admission does not publish a stale blueprint path', () => {
  const outDir = mkdtempSync(join(tmpdir(), 'moriarty-eutxo-stale-'));
  writeFileSync(join(outDir, 'plutus.json'), '{"stale":true}\n');
  const result = compilePublic({
    source: new URL('../../moriarty-language/spec/examples/loan.mori', import.meta.url).pathname,
    manifest,
    outDir,
  });
  assert.equal(result.ok, false);
  assert.ok(result.admission?.code);
  assert.equal(result.blueprintPath, null);
  assert.equal(existsSync(join(outDir, 'plutus.json')), false);
  rmSync(outDir, { recursive: true, force: true });
});

test('compilePublic rejects quantum 2 and larger without publishing a blueprint', () => {
  const example = readFileSync(join(root, 'examples/revision-step.mori'), 'utf8');
  for (const quantum of ['2', '10']) {
    const outDir = mkdtempSync(join(tmpdir(), `moriarty-eutxo-q${quantum}-`));
    writeFileSync(join(outDir, 'plutus.json'), '{"stale":true}\n');
    const source = example.replace('quantum amount(1, Cash)', `quantum amount(${quantum}, Cash)`);
    const result = compilePublic({ source, manifest, outDir });
    assert.equal(result.ok, false, `quantum ${quantum} compilePublic must fail`);
    assert.equal(result.admission?.code, 'SETTLEMENT_QUANTUM_UNSUPPORTED');
    assert.equal(result.blueprintPath, null);
    assert.equal(existsSync(join(outDir, 'plutus.json')), false);
    assert.equal(existsSync(join(outDir, 'atomic.ak')), false);
    rmSync(outDir, { recursive: true, force: true });
  }
});

test('overflow program compiles as a real rejection fixture without positive UPLC claim', () => {
  const outDir = mkdtempSync(join(tmpdir(), 'moriarty-eutxo-overflow-'));
  const result = compilePublic({
    source: join(root, 'examples/overflow-step.mori'),
    manifest,
    outDir,
  });
  if (!result.ok) {
    assert.fail(`overflow compile failed check=${result.check?.status} ${result.check?.stderr}\n${result.check?.stdout}`);
  }
  assert.equal(result.positiveUplcValidation, false);
  assert.equal(result.metadata.positiveUplcValidation, false);
  const titles = (result.check.tests || []).map(t => t.title);
  assert.ok(titles.some(t => t === 'overflow_step' || t.endsWith('overflow_step')));
  assert.equal(titles.some(t => t === 'positive_step' || t.endsWith('positive_step')), false);
  assert.ok(result.blueprintPath);
  rmSync(outDir, { recursive: true, force: true });
});

test('Amount state is closed admission, not a pool/loan inventory projection', () => {
  const swap = readFileSync(join(designers, '03-swaps-batch/constant-product.mori'), 'utf8');
  const bound = compile(swap, bounds).bound;
  const admitted = admitPublicAtomic(bound, {
    ...manifest,
    actors: {
      ...manifest.actors,
      trader: manifest.actors.payer,
      pool: manifest.actors.payee,
      provider: manifest.actors.payee,
    },
    assets: {
      ASSET_A: manifest.assets.CASH,
      ASSET_B: { policy: 'eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee', name: '42544f4b' },
    },
  });
  assert.equal(admitted.ok, false);
  assert.ok(admitted.code === 'STATE_INVENTORY_UNSUPPORTED' || admitted.code === 'OBSERVATION_UNSUPPORTED' || admitted.code === 'ACTOR_MAPPING_UNBOUND');
});

test('non-injective actor mapping and ADA settlement fail closed', () => {
  const example = readFileSync(join(root, 'examples/revision-step.mori'), 'utf8');
  const bound = compile(example, bounds).bound;
  const dup = admitPublicAtomic(bound, {
    ...manifest,
    actors: { payer: manifest.actors.payer, payee: manifest.actors.payer },
  });
  assert.equal(dup.ok, false);
  assert.equal(dup.code, 'ACTOR_MAPPING_NOT_INJECTIVE');
  const ada = admitPublicAtomic(bound, {
    ...manifest,
    assets: { CASH: { policy: '', name: '43415348' } },
  });
  assert.equal(ada.ok, false);
  assert.equal(ada.code, 'ADA_SETTLEMENT_UNSUPPORTED');
});

test('source Simulation short-circuits true Or and false And with division by zero', () => {
  const source = readFileSync(join(root, 'examples/revision-step.mori'), 'utf8');
  const sim = createSimulator(source, bounds);
  const named = (name, value) => ({ name, value });
  const text = value => ({ tag: 'Text', value });
  const uint = value => ({ tag: 'UInt128', value: String(value) });
  const amount = (value, unit) => ({ tag: 'Amount', value: String(value), unit });
  const genesis = sim.makeGenesis({
    domain: { network: 'simulation', deployment: 'shortcircuit' },
    instanceId: 'instance-1',
    principalBindings: ['payee', 'payer'].map(actor => ({ actor, principal: actor })),
    observationBindings: [{ name: 'now', provider: 'clock', authenticationPolicy: 'external' }],
  });
  const state = sim.initialState(genesis);
  const actor = 'payer';
  const args = [named('actor', text(actor)), named('payout', amount(5, 'Cash'))];
  const input = structuredClone({
    action: { schemaVersion: 'moriarty-action/1', name: 'step', arguments: args },
    authority: {
      domain: 'MORIARTY-OUTCOME-bounded-atomic/1',
      schemaVersion: 'moriarty-authority/1',
      signature: { algorithm: 'simulation-only', bytes: '', keyId: actor },
      tag: 'IntentRefinement',
      statement: {
        allowedActions: sim.bound.manifest.core.actions.map(a => a.name),
        grossDebitCaps: sim.bound.manifest.settlementBindings.map(b => ({ actor, asset: b.asset, maximumLedgerAmount: '340282366920938463463374607431768211455' })).sort((a, b) => a.asset.localeCompare(b.asset)),
        minimumNetCredits: [],
        beforeStateHash: state.stateHash,
        domain: genesis.body.domain,
        genesisHash: genesis.genesisHash,
        instanceId: 'instance-1',
        mode: 'IntentRefinement',
        nonce: 'nonce' + state.body.revision,
        permittedCalls: [],
        permittedRecipients: ['payee', 'payer'],
        predecessors: [state.stateHash],
        principal: actor,
        program: sim.program,
        requiredClaimRoot: genesis.body.requiredClaimRoot,
        requiredClaims: sim.bound.manifest.requiredClaims,
        schemaVersion: 'moriarty-outcome-intent/1',
        validity: { notBefore: '0', notAfterExclusive: '2000000000' },
      },
    },
    checks: {
      authenticatedPrincipal: actor,
      genesisValid: true,
      nonceFresh: true,
      observationsAuthentic: true,
      predecessorSetValid: true,
      signatureValid: true,
      stateCurrentAndUnconsumed: true,
    },
    genesis,
    observations: {
      schemaVersion: 'moriarty-observations/1',
      observations: [{ name: 'now', provider: 'clock', evidenceDigest: '0'.repeat(64), value: uint(1) }],
    },
    program: sim.program,
    schemaVersion: 'moriarty-evaluation/1',
    state,
  });
  const result = sim.simulate(input);
  assert.equal(result.kind, 'Simulation', JSON.stringify(result));
  assert.equal(result.candidate.body.after.body.values.find(v => v.name === 'count').value.value, '1');
});
