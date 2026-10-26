// Opt-in integration test using public finalized transaction bytes, never a wallet.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {sdkLoader} from './stagenet-session.mjs';
import {validateIndexedCall} from './rollup-validate-indexed.mjs';
const fixturePath = process.env.SHERLOCK_INDEXED_FIXTURE;
const validator = process.env.SHERLOCK_NATIVE_VALIDATOR;
const proofPath = process.env.SHERLOCK_INDEXED_PROOF;

test('indexed bytes pass fresh native verification and a changed proof is refused',
  {skip: !(fixturePath && validator && proofPath)}, async () => {
    const load = sdkLoader(process.env.SHERLOCK_SDK_ROOT, 'target/sherlock-indexed-test/imports');
    const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
    const fixture = JSON.parse(readFileSync(fixturePath));
    for (const name of ['deploymentRaw', 'preStateRaw', 'transactionRaw', 'postStateRaw'])
      fixture[name] = Buffer.from(fixture[name], 'hex');
    const native = await validateIndexedCall({ledger, validator, ...fixture});
    assert.equal(native.applied, true);
    const altered = Buffer.from(fixture.transactionRaw), proof = readFileSync(proofPath);
    const at = altered.indexOf(proof); assert.ok(at >= 0, 'public proof must occur in transaction');
    assert.equal(altered.indexOf(proof, at + 1), -1);
    altered[at + 5] ^= 1;
    await assert.rejects(validateIndexedCall({ledger, validator, ...fixture, transactionRaw: altered}));
    const wrongPost = Buffer.from(fixture.postStateRaw); wrongPost[wrongPost.length - 1] ^= 1;
    await assert.rejects(validateIndexedCall({ledger, validator, ...fixture, postStateRaw: wrongPost}));
  });
