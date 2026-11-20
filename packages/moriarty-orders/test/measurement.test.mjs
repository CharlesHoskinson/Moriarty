import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {test} from 'node:test';
import {measureTransaction, checkInventory} from '../../../scripts/qa/measure-order-lifecycle.mjs';
const recipe = JSON.parse(readFileSync(new URL('../../../deliverables/competitor-report/III.1/recipe.json',import.meta.url)));
const resource = {blockUsage:'0',bytesChurned:'8',bytesWritten:'4',readTime:'7',computeTime:'17'};
const strict = {verifyNativeProofs:true,verifyContractProofs:true,verifySignatures:true,enforceLimits:true,enforceBalancing:true};
const tx = {label:'fill',status:'success',time:12,transactionHash:'ab',strict,resources:resource,
  requiredFeeSpeck:'5',nativeFeeSpeck:'6',dustTimeBefore:12,dustTimeAfter:12,dustBefore:'21',dustAfter:'15',
  dustPayments:[{creationTime:12,accepted:true,segment:1,oldNullifier:'n',newCommitment:'c',vFee:'6'}],
  dustSpendEvents:[{source:{transactionHash:'ab',physicalSegment:1},content:{tag:'dustSpendProcessed',nullifier:'n',commitment:'c',vFee:'6'},serialized:[1]}]};
test('reject an incomplete inventory, wrong order or missing owner spend', () => {
  for (const spec of Object.values(recipe.operations)) {
    const steps = [...spec.setup,...spec.operation,...spec.recovery];
    checkInventory(spec,steps);
    for (const s of steps) assert.throws(()=>checkInventory(spec,steps.filter(x=>x!==s)));
    assert.throws(()=>checkInventory(spec,steps.toReversed()));
  }
});
test('count actual charged fees, all native resources and strict completion', () => {
  assert.equal(measureTransaction(tx).fee,6n);
  for(const edit of [r=>delete r.resources.readTime,r=>r.dustAfter='16',r=>r.dustTimeBefore++,
    r=>r.dustPayments[0].creationTime++,r=>r.dustSpendEvents[0].source.transactionHash='cd',
    r=>r.dustSpendEvents.push(r.dustSpendEvents[0]),r=>r.strict.verifyNativeProofs=false,
    r=>r.status='partialSuccess',r=>r.requiredFeeSpeck='7',r=>delete r.dustBefore]) {
    const changed=structuredClone(tx);edit(changed);assert.throws(()=>measureTransaction(changed));
  }
});

test('an omitted setup or recovery step, altered fee, missing field, broken finality or failed check invalidates a sample', async t => {
  const {cpSync,mkdtempSync,rmSync,writeFileSync,existsSync} = await import('node:fs');
  const {tmpdir} = await import('node:os');
  const {join} = await import('node:path');
  const {fileURLToPath} = await import('node:url');
  const {measureSample} = await import('../../../scripts/qa/measure-order-lifecycle.mjs');
  const root = fileURLToPath(new URL('../../../',import.meta.url));
  const validator = process.env.MORIARTY_ORDER_VALIDATOR ?? join(root,'target/debug/examples/validate_call');
  assert.ok(existsSync(validator),'build the strict validator: cargo build --locked -p mori-recursion --features sherlock-ledger-validation --example validate_call');
  const work = mkdtempSync(join(tmpdir(),'order-sample-'));
  t.after(()=>rmSync(work,{recursive:true,force:true}));
  const edit = (dir,file,change)=>{
    const value = JSON.parse(readFileSync(join(dir,file)));
    change(value);
    writeFileSync(join(dir,file),JSON.stringify(value,null,2)+'\n');
  };
  const scored = (rows,label)=>rows.find(r=>r.label===label);
  const cases = {
    'o1-fill':[
      ['omitted setup',dir=>edit(dir,'receipts.json',rows=>rows.splice(rows.findIndex(r=>r.label==='fund-order'),1))],
      ['altered fee',dir=>edit(dir,'receipts.json',rows=>{const r=scored(rows,'fill');r.nativeFeeSpeck=String(BigInt(r.nativeFeeSpeck)+1n);})],
      ['missing resource field',dir=>edit(dir,'receipts.json',rows=>{delete scored(rows,'fill').resources.readTime;})],
      ['broken finality',dir=>edit(dir,'receipts.json',rows=>{scored(rows,'fill').afterBlockStateSha256='00'.repeat(32);})],
      ['failed completion check',dir=>edit(dir,'checks.json',c=>{c.results.expected_completion=false;})],
      ['uncommitted generator',dir=>edit(dir,'checks.json',c=>{c.generator.draft=true;c.generator.revision=null;})],
    ],
    'cancel-unfilled':[
      ['omitted recovery',dir=>edit(dir,'receipts.json',rows=>rows.splice(rows.findIndex(r=>r.label==='spend-refund'),1))],
    ],
  };
  for (const [operation,mutations] of Object.entries(cases)) {
    const source = join(root,'deliverables/competitor-report/III.1/native',operation,'sample-1');
    const clean = join(work,operation+'-clean');
    cpSync(source,clean,{recursive:true});
    const measured = await measureSample(clean,operation,validator);
    assert.equal(measured.operation,operation);
    for (const [name,mutate] of mutations) {
      const copy = join(work,operation+'-'+name.replaceAll(' ','-'));
      cpSync(source,copy,{recursive:true});
      mutate(copy);
      await assert.rejects(()=>measureSample(copy,operation,validator),name);
    }
  }
});
