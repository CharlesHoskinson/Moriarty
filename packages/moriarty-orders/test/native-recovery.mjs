// Execute and retain independent branches rooted at one funded local ledger.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {NativeLedger, hash} from './native-ledger.mjs';
import {issueEnrollment, signOrder, verifyEnrollment} from '../src/authorization.mjs';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {checkRecoveryReadback, recoveryReadback} from '../src/recovery.mjs';
import {retainedTransactions, writeEvidence} from '../native/record.mjs';

const [modules, buildDir, mintDir, evidence] = process.argv.slice(2);
if (!evidence) throw Error('usage: node native-recovery.mjs MODULES RECOVERY_BUILD MINT_BUILD OUT');
const out = resolve(evidence);
const build = JSON.parse(readFileSync(join(buildDir,'acceptance-build.json')));
const expected = JSON.parse(readFileSync(new URL('./fixtures/recovery.json',import.meta.url)));
const proofServer = process.env.MORIARTY_ORDER_PROVER ?? 'http://127.0.0.1:16311';
assert.equal((await (await fetch(new URL('/version',proofServer))).text()).trim(), '9.0.1');
const h = await NativeLedger.create({modules,proofServer,retain:retainedTransactions(out)});
const contract = await h.loadContract(buildDir);
const mint = await h.deploy(await h.loadContract(mintDir),[],'deploy-setup-faucet');
const {owner, recipient, executor} = h.actors;
const address = actor => ({bytes:h.runtime.encodeUserAddress(actor.address)});
await h.call(mint,'mint',[address(owner),address(executor),207n],'mint-test-assets');
const tokens = mint.contract.generated.ledger(h.runtime.ContractState.deserialize(h.state.index(mint.address).serialize()).data);
const assetA = Buffer.from(tokens.colorA).toString('hex');
async function transferA(from, to, value, label) {
  const intent = h.L.Intent.new(h.ttl);
  intent.guaranteedUnshieldedOffer = h.L.UnshieldedOffer.new([], [{owner:to.address,type:assetA,value}], []);
  const raw = h.L.Transaction.fromParts(h.network, undefined, undefined, intent);
  return h.apply(await h.fundAndProve(raw, contract, from), label);
}
await transferA(owner, recipient, 100n,'second-owner-acquires-A');
async function openOrder(id, wallet, beneficiary, secret) {
  const terms = {account:address(wallet),recipient:address(beneficiary),executor:address(executor),
    assetA:tokens.colorA,assetB:tokens.colorB,network:Buffer.from(hash(h.network),'hex'),
    orderId:Buffer.from(hash(id),'hex'),program:Buffer.from(build.programHash,'hex'),epoch:BigInt(h.time)-1000n,
    input:100n,minimum:80n,expiry:1400n,fee:0n};
  const key = h.runtime.ecMulGenerator(secret);
  const instance = await h.deploy(contract,[terms,key],id+'-deploy');
  const deployment = {address:instance.address,network:h.network,programHash:build.programHash,verifiers:build.verifiers};
  deployment.enrollment = issueEnrollment(h.L,h.runtime,deployment,{terms,owner:key},wallet.key);
  await h.call(instance,'fund',[],id+'-fund',wallet);
  function grant(action,revision) {
    const head = readAcceptedOrder({network:h.network,index:a=>h.state.index(a)},h.runtime,contract.generated,deployment);
    verifyEnrollment(h.L,h.runtime,deployment,head,deployment.enrollment);
    return signOrder(h.runtime,contract.generated.pureCircuits,{terms:head.terms,owner:head.owner,
      contract:{bytes:h.runtime.encodeContractAddress(instance.address)},action,revision},secret);
  }
  return {instance,deployment,grant};
}
const first = await openOrder('first',owner,recipient,123456789n);
const second = await openOrder('second',recipient,owner,987654321n);
await h.call(first.instance,'depositSurplus',[7n],'unassigned-deposit');
const readbacks = [];
function check(label, key) {
  const observed = recoveryReadback(h,contract,first.deployment,second.deployment);
  checkRecoveryReadback(observed, expected[key]);
  readbacks.push({label,expected:key,observed,stateSha256:hash(h.state.serialize())});
}
check('funded-two-orders','funded');
const root = h.snapshot();
const fillArgs = () => [0n,40n,80n,0n,1000n,first.grant(0n,0n)];
const cancelArgs = n => [n,first.grant(1n,n)];
async function stale(prepared,label,payer) {
  const tx = await h.fundAndProve(h.assemble([prepared]),contract,payer);
  const row = h.apply(tx,label,{allowFailure:true});
  assert.notEqual(row.status,'success');
  assert.match(String(row.error),/mismatch|state.*changed/i);
}
const pendingCancel = await h.prepareCall(first.instance,'cancel',cancelArgs(0n));
await h.call(first.instance,'fill',fillArgs(),'partial-fill',executor);
check('partial-fill','partial');
await stale(pendingCancel,'prior-cancel-after-fill',owner);
check('prior-cancel-after-fill','partial');
await h.call(first.instance,'cancel',cancelArgs(1n),'refund-partial');
check('refund-partial','partialRefund');
await transferA(owner,recipient,60n,'spend-owner-partial-refund');
check('spend-owner-partial-refund','spentPartial');

h.restore(root);
const pendingFill = await h.prepareCall(first.instance,'fill',fillArgs());
await h.call(first.instance,'cancel',cancelArgs(0n),'early-full-refund');
await stale(pendingFill,'prior-fill-after-cancel',executor);
check('prior-fill-after-cancel','fullRefund');
await assert.rejects(()=>h.prepareCall(first.instance,'fund',[]),/ALREADY_FUNDED/);
await h.call(first.instance,'depositSurplus',[1n],'deposit-after-revocation');
await assert.rejects(()=>h.prepareCall(first.instance,'fill',fillArgs()),/REVISION|CLOSED/);
check('deposit-after-revocation','revokedWithFreshA');

h.restore(root);
h.time += 400;
await assert.rejects(()=>h.prepareCall(first.instance,'fill',fillArgs()),/EXPIRED/);
check('at-expiry','funded');
h.time += 601;
await h.call(first.instance,'cancel',cancelArgs(0n),'refund-beyond-horizon');
check('refund-beyond-horizon','fullRefund');
await transferA(owner,recipient,100n,'spend-owner-full-refund');
check('spend-owner-full-refund','spentFull');
writeEvidence(out,'readback.json',{build,network:h.network,deployments:[first.deployment,second.deployment],readbacks});
