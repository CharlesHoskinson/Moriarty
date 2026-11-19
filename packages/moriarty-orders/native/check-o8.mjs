import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtempSync,readFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {buildO8} from '../src/build-o8.mjs';
import {replayNativeReceipt} from './replay.mjs';

const root=fileURLToPath(new URL('../../../',import.meta.url));
const pkg=join(root,'packages/moriarty-orders');
const dir=resolve(process.argv[2]||join(root,'deliverables/competitor-report/IV.3'));
const validator=process.env.MORIARTY_ORDER_VALIDATOR||join(root,'target/debug/examples/validate_call');
const modules=resolve(process.env.MORIARTY_ORDER_MODULES||join(pkg,'../../tools/sherlock-sdk/node_modules'));
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const hex=b=>Buffer.from(b).toString('hex');
const read=name=>readFileSync(join(dir,name));
const report=JSON.parse(read('result.json'));
const receipts=JSON.parse(read('receipts.json'));
const fixture=JSON.parse(readFileSync(join(pkg,'test/fixtures/o8.json')));
assert.equal(report.status,'passed');assert.equal(report.requirement,'IV.3');
assert.equal(hash(read('receipts.json')),report.receiptsSha256);
assert.equal(hash(read('parameters.bin')),report.protocolParameters.sha256);
assert.deepEqual(receipts.map(r=>r.label),['setup-night-reward','setup-dust-registration','deploy-O8-faucet',
  'underfunded-mint-three-assets','underfunded-deploy-exchange','funded-mint-three-assets','funded-deploy-exchange',
  'atomic-three-owner-settlement','unchanged-O8-at-horizon','carol-spends-one-C-before-inclusion','unchanged-O8-missing-C']);
const output=mkdtempSync(join(tmpdir(),'moriarty-o8-replay-'));
const L=await import(pathToFileURL(join(modules,'@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
const rt=await import(pathToFileURL(join(modules,'@midnight-ntwrk/compact-runtime/dist/index.js')));
try{
  const rebuilt=buildO8(output,{skipZk:true});
  for(const key of Object.keys(rebuilt))assert.deepEqual(rebuilt[key],report.build[key],key);
  const generated=await import(pathToFileURL(join(output,'contract/index.js')));
  const validated=receipts.map(row=>replayNativeReceipt(validator,dir,row));
  const settle=receipts[7],rejected=receipts[8],spent=receipts[10];
  for (const [record,status] of [[validated[7],'success'],[validated[8],'failure'],[validated[10],'failure']]){
    assert.equal(record.outcome,status);
    assert.equal(record.contractCalls.length,1);
    const [call]=record.contractCalls;
    assert.equal(call.address,report.deployment.address);assert.equal(call.proofVersion,'V3');
    assert.equal(call.entryPoint,'settle');assert.equal(call.verifierKeySHA256,report.build.verifierSha256);
  }
  assert.equal(settle.transaction.sha256,rejected.transaction.sha256);
  assert.equal(settle.transaction.sha256,spent.transaction.sha256);
  assert.match(spent.error,/UTXO|unshielded.*input/i);
  assert.match(rejected.error,/mismatch.*read/i);
  const address=report.deployment.address;
  function stateAt(ref){
    const state=L.LedgerState.deserialize(read(ref.file));
    const raw=state.index(address);
    const contract=rt.ContractState.deserialize(raw.serialize()),fields=generated.ledger(contract.data);
    assert.equal(hex(fields.exchange.program),report.build.programHash);
    assert.equal(hex(fields.exchange.network),hash('undeployed'));
    const wallets=fields.exchange.wallets.map(w=>rt.decodeUserAddress(w.bytes)),colors=fields.exchange.colors.map(hex);
    const funds=wallets.map(a=>colors.map(color=>[...state.utxo.filter(a)].filter(c=>c.type===color).reduce((n,c)=>n+Number(c.value),0)));
    assert.ok([...contract.balance].every(([,v])=>v===0n));
    return {state,fields,wallets,colors,funds};
  }
  const pre=stateAt(settle.beforeState),post=stateAt(settle.afterState),afterRefusal=stateAt(rejected.afterState);
  assert.ok(spent.error.includes(`value: ${fixture.amounts[2]}, owner: UserAddress(${pre.wallets[2]})`));
  assert.ok(spent.error.includes(`UnshieldedTokenType(${pre.colors[2]})`));
  assert.deepEqual(pre.funds,fixture.funded);
  assert.deepEqual(post.funds,fixture.settled);assert.deepEqual(afterRefusal.funds,fixture.funded);
  assert.equal(pre.fields.revision,0n);assert.equal(post.fields.revision,1n);assert.equal(post.fields.remaining,0n);
  assert.equal(afterRefusal.fields.revision,0n);assert.equal(afterRefusal.fields.remaining,1n);
  assert.deepEqual(stateAt(spent.beforeState).funds,fixture.unfunded);
  assert.deepEqual(stateAt(spent.afterState).funds,fixture.unfunded);
  assert.equal(stateAt(spent.afterState).fields.revision,0n);
  assert.equal(spent.beforeState.sha256,report.spentLeg.stateSha256);
  assert.deepEqual(pre.wallets,report.fundingWallets);assert.deepEqual(pre.fields.exchange.amounts,fixture.amounts.map(BigInt));
  const tx=L.Transaction.deserialize('signature','proof','binding',read(settle.transaction.file));
  const signedInputs=[],assetOutputs=[];
  for(const [segment,intent] of tx.intents){
    for(const prop of ['guaranteedUnshieldedOffer','fallibleUnshieldedOffer']){
      const offer=intent[prop];if(!offer)continue;
      for(const [i,input] of offer.inputs.entries()){
        const signature=offer.signatures[i].value;
        assert.ok(L.verifySignature(input.owner,intent.signatureData(segment),signature));
        signedInputs.push({wallet:L.addressFromKey(input.owner),color:input.type,amount:Number(input.value)});
      }
      for(const coin of offer.outputs)assetOutputs.push({to:coin.owner,color:coin.type,amount:Number(coin.value)});
    }
  }
  assert.equal(signedInputs.length,3);assert.equal(assetOutputs.length,3);
  for(let i=0;i<3;i++){
    const expected={wallet:pre.wallets[i],color:pre.colors[i],amount:fixture.amounts[i]};
    assert.deepEqual(signedInputs.find(x=>x.color===pre.colors[i]),expected);
    assert.deepEqual(assetOutputs.find(x=>x.color===pre.colors[i]),{to:pre.wallets[(i+1)%3],color:pre.colors[i],amount:fixture.amounts[i]});
    const grant=report.authorizations[i], key=pre.fields.signingKeys[i];
    assert.equal(grant.ownerKey.x,String(key.x));assert.equal(grant.ownerKey.y,String(key.y));
    const message=generated.pureCircuits.legMessage(pre.fields.exchange,{bytes:rt.encodeContractAddress(address)},BigInt(i),0n);
    assert.equal(grant.message,String(message));
    const sig={announcement:{x:BigInt(grant.signature.announcement.x),y:BigInt(grant.signature.announcement.y)},response:BigInt(grant.signature.response)};
    assert.equal(rt.jubjubSchnorrVerify(new rt.CompactTypeVector(1,rt.CompactTypeField),[message],key,sig),true);
  }
  assert.deepEqual(report.refusals.map(r=>r.label).sort(),[...fixture.refusals].sort());
  for(const row of report.refusals){assert.equal(row.before,row.after);assert.deepEqual(row.balances,row.label==='unfunded third leg'?fixture.unfunded:fixture.funded);}
  assert.ok(BigInt(settle.nativeFeeSpeck)>0n);assert.deepEqual(report.resources,settle.resources);
  console.log(JSON.stringify({schema:'moriarty-orders/o8-native-replay/1',requirement:'IV.3',receiptsSha256:report.receiptsSha256,
    validatorSha256:hash(readFileSync(validator)),signedInputs,assetOutputs,transactions:validated}));
}finally{rmSync(output,{recursive:true,force:true});}
