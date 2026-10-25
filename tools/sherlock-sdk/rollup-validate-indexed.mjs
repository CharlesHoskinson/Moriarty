// Recheck public indexed bytes in a fresh native proof-verifying process.
// The ledger bootstrap is isolated and fee-waived; no wallet or submission.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeFileSync} from 'node:fs';
const sha = bytes => createHash('sha256').update(bytes).digest('hex');

export async function validateIndexedCall({ledger, validator, deploymentRaw, preStateRaw,
  transactionRaw, postStateRaw, blockSeconds, address, entry, logPath}) {
  const deploymentTx = ledger.Transaction.deserialize('signature', 'proof', 'binding', deploymentRaw);
  const deployments = [...deploymentTx.intents.values()].flatMap(intent => intent.actions)
    .filter(action => action instanceof ledger.ContractDeploy);
  assert.equal(deployments.length, 1); assert.equal(deployments[0].address, address);
  const pre = ledger.ContractState.deserialize(preStateRaw);
  const strictness = flags => Object.assign(new ledger.WellFormedStrictness(), flags);
  const now = new Date(Number(blockSeconds) * 1000);
  const block = {secondsSinceEpoch: BigInt(blockSeconds), secondsSinceEpochErr: 30,
    parentBlockHash: '00'.repeat(32), lastBlockTime: BigInt(blockSeconds) - 6n};
  let local = ledger.LedgerState.blank('stagenet');
  local.parameters = ledger.LedgerParameters.initialParameters();
  const bootstrap = ledger.Transaction.fromParts('stagenet', undefined, undefined,
    ledger.Intent.new(new Date(now.getTime() + 3600_000)).addDeploy(deployments[0])).eraseProofs();
  const waived = strictness({enforceBalancing: false, verifyNativeProofs: false,
    verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
  const [installed, setup] = local.apply(bootstrap.wellFormed(local, waived, now),
    new ledger.TransactionContext(local, block));
  assert.equal(setup.type, 'success');
  local = installed.updateIndex(address, pre.data, pre.balance);
  const native = await new Promise((ok, reject) => {
    const child = spawn(validator, [], {stdio: ['pipe', 'pipe', 'pipe']});
    let stdout = '', stderr = '';
    child.stdout.on('data', data => { stdout += data; });
    child.stderr.on('data', data => { stderr += data; });
    child.on('error', reject);
    child.on('close', code => {
      if (logPath) writeFileSync(logPath, stderr);
      if (code !== 0) reject(Error('native indexed-call validation failed'));
      else { try { ok(JSON.parse(stdout)); } catch (error) { reject(error); } }
    });
    child.stdin.end(JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'),
      transaction: Buffer.from(transactionRaw).toString('hex'), block_seconds: Number(blockSeconds)}));
  });
  assert.equal(native.proofVerification, 'native/proof-verifying');
  assert.equal(native.contractProofsSignaturesLimitsAccepted, true, native.contractProfileFailure);
  assert.equal(native.applied, true); assert.equal(native.transactionSHA256, sha(transactionRaw));
  assert.equal(native.contractCalls.length, 1);
  assert.equal(native.contractCalls[0].address, address);
  assert.equal(native.contractCalls[0].entryPoint, entry);
  assert.equal(native.contractCalls[0].proofVersion, 'V3');
  // SDK WASM proof verification is unavailable; cryptographic acceptance above
  // comes from native Rust. This second VM execution checks complete post-state.
  const tx = ledger.Transaction.deserialize('signature', 'proof', 'binding', transactionRaw);
  const contractChecks = strictness({enforceBalancing: false, verifyNativeProofs: false,
    verifyContractProofs: true, enforceLimits: true, verifySignatures: true});
  const [post, applied] = local.apply(tx.wellFormed(local, contractChecks, now),
    new ledger.TransactionContext(local, block));
  assert.equal(applied.type, 'success');
  assert.equal(sha(post.serialize()), native.postStateSHA256, 'native/SDK VM post-state mismatch');
  assert.deepEqual(Buffer.from(post.index(address).serialize()), Buffer.from(postStateRaw), 'indexed complete contract effect mismatch');
  return native;
}
