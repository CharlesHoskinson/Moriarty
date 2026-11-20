import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
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

function acceptedExchange(ledger,instance,deployment) {
  if (deployment.address!==instance.address || deployment.network!==ledger.network) throw Error('O8 deployment mismatch');
  const native=ledger.state.index(deployment.address);
  if (!native) throw Error('O8 deployment absent from accepted head');
  verifyInstalledContract(native,deployment);
  const state=ledger.runtime.ContractState.deserialize(native.serialize());
  const head=instance.contract.generated.ledger(state.data);
  if (hex(head.exchange.program)!==deployment.programHash || hex(head.exchange.network)!==digest(ledger.network)) throw Error('O8 source or network mismatch');
  requireJointMaintenance(ledger.L,ledger.runtime,head.exchange,deployment);
  return head;
}

export function prepareExchange(ledger,instance,deployment,request) {
  const head=acceptedExchange(ledger,instance,deployment);
  if (head.remaining!==1n || head.revision!==request.revision) throw Error('O8_CONSUMED');
  return ledger.prepareCall(instance,'settle',[request.plan,request.revision,request.now,request.signatures]);
}

/** Verify all owner debits, payments and change in the same guaranteed offer. */
export function checkExchangeOffer(L, runtime, plan, offer) {
  const owners = plan.wallets.map(wallet => runtime.decodeUserAddress(wallet.bytes));
  const colors = plan.colors.map(hex);
  const inputs = [0n, 0n, 0n], payments = [0n, 0n, 0n], change = [0n, 0n, 0n];
  for (const coin of offer.inputs) {
    const leg = colors.indexOf(coin.type);
    if (leg < 0 || L.addressFromKey(coin.owner) !== owners[leg]) throw Error('O8 unexpected funding asset or owner');
    inputs[leg] += coin.value;
  }
  for (const coin of offer.outputs) {
    const leg = colors.indexOf(coin.type);
    if (leg < 0) throw Error('O8 unexpected output asset');
    if (coin.owner === owners[(leg + 1) % 3]) payments[leg] += coin.value;
    else if (coin.owner === owners[leg]) change[leg] += coin.value;
    else throw Error('O8 leg pays an unexpected recipient');
  }
  for (const [leg, amount] of amounts.entries()) {
    if (plan.amounts[leg] !== amount || payments[leg] !== amount || inputs[leg] !== amount + change[leg]) {
      throw Error('O8 final intent differs from leg '+leg);
    }
  }
}

function settlementIntent(transaction, contractAddress) {
  const intents = [...(transaction.intents ?? [])];
  if (intents.length !== 2 || transaction.guaranteedOffer || transaction.fallibleOffer?.size || transaction.rewards) {
    throw Error('O8 requires one settlement and a separate DUST fee');
  }
  const settlement = intents.find(([, intent]) => intent.actions.length);
  if (!settlement) throw Error('O8 settlement call missing');
  const [segment, intent] = settlement;
  const [, fee] = intents.find(([id]) => id !== segment);
  if (fee.actions.length || fee.guaranteedUnshieldedOffer || fee.fallibleUnshieldedOffer ||
      fee.dustActions?.spends.length !== 1 || fee.dustActions.registrations.length) {
    throw Error('O8 fee intent must contain only one DUST spend');
  }
  const [call] = intent.actions;
  if (intent.actions.length !== 1 || call.address !== contractAddress || call.entryPoint !== 'settle' ||
      !call.guaranteedTranscript || call.fallibleTranscript || !intent.guaranteedUnshieldedOffer ||
      intent.fallibleUnshieldedOffer || intent.dustActions) {
    throw Error('O8 requires one guaranteed settlement call and offer for the enrolled contract');
  }
  return settlement;
}

// Enrollment uses a trusted ledger connection and a separate copy of the approved terms.
// The signer accepts only serialized, proved transaction data and derives the native preimage.
export function exchangeWallet(ledger, instance, deployment, plan, leg, actor, sign = message => ledger.L.signData(actor.key, message)) {
  const {L, runtime} = ledger;
  const enrolled = structuredClone(plan), pins = structuredClone(deployment);
  const address = L.addressFromKey(actor.vk);
  if (!enrolled.wallets[leg] || address !== actor.address || runtime.decodeUserAddress(enrolled.wallets[leg].bytes) !== address) {
    throw Error('O8 wallet differs from enrolled owner');
  }
  return {address, vk: actor.vk, signSettlement(bytes, binding = 'pre-binding') {
    const head = acceptedExchange(ledger, instance, pins);
    if (head.remaining !== 1n || head.revision !== 0n) throw Error('O8_CONSUMED');
    if (!isDeepStrictEqual(structuredClone(head.exchange), enrolled)) throw Error('O8 enrolled plan differs from accepted contract');
    if (binding !== 'pre-binding' && binding !== 'binding') throw Error('O8 requires a proved native transaction');
    const tx = L.Transaction.deserialize('signature', 'proof', binding, bytes);
    const [segment, intent] = settlementIntent(tx, pins.address);
    checkExchangeOffer(L, runtime, enrolled, intent.guaranteedUnshieldedOffer);
    const time = BigInt(ledger.time);
    const verified = tx.wellFormed(ledger.state, new L.WellFormedStrictness(), new Date(ledger.time * 1000));
    const context = new L.TransactionContext(ledger.state, {
      secondsSinceEpoch: time, secondsSinceEpochErr: 0, lastBlockTime: time, parentBlockHash: '00'.repeat(32)
    });
    // Native apply returns a new state. The wallet reads the outcome and never publishes that state.
    const [, outcome] = ledger.state.apply(verified, context);
    if (outcome.type !== 'success') throw Error('O8 cannot settle at accepted head: '+outcome.error);
    return sign(intent.signatureData(segment));
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
