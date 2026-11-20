import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {NativeLedger,hash} from './native-ledger.mjs';
import {generatorRecord,retainedTransactions,writeEvidence} from '../native/record.mjs';
import {retainedRefusals} from '../native/refusals.mjs';
import {issueEnrollment,signOrder,verifyEnrollment} from '../src/authorization.mjs';
import {readAcceptedOrder} from '../src/accepted-state.mjs';
import {signExecutor} from '../src/executors.mjs';

const [modules,buildDir,mintDir,out] = process.argv.slice(2);
if (!out) throw Error('usage: native-executors.mjs MODULES EXECUTORS_BUILD MINT_BUILD EVIDENCE');
const build = JSON.parse(readFileSync(join(buildDir,'acceptance-build.json')));
const generator = generatorRecord(['node','packages/moriarty-orders/test/native-executors.mjs',...process.argv.slice(2)]);
const proofServer = process.env.MORIARTY_ORDER_PROVER ?? 'http://127.0.0.1:16311';
assert.equal((await (await fetch(new URL('/version',proofServer))).text()).trim(),'9.0.1');
const h = await NativeLedger.create({modules,proofServer,retain:retainedTransactions(out),refusals:retainedRefusals(out)});
const {L,runtime} = h;
const contract = await h.loadContract(buildDir);
const mint = await h.deploy(await h.loadContract(mintDir),[],'deploy-fixture-faucet');
const {owner,executor:primary,recipient} = h.actors;
const fallbackWalletKey = L.signingKeyFromBip340(new Uint8Array(32).fill(37));
const fallbackVk = L.signatureVerifyingKey(fallbackWalletKey);
const fallback = {key:fallbackWalletKey,vk:fallbackVk,address:L.addressFromKey(fallbackVk)};
assert.notEqual(primary.address,fallback.address);
const addr = actor=>({bytes:runtime.encodeUserAddress(actor.address)});
await h.call(mint,'mint',[addr(owner),addr(primary),100n],'mint-test-assets');
const colors = mint.contract.generated.ledger(runtime.ContractState.deserialize(h.state.index(mint.address).serialize()).data);
const [A,B] = [colors.colorA,colors.colorB].map(c=>Buffer.from(c).toString('hex'));
const transfer = L.Intent.new(h.ttl);
transfer.guaranteedUnshieldedOffer=L.UnshieldedOffer.new([],[{owner:fallback.address,type:B,value:400n}],[]);
h.apply(await h.fundAndProve(L.Transaction.fromParts(h.network,undefined,undefined,transfer),contract,primary),'fund-independent-fallback-wallet');

const secrets = {owner:123456789n,primary:93751n,fallback:681637n,outsider:1823n}; // Public test scalars.
const keys = Object.fromEntries(Object.entries(secrets).map(([role,k])=>[role,runtime.ecMulGenerator(k)]));
const terms = {account:addr(owner),recipient:addr(recipient),executor:addr(primary),orderId:Buffer.from(hash('two-client-order'),'hex'),
  network:Buffer.from(hash(h.network),'hex'),assetA:colors.colorA,assetB:colors.colorB,program:Buffer.from(build.programHash,'hex'),
  epoch:BigInt(h.time+2)-1000n,input:100n,minimum:200n,fee:0n,expiry:1400n};
// The fallback client's wallet receives the A from a fallback fill. Only Alice maintains the order.
const instance = await h.deploy(contract,[terms,keys.owner,keys.primary,keys.fallback,addr(fallback)],'deploy-restricted-order',{maintainers:[owner]});
const d = {address:instance.address,network:h.network,programHash:build.programHash,verifiers:build.verifiers,maintenance:instance.maintenance};
d.enrollment = issueEnrollment(L,runtime,d,{terms,owner:keys.owner},owner.key);
const reader = {network:h.network,index:a=>h.state.index(a)};
const head = ()=>readAcceptedOrder(reader,runtime,contract.generated,d);
verifyEnrollment(L,runtime,d,head(),d.enrollment);
await h.call(instance,'fund',[],'fund-order');
const base = {terms,contract:{bytes:runtime.encodeContractAddress(instance.address)},revision:0n};
const grant = action=>signOrder(runtime,contract.generated.pureCircuits,{...base,owner:keys.owner,action},secrets.owner);
const permit = grant(0n);
const operatorSig = (role,secret)=>signExecutor(runtime,contract.generated.pureCircuits,base,role,secret);
const fillArgs = (role,secret)=>[0n,100n,200n,0n,BigInt(h.time)-terms.epoch,permit,role==='fallback',operatorSig(role,secret)];
const prepared = (role,secret)=>h.prepareCall(instance,'fill',fillArgs(role,secret));
const capture = ()=>{
  const s=head();
  const balances = [owner,recipient,primary,fallback].map(actor=>[A,B].map(color=>String(h.balance(actor.address,color))));
  balances.push([A,B].map(color=>String(s.custody.get(color)??0n)));
  return {balances,revision:String(s.revision),closed:String(s.kernelState.f3)};
};
const mat = (ownersA,bobB,primaryA,primaryB,backupA,backupB,escrowA)=>[
  [String(ownersA),'0'],['0',String(bobB)],[String(primaryA),String(primaryB)],[String(backupA),String(backupB)],[String(escrowA),'0']];
const opened = h.snapshot();
const before = capture();
assert.deepEqual(before,{balances:mat(0,0,0,999600,0,400,100),revision:'0',closed:'0'});
const refusals = [];
async function refuse(role,secret,reason,label) {
  const row=await h.refuseCall(instance,'fill',fillArgs(role,secret),reason,label);
  refusals.push({label,role,reason:reason.source,boundary:row.boundary,ledgerSha256:row.stateSha256});
}
await refuse('primary',secrets.outsider,/CLIENT_SIGNATURE/,'outsider-as-primary');
await refuse('fallback',secrets.fallback,/FALLBACK_WINDOW/,'fallback-before-window');
async function fill(role,wallet) {
  const call=await prepared(role,secrets[role]);
  const tx=await h.fundAndProve(h.assemble([call]),contract,wallet);
  const inputs=[...tx.intents.values()].flatMap(i=>[...(i.guaranteedUnshieldedOffer?.inputs??[]),...(i.fallibleUnshieldedOffer?.inputs??[])]).filter(x=>x.type===B);
  assert.ok(inputs.length && inputs.every(x=>L.addressFromKey(x.owner)===wallet.address),'B requires selected client wallet');
  for (const [segment,intent] of tx.intents) {
    for (const offer of [intent.guaranteedUnshieldedOffer,intent.fallibleUnshieldedOffer].filter(Boolean)) {
      offer.inputs.forEach((input,i)=>assert.ok(L.verifySignature(input.owner,intent.signatureData(segment),offer.signatures[i].value),
        'Final '+role+' input signature failed before native application at segment '+segment));
    }
  }
  return h.apply(tx,role+'-fill').transactionHash;
}
const transactions = {};
transactions.primary=await fill('primary',primary);
const primaryResult=capture();
assert.deepEqual(primaryResult,{balances:mat(0,200,100,999400,0,400,0),revision:'1',closed:'1'});
h.restore(opened);
const primaryCall=await prepared('primary',secrets.primary);
const expiredWindow=await h.fundAndProve(h.assemble([primaryCall]),contract,primary);
h.time=Number(terms.epoch)+1100;
const stale = h.apply(expiredWindow,'primary-proof-after-switch',{allowFailure:true});
assert.equal(stale.status,'failure');
assert.match(String(stale.error),/mismatch/i);
assert.deepEqual(capture(),before);
await refuse('fallback',secrets.primary,/CLIENT_SIGNATURE/,'primary-key-as-fallback');
transactions.fallback=await fill('fallback',fallback);
const fallbackResult=capture();
assert.deepEqual(fallbackResult,{balances:mat(0,200,0,999600,100,200,0),revision:'1',closed:'1'});
h.restore(opened);
h.time=Number(terms.epoch)+2001;
await h.call(instance,'cancel',[0n,grant(1n)],'owner-exit-after-both-stop',owner);
const recovered=capture();
assert.deepEqual(recovered,{balances:mat(100,0,0,999600,0,400,0),revision:'1',closed:'1'});
writeEvidence(out,'executors.json',{schema:'moriarty-orders/venue/1',network:h.network,build,generator,deployment:d,
  clients:{primary:{wallet:primary.address,key:keys.primary,payee:primary.address},fallback:{wallet:fallback.address,key:keys.fallback,payee:fallback.address}},refusalRecords:'refusals.json',
  accountOrder:['Alice','Bob','primary/venue','fallback','custody'],assetOrder:['A','B'],before,primaryResult,
  fallbackResult,recovered,refusals,transactions,applicationFee:0,fallbackRelativeTime:1100,expiryRelativeTime:1400,
  assessment:'independent external participation pending'});
