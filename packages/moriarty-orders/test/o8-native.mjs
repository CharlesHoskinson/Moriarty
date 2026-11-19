import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {join} from 'node:path';
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {generatorRecord,retainedTransactions,writeEvidence} from '../native/record.mjs';
import {retainedRefusals} from '../native/refusals.mjs';
import {exchangeWallet,fundExchangeInputs,prepareExchange,signExchangeLeg} from '../src/atomic-exchange.mjs';
import {NativeLedger,hash} from './native-ledger.mjs';

const [buildDir,output]=process.argv.slice(2);
if(!output || !process.env.MORIARTY_ORDER_MODULES)throw Error('usage: o8-native.mjs BUILD OUTPUT');
const build=JSON.parse(readFileSync(join(buildDir,'o8-build.json')));
const f=JSON.parse(readFileSync(new URL('./fixtures/o8.json',import.meta.url)));
const proofServer=process.env.MORIARTY_ORDER_PROVER||'http://127.0.0.1:16311';
const r={requirement:'IV.3',target:'native V9 local ledger',assessment:'independent assessment pending',node:process.version,
  generator:generatorRecord(['node','packages/moriarty-orders/test/o8-native.mjs',buildDir,output]),build,network:'undeployed',
  command:['node','packages/moriarty-orders/test/o8-native.mjs',buildDir,output],
  assumption:'Fresh, fixed native fixture issues only the specified asset supplies. After proving, each owner wallet callback checks its own leg in the final transaction (its inputs, the payment to the next owner and its change) and only then signs its own input. Native DUST fee uses a fourth credential. Maintenance requires all three owners.',
  limitation:'The common single-payer backend supports one wallet. The O8 adapter balances exactly three unshielded colors, three signature-enabled wallets and one separately paid DUST fee. The three wallet callbacks run in one local process, and the public test operator holds all three local keys.',
  sources:Object.fromEntries(['contracts/atomic-exchange.compact','src/atomic-exchange.mjs','orders/o8.mori','test/fixtures/o8-mint.compact',
    'test/fixtures/o8.json','src/build-o8.mjs','test/o8-native.mjs','test/native-ledger.mjs'].map(name=>[name,hash(readFileSync(join('packages/moriarty-orders',name)))])),
  refusals:[]};
const retain=retainedTransactions(output);
const refusals=retainedRefusals(output);
async function run(){
  r.proofServerVersion=(await (await fetch(new URL('/version',proofServer))).text()).trim();
  assert.equal(r.proofServerVersion,'9.0.1');
  const h=await NativeLedger.create({modules:process.env.MORIARTY_ORDER_MODULES,proofServer,retain,refusals});
  const exchange=await h.loadContract(buildDir), faucet=await h.loadContract(join(buildDir,'mint'));
  const mint=await h.deploy(faucet,[],'deploy-O8-faucet');
  const actors=[h.actors.owner,h.actors.recipient,h.actors.executor];
  const addresses=actors.map(actor=>({bytes:h.runtime.encodeUserAddress(actor.address)}));
  const secrets=[17n,23n,37n]; // Public test credentials for three separate roles.
  const keys=secrets.map(h.runtime.ecMulGenerator);
  const snapshot=h.snapshot();
  for(const underfunded of [true,false]){
    h.restore(snapshot);
    const prefix=underfunded?'underfunded-':'funded-';
    await h.call(mint,'mint',[addresses,underfunded?299n:300n],prefix+'mint-three-assets');
    const colors=faucet.generated.ledger(h.runtime.ContractState.deserialize(h.state.index(mint.address).serialize()).data).colors;
    const hex=x=>Buffer.from(x).toString('hex');
    const balances=()=>actors.map(actor=>colors.map(c=>Number(h.balance(actor.address,hex(c)))));
    const plan={network:Buffer.from(hash(h.network),'hex'),id:Buffer.from(hash('O8'), 'hex'),program:Buffer.from(build.programHash,'hex'),
      wallets:addresses,colors,amounts:f.amounts.map(BigInt),epoch:BigInt(h.time)-1000n};
    const instance=await h.deploy(exchange,[plan,keys],prefix+'deploy-exchange',{maintainers:actors,threshold:3});
    const deployment={address:instance.address,programHash:build.programHash,verifierSha256:build.verifierSha256,network:h.network,
      verifiers:{settle:build.verifierSha256},maintenance:instance.maintenance};
    const wallets=actors.map((actor,i)=>exchangeWallet(h.L,h.runtime,plan,i,actor));
    const signatures=secrets.map((sk,i)=>signExchangeLeg(h.runtime,exchange.generated,plan,instance.address,i,0n,sk));
    const request={plan,revision:0n,now:BigInt(h.time)-plan.epoch,signatures};
    const prepare=async(req=request)=>h.assemble([await prepareExchange(h,instance,deployment,req)]);
    const expected=underfunded?f.unfunded:f.funded;
    assert.deepEqual(balances(),expected);
    async function refuse(label,operation,pattern,stage='wallet or adapter before transaction assembly'){
      const before=hash(h.state.serialize());
      await assert.rejects(async()=>operation(),err=>{
        assert.match(err.message,pattern);
        r.refusals.push({label,before,after:hash(h.state.serialize()),balances:balances(),stage,diagnostic:err.message});
        return true;
      });
      assert.equal(hash(h.state.serialize()),before);
      assert.deepEqual(balances(),expected);
    }
    // Authority and plan changes: the compiled settle circuit refuses; the call is recorded for re-execution.
    async function refuseSettle(label,req,pattern){
      await refuse(label,()=>prepare(req),pattern,'compiled-circuit-preparation');
      await h.refuseCall(instance,'settle',[req.plan,req.revision,req.now,req.signatures],pattern,label);
    }
    if(underfunded){
      await refuse('unfunded third leg',async()=>fundExchangeInputs(h,await prepare(),plan,wallets),/insufficient funding for leg 2/);
      continue;
    }
    r.funded=balances();
    // Carol's grant is absent: her slot carries no signature (identity announcement, zero response).
    const absent={announcement:{x:0n,y:1n},response:0n};
    await refuseSettle('missing grant',{...request,signatures:[signatures[0],signatures[1],absent]},/O8_SIGNATURE/);
    await refuseSettle('wrong grant',{...request,signatures:[signatures[0],signatures[1],signatures[1]]},/O8_SIGNATURE/);
    await refuseSettle('changed recipient',{...request,plan:{...plan,wallets:[addresses[0],addresses[2],addresses[1]]}},/O8_PLAN/);
    await refuseSettle('changed amount',{...request,plan:{...plan,amounts:[99n,200n,300n]}},/O8_PLAN/);
    await refuse('wrong funding wallet',async()=>fundExchangeInputs(h,await prepare(),plan,[wallets[2],wallets[1],wallets[0]]),/funding wallet mismatch/);
    // Carol's wallet declines at the final signing step, after proving; no transaction exists.
    const declining=[wallets[0],wallets[1],exchangeWallet(h.L,h.runtime,plan,2,actors[2],()=>{throw Error('Carol declined to sign');})];
    await refuse('missing native wallet signature',async()=>h.fundBalancedAndProve(fundExchangeInputs(h,await prepare(),plan,declining),
      exchange,address=>declining.find(w=>w.address===address)),/declined to sign/);
    h.clearReservations();
    const funded=fundExchangeInputs(h,await prepare(),plan,wallets);
    const complete=await h.fundBalancedAndProve(funded,exchange,address=>wallets.find(w=>w.address===address));
    const inputs=[];
    const signed=[];
    for(const [segment,intent] of complete.intents){
      const message=intent.signatureData(segment);
      for(const prop of ['guaranteedUnshieldedOffer','fallibleUnshieldedOffer']){
        const offer=intent[prop];
        if(!offer)continue;
        for(const [idx,input] of offer.inputs.entries()){
          const signature=offer.signatures[idx].value;
          assert.equal(h.L.verifySignature(input.owner,message,signature),true);
          inputs.push(input);
          signed.push({key:input.owner,segment,signature,messageSha256:hash(message)});
        }
      }
    }
    assert.equal(inputs.length,3);
    r.inputAmounts=colors.map((color,i)=>{
      const actual=inputs.filter(input=>input.type===hex(color));
      assert.equal(actual.length,1); assert.equal(h.L.addressFromKey(actual[0].owner),actors[i].address);
      return Number(actual[0].value);
    });
    assert.deepEqual(r.inputAmounts,f.amounts);
    const pre=h.snapshot();
    const result=h.apply(complete,'atomic-three-owner-settlement');
    r.outcome=result.status;
    r.settlementTx=result.transactionHash;
    r.settled=balances();
    assert.deepEqual(r.settled,f.settled);
    const installed=exchange.generated.ledger(h.runtime.ContractState.deserialize(h.state.index(instance.address).serialize()).data);
    r.orderRevision=Number(installed.revision);
    assert.equal(installed.kernelState.f0,1n);
    assert.equal(installed.remaining,0n);
    assert.equal(r.orderRevision,1);
    r.resources=result.resources;
    const limits=h.params.toString().match(/limits: TransactionLimits \{[\s\S]*?\n    \},/);
    assert.ok(limits,'transaction limits absent from native ledger parameters');
    writeFileSync(join(output,'parameters.bin'),h.params.serialize());
    r.protocolParameters={file:'parameters.bin',sha256:hash(h.params.serialize()),limits:limits[0]};
    r.separateFee={dustPublicKey:String(h.dustKey.publicKey),nativeFeeSpeck:result.nativeFeeSpeck,requiredFeeSpeck:result.requiredFeeSpeck};
    r.deployment=deployment;
    r.fundingWallets=actors.map(a=>a.address);
    r.authorizations=keys.map((ownerKey,i)=>({owner:f.owners[i],wallet:actors[i].address,ownerKey,
      recipient:f.recipients[i],color:hex(colors[i]),amount:f.amounts[i],signature:signatures[i],
      message:exchange.generated.pureCircuits.legMessage(plan,{bytes:h.runtime.encodeContractAddress(instance.address)},BigInt(i),0n)}));
    r.nativeSignatures=signed;
    // At the original funded head, the same fully proved and signed settlement
    // fails atomically once the ledger clock reaches the exclusive horizon.
    h.restore(pre); h.time=Number(plan.epoch+2000n);
    const expired=h.apply(complete,'unchanged-O8-at-horizon',{allowFailure:true});
    assert.notEqual(expired.status,'success');assert.match(String(expired.error),/mismatch.*read/is);
    assert.deepEqual(balances(),f.funded);
    r.atHorizon={status:expired.status,balances:balances(),transaction:expired.transactionHash};

    // Consume Carol's original C UTXO separately while Alice and Bob retain
    // their inputs. The published settlement bytes and the contract stay fixed.
    h.restore(pre);
    h.time++;
    h.reserveFunding(complete);
    try {
      const carol=actors[2], type=hex(colors[2]);
      const coin=[...h.state.utxo.filter(carol.address)].find(x=>x.type===type);
      const outsideKey=h.L.signingKeyFromBip340(new Uint8Array(32).fill(51));
      const outside=h.L.addressFromKey(h.L.signatureVerifyingKey(outsideKey));
      const move=h.L.Intent.new(h.ttl);
      move.guaranteedUnshieldedOffer=h.L.UnshieldedOffer.new([{...coin,owner:carol.vk}],[
        {owner:outside,type,value:1n},{owner:carol.address,type,value:299n}],[]);
      const native=await h.fundAndProve(h.L.Transaction.fromParts(h.network,undefined,undefined,move),exchange,carol);
      h.apply(native,'carol-spends-one-C-before-inclusion');
    } finally { h.clearReservations(); }
    assert.deepEqual(balances(),f.unfunded);
    const refusalState=h.state.serialize();
    writeFileSync(join(output,'spent-leg-before.bin'),refusalState);
    const body={stateSha256:hash(refusalState),transactionSha256:hash(complete.serialize()),time:h.time,balances:balances()};
    const refused=h.apply(complete,'unchanged-O8-missing-C',{allowFailure:true});
    assert.notEqual(refused.status,'success');
    assert.match(String(refused.error),/Utxo|UTXO|unshielded.*input/i);
    assert.deepEqual(balances(),f.unfunded);
    r.spentLeg={...body,status:refused.status,diagnostic:refused.error};

  }
}
try{await run();r.status='passed';}
catch(e){r.status='failed';r.error=String(e?.message??e);throw e;}
finally{
  if(existsSync(join(output,'receipts.json')))r.receiptsSha256=hash(readFileSync(join(output,'receipts.json')));
  writeEvidence(output,'result.json',r);
}
