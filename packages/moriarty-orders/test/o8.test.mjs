import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {checkExchangeOffer,exchangeWallet,fundExchangeInputs,prepareExchange,signExchangeLeg} from '../src/atomic-exchange.mjs';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const fixture=JSON.parse(readFileSync(new URL('./fixtures/o8.json',import.meta.url)));
const directory=new URL('../../../deliverables/competitor-report/IV.3/',import.meta.url);
const read=name=>JSON.parse(readFileSync(new URL(name,directory)));
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');

test('the O8 source lowers to three named transfers and the archived kernel',()=>{
  const mapping=lowerCompact(readFileSync(new URL('../orders/o8.mori',import.meta.url)),readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json',import.meta.url)));
  assert.equal(mapping.metadata.actions[0].effects.length,3);
  assert.deepEqual(mapping.metadata.textTable.map(x=>x.text),['ASSET_A','ASSET_B','ASSET_C',...fixture.owners]);
  assert.equal(mapping.metadata.programHash,'388882b7f449190fe54528955b3934f80d6a8ee885faff800e475adf7c61ce07');
  assert.equal(digest(mapping.source),read('result.json').build.kernelSha256);
});

test('three independently funded owner wallets exchange their exact approved legs in one transaction',()=>{
  const r=read('result.json');
  assert.deepEqual(r.funded,fixture.funded);
  assert.deepEqual(r.settled,fixture.settled);
  assert.deepEqual(r.inputAmounts,fixture.amounts);
  assert.equal(new Set(r.fundingWallets).size,3);
  assert.equal(new Set(r.authorizations.map(g=>g.ownerKey.x)).size,3);
  assert.equal(r.outcome,'success');
  assert.equal(r.orderRevision,1);
  const matched=read('receipts.json').filter(row=>row.label==='atomic-three-owner-settlement');
  assert.equal(matched.length,1);
  assert.equal(matched[0].transactionHash,r.settlementTx);
});

test('all seven missing, changed and underfunded cases move zero assets',()=>{
  const r=read('result.json');
  assert.deepEqual(r.refusals.map(x=>x.label).sort(),[...fixture.refusals].sort());
  for(const row of r.refusals){
    assert.equal(row.before,row.after);
    assert.deepEqual(row.balances,row.label==='unfunded third leg'?fixture.unfunded:fixture.funded);
  }
});

test('the same signed settlement fails after Carol spends one C and preserves all three balances',()=>{
  const r=read('result.json');
  const receipts=read('receipts.json');
  const successful=receipts.find(row=>row.label==='atomic-three-owner-settlement');
  const rejected=receipts.find(row=>row.label==='unchanged-O8-missing-C');
  assert.equal(successful.transaction.sha256,rejected.transaction.sha256);
  assert.equal(rejected.transaction.sha256,r.spentLeg.transactionSha256);
  assert.equal(rejected.beforeState.sha256,r.spentLeg.stateSha256);
  assert.equal(r.spentLeg.status,'failure');
  assert.deepEqual(r.spentLeg.balances,fixture.unfunded);
  assert.ok(rejected.error.includes(`value: 300, owner: UserAddress(${r.fundingWallets[2]})`));
});

test('the wallet adapter rejects an incomplete funding group and a substituted contract before signing',()=>{
  assert.throws(()=>fundExchangeInputs(undefined,undefined,undefined,[]),/three distinct wallets/);
  assert.throws(()=>prepareExchange({network:'undeployed'},{address:'substituted'},{address:'pinned',network:'undeployed'},{}),/deployment mismatch/);
});

async function compileO8(t) {
  const {mkdtempSync,rmSync}=await import('node:fs');
  const {tmpdir}=await import('node:os');
  const {join}=await import('node:path');
  const {pathToFileURL}=await import('node:url');
  const {buildO8}=await import('../src/build-o8.mjs');
  const out=mkdtempSync(join(tmpdir(),'moriarty-o8-unit-'));
  t.after(()=>rmSync(out,{recursive:true,force:true}));
  const build=buildO8(out,{skipZk:true});
  const rt=await import(pathToFileURL(join(out,'node_modules/@midnight-ntwrk/compact-runtime/dist/index.js')));
  const generated=await import(pathToFileURL(join(out,'contract/index.js')));
  const L=await import(pathToFileURL(join(out,'node_modules/@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
  return {build,rt,generated,L};
}

test('all three wallets derive their input signature from the enrolled native settlement bytes',async t=>{
  const {rt,generated,L}=await compileO8(t);
  const {deployment}=read('result.json');
  const receipt=read('receipts.json').find(row=>row.label==='atomic-three-owner-settlement');
  const bytes=readFileSync(new URL(receipt.transaction.file,directory));
  const state=L.LedgerState.deserialize(readFileSync(new URL(receipt.beforeState.file,directory)));
  const ledger={runtime:rt,L,network:deployment.network,state,time:receipt.time};
  const instance={address:deployment.address,contract:{generated}};
  const installed=rt.ContractState.deserialize(state.index(instance.address).serialize());
  const plan=generated.ledger(installed.data).exchange;
  const tx=L.Transaction.deserialize('signature','proof','binding',bytes);
  const [segment,intent]=[...tx.intents].find(([,i])=>i.actions.length);
  const inputs=intent.guaranteedUnshieldedOffer.inputs;
  for(const [leg,owner] of plan.wallets.entries()){
    const address=rt.decodeUserAddress(owner.bytes);
    const index=inputs.findIndex(input=>L.addressFromKey(input.owner)===address);
    const signature=intent.guaranteedUnshieldedOffer.signatures[index].value;
    const actor={address,vk:inputs[index].owner};
    let calls=0;
    const wallet=exchangeWallet(ledger,instance,deployment,plan,leg,actor,message=>{
      assert.deepEqual(message,intent.signatureData(segment));
      calls++;
      return signature;
    });
    assert.equal(typeof wallet.signSettlement,'function','wallet takes serialized settlement as its only signing input');
    const signed=wallet.signSettlement(bytes,'binding');
    assert.equal(calls,1);
    assert.ok(L.verifySignature(actor.vk,intent.signatureData(segment),signed));
    ledger.time=read('receipts.json').find(row=>row.label==='unchanged-O8-at-horizon').time;
    assert.throws(()=>wallet.signSettlement(bytes,'binding'),/cannot settle at accepted head/);
    assert.equal(calls,1);
    ledger.time=receipt.time;
  }
});

test('the compiled settle circuit refuses an absent or substituted grant and accepts all three',async t=>{
  const {build,rt,generated}=await compileO8(t);
  const bytes=n=>new Uint8Array(32).fill(n);
  const address=rt.sampleContractAddress();
  const plan={network:bytes(1),id:bytes(2),program:Buffer.from(build.programHash,'hex'),wallets:[1,2,3].map(n=>({bytes:bytes(10+n)})),
    colors:[bytes(21),bytes(22),bytes(23)],amounts:[100n,200n,300n],epoch:0n};
  const secrets=[17n,23n,37n];
  const contract=new generated.Contract({});
  const initial=(await contract.initialState(rt.createConstructorContext({},'00'.repeat(32)),plan,secrets.map(rt.ecMulGenerator))).currentContractState;
  const settle=signatures=>{
    const ctx=rt.createCircuitContext({circuitId:'settle',contractAddress:address,coinPublicKeyOrZswapState:'00'.repeat(32),contractState:initial,privateState:{},time:1000});
    const q=ctx.callContext.currentQueryContext;
    q.block={...q.block,secondsSinceEpoch:1000n,lastBlockTime:1000n,secondsSinceEpochErr:0};
    return contract.provableCircuits.settle(ctx,plan,0n,1000n,signatures);
  };
  const grants=secrets.map((sk,i)=>signExchangeLeg(rt,generated,plan,address,i,0n,sk));
  const absent={announcement:{x:0n,y:1n},response:0n};
  await assert.rejects(()=>settle([grants[0],grants[1],absent]),/O8_SIGNATURE/);
  await assert.rejects(()=>settle([grants[0],grants[1],grants[1]]),/O8_SIGNATURE/);
  const done=await settle(grants);
  assert.equal(generated.ledger(done.context.callContext.currentQueryContext.state.state).revision,1n);
});

test('the complete settlement offer pays all three recipients and returns change to each owner',()=>{
  const L={addressFromKey:key=>key.address};
  const runtime={decodeUserAddress:bytes=>bytes.address};
  const plan={wallets:['alice','bob','carol'].map(address=>({bytes:{address}})),
    colors:[1,2,3].map(n=>new Uint8Array(32).fill(n)),amounts:[100n,200n,300n]};
  const input=(i,value)=>({owner:{address:plan.wallets[i].bytes.address},type:Buffer.from(plan.colors[i]).toString('hex'),value});
  const output=(i,address,value)=>({owner:address,type:Buffer.from(plan.colors[i]).toString('hex'),value});
  const inputs=[input(0,45n),input(0,60n),input(1,210n),input(2,315n)];
  const outputs=[output(0,'bob',100n),output(0,'alice',5n),output(1,'carol',200n),
    output(1,'bob',10n),output(2,'alice',300n),output(2,'carol',15n)];
  const check=outputs=>checkExchangeOffer(L,runtime,plan,{inputs,outputs});
  check(outputs);
  assert.throws(()=>check([...outputs.slice(0,4),output(2,'bob',300n),outputs[5]]),/unexpected recipient/);
  assert.throws(()=>check([...outputs.slice(0,4),output(2,'alice',315n)]),/differs from leg 2/);
  for(let leg=0;leg<3;leg++){
    const withoutChange=outputs.filter((_,i)=>i!==2*leg+1);
    assert.throws(()=>check(withoutChange),/differs from leg/);
  }
});
