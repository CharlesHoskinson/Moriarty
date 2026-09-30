// Local pinned ledger primitive experiment; no connector wallet or ledger acceptance.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { PINNED_NM } from '../../../../moriarty-midnight-financial/ledger/providers.mjs';

const ledger = await import(pathToFileURL(join(PINNED_NM,
  '@midnight-ntwrk/ledger-v8/midnight_ledger_wasm_fs.js')).href);
const fixturesPath = fileURLToPath(new URL('../wire/fixtures.json', import.meta.url));
const fixtures = JSON.parse(readFileSync(fixturesPath, 'utf8')).fixtures;
const hash = bytes => createHash('sha256').update(bytes).digest();
const signedMessage = digest => Buffer.concat([
  Buffer.from('midnight_signed_message:32:', 'ascii'), digest,
]);

test('pinned ledger primitive verifies the prefixed canonical digest input only', () => {
  const transfer = fixtures.find(f => f.id === 'transfer');
  const repayment = fixtures.find(f => f.id === 'repayment');
  const C = Buffer.from(transfer.expected.wireHex, 'hex');
  const D = hash(C);
  const M = signedMessage(D);
  const otherM = signedMessage(hash(Buffer.from(repayment.expected.wireHex, 'hex')));
  assert.equal(C.length, transfer.expected.length);
  assert.equal(D.toString('hex'), transfer.expected.digestHex);
  assert.equal(D.length, 32);
  assert.equal(M.length, 59);
  const sk = ledger.sampleSigningKey();
  const vk = ledger.signatureVerifyingKey(sk);
  const signature = ledger.signData(sk, M);
  assert.equal(ledger.verifySignature(vk, M, signature), true);
  assert.equal(ledger.verifySignature(vk, D, signature), false);
  assert.equal(ledger.verifySignature(vk, hash(M), signature), false);
  assert.equal(ledger.verifySignature(vk, otherM, signature), false);
  assert.equal(ledger.verifySignature(vk,
    Buffer.concat([Buffer.from('midnight_signed_message:32:', 'ascii'), M]), signature), false);
  const changed = Buffer.from(M);
  changed[changed.length - 1] ^= 1;
  assert.equal(ledger.verifySignature(vk, changed, signature), false);
});
