/**
 * Publication gate decision table for compileCustody.
 * Helper-only: this does not call compileCustody and does not establish
 * either original EUC-012 scenario.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { shouldApply, shouldPublish, SUCCESS_ARTIFACTS } from '../src/custody-publication.mjs';

test('only a passing check, build and application publish a blueprint', () => {
  const applied = { ok: true };
  assert.equal(shouldPublish({ checkOk: true, compiled: true, applied }), true);
  assert.equal(shouldPublish({ checkOk: false, compiled: true, applied }), false);
  assert.equal(shouldPublish({ checkOk: true, compiled: false, applied }), false);
  assert.equal(shouldPublish({ checkOk: true, compiled: true, applied: { ok: false, code: 'AIKEN_CHECK' } }), false);
  assert.equal(shouldPublish({ checkOk: true, compiled: true, applied: null }), false);
  assert.equal(shouldPublish({ checkOk: 1, compiled: true, applied }), false);
});

test('parameter application is not attempted after a failed check or build', () => {
  assert.equal(shouldApply({ checkOk: true, compiled: true }), true);
  assert.equal(shouldApply({ checkOk: false, compiled: true }), false);
  assert.equal(shouldApply({ checkOk: true, compiled: false }), false);
});

test('the applied success artifacts are cleared with the blueprint', () => {
  assert.deepEqual([...SUCCESS_ARTIFACTS], ['plutus.json', 'plutus.applied.json', 'cash.script.json', 'apply-report.json']);
  assert.equal(Object.isFrozen(SUCCESS_ARTIFACTS), true);
});
