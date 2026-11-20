import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {existsSync, mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {issueEnrollment, signOrder} from '../src/authorization.mjs';
import {signFeedUpdate} from '../src/feed.mjs';
import {FEED_AB, prepareO5Fill, signProviderQuote} from '../src/o5.mjs';
import {NativeLedger, hash, encode} from './native-ledger.mjs';
import {generatorRecord, retainedTransactions} from '../native/record.mjs';
import {retainedRefusals} from '../native/refusals.mjs';
import {encodeValue} from '../src/values.mjs';

const [buildDir, mintDir, output] = process.argv.slice(2);
const proofServer = process.env.MORIARTY_ORDER_PROVER || 'http://127.0.0.1:16311';
if (!output || !process.env.MORIARTY_ORDER_MODULES) throw Error('usage: o5-native.mjs BUILD MINT OUTPUT');
mkdirSync(output,{recursive:true});
const write = (name,data) => writeFileSync(join(output,name),encode(data)+'\n');
const build = JSON.parse(readFileSync(join(buildDir,'o5-build.json')));
const report = {requirement:'IV.2',grade:'tested unreleased',network:'undeployed',target:'strict V9 WASM ledger application',
  command:['node','packages/moriarty-orders/test/o5-native.mjs',buildDir,mintDir,output],
  generator:generatorRecord(['node','packages/moriarty-orders/test/o5-native.mjs',buildDir,mintDir,output]),node:process.version,build,
  assessment:'independent assessment pending',assumptions:['Controller price and timely updates','Ledger time, ordering and inclusion',
    'Pinned controller, provider and deployment verifiers','Exact execution-second witness'],
  sources:Object.fromEntries(['contracts/o5.compact','contracts/feed-policy.compact','orders/o5.mori','src/o5.mjs','src/feed.mjs',
    'src/authorization.mjs','src/accepted-state.mjs','src/build-o5.mjs','test/o5-native.mjs','test/native-ledger.mjs'].map(name=>
      [name,hash(readFileSync(join('packages/moriarty-orders',name)))])),fills:[],refusals:[]};
const retain = retainedTransactions(output);
const refusals = retainedRefusals(output);

async function run() {
  report.proofServerVersion = (await (await fetch(new URL('/version',proofServer))).text()).trim();
  assert.equal(report.proofServerVersion,'9.0.1');
  const h = await NativeLedger.create({modules:process.env.MORIARTY_ORDER_MODULES,proofServer,retain,refusals});
  const order = await h.loadContract(buildDir);
  const mintContract = await h.loadContract(mintDir);
  const mint = await h.deploy(mintContract,[],'deploy-test-mint');
  const addr = actor => ({bytes:h.runtime.encodeUserAddress(actor.address)});
  await h.call(mint,'mint',[addr(h.actors.owner),addr(h.actors.executor),100n],'mint-order-assets');
  const colors = mintContract.generated.ledger(h.runtime.ContractState.deserialize(h.state.index(mint.address).serialize()).data);
  const terms = {account:addr(h.actors.owner),recipient:addr(h.actors.recipient),executor:addr(h.actors.executor),
    assetA:colors.colorA,assetB:colors.colorB,network:Buffer.from(hash(h.network),'hex'),orderId:Buffer.from(hash('test-O5'),'hex'),
    program:Buffer.from(build.programHash,'hex'),epoch:BigInt(h.time)-1000n,input:100n,minimum:200n,fee:0n,expiry:1400n};
  const ownerSecret = 123456789n, controllerSecret = 42n, providerSecret = 7654321n; // public fixture keys
  const owner = h.runtime.ecMulGenerator(ownerSecret);
  const provider = h.runtime.ecMulGenerator(providerSecret);
  const controller = h.runtime.ecMulGenerator(controllerSecret);
  const instance = await h.deploy(order,[terms,owner,controller,provider],'deploy-O5',{maintainers:[h.actors.owner]});
  const address = {bytes:h.runtime.encodeContractAddress(instance.address)};
  // The owner enrolls the feed controller and quote provider together with the verifiers and committee.
  const deployment = {address:instance.address,network:h.network,programHash:build.programHash,verifiers:build.verifiers,
    maintenance:instance.maintenance,pins:{feedController:encodeValue(controller),providerKey:encodeValue(provider)}};
  deployment.enrollment = issueEnrollment(h.L,h.runtime,deployment,{terms,owner},h.actors.owner.key);
  report.deployment = deployment;
  report.authorities = {owner,controller,provider,ownerWallet:h.actors.owner.address,providerWallet:h.actors.executor.address,
    recipient:h.actors.recipient.address,address:instance.address,enrollment:deployment.enrollment};
  const funded = await h.call(instance,'fund',[],'fund-O5');
  const hex = bytes => Buffer.from(bytes).toString('hex');
  function assertInput(tx,type,actor,amount) {
    const inputs = [...tx.intents.values()].flatMap(i=>[...(i.guaranteedUnshieldedOffer?.inputs??[]),...(i.fallibleUnshieldedOffer?.inputs??[])]).filter(c=>c.type===type);
    assert.ok(inputs.length>0 && inputs.every(c=>h.L.addressFromKey(c.owner)===actor.address));
    assert.ok(inputs.reduce((n,c)=>n+c.value,0n)>=amount);
  }
  assertInput(funded.tx,hex(terms.assetA),h.actors.owner,100n);
  const read = () => readAcceptedOrder({network:h.network,index:a=>h.state.index(a)},h.runtime,order.generated,deployment);
  function balances() {
    const state = read();
    const funds = {};
    for (const [role,actor] of Object.entries(h.actors)) funds[role]=[Number(h.balance(actor.address,hex(terms.assetA))),Number(h.balance(actor.address,hex(terms.assetB)))];
    funds.custody = [Number(state.custody.get(hex(terms.assetA))??0n),Number(state.custody.get(hex(terms.assetB))??0n)];
    assert.deepEqual([0,1].map(i=>Object.values(funds).reduce((n,pair)=>n+pair[i],0)),[100,1000000]);
    return funds;
  }
  const baseline = {owner:[0,0],executor:[0,1000000],recipient:[0,0],custody:[100,0]};
  assert.deepEqual(balances(),baseline);
  function request(delivery=200n,providerKey=providerSecret) {
    const grant={terms,contract:address,revision:0n,input:100n,delivery};
    return {...grant,fee:0n,feed:FEED_AB,now:BigInt(h.time)-terms.epoch,
      signature:signOrder(h.runtime,order.generated.pureCircuits,{...grant,owner,action:0n},ownerSecret),
      quote:signProviderQuote(h.runtime,order.generated,grant,providerKey)};
  }
  const prepare = req => prepareO5Fill(h,instance,deployment,req);
  const signUpdate = (price,observedAt,key=controllerSecret) => {
    const observation = {feed:FEED_AB,price,observedAt};
    return [observation,signFeedUpdate(h.runtime,order.generated.pureCircuits,{network:terms.network,address},observation,key)];
  };
  const fillArgs = req => [req.revision,req.input,req.delivery,req.fee,req.now,req.signature,req.feed,req.quote];
  // The adapter path refuses first; the compiled circuit must refuse the same call, which is recorded for re-execution.
  async function refuse(label,error,{req,circuit='fill',args}={}) {
    const before = hash(h.state.serialize());
    if (req) await assert.rejects(async()=>prepare(req),error);
    const row = await h.refuseCall(instance,circuit,args ?? fillArgs(req),error,label);
    assert.equal(hash(h.state.serialize()),before);
    assert.deepEqual(balances(),baseline);
    report.refusals.push({label,stage:row.boundary,beforeStateSha256:before,expectedError:error.source,circuitError:row.observedError});
  }
  const sampleTime = h.time;
  await refuse('missing',/FEED_UNAVAILABLE/,{req:request()});
  await refuse('wrong controller',/FEED_SIGNATURE/,{circuit:'publishFeedUpdate',args:signUpdate(2n,BigInt(sampleTime),7n)});
  await h.call(instance,'publishFeedUpdate',signUpdate(2n,BigInt(sampleTime)),'publish-price-2');
  const live = h.snapshot();
  await refuse('substitution',/FEED_ID/,{req:{...request(),feed:new Uint8Array(32)}});
  await refuse('wrong provider',/PROVIDER_SIGNATURE/,{req:request(200n,8n)});
  const forged=request();
  forged.signature.response++;
  await refuse('wrong owner',/ORDER_SIGNATURE/,{req:forged});
  h.time=sampleTime-1;
  await refuse('future',/FEED_FUTURE/,{req:request()});
  h.time=sampleTime+31;
  await refuse('stale',/FEED_STALE/,{req:request()});
  for (const [age,amount] of [[0,200n],[30,201n]]) {
    h.restore(live);
    h.time=sampleTime+age;
    const p=await prepare(request(amount));
    const tx=await h.fundAndProve(h.assemble([p]),order,h.actors.executor);
    const record=h.apply(tx,'accepted-O5-age-'+age);
    assertInput(tx,hex(terms.assetB),h.actors.executor,amount);
    const funds=balances(), state=read(), delivery=Number(amount);
    assert.deepEqual(funds,{owner:[0,0],executor:[100,1000000-delivery],recipient:[0,delivery],custody:[0,0]});
    const resultingOrder={revision:Number(state.revision),closed:Number(state.kernelState.f3),remaining:Number(state.remaining)};
    assert.deepEqual(resultingOrder,{revision:1,closed:1,remaining:127});
    report.fills.push({age,delivery,balances:funds,order:resultingOrder,transactionHash:record.transactionHash});
  }
  h.restore(live);
  h.time=Number(terms.epoch+terms.expiry);
  await h.call(instance,'publishFeedUpdate',signUpdate(2n,BigInt(h.time)),'publish-at-expiry');
  await refuse('expiry',/ORDER_EXPIRED/,{req:request()});
  h.restore(live);
  // Prepare for the target second. Pay its DUST from the earlier accepted head
  // before a competing publication can change the accepted block's DUST root.
  const targetSecond=sampleTime+2;
  h.time=targetSecond;
  const prepared=await prepare(request());
  h.time=sampleTime;
  const original=await h.fundAndProve(h.assemble([prepared]),order,h.actors.executor);
  const pre=h.snapshot();
  h.time=targetSecond;
  h.apply(original,'baseline-prepared-fill');
  h.restore(pre);
  h.reserveFunding(original);
  try {
    h.time=sampleTime+1;
    await h.call(instance,'publishFeedUpdate',signUpdate(3n,BigInt(h.time)),'change-trigger-before-inclusion');
  } finally { h.clearReservations(); }
  await refuse('threshold equality',/FEED_THRESHOLD/,{req:request()});
  h.time=targetSecond; // Recheck the captured fill at exactly its original inclusion second.
  const before=balances(), record=h.apply(original,'unchanged-fill-after-price-change',{allowFailure:true});
  const after=balances();
  assert.notEqual(record.status,'success');
  assert.match(encode(record.error),/mismatch.*read/is);
  assert.deepEqual(before,baseline);
  assert.deepEqual(after,before);
  report.changed={before,after,time:record.time,transactionHash:record.transactionHash,error:record.error};
  // Control: a newer sample at the same price also invalidates the unchanged transaction, while a
  // freshly prepared fill at that second is accepted. Only the price refuses a fresh fill.
  h.restore(pre);
  h.reserveFunding(original);
  try {
    h.time=sampleTime+1;
    await h.call(instance,'publishFeedUpdate',signUpdate(2n,BigInt(h.time)),'newer-sample-same-price');
  } finally { h.clearReservations(); }
  h.time=targetSecond;
  const same=h.apply(original,'unchanged-fill-after-newer-same-price',{allowFailure:true});
  assert.notEqual(same.status,'success');
  assert.match(encode(same.error),/mismatch.*read/is);
  assert.deepEqual(balances(),baseline);
  const refreshed=await prepare(request());
  const fresh=h.apply(await h.fundAndProve(h.assemble([refreshed]),order,h.actors.executor),'refreshed-fill-same-price');
  assert.deepEqual(balances(),{owner:[0,0],executor:[100,999800],recipient:[0,200],custody:[0,0]});
  report.sameSampleControl={unchanged:{status:same.status,error:same.error,transactionHash:same.transactionHash},
    refreshed:{status:fresh.status,transactionHash:fresh.transactionHash}};
}
try {
  await run();
  report.status='passed';
} catch(error) {
  report.status='failed';
  report.error=String(error?.message??error);
  throw error;
} finally {
  if(existsSync(join(output,'receipts.json'))) report.receiptsSha256=hash(readFileSync(join(output,'receipts.json')));
  write('result.json',report);
}
