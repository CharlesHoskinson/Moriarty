import test from 'node:test';
import assert from 'node:assert/strict';
import {parseDeployArgs, constructorArgs, FR} from './deploy-args.mjs';

const expected = {constructor: ['11', '2', '3', '4', '5', '6', '7', '8', '0', '0']};

test('positional arguments and the submit flag parse as before', () => {
  assert.deepEqual(parseDeployArgs(['s', 'b', 'p', 'o']), {sdkRoot: 's', build: 'b', preparation: 'p', output: 'o', submit: false, instanceId: undefined});
  assert.equal(parseDeployArgs(['s', 'b', 'p', 'o', '--submit']).submit, true);
  assert.throws(() => parseDeployArgs(['s', 'b', 'p']), /usage/);
  assert.throws(() => parseDeployArgs(['s', 'b', 'p', 'o', '--bogus']), /usage/);
});

test('--instance-id overrides constructor argument 0 only', () => {
  const a = parseDeployArgs(['s', 'b', 'p', 'o', '--instance-id', '0x1f', '--submit']);
  assert.equal(a.instanceId, 31n); assert.equal(a.submit, true);
  assert.deepEqual(constructorArgs(expected, a.instanceId), [31n, 2n, 3n, 4n, 5n, 6n, 7n, 8n, 0n, 0n]);
  assert.deepEqual(constructorArgs(expected, undefined), [11n, 2n, 3n, 4n, 5n, 6n, 7n, 8n, 0n, 0n]);
  assert.equal(parseDeployArgs(['s', 'b', 'p', 'o', '--instance-id', '42']).instanceId, 42n);
});

test('the instance id must be a nonzero field element', () => {
  for (const bad of ['0', '0x0', String(FR), '-1', 'abc', ''])
    assert.throws(() => parseDeployArgs(['s', 'b', 'p', 'o', '--instance-id', bad]), /instance/, bad);
  assert.throws(() => parseDeployArgs(['s', 'b', 'p', 'o', '--instance-id']), /instance/);
});
