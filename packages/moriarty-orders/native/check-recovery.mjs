import assert from 'node:assert/strict';
import {mkdtempSync, readFileSync, rmSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {replayNativeReceipt} from './replay.mjs';
import {buildAcceptance} from '../src/build-acceptance.mjs';
import {checkRecoveryReadback, recoveryReadback} from '../src/recovery.mjs';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {NativeLedger} from '../test/native-ledger.mjs';
import {signOrder,verifyEnrollment} from '../src/authorization.mjs';

const root = fileURLToPath(new URL('../../../',import.meta.url));
const directory = process.argv[2] ?? join(root,'deliverables/competitor-report/II.3/native');
const validator = process.env.MORIARTY_ORDER_VALIDATOR ?? join(root,'target/debug/examples/validate_call');
const modules = process.env.MORIARTY_ORDER_MODULES ?? join(root,'tools/sherlock-sdk/node_modules');
const json = path => JSON.parse(readFileSync(path));
const records = json(join(directory,'receipts.json'));
const report = json(join(directory,'readback.json'));
const expected = json(join(root,'packages/moriarty-orders/test/fixtures/recovery.json'));
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const checks = [
  ['unassigned-deposit','funded'],['partial-fill','partial'],['prior-cancel-after-fill','partial'],
  ['refund-partial','partialRefund'],['spend-owner-partial-refund','spentPartial'],
  ['prior-fill-after-cancel','fullRefund'],['deposit-after-revocation','revokedWithFreshA'],
  ['refund-beyond-horizon','fullRefund'],['spend-owner-full-refund','spentFull'],
];
const rows = new Map(records.map(r=>[r.label,r]));
assert.equal(rows.size,records.length);
assert.deepEqual([...rows.keys()],['setup-night-reward','setup-dust-registration','deploy-setup-faucet',
  'mint-test-assets','second-owner-acquires-A','first-deploy','first-fund','second-deploy','second-fund',
  'unassigned-deposit','partial-fill','prior-cancel-after-fill','refund-partial','spend-owner-partial-refund',
  'early-full-refund','prior-fill-after-cancel','deposit-after-revocation','refund-beyond-horizon','spend-owner-full-refund']);
const runtime = await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
const ledger = await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
const output = mkdtempSync(join(tmpdir(),'moriarty-recovery-'));
try {
  const pins = buildAcceptance(output,{skipZk:true,policy:'recovery'});
  for (const key of ['compilerVersion','sourceSha256','kernelSha256','programHash','arithmeticSha256','custodySha256','zkir'])
    assert.deepEqual(pins[key],report.build[key],key);
  const generated = await import(pathToFileURL(join(output,'contract/index.js')));
  const replays = records.map(r=>replayNativeReceipt(validator,directory,r));
  for (const replay of replays) {
    const refused = ['prior-cancel-after-fill','prior-fill-after-cancel'].includes(replay.label);
    assert.equal(replay.outcome,refused?'failure':'success',replay.label);
  }
  const [first,second] = report.deployments;
  assert.equal(report.network,'undeployed');
  for (const [label,key] of checks) {
    const row = rows.get(label);
    const state = ledger.LedgerState.deserialize(readFileSync(join(directory,row.afterState.file)));
    const h = {state, runtime, network:report.network, balance:(address,asset)=>[...state.utxo.filter(address)]
      .filter(c=>c.type===asset).reduce((sum,c)=>sum+c.value,0n)};
    for (const deployment of [first,second]) {
      assert.equal(deployment.programHash,pins.programHash);
      const view = readAcceptedOrder({network:h.network,index:a=>state.index(a)},runtime,generated,deployment);
      verifyEnrollment(ledger,runtime,deployment,view,deployment.enrollment);
      assert.equal(digest(state.index(deployment.address).operation('depositSurplus').verifierKey),report.build.verifiers.depositSurplus);
      if (label==='refund-beyond-horizon') assert.ok(BigInt(row.time)>=view.terms.epoch+2000n);
    }
    checkRecoveryReadback(recoveryReadback(h,{generated},first,second),expected[key]);
  }
  // Each branch starts from the sealed two-order ledger. Confirm both
  // competing calls use that predecessor, including the independently funded loser.
  const parents = [null,0,1,2,3,4,5,6,7,8,9,10,11,12,9,14,15,9,17];
  for (const [i,parent] of parents.entries()) {
    if (parent !== null) assert.equal(records[i].beforeState.sha256,records[parent].afterBlockStateSha256);
  }
  async function preparation(label, circuit, argsFor, relativeTime, error) {
    const row = rows.get(label);
    const state = ledger.LedgerState.deserialize(readFileSync(join(directory,row.afterState.file)));
    const reader = {network:report.network,index:address=>state.index(address)};
    const head = readAcceptedOrder(reader,runtime,generated,first);
    const harness = Object.assign(Object.create(NativeLedger.prototype),{L:ledger,runtime,state,
      time:Number(head.terms.epoch)+relativeTime});
    const instance = {address:first.address,contract:{instance:new generated.Contract({})}};
    await assert.rejects(()=>harness.prepareCall(instance,circuit,argsFor(head)),error);
  }
  const fillArgs = head => [0n,40n,80n,0n,1000n,signOrder(runtime,generated.pureCircuits,{
    terms:head.terms,owner:head.owner,contract:{bytes:runtime.encodeContractAddress(first.address)},
    action:0n,revision:0n},123456789n)];
  await preparation('unassigned-deposit','fill',fillArgs,1400,/ORDER_EXPIRED/);
  await preparation('deposit-after-revocation','fund',()=>[],1000,/ALREADY_FUNDED/);
  await preparation('deposit-after-revocation','fill',fillArgs,1000,/REVISION|CLOSED/);
  console.log(JSON.stringify({requirement:'II.3',receipts:records.length,replays,readbacks:checks},null,2));
} finally { rmSync(output,{recursive:true,force:true}); }
