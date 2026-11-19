import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtempSync,readFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {buildO5} from '../src/build-o5.mjs';
import {signProviderQuote,prepareO5Fill} from '../src/o5.mjs';
import test from 'node:test';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';

const source = new URL('../orders/o5.mori', import.meta.url);
const reportDir = new URL('../../../deliverables/competitor-report/IV.2/', import.meta.url);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const report = () => JSON.parse(readFileSync(new URL('result.json',reportDir)));

test('O5 binds the compiled source, feed and financial observation slots', () => {
  const mapping = lowerCompact(readFileSync(source),readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json',import.meta.url)));
  assert.deepEqual(mapping.metadata.observationFields.map(row=>row.name),['now','price_ab','sample_age']);
  assert.equal(mapping.metadata.programHash,'d218b0975575defffb1a8c3888aba4a26c3863636a6d6e7c87098f8e41ffb1b8');
  assert.equal(report().build.kernelSha256,hash(mapping.source));
});

test('the funded O5 native fixture delivers 100 A and at least 200 B at both inclusive age boundaries', () => {
  const evidence = report();
  assert.deepEqual(evidence.fills.map(row=>[row.age,row.delivery]),[[0,200],[30,201]]);
  for (const row of evidence.fills) {
    assert.deepEqual(row.balances,{owner:[0,0],executor:[100,1000000-row.delivery],recipient:[0,row.delivery],custody:[0,0]});
    assert.deepEqual(row.order,{revision:1,closed:1,remaining:127});
  }
});

test('refusals preserve the order and an earlier funded fill becomes invalid at inclusion', () => {
  const evidence = report();
  assert.deepEqual(evidence.refusals.map(row=>row.label).sort(),[
    'expiry','future','missing','stale','substitution','threshold equality','wrong controller','wrong owner','wrong provider'
  ].sort());
  assert.deepEqual(evidence.changed.before,evidence.changed.after);
  assert.deepEqual(evidence.changed.after,{owner:[0,0],executor:[0,1000000],recipient:[0,0],custody:[100,0]});
  const raw = readFileSync(new URL('receipts.json',reportDir));
  assert.equal(hash(raw),evidence.receiptsSha256);
  const receipts = JSON.parse(raw);
  const valid = receipts.find(row=>row.label==='baseline-prepared-fill');
  const changed = receipts.find(row=>row.label==='unchanged-fill-after-price-change');
  assert.equal(valid.status,'success');
  assert.notEqual(changed.status,'success');
  assert.match(JSON.stringify(changed.error),/mismatch.*read/i);
  assert.equal(valid.transaction.sha256,changed.transaction.sha256);
  for (const row of receipts) {
    assert.ok(Object.values(row.strict).every(Boolean));
    for (const ref of [row.beforeState,row.afterState,row.transaction]) {
      assert.equal(hash(readFileSync(new URL(ref.file,reportDir))),ref.sha256);
    }
  }
});

test('the provider signs the precise accepted deployment, amount and recipient terms',async()=>{
  const output=mkdtempSync(join(tmpdir(),'moriarty-o5-quote-'));
  try{
    buildO5(output,{skipZk:true});
    const generated=await import(pathToFileURL(join(output,'contract/index.js')));
    const modules=resolve(process.env.MORIARTY_ORDER_MODULES||new URL('../../../tools/sherlock-sdk/node_modules',import.meta.url).pathname);
    const rt=await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
    const L=await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
    const evidence=report(), address=evidence.authorities.address;
    const raw=JSON.parse(readFileSync(new URL('receipts.json',reportDir))).find(row=>row.label==='fund-O5');
    const state=L.LedgerState.deserialize(readFileSync(new URL(raw.afterState.file,reportDir)));
    const fields=generated.ledger(rt.ContractState.deserialize(state.index(address).serialize()).data);
    const grant={terms:fields.terms,contract:{bytes:rt.encodeContractAddress(address)},revision:0n,input:100n,delivery:200n};
    const sig=signProviderQuote(rt,generated,grant,7654321n);
    const verify=(terms,contract,delivery)=>rt.jubjubSchnorrVerify(new rt.CompactTypeVector(1,rt.CompactTypeField),
      [generated.pureCircuits.quoteMessage(terms,contract,0n,100n,delivery)],fields.providerKey,sig);
    assert.equal(verify(grant.terms,grant.contract,200n),true);
    assert.equal(verify(grant.terms,grant.contract,201n),false);
    assert.equal(verify({...grant.terms,recipient:grant.terms.account},grant.contract,200n),false);
    assert.throws(()=>prepareO5Fill(undefined,{address:'other'},{address},{}),/Wrong order contract/);
  }finally{rmSync(output,{recursive:true,force:true});}
});

test('the compiled O5 fill delivers only under a fresh authenticated price below three',async()=>{
  const output=mkdtempSync(join(tmpdir(),'moriarty-o5-fill-'));
  try{
    const pins=buildO5(output,{skipZk:true});
    const generated=await import(pathToFileURL(join(output,'contract/index.js')));
    const modules=resolve(process.env.MORIARTY_ORDER_MODULES||new URL('../../../tools/sherlock-sdk/node_modules',import.meta.url).pathname);
    const rt=await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
    const {signOrder}=await import('../src/authorization.mjs');
    const {signFeedUpdate}=await import('../src/feed.mjs');
    const {FEED_AB}=await import('../src/o5.mjs');
    const bytes=n=>new Uint8Array(32).fill(n), hex=b=>Buffer.from(b).toString('hex');
    const address=rt.sampleContractAddress(), self={bytes:rt.encodeContractAddress(address)};
    const terms={account:{bytes:bytes(1)},recipient:{bytes:bytes(2)},executor:{bytes:bytes(3)},orderId:bytes(4),network:bytes(5),
      assetA:bytes(6),assetB:bytes(7),program:Buffer.from(pins.programHash,'hex'),epoch:0n,input:100n,minimum:200n,fee:0n,expiry:1400n};
    const [ownerSk,controllerSk,providerSk]=[123456789n,42n,7654321n];
    const contract=new generated.Contract({});
    const ctx=(id,state,time)=>{
      const c=rt.createCircuitContext({circuitId:id,contractAddress:address,coinPublicKeyOrZswapState:'00'.repeat(32),contractState:state,privateState:{},time});
      const q=c.callContext.currentQueryContext;
      q.block={...q.block,secondsSinceEpoch:BigInt(time),lastBlockTime:BigInt(time),secondsSinceEpochErr:0,
        balance:new Map([[{tag:'unshielded',raw:hex(terms.assetA)},100n]])};
      return c;
    };
    const after=result=>result.context.callContext.currentQueryContext;
    const deployed=(await contract.initialState(rt.createConstructorContext({},'00'.repeat(32)),terms,rt.ecMulGenerator(ownerSk),
      rt.ecMulGenerator(controllerSk),rt.ecMulGenerator(providerSk))).currentContractState;
    const funded=after(await contract.provableCircuits.fund(ctx('fund',deployed,990))).state;
    const publish=async(state,price,observedAt,time=observedAt)=>{
      const observation={feed:FEED_AB,price,observedAt};
      const signature=signFeedUpdate(rt,generated.pureCircuits,{network:terms.network,address:self},observation,controllerSk);
      return after(await contract.provableCircuits.publishFeedUpdate(ctx('publishFeedUpdate',state,Number(time)),observation,signature)).state;
    };
    const grant=signOrder(rt,generated.pureCircuits,{terms,contract:self,owner:rt.ecMulGenerator(ownerSk),action:0n,revision:0n},ownerSk);
    const quote=(delivery,sk=providerSk)=>generated.pureCircuits && rt.jubjubSchnorrSign(new rt.CompactTypeVector(1,rt.CompactTypeField),
      [generated.pureCircuits.quoteMessage(terms,self,0n,100n,delivery)],sk);
    const fill=(state,time,{delivery=200n,fee=0n,now=BigInt(time),feed=FEED_AB,providerSk:sk=providerSk}={})=>
      contract.provableCircuits.fill(ctx('fill',state,time),0n,100n,delivery,fee,now,grant,feed,quote(delivery,sk));
    const payouts=result=>[...after(result).effects.claimedUnshieldedSpends].map(([[t,to],v])=>[hex(Buffer.from(t.raw,'hex')).slice(0,2),hex(Buffer.from(to.address,'hex')).slice(0,2),Number(v)]).sort();
    await assert.rejects(()=>fill(funded,1000),/FEED_UNAVAILABLE/);
    const live=await publish(funded,2n,1000n);
    assert.deepEqual(payouts(await fill(live,1000)),[['06','03',100],['07','02',200]]);
    assert.deepEqual(payouts(await fill(live,1030,{delivery:201n})),[['06','03',100],['07','02',201]]);
    await assert.rejects(()=>fill(live,1031),/FEED_STALE/);
    await assert.rejects(()=>fill(live,999),/FEED_FUTURE/);
    await assert.rejects(()=>fill(live,1000,{feed:bytes(0)}),/FEED_ID/);
    await assert.rejects(()=>fill(live,1000,{providerSk:8n}),/PROVIDER_SIGNATURE/);
    await assert.rejects(()=>fill(live,1000,{fee:1n}),/APPLICATION_FEE/);
    await assert.rejects(()=>fill(live,1001,{now:1000n}),/O5_EXECUTION_SECOND/);
    // The grant and quote issued while the price was 2 cannot execute once a newer sample says 3.
    const changed=await publish(live,3n,1001n);
    await assert.rejects(()=>fill(changed,1001),/FEED_THRESHOLD/);
  }finally{rmSync(output,{recursive:true,force:true});}
});
