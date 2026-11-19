import assert from 'node:assert/strict';
import {mkdtempSync,readFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {buildAcceptance} from '../src/build-acceptance.mjs';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {verifyEnrollment} from '../src/authorization.mjs';
import {replayNativeReceipt} from './replay.mjs';
import {rerunRefusals} from './refusals.mjs';

const root=fileURLToPath(new URL('../../../',import.meta.url));
const evidence=resolve(process.argv[2]??join(root,'deliverables/competitor-report/III.5/native'));
const modules=process.env.MORIARTY_ORDER_MODULES??join(root,'tools/sherlock-sdk/node_modules');
const validator=process.env.MORIARTY_ORDER_VALIDATOR??join(root,'target/debug/examples/validate_call');
const L=await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
const rt=await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
const read=name=>JSON.parse(readFileSync(join(evidence,name)));
const records=read('receipts.json');
const report=read('executors.json');
const work=mkdtempSync(join(tmpdir(),'moriarty-executor-check-'));
try {
  const build=buildAcceptance(work,{policy:'executors',skipZk:true});
  for(const key of ['programHash','sourceSha256','kernelSha256','custodySha256','arithmeticSha256','compilerVersion','zkir']){
    assert.deepEqual(build[key],report.build[key],key);
  }
  const compiled=await import(pathToFileURL(join(work,'contract/index.js')));
  const replays=records.map(row=>replayNativeReceipt(validator,evidence,row));
  assert.deepEqual(records.map(r=>r.label),['setup-night-reward','setup-dust-registration','deploy-fixture-faucet','mint-test-assets',
    'fund-independent-fallback-wallet','deploy-restricted-order','fund-order','primary-fill','primary-proof-after-switch','fallback-fill','owner-exit-after-both-stop']);
  [null,0,1,2,3,4,5,6,6,8,6].forEach((p,i)=>{
    if(p!==null) assert.equal(records[i].beforeState.sha256,records[p].afterBlockStateSha256,'parent '+i);
  });
  const d=report.deployment;
  assert.deepEqual(d.verifiers,report.build.verifiers);
  const branches=[
    // The client that delivers B receives the 100 A: the primary in row 7, the fallback in row 9.
    {i:6,op:'fund',outcome:'success',state:[[0,0],[0,0],[0,999600],[0,400],[100,0]],revision:0},
    {i:7,op:'fill',outcome:'success',state:[[0,0],[0,200],[100,999400],[0,400],[0,0]],revision:1,client:'primary'},
    {i:8,op:'fill',outcome:'failure',state:[[0,0],[0,0],[0,999600],[0,400],[100,0]],revision:0},
    {i:9,op:'fill',outcome:'success',state:[[0,0],[0,200],[0,999600],[100,200],[0,0]],revision:1,client:'fallback'},
    {i:10,op:'cancel',outcome:'success',state:[[100,0],[0,0],[0,999600],[0,400],[0,0]],revision:1},
  ];
  for(const b of branches){
    const row=records[b.i];
    const replay=replays[b.i];
    assert.equal(replay.outcome,b.outcome);
    assert.equal(replay.contractCalls.length,1);
    const [call]=replay.contractCalls;
    assert.equal(call.entryPoint,b.op);
    assert.equal(call.address,d.address);
    assert.equal(call.proofVersion,'V3');
    assert.equal(call.verifierKeySHA256,d.verifiers[b.op]);
    const state=L.LedgerState.deserialize(readFileSync(join(evidence,row.afterState.file)));
    const order=readAcceptedOrder({network:'undeployed',index:a=>state.index(a)},rt,compiled,d);
    verifyEnrollment(L,rt,d,order,d.enrollment);
    const installed=compiled.ledger(rt.ContractState.deserialize(state.index(d.address).serialize()).data);
    const encodePoint=p=>[p.x.toString(),p.y.toString()];
    for(const role of ['primary','fallback']){
      assert.deepEqual(encodePoint(installed[role+'Key']),[report.clients[role].key.x,report.clients[role].key.y]);
    }
    assert.notDeepEqual(installed.primaryKey,installed.fallbackKey);
    assert.equal(rt.decodeUserAddress(installed.fallbackAccount.bytes),report.clients.fallback.payee);
    assert.equal(rt.decodeUserAddress(order.terms.executor.bytes),report.clients.primary.payee);
    const assets=[order.terms.assetA,order.terms.assetB].map(v=>Buffer.from(v).toString('hex'));
    const wallets=[rt.decodeUserAddress(order.terms.account.bytes),rt.decodeUserAddress(order.terms.recipient.bytes),
      report.clients.primary.wallet,report.clients.fallback.wallet];
    assert.equal(new Set(wallets).size,4);
    const observed=wallets.map(w=>assets.map(a=>Number([...state.utxo.filter(w)].filter(c=>c.type===a).reduce((sum,c)=>sum+c.value,0n))));
    observed.push(assets.map(a=>Number(order.custody.get(a)??0n)));
    assert.deepEqual(observed,b.state);
    assert.equal(Number(order.revision),b.revision);
    if(b.client){
      const wire=readFileSync(join(evidence,row.transaction.file));
      const tx=L.Transaction.deserialize('signature','proof','binding',wire);
      const coins=[...tx.intents.values()].flatMap(i=>[...(i.guaranteedUnshieldedOffer?.inputs??[]),...(i.fallibleUnshieldedOffer?.inputs??[])]).filter(c=>c.type===assets[1]);
      assert.ok(coins.length>0 && coins.every(c=>L.addressFromKey(c.owner)===report.clients[b.client].wallet));
      const t=BigInt(row.time)-order.terms.epoch;
      assert.ok(b.client==='primary' ? t<1100n : t>=1100n && t<1400n);
    }
  }
  // Re-execute each restricted call: an outsider, a fallback before its window and a reused primary key.
  const refusals=read('refusals.json');
  assert.deepEqual(refusals.map(r=>r.label),['outsider-as-primary','fallback-before-window','primary-key-as-fallback']);
  const rerun=await rerunRefusals({L,runtime:rt,compiled,directory:evidence,rows:refusals});
  assert.match(rerun[0].error,/CLIENT_SIGNATURE/);assert.match(rerun[1].error,/FALLBACK_WINDOW/);assert.match(rerun[2].error,/CLIENT_SIGNATURE/);
  assert.deepEqual(d.maintenance.committee,[d.enrollment.key]);
  console.log(JSON.stringify({schema:'moriarty-orders/executors-replay/1',
    evidenceClasses:{ledgerApplied:replays.map(r=>r.label),compiledCircuitRefusals:rerun},validated:replays},null,2));
} finally {rmSync(work,{recursive:true,force:true});}
