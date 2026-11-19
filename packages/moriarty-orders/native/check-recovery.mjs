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
import {verifyEnrollment} from '../src/authorization.mjs';

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
  console.log(JSON.stringify({requirement:'II.3',receipts:records.length,replays,readbacks:checks},null,2));
} finally { rmSync(output,{recursive:true,force:true}); }
