import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { compilePublic } from '../src/compile-public.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, '..');
const manifest = JSON.parse(readFileSync(join(root, 'examples/revision-step.manifest.json'), 'utf8'));

const REQUIRED = [
  'positive_step',
  'missing_continuation',
  'multiple_continuation',
  'wrong_token',
  'wrong_address',
  'wrong_datum',
  'changed_result',
  'signer_missing',
  'stale_program_hash',
  'conflicting_predecessor',
  'shared_payout',
  'unbounded_time',
  'arithmetic_boundary_count',
  'wallet_debit_required',
  'cross_script_head',
  'forged_genesis_values',
  'prestate_out_of_range',
  'argument_out_of_range',
  'shortcircuit_or',
  'shortcircuit_and',
];

test('compiles revision-step through Aiken check/build to UPLC with Transaction negatives', () => {
  const result = compilePublic({
    source: join(root, 'examples/revision-step.mori'),
    manifest,
    outDir: join(root, 'artifacts'),
  });
  if (!result.ok) {
    assert.fail(`aiken failed check=${result.check?.status} build=${result.build?.status}\n${result.check?.stderr}\n${result.check?.stdout}\n${result.build?.stderr}\nmissing=${JSON.stringify(result.check?.missingTests)}`);
  }
  assert.equal(result.check.status, 0);
  assert.equal(result.build.status, 0);
  assert.equal(result.positiveUplcValidation, true);
  assert.ok(existsSync(join(root, 'artifacts/plutus.json')));
  assert.ok(existsSync(join(root, 'artifacts/sample-datum.json')));
  assert.ok(existsSync(join(root, 'artifacts/sample-redeemer.json')));
  assert.ok(existsSync(join(root, 'artifacts/data-schemas.json')));
  const blueprint = JSON.parse(readFileSync(join(root, 'artifacts/plutus.json'), 'utf8'));
  assert.ok(blueprint.validators?.length >= 1);
  assert.ok(result.check.parseOk, `aiken JSON parse failed: ${JSON.stringify(result.check.parseErrors)}`);
  const titles = (result.check.tests || []).map(t => t.title);
  for (const name of REQUIRED) {
    const found = result.check.tests.find(t => t.title === name || t.title.endsWith(`.${name}`) || t.title.endsWith(name));
    assert.ok(found, `expected exact Aiken test ${name}, got ${JSON.stringify(titles)}`);
    assert.equal(found.status, 'success', `test ${name} status ${found.status}`);
  }
  assert.equal(result.check.missingTests.length, 0);
});
