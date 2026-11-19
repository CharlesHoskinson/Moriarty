import {createHash} from 'node:crypto';

const hex = value => Buffer.from(value).toString('hex');
const digest = value => createHash('sha256').update(value).digest('hex');
const amounts = [100n, 200n, 300n];

export function signExchangeLeg(runtime, generated, plan, contractAddress, leg, revision, key) {
  const message = generated.pureCircuits.legMessage(plan,{bytes:runtime.encodeContractAddress(contractAddress)},BigInt(leg),revision);
  return runtime.jubjubSchnorrSign(new runtime.CompactTypeVector(1,runtime.CompactTypeField),[message],key);
}

export function prepareExchange(ledger,instance,deployment,request) {
  if (deployment.address!==instance.address || deployment.network!==ledger.network) throw Error('O8 deployment mismatch');
  const native=ledger.state.index(deployment.address);
  const vk=native?.operation('settle')?.verifierKey;
  if (!vk || digest(vk)!==deployment.verifierSha256) throw Error('O8 verifier mismatch');
  const state=ledger.runtime.ContractState.deserialize(native.serialize());
  const head=instance.contract.generated.ledger(state.data);
  if (hex(head.exchange.program)!==deployment.programHash || hex(head.exchange.network)!==digest(ledger.network)) throw Error('O8 source or network mismatch');
  if (head.remaining!==1n || head.revision!==request.revision) throw Error('O8_CONSUMED');
  return ledger.prepareCall(instance,'settle',[request.plan,request.revision,request.now,request.signatures]);
}

// Each callback belongs to an independent wallet and signs the same fully assembled intent.
export function signExchangeInputs(ledger,tx,wallets) {
  const byAddress=new Map(wallets.map(wallet=>[ledger.addressFromKey(wallet.vk),wallet]));
  for (const [segment,intent] of tx.intents??[]) {
    const payload=intent.signatureData(segment);
    for (const property of ['guaranteedUnshieldedOffer','fallibleUnshieldedOffer']) {
      const offer=intent[property];
      if (!offer?.inputs.length) continue;
      const signatures=offer.inputs.map(input=>{
        const wallet=byAddress.get(ledger.addressFromKey(input.owner));
        if (!wallet) throw Error('O8 wallet signature absent');
        const signature=wallet.sign(payload);
        if (!ledger.verifySignature(input.owner,payload,signature)) throw Error('O8 wallet signature invalid');
        return new ledger.SignatureEnabled(signature);
      });
      intent[property]=offer.addSignatures(signatures);
    }
    tx.intents=tx.intents.set(segment,intent);
  }
  return tx;
}

export function fundExchangeInputs(ledger,transaction,plan,wallets) {
  if (wallets.length!==3 || new Set(wallets.map(wallet=>ledger.L.addressFromKey(wallet.vk))).size!==3) throw Error('O8 needs three distinct wallets');
  const colors=plan.colors.map(hex);
  const owners=plan.wallets.map(wallet=>ledger.runtime.decodeUserAddress(wallet.bytes));
  const supplied=new Set();
  for (const [segment,intent] of transaction.intents??[]) {
    for (const [section,property] of [[0,'guaranteedUnshieldedOffer'],[segment,'fallibleUnshieldedOffer']]) {
      const inputs=[], outputs=[...(intent[property]?.outputs??[])];
      if(intent[property]?.inputs.length) throw Error('O8 input offer was already funded');
      for (const [type,imbalance] of transaction.imbalances(section,0n)) {
        if(type.tag==='dust' || imbalance===0n)continue;
        const i=colors.indexOf(type.raw);
        if (i<0 || imbalance!==-amounts[i] || plan.amounts[i]!==amounts[i] || supplied.has(i)) throw Error('O8 unexpected funding demand');
        if (ledger.L.addressFromKey(wallets[i].vk)!==owners[i]) throw Error('O8 funding wallet mismatch');
        const destinations=outputs.filter(o=>o.type===colors[i]);
        if(destinations.length!==1 || destinations[0].value!==amounts[i] || destinations[0].owner!==owners[(i+1)%3]) throw Error('O8 native destination mismatch');
        let total=0n;
        for(const coin of ledger.state.utxo.filter(owners[i])) {
          if(coin.type!==colors[i])continue;
          inputs.push({...coin,owner:wallets[i].vk});
          total+=coin.value;
          if(total>=amounts[i])break;
        }
        if(total<amounts[i])throw Error('O8 insufficient funding for leg '+i);
        if(total>amounts[i])outputs.push({owner:owners[i],type:colors[i],value:total-amounts[i]});
        supplied.add(i);
      }
      if(inputs.length)intent[property]=ledger.L.UnshieldedOffer.new(inputs,outputs,[]);
    }
    transaction.intents=transaction.intents.set(segment,intent);
  }
  if(supplied.size!==3)throw Error('O8 funding incomplete');
  return signExchangeInputs(ledger.L,transaction,wallets);
}
