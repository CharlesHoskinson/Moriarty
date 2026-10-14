import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { admitPublicAtomic } from '../src/admit.mjs';
import { compile } from '../../moriarty-language/src/frontend.ts';

const here = dirname(fileURLToPath(import.meta.url));
const bounds = readFileSync(new URL('../../moriarty-language/spec/bounds.json', import.meta.url), 'utf8');
const example = readFileSync(join(here, '../examples/revision-step.mori'), 'utf8');
const manifest = JSON.parse(readFileSync(join(here, '../examples/revision-step.manifest.json'), 'utf8'));
const loan = readFileSync(new URL('../../moriarty-language/spec/examples/loan.mori', import.meta.url), 'utf8');
const designers = '/home/charl/moriarty-eutxo-validation-2026-10-12/designers';
const swap = readFileSync(join(designers, '03-swaps-batch/bilateral-swap.mori'), 'utf8');
const successor = readFileSync(join(designers, '01-state/head-step.mori'), 'utf8');

test('admits revision/count head with Transfer and interval-backed now', () => {
  const bound = compile(example, bounds).bound;
  const admitted = admitPublicAtomic(bound, manifest);
  assert.equal(admitted.ok, true);
  assert.equal(admitted.profile, 'moriarty-bounded-atomic/1');
  assert.equal(admitted.timePredicate, 'TIME_PREDICATE_v1');
  assert.deepEqual(admitted.unsupportedPrivateClaims.map(c => c.kind).sort(), [
    'ContractProperty', 'ContractProperty', 'IntentRefinement', 'PredecessorHistory', 'TransitionValidity',
  ].sort());
  assert.equal(admitted.production, false);
});

test('rejects successor financial source/6 for public UPLC compilation', () => {
  const admitted = admitPublicAtomic({ source: successor }, manifest);
  assert.equal(admitted.ok, false);
  assert.equal(admitted.code, 'PROFILE_UNSUPPORTED');
});

test('rejects DueCreated/DueSettled loan effects for public UPLC subset', () => {
  const bound = compile(loan, bounds).bound;
  const admitted = admitPublicAtomic(bound, manifest);
  assert.equal(admitted.ok, false);
  assert.equal(admitted.code, 'EFFECT_KIND_UNSUPPORTED');
  assert.ok(admitted.kinds.includes('DueCreated'));
});

test('rejects consent observations that have no documented ledger predicate', () => {
  const bound = compile(swap, bounds).bound;
  const admitted = admitPublicAtomic(bound, manifest);
  assert.equal(admitted.ok, false);
  assert.equal(admitted.code, 'OBSERVATION_UNSUPPORTED');
  assert.ok(admitted.names.includes('alice_consented'));
});

test('rejects missing actor/asset mapping instead of inventing addresses', () => {
  const bound = compile(example, bounds).bound;
  const admitted = admitPublicAtomic(bound, { ...manifest, actors: { payee: manifest.actors.payee } });
  assert.equal(admitted.ok, false);
  assert.equal(admitted.code, 'ACTOR_MAPPING_UNBOUND');
});

test('rejects Amount state as unsupported ledger inventory projection', () => {
  const swap = readFileSync(new URL('../../moriarty-language/spec/examples/swap.mori', import.meta.url), 'utf8');
  const bound = compile(swap, bounds).bound;
  const admitted = admitPublicAtomic(bound, {
    ...manifest,
    actors: {
      pool: manifest.actors.payer,
      trader: manifest.actors.payee,
      provider: 'dddddddddddddddddddddddddddddddddddddddddddddddddddddddd',
    },
    assets: {
      ASSET_A: manifest.assets.CASH,
      ASSET_B: { policy: 'eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee', name: '42544f4b' },
    },
  });
  assert.equal(admitted.ok, false);
  assert.equal(admitted.code, 'STATE_INVENTORY_UNSUPPORTED');
});

test('records single-head wallet-transfer subset and external genesis precondition', () => {
  const bound = compile(example, bounds).bound;
  const admitted = admitPublicAtomic(bound, manifest);
  assert.equal(admitted.ok, true);
  assert.equal(admitted.nativePredicate, 'public-single-head-wallet-transfer/1');
  assert.equal(admitted.nativeSubset.scriptInputs, 'exactly-one-script-credential-input');
  assert.equal(admitted.nativeSubset.quantum, 'exactly-1-nominal-per-ledger-unit');
  assert.equal(admitted.nativeSubset.frozenComplete, false);
  assert.equal(admitted.genesisPrecondition.pcdVerification, 'explicitly-not-performed');
});

test('admits settlement quantum 1 and rejects every other quantum as SETTLEMENT_QUANTUM_UNSUPPORTED', () => {
  const bound1 = compile(example, bounds).bound;
  assert.equal(bound1.manifest.settlementBindings[0].quantum.value, '1');
  const admitted1 = admitPublicAtomic(bound1, manifest);
  assert.equal(admitted1.ok, true);

  for (const quantum of ['2', '10']) {
    const source = example.replace('quantum amount(1, Cash)', `quantum amount(${quantum}, Cash)`);
    const compiled = compile(source, bounds);
    assert.ok(compiled.bound, `frontend must still compile quantum ${quantum}`);
    assert.equal(compiled.bound.manifest.settlementBindings[0].quantum.value, quantum);
    const admitted = admitPublicAtomic(compiled.bound, manifest);
    assert.equal(admitted.ok, false, `quantum ${quantum} must fail public admission`);
    assert.equal(admitted.code, 'SETTLEMENT_QUANTUM_UNSUPPORTED');
  }
});
