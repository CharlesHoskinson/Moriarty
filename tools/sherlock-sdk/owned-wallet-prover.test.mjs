import assert from 'node:assert/strict';
import {mkdtempSync, mkdirSync, writeFileSync, chmodSync, readFileSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {ownedWalletProver} from './owned-wallet-prover.mjs';
const worker = new URL('./owned-proof-server.mjs', import.meta.url).pathname;
const active = pid => {try{return !['Z','X'].includes(readFileSync(`/proc/${pid}/stat`,'utf8').split(') ')[1].split(' ')[0]);}catch{return false;}};

test('parent waits for prover cleanup and suppresses private operation errors', async () => {
  const root = mkdtempSync(join(tmpdir(), 'wallet-prover-parent-')), original = process.cwd();
  try {
    mkdirSync(join(root, 'scripts'));
    // The factory's admission/cap transport has its own tests; this fixture
    // exercises parent/worker control without taking the shared heavy slot.
    const doctor = join(root, 'scripts/doctorwatson');
    writeFileSync(doctor, `#!/usr/bin/env node
import {spawn} from 'node:child_process';
const args=process.argv.slice(process.argv.indexOf('--')+1);
const child=spawn(args[0],args.slice(1),{stdio:['pipe','inherit','inherit']});
process.stdin.pipe(child.stdin);child.on('exit',code=>{process.exitCode=code;});
`); chmodSync(doctor, 0o755);
    const binary = join(root, 'fake-server.mjs');
    writeFileSync(binary, `#!/usr/bin/env node
import {createServer} from 'node:http';
const port=Number(process.argv[process.argv.indexOf('--port')+1]);
createServer((q,r)=>r.end(q.url==='/version'?'9.0.0-rc.5':String(process.pid))).listen(port,'127.0.0.1');
`); chmodSync(binary, 0o755);
    process.chdir(root);
    for (const fail of [false, true]) {
      const output = join(root, fail ? 'failure' : 'success'); mkdirSync(output);
      const guard = await ownedWalletProver({binary, parameters: root, output, worker, purpose: 'deploy'});
      let pid;
      const operation = async () => {
        pid = Number(await (await fetch(guard.url + '/pid')).text());
        if (fail) throw Error('PRIVATE_WALLET_ERROR_TEST_SENTINEL');
        return 'complete';
      };
      if (fail) await assert.rejects(guard.run(operation), error => !error.message.includes('PRIVATE_WALLET_ERROR_TEST_SENTINEL'));
      else assert.equal(await guard.run(operation), 'complete');
      assert.equal(active(pid), false, 'prover must exit before parent returns');
      assert.ok(!readFileSync(join(output, 'wallet-native-proof.log'), 'utf8').includes('PRIVATE_WALLET_ERROR_TEST_SENTINEL'));
      await assert.rejects(guard.run(operation), /one wallet finalization/);
    }
  } finally { process.chdir(original); rmSync(root, {recursive: true, force: true}); }
});
