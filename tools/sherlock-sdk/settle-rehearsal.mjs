// Local native ledger-9 rehearsal of the A9 settle cases through settle-harness.mjs. No wallet, no submission.
// Usage (repo root): node tools/sherlock-sdk/settle-rehearsal.mjs SDK_ROOT OUTPUT [--dry]
//   --dry   stop every case at its proof request (no proof server proof; no heavy slot needed)
//   else    prove each case with the proof server (run under `scripts/doctorwatson heavy`) and validate it with
//           the native proof-verifying validate_call over a local ledger that holds the pinned deployment.
// Cases, all from the A8 post-state S0 (validity_ok = {A8 stmt}) and the A3 parent fields:
//   C   control settle over S0, applied to S0                                   -> must apply
//   P1  stale head (s-c, s-e iii): a second settle prepared over S0, applied to post-C  -> must not apply
//   P1' honest second settle prepared over post-C                               -> SettleUnsatisfiable (local)
//   P2  forged head (s-c): S0 with root R' claiming stmt(S0'), applied to S0   -> must not apply
//   P3  reordered batch (s-d): swapped fields claiming their stmt, applied to S0 -> must not apply
//   P4  other domain (s-e iv): instance_id I2 deployment claiming stmt(I2), applied to that deployment -> must not apply
// Every refused transaction is also applied to the snapshot it was proven over, which must apply: the refusal is
// then caused only by the ledger state the transaction reads. Receipts hold hashes and verdicts only.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {sdkLoader, endpoints} from './stagenet-session.mjs';
import {prepareSettle, predictSettle, settleTrace, settleStatement, editSnapshot, ledgerView} from './settle-harness.mjs';

const [sdkRoot, outputArg, flag] = process.argv.slice(2);
assert.ok(sdkRoot && outputArg && (flag === undefined || flag === '--dry'), 'usage: settle-rehearsal.mjs SDK_ROOT OUTPUT [--dry]');
const dry = flag === '--dry';
const output = resolve(outputArg); mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'receipt.json')), 'rehearsal already recorded in this output directory');
const load = sdkLoader(sdkRoot, join(output, 'sdk-imports'));
const sha = b => createHash('sha256').update(b).digest('hex');
const fromLe = s => BigInt('0x' + Buffer.from(s, 'hex').reverse().toString('hex'));
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';
const fixture = name => readFileSync(new URL(`./fixtures/settle/${name}`, import.meta.url));
const log = m => console.log(`[settle-rehearsal] ${m}`);
const pins = JSON.parse(readFileSync('crates/mori-recursion/tests/fixtures/stagenet-verifier.json'));
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const graphql = async query => {
  const r = await fetch(endpoints.indexer, {method: 'POST', headers: {'content-type': 'application/json'}, body: JSON.stringify({query})});
  assert.ok(r.ok); const body = await r.json(); assert.ok(!body.errors, 'indexer query failed'); return body.data;
};
const child = (command, argv, input, name) => new Promise((ok, reject) => {
  const p = spawn(command, argv, {stdio: ['pipe', 'pipe', 'pipe']}); let out = '', err = '';
  p.stdout.on('data', b => { out += b; }); p.stderr.on('data', b => { err += b; }); p.on('error', reject);
  p.on('close', code => { writeFileSync(join(output, name + '-stderr.log'), err); code === 0 ? ok(out) : reject(Error('child failed: ' + name)); });
  p.stdin.end(input);
});

const pub = JSON.parse(fixture('a3-a8.json'));
const fields = pub.fieldsLe.map(fromLe), A8_STMT = fromLe(pub.a8StmtLe);
const S0 = ledger.ContractState.deserialize(fixture('s0-a8-post-state.bin'));
const deployedEmpty = ledger.ContractState.deserialize(fixture('deployed-state.bin'));
assert.equal(settleStatement(S0, 0n, fields), A8_STMT);
assert.ok(ledgerView(S0).validity_ok.member(A8_STMT));
const I2 = BigInt('0x' + sha('sherlock A9 s-e(iv) second instance_id')) % 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;

// Network parameters only (cost model, local ledger); contract state comes from the fixtures.
const {parseHexLedgerParameters} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const ledgerParameters = parseHexLedgerParameters((await graphql('{block{ledgerParameters}}')).block.ledgerParameters);

// A session that only binds: the harness's balancing step yields the bound, unbalanced transaction.
const session = {walletProvider: {getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32),
  balanceTx: async tx => ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', tx.serialize()).bind(),
  submitTx: () => { throw Error('rehearsal never submits'); }}};
const STOP = 'STOP_AT_PROOF';
const stopProver = {check: () => { throw Error(STOP); }, lookupKey: () => { throw Error(STOP); }, prove: () => { throw Error(STOP); }};

let deployAction;
const nativeValidate = async (tx, contractState, name) => {
  if (!deployAction) {
    const deployed = JSON.parse(readFileSync('target/a4-anchor/deploy-run3/receipt.json'));
    assert.equal(deployed.address, pins.address);
    const data = await graphql(`{transactions(offset:{identifier:"${deployed.txId}"}){raw}}`);
    const dtx = ledger.Transaction.deserialize('signature', 'proof', 'binding', Buffer.from(data.transactions[0].raw, 'hex'));
    const deps = [...dtx.intents.values()].flatMap(i => i.actions).filter(a => a instanceof ledger.ContractDeploy);
    assert.equal(deps.length, 1); assert.equal(deps[0].address, pins.address);
    deployAction = deps[0];
  }
  let local = ledger.LedgerState.blank('stagenet'); local.parameters = ledgerParameters;
  const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(new Date(Date.now() + 3600_000)).addDeploy(deployAction)).eraseProofs();
  const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
  const secs = BigInt(Math.floor(Date.now() / 1000)), block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
  const [boot, result] = local.apply(setup.wellFormed(local, lax, new Date()), new ledger.TransactionContext(local, block));
  assert.equal(result.type, 'success');
  const st = ledger.ContractState.deserialize(contractState.serialize());
  local = boot.updateIndex(pins.address, st.data, st.balance);
  const report = JSON.parse(await child(resolve('target/release/examples/validate_call'), [],
    JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'), transaction: Buffer.from(tx.serialize()).toString('hex'), block_seconds: Number(secs)}), name));
  assert.equal(report.proofVerification, 'native/proof-verifying');
  const {contractProofsSignaturesLimitsAccepted, contractProfileFailure, applied, transactionSHA256} = report;
  return {contractStateSHA256: sha(contractState.serialize()), contractProofsSignaturesLimitsAccepted, contractProfileFailure, applied,
    transactionSHA256, calls: report.contractCalls.map(c => ({address: c.address, entryPoint: c.entryPoint, proofSHA256: c.proofSHA256}))};
};
const prepare = async (name, snapshot, batch, f) => {
  log(`prepare ${name}`);
  try {
    return await prepareSettle({load, session, snapshot, batch, fields: f, address: pins.address, ledgerParameters, prover: dry ? stopProver : undefined});
  } catch (error) {
    if (dry && String(error?.message).includes(STOP)) return {stopped: true};
    throw error;
  }
};

const cases = {};
// C and P1: two independent preparations over the same live snapshot.
const C = await prepare('C', S0, 0n, fields);
const P1 = await prepare('P1', S0, 0n, fields);
// P1': an honest second settle prepared over the VM-predicted post-C state.
const postCState = await predictSettle(S0, 0n, fields, pins.address);
const vPost = ledgerView(postCState);
assert.ok(vPost.settled.member(A8_STMT) && !vPost.validity_ok.member(A8_STMT) && vPost.root === fields[9] && vPost.batch_no === 1n);
for (const b of [0n, 1n])
  await assert.rejects(prepareSettle({load, session, snapshot: postCState, batch: b, fields, ledgerParameters}),
    e => e.name === 'SettleUnsatisfiable');
cases['P1-honest-second-settle'] = {refusedLocally: 'SettleUnsatisfiable', note: 'local refusal; does not count as A9 evidence'};
// P2: forged head R' = R + 1.
const forged = editSnapshot(S0, {root: fields[6] + 1n});
const f2 = [...fields]; f2[6] = fields[6] + 1n; f2[15] = settleTrace(forged, f2);
const S0p = editSnapshot(forged, {validity_ok_add: [settleStatement(forged, 0n, f2)]});
const P2 = await prepare('P2', S0p, 0n, f2);
// P3: swapped order claiming its own statement.
const f3 = [...fields]; [f3[8], f3[9]] = [fields[9], fields[8]]; [f3[11], f3[12]] = [fields[12], fields[11]]; f3[15] = settleTrace(S0, f3);
const S0pp = editSnapshot(S0, {validity_ok_add: [settleStatement(S0, 0n, f3)]});
const P3 = await prepare('P3', S0pp, 0n, f3);
// P4: a second deployment with instance_id I2 that holds the replayed A8 statement (domain I), claiming stmt(I2).
const D3 = editSnapshot(deployedEmpty, {instance_id: I2, validity_ok_add: [A8_STMT]});
const D3claimed = editSnapshot(D3, {validity_ok_add: [settleStatement(D3, 0n, fields)]});
assert.notEqual(settleStatement(D3, 0n, fields), A8_STMT);
const P4 = await prepare('P4', D3claimed, 0n, fields);

if (dry) {
  for (const [k, v] of Object.entries({C, P1, P2, P3, P4})) assert.ok(v.stopped, `${k} must reach its proof request`);
  writeFileSync(join(output, 'receipt.json'), json({schema: 'sherlock-settle-rehearsal/1', mode: 'dry', reachedProof: ['C', 'P1', 'P2', 'P3', 'P4'], cases}));
  log('PASS dry: every case reached its one settleValidity proof request; P1 honest second settle is SettleUnsatisfiable');
  process.exit(0);
}
const expectApplied = (r, want, what) => {
  assert.equal(r.contractProofsSignaturesLimitsAccepted && r.applied, want, `${what}: ${json(r)}`);
  return r;
};
assert.equal(C.report.predictedPostStateSHA256, sha(postCState.serialize()));
const C_on_S0 = expectApplied(await nativeValidate(C.tx, S0, 'C-on-S0'), true, 'C on S0');
cases.C = {report: C.report, onSnapshot: C_on_S0};
cases.P1 = {report: P1.report, onSnapshot: expectApplied(await nativeValidate(P1.tx, S0, 'P1-on-S0'), true, 'P1 on S0'),
  onTarget: expectApplied(await nativeValidate(P1.tx, postCState, 'P1-on-postC'), false, 'P1 on post-C')};
cases.P2 = {report: P2.report, onSnapshot: expectApplied(await nativeValidate(P2.tx, S0p, 'P2-on-S0p'), true, 'P2 on S0\''),
  onTarget: expectApplied(await nativeValidate(P2.tx, S0, 'P2-on-S0'), false, 'P2 on S0')};
cases.P3 = {report: P3.report, onSnapshot: expectApplied(await nativeValidate(P3.tx, S0pp, 'P3-on-S0pp'), true, 'P3 on S0\'\''),
  onTarget: expectApplied(await nativeValidate(P3.tx, S0, 'P3-on-S0'), false, 'P3 on S0')};
cases.P4 = {report: P4.report, onSnapshot: expectApplied(await nativeValidate(P4.tx, D3claimed, 'P4-on-D3claimed'), true, 'P4 on D3 claimed'),
  onTarget: expectApplied(await nativeValidate(P4.tx, D3, 'P4-on-D3'), false, 'P4 on D3')};
writeFileSync(join(output, 'receipt.json'), json({schema: 'sherlock-settle-rehearsal/1', mode: 'native', address: pins.address,
  instanceId2: I2, s0SHA256: sha(S0.serialize()), cases}));
log('PASS native: C applies; P1 (stale), P2 (forged head), P3 (reordered), P4 (other domain) are refused by native ledger-9 and apply only over their own snapshots');
process.exit(0);
