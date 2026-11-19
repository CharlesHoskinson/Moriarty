import assert from 'node:assert/strict';
import {after, before, test} from 'node:test';
import {readFileSync, mkdtempSync, rmSync, existsSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL, fileURLToPath} from 'node:url';

const fixture = JSON.parse(readFileSync(new URL('./fixtures/acceptance.json', import.meta.url)));
const path = name => fileURLToPath(new URL(name, import.meta.url));
const out = mkdtempSync(join(tmpdir(), 'order-acceptance-'));
const hex = b => Buffer.from(b).toString('hex');
const bytes = n => new Uint8Array(32).fill(n);
const addr = n => ({bytes:bytes(n)});
let rt, compiled, signOrder, ledger9;
const ownerSecret = 123456789n; // Public synthetic fixture scalar.
const address = '52'.repeat(32);
const self = {bytes:Buffer.from(address,'hex')};
const terms = {
  account:addr(1), recipient:addr(2), executor:addr(3), orderId:bytes(4), network:bytes(5),
  assetA:bytes(6), assetB:bytes(7), program:Buffer.from(fixture.program,'hex'), epoch:30n,
  input:100n, minimum:200n, fee:0n, expiry:1400n,
};

before(async () => {
  assert.ok(existsSync(path('../src/build-acceptance.mjs')), 'acceptance build entry point is missing');
  const {buildAcceptance} = await import('../src/build-acceptance.mjs');
  const build = buildAcceptance(out, {skipZk:true});
  ledger9 = build.compilerVersion.startsWith('0.36.0');
  assert.equal(build.programHash, fixture.program);
  assert.equal(build.kernelSha256, fixture.kernel);
  rt = await import(pathToFileURL(build.runtimePath));
  compiled = await import(pathToFileURL(join(out,'contract/index.js')));
  ({signOrder} = await import('../src/authorization.mjs'));
});
after(() => rmSync(out,{recursive:true,force:true}));

const query = context => (context.callContext ?? context).currentQueryContext;

function context(fn,state,time,amount) {
  const q = ledger9 ? rt.createCircuitContext({circuitId:fn,contractAddress:address,coinPublicKeyOrZswapState:'00'.repeat(32),contractState:state,privateState:{},time}) :
    rt.createCircuitContext(address,'00'.repeat(32),state,{},undefined,undefined,time);
  const block = query(q);
  block.block = {...block.block,secondsSinceEpoch:BigInt(time),lastBlockTime:BigInt(time),secondsSinceEpochErr:0,
    balance:new Map([[{tag:'unshielded',raw:hex(terms.assetA)},amount]])};
  return q;
}

async function setup() {
  const owner = rt.ecMulGenerator(ownerSecret);
  const contract = new compiled.Contract({});
  const deployed = await contract.initialState(rt.createConstructorContext({},'00'.repeat(32)),terms,owner);
  const funded = await contract.provableCircuits.fund(context('fund',deployed.currentContractState,1030,0n));
  const fundedState = query(funded.context).state;
  const signature = (overrides={}) => signOrder(rt,compiled.pureCircuits,
    {terms,contract:self,owner,action:0n,revision:0n,...overrides},ownerSecret);
  async function fill({state=fundedState,sig=signature(),revision=0n,input=100n,delivery=200n,fee=0n,time=1000}={}) {
    return contract.provableCircuits.fill(context('fill',state,time+30,100n),revision,input,delivery,fee,BigInt(time),sig);
  }
  return {owner,signature,fill,contract,fundedState};
}

for (const [label,delivery] of [['exact',200],['improved',201]]) {
  test('compiled fill produces the frozen '+label+' effects', async () => {
    const {fill} = await setup();
    const result = await fill({delivery:BigInt(delivery)});
    const q = query(result.context);
    const state = compiled.ledger(q.state.state);
    assert.equal(Number(state.revision),fixture[label].revision);
    assert.equal(Number(state.remaining),fixture[label].remaining);
    assert.equal(Number(state.kernelState.f3),fixture[label].closed);
    assert.deepEqual([...q.effects.claimedUnshieldedSpends].map(([[token,to],value])=>[token.raw,to.address,Number(value)]).sort(),
      [[hex(terms.assetA),hex(terms.executor.bytes),100],[hex(terms.assetB),hex(terms.recipient.bytes),delivery]].sort());
    await assert.rejects(()=>fill({state:q.state,revision:0n}),/REVISION/);
  });
}
test('compiled verifier independently binds each signed field', async () => {
  const {owner,fill,signature} = await setup();
  for (const [name,value] of Object.entries({...terms})) {
    const other = typeof value==='bigint' ? value+1n : value.bytes ? addr(44) : bytes(44);
    const sig = signature({terms:{...terms,[name]:other}});
    await assert.rejects(()=>fill({sig}),/SIGNATURE/,name);
  }
  for (const changed of [{contract:addr(44)},{action:1n},{revision:1n}]) {
    await assert.rejects(()=>fill({sig:signature(changed)}),/SIGNATURE/);
  }
  const forged = signOrder(rt,compiled.pureCircuits,{terms,contract:self,owner:rt.ecMulGenerator(7n),action:0n,revision:0n},7n);
  await assert.rejects(()=>fill({sig:forged}),/SIGNATURE/);
  assert.throws(()=>signature({owner:rt.ecMulGenerator(7n)}),/owner/i);
  assert.notDeepEqual(owner,rt.ecMulGenerator(7n));
});
test('canonical guards refuse a short input, short output, fee, stale head and expiry', async () => {
  const {fill} = await setup();
  for (const [args,reason] of [[{input:99n},/exact input/],[{delivery:199n},/at least/],[{fee:1n},/FEE/],
    [{revision:1n,fresh:true},/REVISION/],[{time:1400},/EXPIRED/]]) await assert.rejects(()=>fill(args),reason);
});

test('accepted-state reader refuses a changed installation, entry point or maintenance authority', async () => {
  const {readAcceptedOrder,verifyInstalledContract} = await import('../src/accepted-state.mjs');
  const {createHash} = await import('node:crypto');
  const digest = v => createHash('sha256').update(v).digest('hex');
  const expected = {address,network:'undeployed',verifiers:{fund:digest(bytes(8)),fill:digest(bytes(8)),cancel:digest(bytes(7))},
    maintenance:{committee:['owner-wallet-key'],threshold:1,counter:'0'}};
  const installed = (changes={}) => ({operations:()=>['fund','fill','cancel'],
    operation:op=>({verifierKey:op==='cancel'?bytes(7):bytes(8)}),
    maintenanceAuthority:{committee:['owner-wallet-key'],threshold:1,counter:0n},...changes});
  verifyInstalledContract(installed(),expected);
  assert.throws(()=>readAcceptedOrder({network:'undeployed',index:()=>undefined},rt,compiled,expected),/deployment/i);
  assert.throws(()=>verifyInstalledContract(installed({operation:()=>({verifierKey:bytes(8)})}),expected),/cancel verifier/i);
  assert.throws(()=>verifyInstalledContract(installed({operations:()=>['fund','fill','cancel','drain']}),expected),/entry points/i);
  assert.throws(()=>verifyInstalledContract(installed({maintenanceAuthority:{committee:['other-wallet-key'],threshold:1,counter:0n}}),expected),/Maintenance authority mismatch/);
  assert.throws(()=>verifyInstalledContract(installed({maintenanceAuthority:{committee:['owner-wallet-key'],threshold:1,counter:1n}}),expected),/maintenance has been exercised/i);
  assert.throws(()=>verifyInstalledContract(installed(),{...expected,maintenance:undefined}),/maintenance/i);
});
test('owner can recover late and a fill signature cannot authorize a refund', async () => {
  const {contract,fundedState,signature} = await setup();
  const cancel = async (amount,action) => contract.provableCircuits.cancel(context('cancel',fundedState,5000,amount),0n,signature({action}));
  await assert.rejects(()=>cancel(100n,0n),/SIGNATURE/);
  const q = query((await cancel(110n,1n)).context);
  assert.deepEqual([...q.effects.claimedUnshieldedSpends].map(([[token,to],v])=>[token.raw,to.address,v]),[[hex(terms.assetA),hex(terms.account.bytes),100n]]);
  assert.equal(compiled.ledger(q.state.state).kernelState.f2,100n);
});
test('native wallet enrollment rejects another wallet and a substituted order signer',async t=>{
  const {issueEnrollment,verifyEnrollment} = await import('../src/authorization.mjs');
  assert.equal(typeof issueEnrollment,'function','wallet enrollment required');
  const {createRequire} = await import('node:module');
  const modules = process.env.MORIARTY_ORDER_SDK9 ?? process.env.MORIARTY_ORDER_MODULES ?? path('../../../tools/sherlock-sdk/node_modules');
  // Compact runtimes expose native signing but only the ledger exposes addressFromKey.
  if (!existsSync(join(modules,'@midnightntwrk/ledger-v9'))) return t.skip('Install tools/sherlock-sdk dependencies to exercise native account enrollment');
  const sdk = createRequire(join(modules,'../package.json'));
  const L = await import(pathToFileURL(sdk.resolve('@midnight-ntwrk/midnight-js-protocol/v9')));
  const wallet = L.signingKeyFromBip340(bytes(14));
  const other = L.signingKeyFromBip340(bytes(15));
  const account = {bytes:rt.encodeUserAddress(L.addressFromKey(L.signatureVerifyingKey(wallet)))};
  const state = {terms:{...terms,account},owner:rt.ecMulGenerator(ownerSecret)};
  const vk = L.signatureVerifyingKey(wallet), otherVk = L.signatureVerifyingKey(other);
  const d = {address,network:'undeployed',verifiers:{fund:hex(bytes(1)),fill:hex(bytes(2)),cancel:hex(bytes(3))},
    maintenance:{committee:[vk],threshold:1,counter:'0'}};
  const cert = issueEnrollment(L,rt,d,state,wallet);
  verifyEnrollment(L,rt,d,state,cert);
  assert.throws(()=>issueEnrollment(L,rt,d,state,other),/wallet/i);
  assert.throws(()=>verifyEnrollment(L,rt,d,{...state,owner:rt.ecMulGenerator(9n)},cert),/enrollment/i);
  assert.throws(()=>verifyEnrollment(L,rt,{...d,address:'88'.repeat(32)},state,cert),/enrollment/i);
  // The signature covers every verifier and the committee. Only the owner wallet may maintain the order.
  assert.throws(()=>verifyEnrollment(L,rt,{...d,verifiers:{...d.verifiers,drain:hex(bytes(4))}},state,cert),/enrollment/i);
  assert.throws(()=>verifyEnrollment(L,rt,{...d,maintenance:{committee:[otherVk],threshold:1,counter:'0'}},state,cert),/maintenance/i);
  assert.throws(()=>verifyEnrollment(L,rt,{...d,maintenance:{committee:[vk,otherVk],threshold:1,counter:'0'}},state,cert),/maintenance/i);
});
