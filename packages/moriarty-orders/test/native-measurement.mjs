// Generate the two frozen workloads serially; each receives independent genesis,
// setup, authorized wallet funding and controllable local block time.
import assert from 'node:assert/strict';
import {randomUUID} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {NativeLedger, hash} from './native-ledger.mjs';
import {generatorRecord,retainedTransactions,writeEvidence} from '../native/record.mjs';
import {retainedRefusals} from '../native/refusals.mjs';
import {signOrder,issueEnrollment} from '../src/authorization.mjs';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {prepareFill} from '../src/acceptance.mjs';

const [modules,orderDir,mintDir,output,count='3'] = process.argv.slice(2);
if (!output) throw Error('usage: node native-measurement.mjs MODULES O1 MINT OUTPUT [SAMPLES]');
const generator = generatorRecord(['node','packages/moriarty-orders/test/native-measurement.mjs',...process.argv.slice(2)]);
const samples = Number(count);
assert.ok(Number.isSafeInteger(samples) && samples >= 2,'run at least two serial samples per workload');
const build = JSON.parse(readFileSync(join(orderDir,'acceptance-build.json')));
const proofServer = process.env.MORIARTY_ORDER_PROVER ?? 'http://127.0.0.1:16311';
assert.equal((await (await fetch(new URL('/version',proofServer))).text()).trim(),'9.0.1');
// Samples run serially: each completes before the next starts, on its own fresh genesis and order.
for (const operation of ['o1-fill','cancel-unfilled']) for (let sample = 1; sample <= samples; sample++) {
  const dir = join(output,operation,'sample-'+sample);
  const h = await NativeLedger.create({modules,proofServer,retain:retainedTransactions(dir),refusals:retainedRefusals(dir)});
  writeFileSync(join(dir,'parameters.bin'),h.params.serialize());
  const order = await h.loadContract(orderDir);
  const mint = await h.deploy(await h.loadContract(mintDir),[],'deploy-faucet');
  const {owner,executor,recipient} = h.actors;
  const address = a => ({bytes:h.runtime.encodeUserAddress(a.address)});
  await h.call(mint,'mint',[address(owner),address(executor),100n],'mint-assets');
  const colors = mint.contract.generated.ledger(h.runtime.ContractState.deserialize(h.state.index(mint.address).serialize()).data);
  const executionId = randomUUID();
  const terms = {network:Buffer.from(hash(h.network),'hex'),orderId:Buffer.from(hash(executionId),'hex'),
    account:address(owner),executor:address(executor),recipient:address(recipient),input:100n,minimum:200n,fee:0n,
    // Two setup blocks follow. Schedule the scored operation at order-relative 1000.
    epoch:BigInt(h.time)+2n-1000n,expiry:1400n,program:Buffer.from(build.programHash,'hex'),assetA:colors.colorA,assetB:colors.colorB};
  const secret = 123456789n;
  const ownerKey = h.runtime.ecMulGenerator(secret);
  const instance = await h.deploy(order,[terms,ownerKey],'deploy-order',{maintainers:[owner]});
  const deployment = {network:h.network,address:instance.address,programHash:build.programHash,verifiers:build.verifiers,
    maintenance:instance.maintenance};
  deployment.enrollment = issueEnrollment(h.L,h.runtime,deployment,{terms,owner:ownerKey},owner.key);
  await h.call(instance,'fund',[],'fund-order');
  assert.equal(BigInt(h.time)-terms.epoch,1000n);
  const sign = (action,sk=secret) => signOrder(h.runtime,order.generated.pureCircuits,{
    terms,owner:h.runtime.ecMulGenerator(sk),action,revision:0n,contract:{bytes:h.runtime.encodeContractAddress(instance.address)}},sk);
  const req = {revision:0n,input:100n,delivery:200n,now:1000n,fee:0n,signature:sign(0n)};
  const reader = {network:h.network,index:a=>h.state.index(a)};
  function balances() {
    const head = readAcceptedOrder(reader,h.runtime,order.generated,deployment);
    return [owner,executor,recipient,null].map(actor=>[colors.colorA,colors.colorB].map(asset=>{
      const color=Buffer.from(asset).toString('hex');
      return Number(actor ? h.balance(actor.address,color) : head.custody.get(color)??0n);
    }));
  }
  const initial = balances();
  assert.deepEqual(initial,[[0,0],[0,1000000],[0,0],[100,0]]);
  const root = h.snapshot();
  const fill = await prepareFill(h,instance,deployment,req);
  const results = {valid_fillable_initial_order:true,owner_authority:true};
  // Each control must be refused by the adapter and, at the same state, by the compiled circuit.
  async function refuse(id,run,error,circuit) {
    const before = h.state.serialize();
    await assert.rejects(async()=>run(),error);
    await h.refuseCall(instance,circuit.name,circuit.args,error,id);
    assert.deepEqual(h.state.serialize(),before);
    results[id] = true;
  }
  const fillCall = r => ({name:'fill',args:[r.revision,r.input,r.delivery,r.fee,r.now,r.signature]});
  async function race(id,prepared,payer) {
    const before = balances();
    const tx = await h.fundAndProve(h.assemble([prepared]),order,payer);
    const row = h.apply(tx,'control-'+id,{allowFailure:true});
    assert.equal(row.status,'failure');
    assert.match(String(row.error),/mismatch|state.*changed/i);
    assert.deepEqual(balances(),before);
    results[id] = true;
  }
  if (operation==='o1-fill') {
    for (const [id,change,error] of [['reject_wrong_input',{input:99n},/exact input/],['reject_underpayment',{delivery:199n},/at least/],
      ['reject_forged_owner',{signature:sign(0n,98765n)},/SIGNATURE/]]) {
      await refuse(id,()=>prepareFill(h,instance,deployment,{...req,...change}),error,fillCall({...req,...change}));
    }
    h.time += 400;
    await refuse('reject_expired_fill',()=>prepareFill(h,instance,deployment,{...req,now:1400n}),/EXPIRED/,fillCall({...req,now:1400n}));
    h.restore(root);
    const tx = await h.fundAndProve(h.assemble([fill]),order,executor);
    h.apply(tx,'fill');
    assert.deepEqual(balances(),[[0,0],[100,999800],[0,200],[0,0]]);
    await race('reject_replay',fill,executor);
  } else {
    const forgedCancel = [0n,sign(1n,98765n)];
    await refuse('reject_unauthorized_cancel',()=>h.prepareCall(instance,'cancel',forgedCancel),/SIGNATURE/,{name:'cancel',args:forgedCancel});
    await h.call(instance,'cancel',[0n,sign(1n)],'cancel');
    const returned = [[100,0],[0,1000000],[0,0],[0,0]];
    assert.deepEqual(balances(),returned);
    // Spend the returned 100 A in an owner-signed transaction. The output stays
    // at the owner, so both bookends of the completion boundary are observable.
    await h.splitAsset(owner,Buffer.from(colors.colorA).toString('hex'),2,order,'spend-refund');
    assert.deepEqual(balances(),returned);
    results.owner_asset_recovery = true;
    await race('cancel_wins_race',fill,executor);
    await refuse('reject_fill_after_cancel',()=>prepareFill(h,instance,deployment,req),/REVISION/,fillCall(req));
  }
  const final = readAcceptedOrder(reader,h.runtime,order.generated,deployment);
  assert.equal(final.kernelState.f3,1n);
  assert.equal(final.revision,1n);
  assert.equal(final.terms.fee,0n);
  Object.assign(results,{zero_protocol_fees:true,expected_completion:true});
  writeEvidence(dir,'checks.json',{operation,sample:'sample-'+sample,executionId,generator,
    protocolFee:0,results,initialBalances:initial,finalBalances:balances(),deployment,build,
    command:['node','packages/moriarty-orders/test/native-measurement.mjs',...process.argv.slice(2)]});
}
