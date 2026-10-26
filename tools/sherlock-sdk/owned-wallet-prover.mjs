// Start only around finalizeRecipe; wallet sync, coin selection and finality stay outside.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {createServer} from 'node:net';
import {resolve, join} from 'node:path';
export async function ownedWalletProver({binary, parameters, output, sha256, purpose,
  worker = resolve('tools/sherlock-sdk/owned-proof-server.mjs')}) {
  assert.ok(binary && parameters && output);
  assert.ok(['deploy', 'acceptValidity', 'settleValidity'].includes(purpose));
  binary = resolve(binary); parameters = resolve(parameters);
  const hash = createHash('sha256').update(readFileSync(binary)).digest('hex');
  if (sha256) assert.equal(hash, sha256);
  writeFileSync(join(output, 'wallet-prover.json'), JSON.stringify({schema: 'sherlock-owned-wallet-prover/1',
    binary, binary_sha256: hash, parameters, purpose, workers: 1, threads: 4,
    scope: 'prover process and lifecycle worker in DoctorWatson process group; stock listener binding preserved; client uses loopback; server logs discarded'}, null, 2) + '\n', {flag: 'wx'});
  const port = await new Promise((ok, reject) => {
    const probe = createServer(); probe.on('error', reject);
    probe.listen(0, '127.0.0.1', () => {
      const selected = probe.address().port;
      probe.close(error => error ? reject(error) : ok(selected));
    });
  });
  const url = `http://127.0.0.1:${port}`;
  let used = false;
  return {url, binary, parameters, sha256: hash, async run(operation) {
    assert.ok(!used, 'one wallet finalization per guard'); used = true;
    const args = ['heavy', '--id', 'B4-wallet-' + purpose, '--budget', 'b4-wallet-' + purpose,
      '--hypothesis', 'Measure and own the native wallet proof server only during finalization; no sync or finality waits',
      '--pipe-stdin', '--', process.execPath, resolve(worker), binary, String(port), parameters];
    const child = spawn(resolve('scripts/doctorwatson'), args, {env: {...process.env,
      CARGO_BUILD_JOBS: '4', RAYON_NUM_THREADS: '4'}, stdio: ['pipe', 'pipe', 'pipe']});
    let logs = '', ready = false, resolveReady, rejectReady, resolveExit, rejectExit;
    const start = new Promise((ok, reject) => { resolveReady = ok; rejectReady = reject; });
    const exit = new Promise((ok, reject) => { resolveExit = ok; rejectExit = reject; });
    // Register the handler immediately; operation may still be waiting on HTTP.
    exit.catch(() => {});
    child.stdout.on('data', chunk => {
      logs += chunk;
      if (!ready && logs.includes('SHERLOCK_OWNED_PROVER_READY ' + url + '\n')) {
        ready = true; resolveReady();
      }
    });
    child.stderr.on('data', chunk => { logs += chunk; });
    child.stdin.on('error', error => { if (error.code !== 'EPIPE') rejectReady(Error('wallet prover control pipe failed')); });
    child.on('error', () => { const error = Error('wallet proof guard could not start'); rejectReady(error); rejectExit(error); });
    child.on('close', code => {
      writeFileSync(join(output, 'wallet-native-proof.log'), logs, {flag: 'wx'});
      if (code === 0) resolveExit();
      else { const error = Error('wallet proof guard failed; inspect public guard log'); rejectReady(error); rejectExit(error); }
    });
    let succeeded = false;
    try {
      await start;
      const result = await Promise.race([operation(), exit.then(() => { throw Error('wallet prover ended during finalization'); })]);
      succeeded = true;
      return result;
    } catch {
      throw Error('wallet finalization failed; private provider details suppressed');
    } finally {
      child.stdin.end(succeeded ? 'ok\n' : 'failed\n');
      await exit;
    }
  }};
}
