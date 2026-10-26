// Child of DoctorWatson: owns the actual wallet prover, not just its HTTP client.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {readFileSync, readdirSync, readlinkSync} from 'node:fs';
function ownsPort(pid, port) {
  const inodes = new Set(readdirSync(`/proc/${pid}/fd`).flatMap(fd => {
    try { const match = readlinkSync(`/proc/${pid}/fd/${fd}`).match(/^socket:\[(\d+)\]$/); return match ? [match[1]] : []; }
    catch { return []; }
  }));
  return ['/proc/net/tcp', '/proc/net/tcp6'].some(path => readFileSync(path, 'utf8').trim().split('\n').slice(1).some(line => {
    const fields = line.trim().split(/\s+/);
    return fields[3] === '0A' && parseInt(fields[1].split(':')[1], 16) === port && inodes.has(fields[9]);
  }));
}
const [binary, portArg, parameters] = process.argv.slice(2);
const pause = ms => new Promise(ok => { const timer = setTimeout(ok, ms); timer.unref(); });
let server, exited, resolveExit, control = '';
const exit = new Promise(ok => { resolveExit = ok; });
const stopped = new Promise(ok => { process.stdin.on('end', ok); process.stdin.on('error', ok); });
process.stdin.on('data', chunk => { control += chunk; if (control.length > 16) process.stdin.destroy(); });
process.stdin.resume();
try {
  assert.ok(parameters && process.argv.length === 5);
  assert.match(portArg, /^[1-9][0-9]*$/);
  const port = Number(portArg); assert.ok(port >= 1024 && port <= 65535);
  server = spawn(binary, ['--port', portArg, '--num-workers', '1', '--job-capacity', '1',
    '--job-timeout', '600', '--no-fetch-params'], {env: {...process.env,
      MIDNIGHT_PP: parameters, MIDNIGHT_PROOF_SERVER_VERBOSE: 'false', RAYON_NUM_THREADS: '4'},
    // The official server can include private payloads in errors or verbose logs.
    // No server log bytes are forwarded or persisted, regardless of its settings.
    stdio: ['ignore', 'ignore', 'ignore']});
  server.on('error', () => { exited = true; resolveExit(); });
  server.on('exit', () => { exited = true; resolveExit(); });
  const url = `http://127.0.0.1:${port}`;
  const deadline = Date.now() + 30_000;
  let ready = false;
  while (!ready) {
    assert.ok(!exited && Date.now() < deadline, 'owned prover failed to start');
    try {
      const response = await fetch(url + '/version', {signal: AbortSignal.timeout(500)});
      ready = response.ok && (await response.text()).trim() === '9.0.0-rc.5'
        && !exited && ownsPort(server.pid, port);
    } catch { /* Startup only; no wallet request has been allowed yet. */ }
    if (!ready) await pause(100);
  }
  console.log('SHERLOCK_OWNED_PROVER_READY ' + url);
  await Promise.race([stopped, exit]);
  assert.ok(!exited, 'owned prover exited before the wallet finished');
  assert.equal(control, 'ok\n', 'wallet finalization did not succeed');
} catch {
  console.error('owned wallet prover failed; server/private details suppressed');
  process.exitCode = 1;
} finally {
  if (server && !exited) {
    server.kill('SIGTERM');
    await Promise.race([exit, pause(2_000)]);
    if (!exited) { server.kill('SIGKILL'); await exit; }
  }
  // Closing the worker's process also closes its HTTP pool. The prover has
  // already exited, so no active server job can outlive normal lock release.
  console.log('SHERLOCK_OWNED_PROVER_STOPPED');
}
