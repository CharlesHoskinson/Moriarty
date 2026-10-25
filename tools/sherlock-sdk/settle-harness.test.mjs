// Settle-path harness tests. Offline: no wallet, no proof server, no chain access.
// Fixtures (public): the A8 post-state S0 (validity_ok = {A8 stmt}), the deployed state (empty sets),
// the A3 parent fields f0..f15 and the A8 statement (A4 terminal public input 10).
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {homedir} from 'node:os';
import {join} from 'node:path';
import {sdkLoader} from './stagenet-session.mjs';
import {prepareSettle, settleTrace, settleStatement, editSnapshot, ledgerView} from './settle-harness.mjs';

const fixture = name => readFileSync(new URL(`./fixtures/settle/${name}`, import.meta.url));
const fromLe = s => BigInt('0x' + Buffer.from(s, 'hex').reverse().toString('hex'));
const pub = JSON.parse(fixture('a3-a8.json'));
const fields = pub.fieldsLe.map(fromLe);
const A8_STMT = fromLe(pub.a8StmtLe);
const S0 = {serialize: () => fixture('s0-a8-post-state.bin')};
const DEPLOYED = {serialize: () => fixture('deployed-state.bin')};
const swapped = f => { const g = [...f]; [g[8], g[9]] = [f[9], f[8]]; [g[11], g[12]] = [f[12], f[11]]; return g; };
const load = sdkLoader(process.env.MORIARTY_SDK_ROOT ?? join(homedir(), '.midnight/stagenet-wallet'),
  join(process.env.TMPDIR ?? '/tmp', `settle-harness-test-${process.pid}`));
// A session whose every property access fails: proves no wallet use before SettleUnsatisfiable.
const untouchable = () => new Proxy({}, {get: (_, k) => { throw Error(`session touched: ${String(k)}`); }});
const unsat = async (args, why) => {
  await assert.rejects(prepareSettle({load, session: untouchable(), ...args}), e => {
    assert.equal(e.name, 'SettleUnsatisfiable', e.message); assert.match(e.message, why); return true;
  });
};

test('settleTrace over S0 reproduces the A3 parent trace f15', () => {
  assert.equal(settleTrace(S0, fields), fields[15]);
});

test('settleStatement over S0 reproduces the A8 decider statement', () => {
  assert.equal(settleStatement(S0, 0n, fields), A8_STMT);
});

test('the statement takes I, R, H, B from the snapshot, not from the fields', () => {
  const other = editSnapshot(S0, {instance_id: 7n});
  assert.equal(ledgerView(other).instance_id, 7n);
  assert.notEqual(settleStatement(other, 0n, fields), A8_STMT);
  assert.equal(settleTrace(other, fields), fields[15]);             // I is not in the trace
  const forged = editSnapshot(S0, {root: fields[6] + 1n});
  assert.notEqual(settleTrace(forged, fields), fields[15]);
  assert.notEqual(settleStatement(forged, 0n, fields), A8_STMT);
  assert.throws(() => settleStatement(S0, 1n, fields), /batch/);
});

test('the swapped order changes both trace and statement', () => {
  const g = swapped(fields);
  assert.notEqual(settleTrace(S0, g), fields[15]);
  g[15] = settleTrace(S0, g);
  assert.notEqual(settleStatement(S0, 0n, g), A8_STMT);
});

test('editSnapshot edits only the named cells and set members', () => {
  const g = swapped(fields); g[15] = settleTrace(S0, g);
  const s = settleStatement(S0, 0n, g);
  const e = editSnapshot(S0, {validity_ok_add: [s], settled_add: [5n], height: 4n, batch_no: 0n});
  const v = ledgerView(e), v0 = ledgerView(S0);
  assert.ok(v.validity_ok.member(s) && v.validity_ok.member(A8_STMT) && v.settled.member(5n));
  assert.equal(v.height, 4n);
  for (const k of ['root', 'batch_no', 'instance_id', 'keys_id', 'program_c', 'genesis_root']) assert.equal(v[k], v0[k], k);
  assert.deepEqual(Buffer.from(editSnapshot(S0, {}).serialize()), fixture('s0-a8-post-state.bin'));
});

test('unsatisfiable settles throw SettleUnsatisfiable before any wallet use', async () => {
  await unsat({snapshot: DEPLOYED, batch: 0n, fields}, /statement not accepted/);
  await unsat({snapshot: S0, batch: 1n, fields}, /batch/);
  await unsat({snapshot: S0, batch: 0n, fields: swapped(fields)}, /trace order/);
  const stale = editSnapshot(S0, {root: fields[9], height: 2n, batch_no: 1n});
  await unsat({snapshot: stale, batch: 1n, fields}, /predecessor root/);
  const settled = editSnapshot(S0, {settled_add: [A8_STMT]});
  await unsat({snapshot: settled, batch: 0n, fields}, /replay/);
});

test('a satisfiable settle passes the VM and only then reaches the session', async () => {
  const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
  const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
  const ledgerParameters = ledger.LedgerParameters.initialParameters();
  await assert.rejects(prepareSettle({load, session: untouchable(), snapshot: S0, batch: 0n, fields, ledgerParameters}),
    /session touched: walletProvider/);
  // Forged head (s-c P2): satisfiable only over the edited snapshot that claims its statement.
  const forged = editSnapshot(S0, {root: fields[6] + 1n});
  const f2 = [...fields]; f2[6] = fields[6] + 1n; f2[15] = settleTrace(forged, f2);
  const claimed = editSnapshot(forged, {validity_ok_add: [settleStatement(forged, 0n, f2)]});
  await assert.rejects(prepareSettle({load, session: untouchable(), snapshot: claimed, batch: 0n, fields: f2, ledgerParameters}),
    /session touched: walletProvider/);
  await unsat({snapshot: forged, batch: 0n, fields: f2}, /statement not accepted/);
});

test('a satisfiable settle builds the SDK call over the snapshot and asks for exactly one settleValidity proof', async () => {
  const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
  const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
  const address = 'ab'.repeat(32), stop = Error('STOP_AT_PROOF');
  const asked = [];
  const prover = {check: (_r, loc) => { asked.push(['check', loc]); throw stop; }, lookupKey: loc => { asked.push(['key', loc]); throw stop; },
    prove: (_r, loc) => { asked.push(['prove', loc]); throw stop; }};
  const session = {walletProvider: {getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32),
    balanceTx: () => { throw Error('must not balance'); }, submitTx: () => { throw Error('must not submit'); }}};
  const realFetch = globalThis.fetch; globalThis.fetch = () => { throw Error('no network expected'); };
  try {
    await assert.rejects(prepareSettle({load, session, snapshot: S0, batch: 0n, fields, address,
      ledgerParameters: ledger.LedgerParameters.initialParameters(), prover}), /STOP_AT_PROOF/);
  } finally { globalThis.fetch = realFetch; }
  assert.ok(asked.length >= 1);
  for (const [, loc] of asked) assert.equal(loc, `contract:${address}/settleValidity?vk=be8b0943d8e2acadeb2fd914e4bcc100bdcce72ffe2a8da4e34f9417141c0b5c`);
});

test('prepareSettle refuses malformed inputs before any wallet use', async () => {
  await assert.rejects(prepareSettle({load, session: untouchable(), snapshot: S0, batch: 0, fields}), /batch/);
  await assert.rejects(prepareSettle({load, session: untouchable(), snapshot: S0, batch: 0n, fields: fields.slice(1)}), /16/);
});
