import assert from 'node:assert/strict';
import test from 'node:test';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
const require = createRequire('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/package.json');
const ledger = await import(pathToFileURL(require.resolve('@midnight-ntwrk/ledger-v8')).href);
const module = await import('./receipt.mjs').catch(() => ({}));

// These are native serialized test transactions, not valid network proofs or receipts.
async function nativeFixture({signed=true,deploy=false}={}) {
  const key = ledger.signingKeyFromBip340(Uint8Array.from({length:32}, (_, i) => i+1));
  const owner = ledger.signatureVerifyingKey(key);
  const address = ledger.addressFromKey(owner);
  const input = {owner, type:'01'.repeat(32), value:100n, intentHash:'02'.repeat(32), outputNo:0};
  let intent = ledger.Intent.new(new Date('2030-01-01T00:00:00Z'));
  const deployment=deploy?new ledger.ContractDeploy(new ledger.ContractState()):undefined;
  if(deployment) intent=intent.addDeploy(deployment);
  intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new([input], [{owner:address,type:input.type,value:70n}], []);
  let tx = ledger.Transaction.fromParts('undeployed',undefined,undefined,intent);
  tx = await tx.prove({check(){throw Error('ACTUAL_PROVER_FORBIDDEN');},prove(){throw Error('ACTUAL_PROVER_FORBIDDEN');}},ledger.CostModel.initialCostModel());
  const intents = tx.intents;
  for (const [segment, item] of signed?intents:[]) {
    item.guaranteedUnshieldedOffer = item.guaranteedUnshieldedOffer.addSignatures([ledger.signData(key,item.signatureData(segment))]);
    intents.set(segment,item);
  }
  tx.intents = intents;
  return {tx:tx.bind(),address,contractAddress:deployment?.address};
}

test('native byte decoder exposes signed owners and complete gross inputs and outputs', async () => {
  assert.equal(typeof module.decodeNativeFinancialTransaction,'function','production byte decoder must exist');
  const {tx,address} = await nativeFixture();
  const result = module.decodeNativeFinancialTransaction(tx.serialize(), ledger);
  assert.equal(result.inputs.length,1);
  assert.equal(result.inputs[0].owner,address);
  assert.equal(result.inputs[0].value,'100');
  assert.equal(result.outputs[0].value,'70');
  assert.equal(result.grossByAsset['01'.repeat(32)],'100');
  assert.equal(result.proofVerified,false);
  assert.equal(result.ledgerAccepted,false);
});

test('native byte decoder rejects noncanonical bytes and missing input signatures', async () => {
  assert.equal(typeof module.decodeNativeFinancialTransaction,'function');
  const {tx} = await nativeFixture();
  assert.throws(() => module.decodeNativeFinancialTransaction(Uint8Array.from([...tx.serialize(),0]),ledger));
  const missing = await nativeFixture({signed:false});
  assert.throws(() => module.decodeNativeFinancialTransaction(missing.tx.serialize(),ledger),/SIGNATURE/);
});

test('finality observer checks canonical block and queries state at the same block hash', async () => {
  assert.equal(typeof module.observeFinalizedStage,'function','production observer must exist');
  const {tx} = await nativeFixture();
  const hash='03'.repeat(32), address='04'.repeat(32); const queries=[];
  const finalized={tx,txId:tx.identifiers()[0],identifiers:tx.identifiers(),txHash:tx.transactionHash(),status:'SucceedEntirely',blockHash:hash,blockHeight:10,protocolVersion:8,fees:{paidFees:'12',estimatedFees:'12'},unshielded:{created:[],spent:[]}};
  const provider={watchForTxData:async()=>finalized,queryContractState:async(a,c)=>{queries.push([a,c]);return {data:'state'};},queryUnshieldedBalances:async(a,c)=>{queries.push([a,c]);return [];} };
  const rpc=async name=>({chain_getFinalizedHead:'0x'+hash,chain_getHeader:{number:'0xa'},chain_getBlockHash:'0x'+hash}[name]);
  // A native no-call fixture cannot masquerade as settlement of the named contract.
  await assert.rejects(module.observeFinalizedStage({provider,rpc,ledger,expectedProtocolVersion:8,txId:finalized.txId,contractAddress:address,circuitId:'settle',decodeState:()=>({}),deadlineMs:Date.now()+1000}),/CONTRACT_ACTION/);
  finalized.status='FailEntirely';
  await assert.rejects(module.observeFinalizedStage({provider,rpc,ledger,expectedProtocolVersion:8,txId:finalized.txId,contractAddress:address,circuitId:'settle',decodeState:()=>({}),deadlineMs:Date.now()+1000}),/TRANSACTION_STATUS/);
});

test('observer traverses native deployment bytes with inert RPC and rejects financial observation corruption', async () => {
  const {tx,address,contractAddress}=await nativeFixture({deploy:true});
  const blockHash='03'.repeat(32), queries=[];
  const [segment,intent]=[...tx.intents][0];
  const data={tx,txId:tx.identifiers()[0],identifiers:tx.identifiers(),txHash:tx.transactionHash(),status:'SucceedEntirely',blockHash,blockHeight:10,protocolVersion:8,fees:{paidFees:'0',estimatedFees:'0'},unshielded:{spent:[{owner:address,tokenType:'01'.repeat(32),value:100n,intentHash:'02'.repeat(32)}],created:[{owner:address,tokenType:'01'.repeat(32),value:70n,intentHash:intent.intentHash(segment)}]}};
  const provider={watchForTxData:async()=>data,queryContractState:async(a,c)=>{queries.push([a,c]);return {data:'observed-state'};},queryUnshieldedBalances:async(a,c)=>{queries.push([a,c]);return [];} };
  let canonical='0x'+blockHash;
  const rpc=async name=>({chain_getFinalizedHead:'0x'+blockHash,chain_getHeader:{number:'0xa'},chain_getBlockHash:canonical}[name]);
  const args={provider,rpc,ledger,expectedProtocolVersion:8,txId:data.txId,contractAddress,circuitId:'deploy',decodeState:x=>({actual:x}),deadlineMs:Date.now()+5000};
  const observed=await module.observeFinalizedStage(args);
  assert.deepEqual(queries,[[contractAddress,{type:'blockHash',blockHash}],[contractAddress,{type:'blockHash',blockHash}]]);
  assert.deepEqual(observed.state,{actual:'observed-state'});
  assert.equal(observed.receipt.transaction.inputs[0].value,'100');
  assert.equal(observed.receipt.transaction.proofVerified,false);
  data.unshielded.created[0].value=71n;
  await assert.rejects(module.observeFinalizedStage(args),/INDEXED_OUTPUTS_MISMATCH/);
  data.unshielded.created[0].value=70n;
  data.fees.paidFees='1';
  await assert.rejects(module.observeFinalizedStage(args),/DUST_FEE_MISMATCH/);
  data.fees.paidFees='0'; canonical='0x'+'04'.repeat(32);
  await assert.rejects(module.observeFinalizedStage(args),/NONCANONICAL_BLOCK/);
});
