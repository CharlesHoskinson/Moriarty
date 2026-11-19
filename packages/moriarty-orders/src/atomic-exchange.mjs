import {createHash} from 'node:crypto';
import {verifyInstalledContract} from './accepted-state.mjs';

const hex = value => Buffer.from(value).toString('hex');
const digest = value => createHash('sha256').update(value).digest('hex');
const amounts = [100n, 200n, 300n];

export function signExchangeLeg(runtime, generated, plan, contractAddress, leg, revision, key) {
  const message = generated.pureCircuits.legMessage(plan,{bytes:runtime.encodeContractAddress(contractAddress)},BigInt(leg),revision);
  return runtime.jubjubSchnorrSign(new runtime.CompactTypeVector(1,runtime.CompactTypeField),[message],key);
}

// O8 maintenance needs all three owners: the committee is exactly the plan's wallets, threshold three.
function requireJointMaintenance(L, runtime, plan, deployment) {
  const {committee, threshold} = deployment.maintenance ?? {};
  const owners = plan.wallets.map(wallet => runtime.decodeUserAddress(wallet.bytes));
  if (threshold !== 3 || committee?.length !== 3 || committee.some((key, i) => L.addressFromKey(key) !== owners[i])) {
    throw Error('O8 maintenance must require all three owners');
  }
}

export function prepareExchange(ledger,instance,deployment,request) {
  if (deployment.address!==instance.address || deployment.network!==ledger.network) throw Error('O8 deployment mismatch');
  const native=ledger.state.index(deployment.address);
  if (!native) throw Error('O8 deployment absent from accepted head');
  verifyInstalledContract(native,deployment);
  const state=ledger.runtime.ContractState.deserialize(native.serialize());
  const head=instance.contract.generated.ledger(state.data);
  if (hex(head.exchange.program)!==deployment.programHash || hex(head.exchange.network)!==digest(ledger.network)) throw Error('O8 source or network mismatch');
  requireJointMaintenance(ledger.L,ledger.runtime,head.exchange,deployment);
  if (head.remaining!==1n || head.revision!==request.revision) throw Error('O8_CONSUMED');
  return ledger.prepareCall(instance,'settle',[request.plan,request.revision,request.now,request.signatures]);
}

/**
 * Check one owner's leg in the final transaction: the owner's inputs carry only its own
 * color, the next owner receives exactly the leg amount, and all change returns to the owner.
 */
export function checkLegTransaction(L, runtime, plan, leg, transaction) {
  const owners = plan.wallets.map(wallet => runtime.decodeUserAddress(wallet.bytes));
  const color = hex(plan.colors[leg]), owner = owners[leg], next = owners[(leg+1)%3];
  if (plan.amounts[leg] !== amounts[leg]) throw Error('O8 leg amount differs from the plan');
  let input = 0n, delivered = 0n, change = 0n;
  for (const [, intent] of transaction.intents ?? []) {
    for (const property of ['guaranteedUnshieldedOffer','fallibleUnshieldedOffer']) {
      const offer = intent[property];
      if (!offer) continue;
      for (const coin of offer.inputs) {
        if (L.addressFromKey(coin.owner) !== owner) continue;
        if (coin.type !== color) throw Error('O8 wallet asked to spend another asset');
        input += coin.value;
      }
      for (const coin of offer.outputs) {
        if (coin.type !== color) continue;
        if (coin.owner === next) delivered += coin.value;
        else if (coin.owner === owner) change += coin.value;
        else throw Error('O8 leg pays an unexpected recipient');
      }
    }
  }
  if (delivered !== amounts[leg] || input - amounts[leg] !== change) throw Error('O8 final intent differs from leg '+leg);
}

// An independent wallet: it signs only after validating its own leg in the final transaction.
export function exchangeWallet(L, runtime, plan, leg, actor, sign = message => L.signData(actor.key, message)) {
  return {address: actor.address, vk: actor.vk, sign(message, {transaction}) {
    checkLegTransaction(L, runtime, plan, leg, transaction);
    return sign(message);
  }};
}

// Add each owner's unsigned inputs. Signatures are added after proving, by each wallet.
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
  return transaction;
}
