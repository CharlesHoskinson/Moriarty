import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {checkLegTransaction,fundExchangeInputs,prepareExchange,signExchangeLeg} from '../src/atomic-exchange.mjs';
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

test('the compiled settle circuit refuses an absent or substituted grant and accepts all three',async t=>{
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

test('each owner wallet refuses a final transaction that misroutes its leg',()=>{
  const L={addressFromKey:key=>key.address};
  const runtime={decodeUserAddress:bytes=>bytes.address};
  const color=n=>new Uint8Array(32).fill(n), hex=b=>Buffer.from(b).toString('hex');
  const plan={wallets:['alice','bob','carol'].map(address=>({bytes:{address}})),colors:[color(1),color(2),color(3)],amounts:[100n,200n,300n]};
  const tx=outputs=>({intents:new Map([[1,{guaranteedUnshieldedOffer:{inputs:[{owner:{address:'carol'},type:hex(color(3)),value:310n}],outputs}}]])});
  const correct=[{owner:'alice',type:hex(color(3)),value:300n},{owner:'carol',type:hex(color(3)),value:10n}];
  checkLegTransaction(L,runtime,plan,2,tx(correct));
  assert.throws(()=>checkLegTransaction(L,runtime,plan,2,tx([{owner:'bob',type:hex(color(3)),value:300n},correct[1]])),/unexpected recipient/);
  assert.throws(()=>checkLegTransaction(L,runtime,plan,2,tx([{owner:'alice',type:hex(color(3)),value:310n}])),/differs from leg 2/);
  assert.throws(()=>checkLegTransaction(L,runtime,plan,2,tx([correct[0]])),/differs from leg 2/);
});
