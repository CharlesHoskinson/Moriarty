// Rebuild the selected source, run the independent native ledger, and decode the accepted states.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {mkdtempSync,readFileSync,rmSync,symlinkSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {buildO5} from '../src/build-o5.mjs';
import {replayNativeReceipt} from './replay.mjs';

const root=fileURLToPath(new URL('../../../',import.meta.url));
const pkg=join(root,'packages/moriarty-orders');
const id=process.argv[2];
assert.ok(['II.5','IV.2'].includes(id),'usage: check-feed.mjs II.5|IV.2 [EVIDENCE_DIRECTORY]');
const dir=resolve(process.argv[3]||join(root,'deliverables/competitor-report',id));
const validator=process.env.MORIARTY_ORDER_VALIDATOR||join(root,'target/debug/examples/validate_call');
const modules=resolve(process.env.MORIARTY_ORDER_MODULES||join(pkg,'../../tools/sherlock-sdk/node_modules'));
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
const hex=bytes=>Buffer.from(bytes).toString('hex');
const read=name=>readFileSync(join(dir,name));
const json=name=>JSON.parse(read(name));
const report=json('result.json'), receipts=json('receipts.json');
const o5=id==='IV.2';
const expected=o5?['setup-night-reward','setup-dust-registration','deploy-test-mint','mint-order-assets','deploy-O5','fund-O5',
  'publish-price-2','accepted-O5-age-0','accepted-O5-age-30','publish-at-expiry','baseline-prepared-fill','change-trigger-before-inclusion','unchanged-fill-after-price-change']:
  ['setup-night-reward','setup-dust-registration','deploy-feed-AB','publish-price-2','fresh-age-0','fresh-age-30','unchanged-transaction-age-31','publish-price-3','unchanged-transaction-new-price'];
assert.deepEqual(receipts.map(row=>row.label),expected);
assert.equal(report.requirement,id);
assert.equal(report.status,'passed');
assert.equal(digest(read('receipts.json')),o5?report.receiptsSha256:report.receipts.sha256);
const compiler=process.env.COMPACTC||'compact';
const selector=process.env.COMPACTC?[]:['compile','+0.36.0'];
assert.match(execFileSync(compiler,[...selector,'--version'],{encoding:'utf8'}),/^0\.36\.0\b/);
const L=await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
const rt=await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
const output=mkdtempSync(join(tmpdir(),'moriarty-feed-replay-'));
try {
  const keys=o5?report.build.verifiers:Object.fromEntries(Object.entries(report.artifacts).map(([name,pin])=>[name,pin.verifier]));
  if(o5){
    const built=buildO5(output,{skipZk:true});
    for(const field of ['programHash','sourceSha256','kernelSha256','kernels','sourceFiles','observations'])assert.deepEqual(built[field],report.build[field],field);
  }else{
    for (const path of ['contracts/feed.compact','contracts/feed-policy.compact','src/feed.mjs']) assert.equal(digest(readFileSync(join(pkg,path))),report.sources[path]);
    execFileSync(compiler,[...selector,'--skip-zk',join(pkg,'contracts/feed.compact'),output]);
    symlinkSync(modules,join(output,'node_modules'),'dir');
    for(const [name,pin] of Object.entries(report.artifacts))assert.equal(digest(readFileSync(join(output,'zkir',name+'.zkir'))),pin.zkir);
  }
  const compiled=await import(pathToFileURL(join(output,'contract/index.js')));
  const address=o5?report.authorities.address:report.contractAddress;
  const records=new Map(receipts.map(row=>[row.label,row]));
  function post(label) {
    const row=records.get(label);
    const state=L.LedgerState.deserialize(read(row.afterState.file));
    const contract=rt.ContractState.deserialize(state.index(address).serialize());
    return {row,state,contract,fields:compiled.ledger(contract.data)};
  }
  const validated=receipts.map(row=>{
    const data=replayNativeReceipt(validator,dir,row);
    const failed=row.label.startsWith('unchanged-');
    assert.equal(data.outcome,failed?'failure':'success',row.label);
    if(failed)assert.match(row.error,/mismatch.*read/i);
    for(const call of data.contractCalls.filter(c=>c.address===address)){
      assert.equal(call.proofVersion,'V3');
      assert.ok(Object.hasOwn(keys,call.entryPoint));
      assert.equal(call.verifierKeySHA256,keys[call.entryPoint]);
    }
    return data;
  });
  const published=post('publish-price-2');
  assert.equal(published.fields.feedPrice,2n);
  assert.equal(published.fields.feedAvailable,true);
  assert.equal(published.fields.feedObservedAt,BigInt(published.row.time));
  const changed=post(o5?'change-trigger-before-inclusion':'publish-price-3');
  assert.equal(changed.fields.feedPrice,3n);
  assert.equal(changed.fields.feedObservedAt,BigInt(changed.row.time));
  assert.equal(changed.row.time,published.row.time+1);
  for (const [label,age] of o5?[['accepted-O5-age-0',0],['accepted-O5-age-30',30]]:[['fresh-age-0',0],['fresh-age-30',30],['unchanged-transaction-age-31',31]]) {
    const row=records.get(label);
    assert.equal(row.time-published.row.time,age);
    assert.equal(validated.find(r=>r.label===label).contractCalls[0].entryPoint,o5?'fill':'checkFeed');
  }
  const original=records.get(o5?'baseline-prepared-fill':'fresh-age-0');
  const stale=records.get(o5?'unchanged-fill-after-price-change':'unchanged-transaction-new-price');
  assert.equal(original.transaction.sha256,stale.transaction.sha256);
  assert.equal(stale.time,changed.row.time+(o5?1:0));
  if(o5){
    assert.equal(original.time,stale.time);
    for (const [label,delivery] of [['fund-O5',0],['accepted-O5-age-0',200],['accepted-O5-age-30',201],['baseline-prepared-fill',200],['unchanged-fill-after-price-change',0]]) {
      const {state,contract,fields}=post(label);
      const colors=[fields.terms.assetA,fields.terms.assetB].map(hex);
      const holdings=address=>colors.map(color=>[...state.utxo.filter(rt.decodeUserAddress(address.bytes))].filter(c=>c.type===color).reduce((n,c)=>n+Number(c.value),0));
      assert.deepEqual(holdings(fields.terms.account),[0,0]);
      assert.deepEqual(holdings(fields.terms.recipient),[0,delivery]);
      assert.deepEqual(holdings(fields.terms.executor),[delivery?100:0,1000000-delivery]);
      const custody=colors.map(color=>[...contract.balance].filter(([asset])=>asset.tag==='unshielded' && asset.raw===color).reduce((n,[,amount])=>n+Number(amount),0));
      assert.deepEqual(custody,[delivery?0:100,0]);
      assert.equal(fields.revision,delivery?1n:0n);
      assert.equal(fields.kernelState.f3,delivery?1n:0n);
    }
  }else{
    assert.equal(original.transaction.sha256,records.get('fresh-age-30').transaction.sha256);
    assert.equal(original.transaction.sha256,records.get('unchanged-transaction-age-31').transaction.sha256);
  }
  console.log(JSON.stringify({schema:'moriarty-orders/feed-native-replay/1',requirement:id,
    validatorSha256:digest(readFileSync(validator)),receiptsSha256:digest(read('receipts.json')),transactions:validated}));
}finally{rmSync(output,{recursive:true,force:true});}
