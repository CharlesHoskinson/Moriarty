// Witness checks stay local too: no private SDK preimage enters a shared daemon.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {resolve, join} from 'node:path';
export async function checkCompiledCall(raw, location, {ir, prover, verifier, output}) {
  const stdout = await new Promise((ok, reject) => {
    const child = spawn(resolve(process.env.COMPILED_CALL_BINARY ?? 'target/release/examples/compiled_call'),
      ['check', resolve(ir), resolve(prover), resolve(verifier), join(resolve(output), 'unused-check-proof'),
        resolve(process.env.MORIARTY_PARENT_SRS ?? 'target/sherlock-srs/bls_midnight_2p20'), location],
      {env: {...process.env, RAYON_NUM_THREADS: '4'}, stdio: ['pipe', 'pipe', 'ignore']});
    let result = '';
    child.stdout.on('data', data => { result += data; });
    child.stdin.on('error', error => { if (error.code !== 'EPIPE') reject(Error('native check transport failed')); });
    child.on('error', () => reject(Error('native compiled check could not start')));
    child.on('close', code => code === 0 ? ok(result) : reject(Error('native compiled witness check failed')));
    // check preserves the original binding field, as the official check endpoint does.
    child.stdin.end(JSON.stringify({binding_input: '00'.repeat(32), preimage: Buffer.from(raw).toString('hex')}));
  });
  const result = JSON.parse(stdout);
  assert.equal(result.schema, 'sherlock-compiled-call-check/1');
  assert.ok(Array.isArray(result.pi_skips));
  return result.pi_skips.map(value => {
    assert.ok(value === null || (Number.isSafeInteger(value) && value >= 0));
    return value === null ? undefined : value;
  });
}
