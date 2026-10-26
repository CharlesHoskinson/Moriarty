// Exercise the actual SDK provider/registry with small public IO fixtures, no proofs.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdtempSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {resolve, join} from 'node:path';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
import test from 'node:test';
import {pinnedRollupKeys} from './rollup-keys.mjs';
const sdk = process.env.SHERLOCK_SDK_ROOT;
assert.ok(sdk);
const require = createRequire(resolve(sdk, 'package.json'));
const types = await import(pathToFileURL(require.resolve('@midnight-ntwrk/midnight-js-types')).href);
const {ZkArtifactIntegrityError} = await import(pathToFileURL(require.resolve('@midnight-ntwrk/midnight-js-utils')).href);

test('pinned manual artifacts resolve through SDK registry and tampering is fatal', async () => {
  const root = mkdtempSync(join(tmpdir(), 'rollup-key-pins-'));
  try {
    const pin = name => {
      const path = join(root, name), bytes = Buffer.from(name);
      writeFileSync(path, bytes);
      return {path, bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex')};
    };
    const record = {keys: {}, provingArtifacts: {}};
    for (const name of ['acceptValidity', 'settleValidity']) {
      record.keys[name] = pin(name + '.verifier');
      record.provingArtifacts[name] = {prover: pin(name + '.prover'), ir: pin(name + '.bzkir')};
    }
    const provider = pinnedRollupKeys(types, ZkArtifactIntegrityError, record);
    const registry = new types.ZKConfigRegistry([provider]);
    for (const name of ['acceptValidity', 'settleValidity']) {
      const location = `contract:${'01'.repeat(32)}/${name}?vk=${record.keys[name].sha256}`;
      const result = await registry.resolveKeyLocation(location);
      assert.deepEqual(Buffer.from(result.proverKey), readFileSync(record.provingArtifacts[name].prover.path));
      assert.deepEqual(Buffer.from(result.zkir), readFileSync(record.provingArtifacts[name].ir.path));
    }
    writeFileSync(record.provingArtifacts.acceptValidity.ir.path, 'changed');
    await assert.rejects(provider.getZKIR('acceptValidity'), ZkArtifactIntegrityError);
    await assert.rejects(provider.getVerifierKey('../settleValidity'), /unsupported/);
  } finally { rmSync(root, {recursive: true, force: true}); }
});
