// Execute only against a new in-memory testing ledger. No network transaction submission.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {checkO1Accounts} from '../native/o1-accounting.mjs';
import {NativeLedger,hash,encode} from './native-ledger.mjs';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {prepareFill,acceptFill} from '../src/acceptance.mjs';
import {issueEnrollment,signOrder} from '../src/authorization.mjs';

const [modules,orderBuild,mintBuild,output] = process.argv.slice(2);
if (!output) throw Error('usage: node native-acceptance.mjs MODULES BUILT_ORDER BUILT_MINT OUTPUT');
const out = resolve(output);
mkdirSync(out,{recursive:true});
const expected = JSON.parse(readFileSync(new URL('./fixtures/acceptance.json',import.meta.url)));
const o1 = JSON.parse(readFileSync(new URL('./fixtures/o1.json',import.meta.url)));
const matrix = snap => o1.accountOrder.map(name=>snap[name]);
const build = JSON.parse(readFileSync(join(orderBuild,'acceptance-build.json')));
assert.equal(build.programHash,expected.program);
assert.equal(build.kernelSha256,expected.kernel);
assert.ok(['fund','fill','cancel'].every(op=>build.zkir[op].version.major===3));
const proofServer = process.env.MORIARTY_ORDER_PROVER || 'http://127.0.0.1:16311';
const proofVersion = (await (await fetch(new URL('/version',proofServer))).text()).trim();
assert.equal(proofVersion,'9.0.1');
const hex = bytes => Buffer.from(bytes).toString('hex');
const write = (path,data) => writeFileSync(join(out,path),encode(data)+'\n');
const records = [];
function retain(record) {
  const index = String(records.length).padStart(2,'0');
  const {beforeState,afterState,transaction,...publicReceipt} = record;
  for (const [name,bytes] of Object.entries({beforeState,afterState,transaction})) {
    const file = index+'-'+name+'.bin';
    writeFileSync(join(out,file),bytes);
    publicReceipt[name] = {file,sha256:hash(bytes)};
  }
  records.push(publicReceipt);
  write('receipts.json',records);
}
const h = await NativeLedger.create({modules,proofServer,retain});
const orderContract = await h.loadContract(orderBuild);
const mintContract = await h.loadContract(mintBuild);
const mint = await h.deploy(mintContract,[],'deploy-faucet');
const addr = a => ({bytes:h.runtime.encodeUserAddress(a)});
await h.call(mint,'mint',[addr(h.actors.owner.address),addr(h.actors.executor.address),100n],'mint-public-test-assets');
const colors = mintContract.generated.ledger(h.runtime.ContractState.deserialize(h.state.index(mint.address).serialize()).data);
const restored = h.snapshot();
const rows = [];
for (const [scenario,delivery] of [['exact',200n],['improved',201n]]) {
  h.restore(restored);
  const terms = {account:addr(h.actors.owner.address),recipient:addr(h.actors.recipient.address),executor:addr(h.actors.executor.address),
    assetA:colors.colorA,assetB:colors.colorB,network:Buffer.from(hash(h.network),'hex'),orderId:Buffer.from(hash(scenario),'hex'),
    program:Buffer.from(build.programHash,'hex'),epoch:BigInt(h.time+2)-1000n,input:100n,minimum:200n,fee:0n,expiry:1400n};
  const secret = 123456789n;
  const owner = h.runtime.ecMulGenerator(secret);
  const instance = await h.deploy(orderContract,[terms,owner],scenario+'-deploy-order');
  const deployment = {address:instance.address,network:h.network,programHash:build.programHash,verifiers:build.verifiers};
  const reader = {network:h.network,index:address=>h.state.index(address)};
  const read = () => readAcceptedOrder(reader,h.runtime,orderContract.generated,deployment);
  const sign = (changes={},sk=secret) => signOrder(h.runtime,orderContract.generated.pureCircuits,{
    terms,owner,contract:{bytes:h.runtime.encodeContractAddress(instance.address)},action:0n,revision:0n,...changes},sk);
  const summarize = () => {
    const s = read();
    const pair = (a,b) => [Number(a),Number(b)];
    const funds = Object.fromEntries(Object.entries(h.actors).map(([role,actor])=>
      [role,pair(h.balance(actor.address,hex(terms.assetA)),h.balance(actor.address,hex(terms.assetB)))]));
    funds.custody = pair(s.custody.get(hex(terms.assetA))??0n,s.custody.get(hex(terms.assetB))??0n);
    for (const [i,asset] of ['A','B'].entries()) {
      assert.equal(Object.values(funds).reduce((sum,p)=>sum+p[i],0),expected.supply[asset]);
    }
    return {...funds,revision:Number(s.revision),closed:Number(s.kernelState.f3),remaining:Number(s.remaining)};
  };
  deployment.enrollment = issueEnrollment(h.L,h.runtime,deployment,{terms,owner},h.actors.owner.key);
  const fundResult = await h.call(instance,'fund',[],scenario+'-fund');
  const AInputs = [...fundResult.tx.intents.values()].flatMap(intent=>[
    ...(intent.guaranteedUnshieldedOffer?.inputs??[]),...(intent.fallibleUnshieldedOffer?.inputs??[])]).filter(coin=>coin.type===hex(terms.assetA));
  assert.equal(AInputs.reduce((n,x)=>n+x.value,0n),100n);
  assert.ok(AInputs.every(x=>h.L.addressFromKey(x.owner)===h.actors.owner.address),'A must be funded from the enrolled wallet');
  assert.deepEqual(summarize(),expected.funded);
  assert.equal(BigInt(h.time)-terms.epoch,1000n);
  const req = {revision:0n,input:100n,delivery,fee:0n,now:1000n,signature:sign()};
  const diagnostics = [];
  async function refuse(name,change,error) {
    const before = h.state.serialize();
    await assert.rejects(async()=>prepareFill(h,instance,deployment,{...req,...change}),error,name);
    assert.deepEqual(h.state.serialize(),before);
    assert.deepEqual(summarize(),expected.funded);
    diagnostics.push({name,boundary:'contract-call preparation',ledgerSha256:hash(before),networkFeeSpeck:'0'});
  }
  await refuse('forged owner',{signature:sign({owner:h.runtime.ecMulGenerator(9n)},9n)},/SIGNATURE/);
  for (const [field,value] of Object.entries(terms)) {
    const substitute = typeof value==='bigint' ? value+1n : value.bytes ? {bytes:new Uint8Array(32).fill(99)} : new Uint8Array(32).fill(99);
    await refuse('substituted '+field,{signature:sign({terms:{...terms,[field]:substitute}})},/SIGNATURE/);
  }
  await refuse('short input',{input:99n},/exact input/);
  await refuse('short output',{delivery:199n},/at least/);
  await refuse('fee',{fee:1n},/APPLICATION_FEE/);
  await refuse('stale asserted fresh',{revision:1n,fresh:true},/REVISION/);
  const live = h.snapshot();
  const preparedBeforeExpiry = await prepareFill(h,instance,deployment,req);
  const paidBeforeExpiry = await h.fundAndProve(h.assemble([preparedBeforeExpiry]),orderContract,h.actors.executor);
  h.time+=400;
  await refuse('at expiry',{now:1400n},/EXPIRED/);
  const expired = h.apply(paidBeforeExpiry,scenario+'-live-proof-at-expiry',{allowFailure:true});
  assert.notEqual(expired.status,'success');
  assert.match(String(expired.error),/mismatch|time/i);
  assert.deepEqual(summarize(),expected.funded);
  checkO1Accounts(o1,'rejected',matrix(expected.funded),matrix(summarize()));
  diagnostics.push({name:'live proof at expiry',boundary:'ledger application',transaction:expired.transactionHash,
    error:String(expired.error),networkFeeSpeck:expired.nativeFeeSpeck});
  h.restore(live);
  const heldCall = await prepareFill(h,instance,deployment,req);
  const {receipt} = await acceptFill(h,instance,deployment,req,h.actors.executor);
  assert.deepEqual(summarize(),expected[scenario]);
  checkO1Accounts(o1,scenario,matrix(expected.funded),matrix(summarize()));
  assert.ok(receipt.nativeFeeSpeck > 0n);
  assert.ok(o1.requiredRefusals.every(name=>diagnostics.some(row=>row.name===name)));
  // A freshly funded, previously prepared call isolates the ledger state race from DUST/UTXO reuse.
  const duplicate = await h.fundAndProve(h.assemble([heldCall]),orderContract,h.actors.executor);
  const stale = h.apply(duplicate,scenario+'-duplicate-fresh-funding',{allowFailure:true});
  assert.notEqual(stale.status,'success');
  assert.match(String(stale.error),/mismatch|state.*changed/i);
  assert.deepEqual(summarize(),expected[scenario]);
  checkO1Accounts(o1,'rejected',matrix(expected[scenario]),matrix(summarize()));
  await assert.rejects(async()=>prepareFill(h,instance,deployment,{...req,fresh:true}),/REVISION/);
  const row = {scenario,network:h.network,address:instance.address,deployment,proofServerVersion:proofVersion,
    funded:expected.funded,after:summarize(),diagnostics,acceptedTransaction:receipt.transactionHash,
    duplicate:{status:stale.status,error:String(stale.error),nativeFeeSpeck:stale.nativeFeeSpeck}};
  rows.push(row);
  write('scenarios.json',rows);
}
write('native-readback.json',{build,proofServerVersion:proofVersion,rows,receipts:'receipts.json',
  boundary:'strict JS ledger application; native cryptographic validation is a separate check'});
