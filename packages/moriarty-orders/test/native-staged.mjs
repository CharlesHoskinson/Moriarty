import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {mkdirSync, readFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {NativeLedger, hash} from './native-ledger.mjs';
import {readContinuation, checkO9Balances, stagedEnrollmentMessage, verifyStagedEnrollment} from '../src/continuation.mjs';
import {generatorRecord, retainedTransactions, writeEvidence} from '../native/record.mjs';
import {retainedRefusals} from '../native/refusals.mjs';

const [modules,buildDir,mintDir,evidence] = process.argv.slice(2).map(x=>resolve(x));
if (!evidence) throw Error('usage: native-staged.mjs MODULES STAGED_BUILD MINT_BUILD EVIDENCE');
const generator = generatorRecord(['node','packages/moriarty-orders/test/native-staged.mjs',...process.argv.slice(2)]);
const proofServer=process.env.MORIARTY_ORDER_PROVER ?? 'http://127.0.0.1:16311';
assert.equal((await (await fetch(new URL('/version',proofServer))).text()).trim(),'9.0.1');
const journals = join(evidence,'journals');
mkdirSync(journals,{recursive:true});
const h = await NativeLedger.create({modules,proofServer,retain:retainedTransactions(evidence),refusals:retainedRefusals(evidence)});
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
const instance=await h.deploy(contract,[terms,ownerKey],'deploy-staged-order',{maintainers:[owner]});
const deployment={address:instance.address,network:h.network,programHash:build.programHash,verifiers:build.verifiers,
  maintenance:instance.maintenance};
const enrollment={key:h.L.signatureVerifyingKey(owner.key),
  signature:h.L.signData(owner.key,stagedEnrollmentMessage(deployment,owner.address,ownerKey))};
const reader={network:h.network,index:a=>h.state.index(a)};
const head=()=>readContinuation(reader,h.runtime,contract.generated,deployment);
verifyStagedEnrollment(h.L,h.runtime,deployment,head(),enrollment);
const clock=()=>BigInt(h.time)-terms.epoch;
const observations=[],clients=[];
function spawnClient(label,input) {
  const result=spawnSync(process.execPath,[fileURLToPath(new URL('./staged-client.mjs',import.meta.url))],
    {input:JSON.stringify(input),encoding:'utf8',timeout:60000});
  assert.equal(result.status,0,result.stderr);
  const {signature,submit,...summary}=JSON.parse(result.stdout);
  clients.push({label,...summary,exit:result.status,ledgerSha256:hash(h.state.serialize())});
  return {signature,submit,summary};
}
const base=()=>({head:Buffer.from(h.state.serialize()).toString('hex'),modules,directory:buildDir,deployment,enrollment});
// An honest client reads the accepted head and signs only an eligible action.
function client(request,label) {
  const {signature}=spawnClient(label,{...base(),request});
  return signature && {announcement:{x:BigInt(signature.announcement.x),y:BigInt(signature.announcement.y)},response:BigInt(signature.response)};
}
// A client prepares its terminal action, persists the unsubmitted transaction and exits.
function journal(request,label,extra={}) {
  const file=join(journals,label+'.json');
  spawnClient(label,{...base(),mode:'journal',request,journal:file,time:h.time,...extra});
  return file;
}
// A restarted process submits its journal without reading the ledger. The ledger arbitrates.
async function resubmit(file,label,payer,allowFailure=false) {
  const {submit,summary}=spawnClient('restart-'+label,{mode:'resubmit',journal:file});
  const unproven=h.L.Transaction.deserialize('signature','pre-proof','pre-binding',Buffer.from(submit,'hex'));
  const r=h.apply(await h.fundAndProve(unproven,contract,payer),label,{allowFailure});
  if (allowFailure) assert.notEqual(r.status,'success',label); else assert.equal(r.status,'success',label);
  clients.at(-1).submitted={label,status:r.status,transactionHash:r.transactionHash,preparedAgainstSha256:summary.preparedAgainstSha256};
  return r;
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
async function spend(label,type,value) {
  const intent=h.L.Intent.new(h.ttl);
  intent.guaranteedUnshieldedOffer=h.L.UnshieldedOffer.new([],[{owner:recipient.address,type:Buffer.from(type).toString('hex'),value}],[]);
  h.apply(await h.fundAndProve(h.L.Transaction.fromParts(h.network,undefined,undefined,intent),contract,owner),label);
}

await h.call(instance,'fund',[],'fund-staged-100A',owner);
check('funded','funded');
const unfilled=h.snapshot();
const firstSignature=client(0,'first-authorizer');
await h.refuseCall(instance,'first',[199n,clock(),firstSignature],/first minimum/,'first minimum');
await h.refuseCall(instance,'first',[200n,1200n,firstSignature],/FIRST_EXPIRED/,'at first expiry');
await h.call(instance,'first',[200n,clock(),firstSignature],'trade-100A-for-200B',executor);
check('first-trade','waiting');
const ticket=head().pending;
const resumeGrant=client(2,'resume-authorizer');
// The kernel's own guard refuses: the accepted event record is still false.
await h.refuseCall(instance,'resume',[ticket,300n,clock(),resumeGrant],/condition false/,'condition false');
await h.call(instance,'enable',[terms.eventId,client(1,'event-authorizer')],'enable-owner-event',owner);
const grant=client(2,'resume-authorizer-after-event');
await h.refuseCall(instance,'resume',[ticket,299n,clock(),grant],/second minimum/,'second minimum');
for (const k of Object.keys(ticket)) {
  const changed=structuredClone(ticket);
  changed[k]=typeof ticket[k]==='bigint'?ticket[k]+1n:k==='custody'?{bytes:new Uint8Array(32).fill(8)}:new Uint8Array(32).fill(8);
  await h.refuseCall(instance,'resume',[changed,300n,clock(),grant],/CONTINUATION_CAPTURE/,'altered '+k);
}
const root=h.snapshot();
// Two separate client processes prepare competing terminal actions against the same
// waiting predecessor at their eligible times, persist them and terminate.
const resumeJournal=journal(2,'resume-client',{delivery:300,now:Number(clock())});
const expiry=Number(terms.epoch)+1400;
h.time=expiry;
await h.refuseCall(instance,'resume',[ticket,300n,1400n,grant],/SECOND_EXPIRED/,'at second expiry');
const refundJournal=journal(3,'refund-client');
h.restore(root);

// Resume first. The replay is still before the second deadline, so only consumption refuses it.
await resubmit(resumeJournal,'resume-wins',executor); check('resume-wins','resumed');
assert.equal(client(3,'restarted-after-resume'),undefined);
await resubmit(resumeJournal,'resume-replay-before-expiry',executor,true); check('resume-replay','resumed');
assert.ok(h.time<expiry);
h.time=expiry;
await resubmit(refundJournal,'refund-loses-after-resume',owner,true); check('refund-loses','resumed');

// Refund first, from the same predecessor.
h.restore(root); h.time=expiry;
await resubmit(refundJournal,'refund-wins',owner); check('refund-wins','refunded');
assert.equal(client(2,'restarted-after-refund'),undefined);
await resubmit(resumeJournal,'resume-loses-after-refund',executor,true); check('resume-loses','refunded');
await resubmit(refundJournal,'refund-replay',owner,true); check('refund-replay','refunded');
await spend('spend-recovered-B',terms.assetB,200n);
check('owner-spent','spent');

// No first trade before its deadline: the owner withdraws the unfilled 100 A and spends it.
h.restore(unfilled);
h.time=Number(terms.epoch)+1199;
const exit=client(4,'withdraw-authorizer');
await h.refuseCall(instance,'withdraw',[exit],/FIRST_OPEN/,'withdraw before first deadline');
h.time=Number(terms.epoch)+1200;
await h.call(instance,'withdraw',[exit],'withdraw-unfilled-100A',owner);
check('withdrawn','withdrawn');
assert.equal(head().terminal,3n);
await h.refuseCall(instance,'first',[200n,1199n,firstSignature],/STAGE_FIRST/,'first after withdrawal');
await spend('spend-withdrawn-A',terms.assetA,100n);
check('owner-spent-withdrawn-A','withdrawnSpent');
writeEvidence(evidence,'readback.json',{build,generator,deployment,enrollment,refusals:'refusals.json',clients,observations,network:h.network,
  actors:{owner:owner.address,executor:executor.address,recipient:recipient.address}});
