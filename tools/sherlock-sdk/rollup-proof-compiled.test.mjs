import assert from 'node:assert/strict';
import {mkdtempSync, existsSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawnSync} from 'node:child_process';
import test from 'node:test';

test('compiled proof worker errors never echo the private request or write it', () => {
  const root = mkdtempSync(join(tmpdir(), 'rollup-proof-worker-'));
  const output = join(root, 'proof');
  const sentinel = 'PRIVATE_OPENING_TEST_SENTINEL';
  try {
    for (const request of [{binding: '0', preimage: sentinel},
      {binding: '01', preimage: '00', extra: sentinel}, {binding: sentinel, preimage: '00'}]) {
      const result = spawnSync(process.execPath, ['tools/sherlock-sdk/rollup-proof-compiled.mjs',
        root, join(root, 'preparation.json'), 'test-location', output],
      {input: JSON.stringify(request), encoding: 'utf8'});
      assert.equal(result.status, 1);
      assert.ok(!result.stdout.includes(sentinel) && !result.stderr.includes(sentinel));
      assert.equal(existsSync(output), false);
    }
  } finally { rmSync(root, {recursive: true, force: true}); }
});
