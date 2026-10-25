// One compiled contract proof; private request arrives only on stdin.
// This worker runs inside DoctorWatson while the parent keeps wallet/indexer waits outside.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {dirname, resolve, join} from 'node:path';
import {spawn} from 'node:child_process';
import {sdkLoader} from './stagenet-session.mjs';
import {pinnedRollupKeys} from './rollup-keys.mjs';
try {
  const [sdkRoot, preparation, location, output] = process.argv.slice(2);
  assert.ok(output && process.argv.length === 6);
  let input = Buffer.alloc(0);
  for await (const chunk of process.stdin) {
    assert.ok(input.length + chunk.length <= 2 * 1024 * 1024);
    input = Buffer.concat([input, chunk]);
  }
  const request = JSON.parse(input.toString());
  assert.deepEqual(Object.keys(request).sort(), ['binding', 'preimage']);
  assert.match(request.binding, /^(0|[1-9][0-9]*)$/);
  assert.match(request.preimage, /^(?:[0-9a-f]{2})+$/);
  const binding = BigInt(request.binding);
  assert.ok(binding < 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n);
  const load = sdkLoader(sdkRoot, join(dirname(resolve(output)), 'compiled-proof-imports'));
  const types = await load('@midnight-ntwrk/midnight-js-types');
  const {ZkArtifactIntegrityError} = await load('@midnight-ntwrk/midnight-js-utils');
  const pins = JSON.parse(readFileSync(preparation));
  const keys = pinnedRollupKeys(types, ZkArtifactIntegrityError, pins);
  const registry = new types.ZKConfigRegistry([keys]);
  await registry.resolveKeyLocation(location);
  const name = 'settleValidity';
  assert.ok(location.includes('/' + name + '?vk='));
  const le = Buffer.from(binding.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
  await new Promise((ok, reject) => {
    const child = spawn(resolve(process.env.COMPILED_CALL_BINARY ?? 'target/release/examples/compiled_call'),
      ['prove', pins.provingArtifacts[name].ir.path, pins.provingArtifacts[name].prover.path,
        pins.keys[name].path, resolve(output), resolve(process.env.MORIARTY_PARENT_SRS ?? 'target/sherlock-srs/bls_midnight_2p20'), location],
      {env: {...process.env, RAYON_NUM_THREADS: '4'}, stdio: ['pipe', 'inherit', 'inherit']});
    child.on('error', reject);
    child.stdin.on('error', error => { if (error.code !== 'EPIPE') reject(Error('compiled input transport failed')); });
    child.on('close', code => code === 0 ? ok() : reject(Error('direct compiled prover failed')));
    child.stdin.end(JSON.stringify({binding_input: le, preimage: request.preimage}));
  });
  console.log('PASS compiled contract proof written; private opening not persisted');
} catch {
  // Provider/codec errors may contain request details. Never echo them.
  console.error('compiled proof worker failed; no private request was logged');
  process.exitCode = 1;
}
