import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {basename, join, resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import assert from 'node:assert/strict';

const digest = bytes => createHash('sha256').update(bytes).digest('hex');

/** Recheck the complete recorded transaction, including fees and native proofs,
 * against the exact input state and the local harness's controlled block. */
export function replayNativeReceipt(validator, directory, row) {
  function readArtifact(artifact) {
    assert.equal(basename(artifact.file), artifact.file);
    const bytes = readFileSync(join(directory, artifact.file));
    assert.equal(digest(bytes), artifact.sha256);
    return bytes;
  }
  const before = readArtifact(row.beforeState), tx = readArtifact(row.transaction);
  const after = readArtifact(row.afterState);
  const block = {uncertainty_seconds: 0, last_block_seconds: row.time, parent_block_hash: '00'.repeat(32)};
  const input = {state: before.toString('hex'), transaction: tx.toString('hex'), block_seconds: row.time, block_context: block};
  const replay = JSON.parse(execFileSync(validator, ['--strict'], {input: JSON.stringify(input), timeout: 120000}));
  assert.equal(replay.schema, 'moriarty-strict-ledger-replay/1');
  assert.deepEqual(replay.blockContext, block);
  assert.equal(replay.blockSeconds, row.time);
  assert.equal(replay.preStateSHA256, digest(before));
  assert.equal(replay.transactionSHA256, digest(tx));
  assert.equal(replay.postStateSHA256, digest(after));
  assert.equal(replay.afterBlockStateSHA256, row.afterBlockStateSha256);
  assert.equal(replay.outcome, row.status);
  return {label: row.label, ...replay};
}

if (import.meta.url === pathToFileURL(resolve(process.argv[1] ?? '')).href) {
  const [, , validator, directory] = process.argv;
  assert.ok(validator && directory && process.argv.length === 4, 'usage: node replay.mjs VALIDATOR EVIDENCE_DIRECTORY');
  const receipts = JSON.parse(readFileSync(join(directory, 'receipts.json')));
  assert.ok(Array.isArray(receipts) && receipts.length > 0, 'missing receipts');
  const replays = receipts.map(row => replayNativeReceipt(validator, directory, row));
  console.log(JSON.stringify(replays, null, 2));
}
