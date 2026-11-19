import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {NativeLedger, hash} from './native-ledger.mjs';
import {readContinuation, prepareTerminal, checkO9Balances} from '../src/continuation.mjs';
import {retainedTransactions,writeEvidence} from '../native/record.mjs';

const [modules,buildDir,mintDir,evidence] = process.argv.slice(2).map(x=>resolve(x));
if (!evidence) throw Error('usage: native-staged.mjs MODULES STAGED_BUILD MINT_BUILD EVIDENCE');
const proofServer=process.env.MORIARTY_ORDER_PROVER ?? 'http://127.0.0.1:16311';
assert.equal((await (await fetch(new URL('/version',proofServer))).text()).trim(),'9.0.1');
const h = await NativeLedger.create({modules,proofServer,retain:retainedTransactions(evidence)});
const contract=await h.loadContract(buildDir), faucet=await h.loadContract(mintDir);
const expected=JSON.parse(readFileSync(new URL('./fixtures/o9.json',import.meta.url)));
const build=JSON.parse(readFileSync(join(buildDir,'acceptance-build.json')));
const owner=h.actors.owner, executor=h.actors.executor, recipient=h.actors.recipient;
const address=a=>({bytes:h.runtime.encodeUserAddress(a.address)});
async function mint(label,destination,quantity) {
  const instance=await h.deploy(faucet,[],label+'-deploy');
  await h.call(instance,'mint',[address(destination),address(executor),quantity],label+'-mint');
  return faucet.generated.ledger(h.runtime.ContractState.deserialize(h.state.index(instance.address).serialize()).data);
}
const ab=await mint('AB',owner,100n), c=await mint('C',recipient,1n);
const assets=[ab.colorA,ab.colorB,c.colorB];
const terms={account:address(owner),executor:address(executor),network:Buffer.from(hash(h.network),'hex'),
  orderId:Buffer.from(hash('O9-trial'),'hex'),program:Buffer.from(build.programHash,'hex'),assetA:assets[0],assetB:assets[1],assetC:assets[2],
  eventId:Buffer.from(hash('O9-owner-execution-event'),'hex'),epoch:BigInt(h.time)-1000n};
const ownerKey=h.runtime.ecMulGenerator(123456789n);
const instance=await h.deploy(contract,[terms,ownerKey],'deploy-staged-order');
const deployment={address:instance.address,network:h.network,programHash:build.programHash,verifiers:build.verifiers};
const message=Buffer.from(JSON.stringify(['O9-wallet-enrollment/1',deployment,owner.address,String(ownerKey.x),String(ownerKey.y)]));
const enrollment={key:h.L.signatureVerifyingKey(owner.key),signature:h.L.signData(owner.key,message)};
const reader={network:h.network,index:a=>h.state.index(a)};
const head=()=>readContinuation(reader,h.runtime,contract.generated,deployment);
const clock=()=>BigInt(h.time)-terms.epoch;
const observations=[],clients=[],controls=[];
function client(request,label) {
  const input=JSON.stringify({head:Buffer.from(h.state.serialize()).toString('hex'),modules,directory:buildDir,deployment,request,enrollment});
  const result=spawnSync(process.execPath,[fileURLToPath(new URL('./staged-client.mjs',import.meta.url))],{input,encoding:'utf8',timeout:30000});
  assert.equal(result.status,0,result.stderr);
  const {signature,...summary}=JSON.parse(result.stdout);
  clients.push({label,...summary,exit:result.status,ledgerSha256:hash(h.state.serialize())});
  return signature && {announcement:{x:BigInt(signature.announcement.x),y:BigInt(signature.announcement.y)},response:BigInt(signature.response)};
}
function check(label,stage) {
  const token=assets.map(a=>Buffer.from(a).toString('hex'));
  const balances=Object.fromEntries([owner,executor,recipient].map((a,i)=>[
    ['owner','executor','recipient'][i],token.map(k=>Number(h.balance(a.address,k)))]));
  const escrow=new Map([...h.state.index(instance.address).balance].map(([t,n])=>[t.raw,n]));
  balances.custody=token.map(k=>Number(escrow.get(k)??0n));
  checkO9Balances(balances,expected[stage]);
  observations.push({label,stage,balances,terminal:Number(head().terminal),revision:Number(head().revision),ledgerSha256:hash(h.state.serialize())});
}
await h.call(instance,'fund',[],'fund-staged-100A',owner);
check('funded','funded');
const firstSignature=client(0,'first-authorizer');
await assert.rejects(()=>h.prepareCall(instance,'first',[199n,clock(),firstSignature]),/first minimum/);
await assert.rejects(()=>h.prepareCall(instance,'first',[200n,1200n,firstSignature]),/FIRST_EXPIRED/);
controls.push('first minimum','at first expiry');
await h.call(instance,'first',[200n,clock(),firstSignature],'trade-100A-for-200B',executor);
check('first-trade','waiting');
const ticket=head().pending;
const resume=()=>[ticket,300n,clock(),client(2,'resume-authorizer')];
await assert.rejects(()=>h.prepareCall(instance,'resume',resume()),/CONDITION_FALSE/);
controls.push('condition false');
await h.call(instance,'enable',[terms.eventId,client(1,'event-authorizer')],'enable-owner-event',owner);
const root=h.snapshot();
const grants=resume();
await assert.rejects(()=>h.prepareCall(instance,'resume',[ticket,299n,...grants.slice(2)]),/second minimum/);
controls.push('second minimum');
for (const k of Object.keys(ticket)) {
  const changed=structuredClone(ticket);
  changed[k]=typeof ticket[k]==='bigint'?ticket[k]+1n:k==='custody'?{bytes:new Uint8Array(32).fill(8)}:new Uint8Array(32).fill(8);
  await assert.rejects(()=>h.prepareCall(instance,'resume',[changed,...grants.slice(1)]),/CONTINUATION_CAPTURE/);
  controls.push('altered '+k);
}
async function resumeCall() {
  const [capture,delivery,now,signature]=resume();
  return prepareTerminal(h,instance,deployment,{action:'resume',capture,delivery,now,signature});
}
async function refundCall() {
  return prepareTerminal(h,instance,deployment,{action:'refund',capture:ticket,signature:client(3,'refund-authorizer')});
}
async function submit(prepared,label,payer,allowFailure=false) {
  const tx=await h.fundAndProve(h.assemble([prepared]),contract,payer);
  const r=h.apply(tx,label,{allowFailure});
  if (allowFailure) assert.notEqual(r.status,'success'); else assert.equal(r.status,'success');
  return r;
}
// Generate competing statements against the identical waiting predecessor at
// their eligible times. Only submission consumes state. Both branches below
// start at the same ledger root with fresh funding for each late submission.
const beforeExpiry=await resumeCall();
const expiry=Number(terms.epoch)+1400;
h.time=expiry;
await assert.rejects(()=>resumeCall(),/SECOND_EXPIRED/);
controls.push('at second expiry');
const atExpiry=await refundCall();
h.restore(root);
await submit(beforeExpiry,'resume-wins',executor); check('resume-wins','resumed');
assert.equal(client(3,'restarted-after-resume'),undefined);
h.time=expiry;
await submit(atExpiry,'refund-loses-after-resume',owner,true); check('refund-loses','resumed');
await submit(beforeExpiry,'resume-replay',executor,true); check('resume-replay','resumed');
h.restore(root); h.time=expiry;
await submit(atExpiry,'refund-wins',owner); check('refund-wins','refunded');
assert.equal(client(2,'restarted-after-refund'),undefined);
await submit(beforeExpiry,'resume-loses-after-refund',executor,true); check('resume-loses','refunded');
await submit(atExpiry,'refund-replay',owner,true); check('refund-replay','refunded');
const intent=h.L.Intent.new(h.ttl);
intent.guaranteedUnshieldedOffer=h.L.UnshieldedOffer.new([],[{owner:recipient.address,type:Buffer.from(terms.assetB).toString('hex'),value:200n}],[]);
const transfer=h.L.Transaction.fromParts(h.network,undefined,undefined,intent);
h.apply(await h.fundAndProve(transfer,contract,owner),'spend-recovered-B');
check('owner-spent','spent');
writeEvidence(evidence,'readback.json',{build,deployment,enrollment,controls,clients,observations,network:h.network,actors:{owner:owner.address,executor:executor.address,recipient:recipient.address}});
