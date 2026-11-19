// Re-lower the frozen O1, decode real balances, and re-apply each proved transaction in Rust.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtempSync, readFileSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {replayNativeReceipt} from './replay.mjs';
import {checkO1Accounts} from './o1-accounting.mjs';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {verifyEnrollment} from '../src/authorization.mjs';
import {buildAcceptance} from '../src/build-acceptance.mjs';

const root = fileURLToPath(new URL('../../../',import.meta.url));
const evidence = resolve(process.argv[2] || join(root,'deliverables/competitor-report/II.1/native'));
const validator = process.env.MORIARTY_ORDER_VALIDATOR || join(root,'target/debug/examples/validate_call');
const modules = process.env.MORIARTY_ORDER_MODULES || join(root,'tools/sherlock-sdk/node_modules');
const read = path=>JSON.parse(readFileSync(path));
const hash = bytes=>createHash('sha256').update(bytes).digest('hex');
const hex = bytes=>Buffer.from(bytes).toString('hex');
const frozen = read(join(root,'packages/moriarty-orders/test/fixtures/o1.json'));
const policy = read(join(root,'packages/moriarty-orders/test/fixtures/acceptance.json'));
const records = read(join(evidence,'receipts.json'));
const report = read(join(evidence,'native-readback.json'));
const rt = await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
const ledger = await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
const output = mkdtempSync(join(tmpdir(),'moriarty-order-replay-'));
try {
  const generated = buildAcceptance(output,{skipZk:true});
  assert.match(generated.compilerVersion,/^0\.36\.0\b/);
  for (const key of ['compilerVersion','zkir','kernelSha256','sourceSha256','programHash','custodySha256','arithmeticSha256']) {
    assert.deepEqual(generated[key],report.build[key],key);
  }
  assert.equal(generated.kernelSha256,policy.kernel);
  assert.equal(generated.programHash,policy.program);
  const compiled = await import(pathToFileURL(join(output,'contract/index.js')));
  const replay = records.map(row=>replayNativeReceipt(validator,evidence,row));
  assert.deepEqual(replay,read(join(evidence,'strict-replay.json')),'retained strict replay');
  const labels = ['setup-night-reward','setup-dust-registration','deploy-faucet','mint-public-test-assets'];
  for (const kind of ['exact','improved']) labels.push(kind+'-deploy-order',kind+'-fund',kind+'-live-proof-at-expiry','accept-order',kind+'-duplicate-fresh-funding');
  assert.deepEqual(records.map(row=>row.label),labels);
  // The exact and improved orders each start at the same shared faucet head.
  const parents = [null,0,1,2,3,4,5,5,7,3,9,10,10,12];
  for (const [i,parent] of parents.entries()) {
    if (parent !== null) assert.equal(records[i].beforeState.sha256,records[parent].afterBlockStateSha256,'branch '+i);
  }
  assert.deepEqual(report.rows.map(r=>r.scenario),['exact','improved']);
  const loadState = file=>ledger.LedgerState.deserialize(readFileSync(join(evidence,file)));
  for (const [i,row] of report.rows.entries()) {
    const start = 4+5*i;
    const {deployment} = row;
    assert.equal(deployment.programHash,generated.programHash);
    assert.deepEqual(deployment.verifiers,report.build.verifiers);
    const decode = artifact=>{
      const state = loadState(artifact.file);
      const view = readAcceptedOrder({network:'undeployed',index:addr=>state.index(addr)},rt,compiled,deployment);
      verifyEnrollment(ledger,rt,deployment,view,deployment.enrollment);
      const roles = {owner:view.terms.account,executor:view.terms.executor,recipient:view.terms.recipient};
      const colors = [view.terms.assetA,view.terms.assetB].map(hex);
      const balances = frozen.accountOrder.map(name=>colors.map(color=>{
        if (name==='custody') return Number(view.custody.get(color)??0n);
        const coins = [...state.utxo.filter(rt.decodeUserAddress(roles[name].bytes))];
        return Number(coins.filter(x=>x.type===color).reduce((n,c)=>n+c.value,0n));
      }));
      return {balances,view};
    };
    const afterFund = decode(records[start+1].afterState);
    assert.deepEqual(afterFund.balances,frozen.funded);
    assert.equal(afterFund.view.revision,0n);
    assert.equal(afterFund.view.kernelState.f0,100n);
    for (const [offset,action,outcome] of [[1,'fund','success'],[2,'fill','failure'],[3,'fill','success'],[4,'fill','failure']]) {
      const data = replay[start+offset];
      assert.equal(data.outcome,outcome);
      assert.equal(data.contractCalls.length,1);
      const [call] = data.contractCalls;
      assert.equal(call.address,deployment.address);
      assert.equal(call.entryPoint,action);
      assert.equal(call.proofVersion,'V3');
      assert.equal(call.verifierKeySHA256,deployment.verifiers[action]);
    }
    const filled = records[start+3];
    const afterFill = decode(filled.afterState);
    checkO1Accounts(frozen,row.scenario,afterFund.balances,afterFill.balances);
    assert.equal(afterFill.view.revision,1n);
    assert.equal(afterFill.view.kernelState.f1,100n);
    assert.equal(afterFill.view.kernelState.f3,1n);
    assert.equal(row.acceptedTransaction,filled.transactionHash);
    assert.ok(BigInt(filled.nativeFeeSpeck) >= BigInt(filled.requiredFeeSpeck));
    assert.equal(filled.dustSpendEvents.reduce((sum,e)=>sum+BigInt(e.content.vFee),0n),BigInt(filled.nativeFeeSpeck));
    for (const offset of [2,4]) {
      const rejected = records[start+offset];
      const before = decode(rejected.beforeState), after = decode(rejected.afterState);
      checkO1Accounts(frozen,'rejected',before.balances,after.balances);
      assert.equal(before.view.contractStateSha256,after.view.contractStateSha256);
    }
    assert.ok(frozen.requiredRefusals.every(name=>row.diagnostics.some(d=>d.name===name)));
  }
  console.log(JSON.stringify({schema:'moriarty-orders/acceptance-replay/1',target:'ZKIRv3',caseIds:['II.1','II.2'],
    receiptsSha256:hash(readFileSync(join(evidence,'receipts.json'))),validated:replay},null,2));
} finally {
  rmSync(output,{recursive:true,force:true});
}
