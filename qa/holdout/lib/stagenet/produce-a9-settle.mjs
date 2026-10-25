// Evaluator-owned A9 settle evidence: s-c, s-d, s-e (iii) on D2 and s-e (iv) on D3, through the implementer's
// tools/sherlock-sdk/settle-harness.mjs. Usage (repo root):
//   node qa/holdout/lib/stagenet/produce-a9-settle.mjs SDK_ROOT PARENT OUTPUT [--submit]
// Phase 0 (read-only): live D2 snapshot S0 (A8 statement accepted, nothing settled) and live D3 snapshot.
// Phase 1 (proofs; heavy.lock held only here, ACQUIRE/RELEASE posted on #sherlock): prove C, P1, P2, P3, P4 with
//   settle-harness.prepareSettle over their claimed snapshots, unbalanced, and predict each with native
//   proof-verifying ledger-9: own snapshot must apply, target must not (C: S0 must apply).
// Phase 2 (--submit; WALLET BUSY/FREE on #sherlock; one fresh wallet session per transaction, because a refused
//   transaction keeps its dust locked locally):  P2, P3 on S0  ->  C (control, must finalize)  ->  P1 (stale)  ->  P4 on D3.
// Never writes or logs a transaction body or wallet material. Output lines prefixed [a9s] go to OUTPUT/settle.log.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {homedir} from 'node:os';
import {sdkLoader, endpoints, openStagenetSession} from '../../../../tools/sherlock-sdk/stagenet-session.mjs';
import {prepareSettle, predictSettle, settleTrace, settleStatement, editSnapshot, ledgerView}
  from '../../../../tools/sherlock-sdk/settle-harness.mjs';

const [sdkRoot, parent, outputArg, mode] = process.argv.slice(2);
assert.ok(sdkRoot && parent && outputArg && (mode === undefined || mode === '--submit'),
  'usage: produce-a9-settle.mjs SDK_ROOT PARENT OUTPUT [--submit]');
const submit = mode === '--submit';
const output = resolve(outputArg); mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'receipt.json')), 'evidence already recorded in this output directory');
const caseOf = id => JSON.parse(readFileSync(`qa/holdout/cases/rejections/${id}/case.json`));
const D2 = caseOf('s-c-wrong-predecessor').target.contract, D3 = caseOf('s-e-replayed-statement').target.settleOtherDomain.contract;
const DEPLOY_TX = {[D2]: '009dda35e550cbb3f96ad1138970d72cf12524c67c3fb5e910671535989d460f07',
  [D3]: '0077429fcaffd690e3a2a09373b453edf220372d774f862abf0f9042dd2c4bb273'};
const D2_S0 = 'e4bb458942c0ead4', A8_STMT_LE = 'deffb4974d1eb0b62b1ffb6d709c439e142efd51a2bcc94405fa864186a5c25a';
const sha = b => createHash('sha256').update(b).digest('hex');
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = s => BigInt('0x' + Buffer.from(s, 'hex').reverse().toString('hex'));
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';
const log = m => console.log(`[a9s] ${m}`);
const bus = (subj, body) => new Promise(ok => {   // coordination posts; never block the run on the bus
  const p = spawn('agentbus', ['--cwd', join(homedir(), 'repos/loki-wt/ids/sherlock-eval'), 'send', '#sherlock', subj, '--body', body], {stdio: 'ignore'});
  p.on('close', ok); p.on('error', ok);
});
log(`output ${output}`);
const load = sdkLoader(sdkRoot, join(output, 'sdk-imports'));
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
const {WebSocket} = await load('ws'); globalThis.WebSocket = WebSocket;
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const {indexerPublicDataProvider, parseHexLedgerParameters} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const pdp = indexerPublicDataProvider(endpoints.indexer, endpoints.indexerWs);
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
const refusal = error => {   // tags and messages along the Effect cause chain; long hex removed
  const out = [], seen = new Set();
  const visit = (e, d) => {
    if (!e || typeof e !== 'object' || seen.has(e) || d > 8 || out.length >= 16) return; seen.add(e);
    const tag = e._tag ?? e.name, msg = typeof e.message === 'string' ? e.message : null;
    if (tag || msg) out.push({tag: String(tag ?? ''), message: String(msg ?? '').replace(/[0-9a-fA-F]{129,}/g, '<hex>').slice(0, 600)});
    for (const k of ['cause', 'error', 'failure', 'reason', 'left']) if (k in e) visit(e[k], d + 1);
    for (const k of Object.getOwnPropertySymbols(e)) if (/Cause/.test(String(k))) visit(e[k], d + 1);
  };
  visit(error, 0); if (!out.length) out.push({tag: typeof error, message: String(error).slice(0, 600)});
  return out;
};

// Phase 0: live snapshots and fields.
const fields = JSON.parse(readFileSync(join(parent, 'w.instance.json'))).slice(0, 16).map(fromLe);
const S0 = await pdp.queryContractState(D2), D3s = await pdp.queryContractState(D3);
assert.ok(S0 && D3s);
const s0sha = sha(S0.serialize()), d3sha = sha(D3s.serialize());
assert.ok(s0sha.startsWith(D2_S0), 'D2 is not at its post-acceptance state S0');
const A8_STMT = fromLe(A8_STMT_LE), v0 = ledgerView(S0), v3 = ledgerView(D3s);
assert.equal(settleStatement(S0, 0n, fields), A8_STMT, 'the A3 fields do not give the A8 statement on D2');
assert.ok(v0.validity_ok.member(A8_STMT) && !v0.settled.member(A8_STMT) && v0.batch_no === 0n);
assert.equal(v3.instance_id, BigInt(caseOf('s-e-replayed-statement').target.settleOtherDomain.instanceId));
assert.equal(v3.batch_no, 0n);
const ledgerParameters = parseHexLedgerParameters((await graphql('{block{ledgerParameters}}')).block.ledgerParameters);
const stmtD3 = settleStatement(D3s, 0n, fields);
assert.notEqual(stmtD3, A8_STMT, 'instance_id must separate the domains');
assert.ok(!v3.validity_ok.member(stmtD3));
const forged = editSnapshot(S0, {root: fields[6] + 1n});
const f2 = [...fields]; f2[6] = fields[6] + 1n; f2[15] = settleTrace(forged, f2);
const S0p = editSnapshot(forged, {validity_ok_add: [settleStatement(forged, 0n, f2)]});
const f3 = [...fields]; [f3[8], f3[9]] = [fields[9], fields[8]]; [f3[11], f3[12]] = [fields[12], fields[11]]; f3[15] = settleTrace(S0, f3);
const S0pp = editSnapshot(S0, {validity_ok_add: [settleStatement(S0, 0n, f3)]});
const D3claimed = editSnapshot(D3s, {validity_ok_add: [stmtD3]});
const postC = await predictSettle(S0, 0n, fields, D2);
const vC = ledgerView(postC);
assert.ok(vC.settled.member(A8_STMT) && !vC.validity_ok.member(A8_STMT) && vC.root === fields[9] && vC.height === v0.height + 2n && vC.batch_no === 1n);

// Native proof-verifying predictor over a local ledger holding the target's real deployment.
const deployOf = {};
const nativeValidate = async (address, contractState, tx, name) => {
  if (!deployOf[address]) {
    const data = await graphql(`{transactions(offset:{identifier:"${DEPLOY_TX[address]}"}){raw}}`);
    const dtx = ledger.Transaction.deserialize('signature', 'proof', 'binding', Buffer.from(data.transactions[0].raw, 'hex'));
    const deps = [...dtx.intents.values()].flatMap(i => i.actions).filter(a => a instanceof ledger.ContractDeploy);
    assert.equal(deps.length, 1); assert.equal(deps[0].address, address); deployOf[address] = deps[0];
  }
  let local = ledger.LedgerState.blank('stagenet'); local.parameters = ledgerParameters;
  const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(new Date(Date.now() + 3600_000)).addDeploy(deployOf[address])).eraseProofs();
  const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
  const secs = BigInt(Math.floor(Date.now() / 1000)), block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
  const [boot, result] = local.apply(setup.wellFormed(local, lax, new Date()), new ledger.TransactionContext(local, block));
  assert.equal(result.type, 'success');
  const st = ledger.ContractState.deserialize(contractState.serialize());
  local = boot.updateIndex(address, st.data, st.balance);
  const r = JSON.parse(await child(resolve('target/release/examples/validate_call'), [],
    JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'), transaction: Buffer.from(tx.serialize()).toString('hex'), block_seconds: Number(secs)}), name));
  assert.equal(r.proofVerification, 'native/proof-verifying');
  return {stateSHA256: sha(contractState.serialize()), accepted: r.contractProofsSignaturesLimitsAccepted, failure: r.contractProfileFailure,
    applied: r.applied, calls: r.contractCalls.map(c => ({address: c.address, entryPoint: c.entryPoint}))};
};
const applies = r => r.accepted === true && r.applied === true;

// Phase 1: proofs only; heavy.lock held through flock for exactly this phase.
const lockPath = process.env.DW_LOCK ?? join(homedir(), '.cache/doctorwatson/heavy.lock');
const acquire = () => new Promise((ok, reject) => {
  const p = spawn('flock', ['-n', lockPath, '-c', 'echo held; cat >/dev/null'], {stdio: ['pipe', 'pipe', 'ignore']});
  p.stdout.once('data', () => ok(p)); p.on('close', code => reject(Error(`heavy lock busy (flock exit ${code})`)));
});
const capture = {walletProvider: {getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32),
  balanceTx: async tx => tx, submitTx: () => { throw Error('phase 1 never submits'); }}};
const plan = {C: [S0, fields, D2, S0, true], P1: [S0, fields, D2, postC, false], P2: [S0p, f2, D2, S0, false],
  P3: [S0pp, f3, D2, S0, false], P4: [D3claimed, fields, D3, D3s, false]};
const prepared = {}, prediction = {};
let lock;
for (;;) { try { lock = await acquire(); break; } catch { log('heavy lock busy; waiting'); await new Promise(r => setTimeout(r, 20_000)); } }
await bus('ACQUIRE A9-settle-prove (sherlock-eval)', 'heavy.lock held for five settleValidity proof-server proofs only; no wallet.');
log('heavy acquired for proofs');
try {
  for (const [name, [snap, f]] of Object.entries(plan)) {
    const {tx, report} = await prepareSettle({load, session: capture, snapshot: snap, batch: 0n, fields: f,
      address: plan[name][2], ledgerParameters, ttl: new Date(Date.now() + 2 * 3600_000)});
    prepared[name] = {proven: tx, report, provenAt: new Date().toISOString()};
    log(`${name} proven over ${report.snapshotSHA256.slice(0, 16)}`);
  }
} finally {
  lock.stdin.end();
  await bus('RELEASE A9-settle-prove (sherlock-eval)', 'heavy.lock released; settle proofs done.');
  log('heavy released');
}
for (const [name, [snap, , address, target, wantOnTarget]] of Object.entries(plan)) {
  const bound = ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', prepared[name].proven.serialize()).bind();
  prediction[name] = {onSnapshot: await nativeValidate(address, snap, bound, `native-${name}-own`),
    onTarget: name === 'C' ? null : await nativeValidate(address, target, bound, `native-${name}-target`)};
  assert.ok(applies(prediction[name].onSnapshot), `${name} must apply over its own snapshot`);
  if (name !== 'C') assert.ok(!applies(prediction[name].onTarget), `${name} must not apply on its target`);
}
assert.equal(prepared.C.report.predictedPostStateSHA256, sha(postC.serialize()));
const receipt = {schema: 'sherlock-a9-settle/1', network: 'stagenet', submitted: submit, d2: D2, d3: D3,
  s0SHA256: s0sha, d3StateSHA256: d3sha, postCPredictedSHA256: sha(postC.serialize()), a8Stmt: le(A8_STMT), stmtD3: le(stmtD3),
  reports: Object.fromEntries(Object.entries(prepared).map(([k, v]) => [k, {...v.report, stmt: le(v.report.stmt), trace: le(v.report.trace), provenAt: v.provenAt}])),
  prediction};
log('PASS phase 1: C applies over S0; P1..P4 apply over their own snapshots and not on their targets');
if (!submit) { writeFileSync(join(output, 'preparation.json'), json(receipt)); process.exit(0); }

// Phase 2: wallet. One fresh session per transaction.
const latestAction = async address => (await graphql(`{contractAction(address:"${address}"){__typename state transaction{hash}}}`)).contractAction;
const txStatus = async hash => {
  for (let i = 0; i < 36; i++) {
    const t = (await graphql(`{transactions(offset:{hash:"${hash}"}){hash block{height hash} ... on RegularTransaction{transactionResult{status}}}}`)).transactions;
    if (t.length) return t;
    await new Promise(r => setTimeout(r, 5000));
  }
  return [];
};
receipt.attempts = [];
let n = 0;
const attempt = async (name, address) => {
  const session = await openStagenetSession(load, join(output, `session-${++n}`));
  try {
    const before = await latestAction(address);
    const tx = await session.walletProvider.balanceTx(prepared[name].proven, new Date(Date.now() + 3600_000));
    const hash = sha(Buffer.from(tx.serialize()));
    const a = {name, address, txSHA256: hash, at: new Date().toISOString(), stateBeforeSHA256: sha(Buffer.from(before.state, 'hex'))};
    try { await session.walletProvider.submitTx(tx); a.nodeOutcome = 'accepted'; log(`${name}: node ACCEPTED ${hash}`); }
    catch (error) { a.nodeOutcome = 'refused'; a.refusal = refusal(error); log(`${name}: node refused ${hash}`); }
    a.indexed = a.nodeOutcome === 'accepted' ? await txStatus(hash) : (await graphql(`{transactions(offset:{hash:"${hash}"}){hash}}`)).transactions;
    const after = await latestAction(address);
    a.latestActionAfter = {type: after.__typename, tx: after.transaction.hash, stateSHA256: sha(Buffer.from(after.state, 'hex'))};
    receipt.attempts.push(a);
    return a;
  } finally { await session.close(); }
};
await bus('WALLET BUSY sherlock-eval A9 settle', 'sherlock-eval holds ~/.midnight/stagenet-wallet: P2, P3, C, P1 on D2 then P4 on D3 (one session each, ~30 min). Please hold wallet transactions until WALLET FREE.');
try {
  await attempt('P2', D2);
  await attempt('P3', D2);
  const c = await attempt('C', D2);
  assert.equal(c.nodeOutcome, 'accepted', 'control settle C was not accepted');
  assert.equal(c.indexed[0]?.transactionResult?.status, 'SUCCESS', 'control settle C did not succeed');
  assert.equal(c.latestActionAfter.stateSHA256, receipt.postCPredictedSHA256, 'C post-state differs from the prediction');
  await attempt('P1', D2);
  await attempt('P4', D3);
  receipt.finishedAt = new Date().toISOString();
} finally {
  writeFileSync(join(output, 'receipt.json'), json(receipt), {flag: 'wx'});
  await bus('WALLET FREE sherlock-eval A9 settle', 'sherlock-eval finished the A9 settle submissions; wallet sessions closed.');
}
console.log(json({attempts: receipt.attempts.map(a => [a.name, a.nodeOutcome, a.txSHA256, a.indexed?.[0]?.transactionResult?.status ?? null, a.refusal?.at(-1)?.message ?? null])}));
process.exit(0);
