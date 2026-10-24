import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {withOwnedResources,ProducerCleanupError} from '../integration/owned-resources.mjs';
import {assertCanonicalTtl} from '../integration/producer-policy.mjs';
// Public byte copies use the existing driver record ceiling, never fee guesses.
const copy=bytes=>{assert.ok(bytes instanceof Uint8Array&&bytes.length<=16*1024*1024);return bytes.slice();};
export function sealedCallFingerprint(tx,ledger,{complete}={}) {
  return withOwnedResources((own,many)=>{
    assert.equal(own(tx.rewards),undefined);assert.equal(own(tx.guaranteedOffer),undefined);assert.equal(many(tx.fallibleOffer),undefined);
    const intents=many(tx.intents);assert.ok(intents instanceof Map);
    const keys=[...intents.keys()];complete??=keys.length===2;assert.deepEqual(keys,complete?[1,2]:[1]);
    const main=intents.get(1);assertCanonicalTtl(main.ttl);
    for(const intent of intents.values()){
      assert.equal(own(intent.guaranteedUnshieldedOffer),undefined);assert.equal(own(intent.fallibleUnshieldedOffer),undefined);
      assert.equal(intent.ttl.getTime(),main.ttl.getTime());
    }
    const actions=many(main.actions);assert.equal(actions.length,1);const call=actions[0];
    assert.ok(call instanceof ledger.ContractCall);assert.equal(call.entryPoint,'acceptValidity');
    assert.equal(own(main.dustActions),undefined);
    const proof=own(call.proof),binding=own(main.binding);assert.equal(proof.instance,'proof');assert.equal(binding.instance,'binding');
    if(complete){
      const fee=intents.get(2),feeActions=many(fee.actions);assert.equal(feeActions.length,0);
      const dust=own(fee.dustActions),spends=many(dust.spends),registrations=many(dust.registrations);
      assert.equal(registrations.length,0);assert.equal(spends.length,1);assert.equal(typeof spends[0].vFee,'bigint');assert.ok(spends[0].vFee>0n);
      const dustProof=own(spends[0].proof);assert.equal(dustProof.instance,'proof');
      const feeBinding=own(fee.binding);assert.equal(feeBinding.instance,'binding');
    }
    return {intent:copy(main.serialize()),proof:copy(proof.serialize()),binding:copy(binding.serialize()),hash0:main.intentHash(0),hash1:main.intentHash(1),address:call.address,entryPoint:call.entryPoint,ttl:main.ttl.getTime()};
  },{borrowed:[tx]});
}
export function assertSealedCallPreserved(before,after) {
  assert.deepEqual(after,before,'ERR_CALL_INTENT_CHANGED_AFTER_FEE_MERGE');
}
// Real ContractCall.address is a wasm hex string. Same hex semantics as the
// unchanged financial/receipt guard plus whole bytes; no guessed byte length.
export function normalizeContractAddress(value) {
  assert.equal(typeof value,'string','ERR_CALL_ADDRESS_TYPE');
  const s=value.replace(/^0x/,'').toLowerCase();
  assert.ok(/^[0-9a-f]+$/.test(s)&&s.length%2===0,'ERR_CALL_ADDRESS_REPRESENTATION');
  return s;
}
const contexts=new WeakSet(),models=new WeakSet();
const sha=bytes=>createHash('sha256').update(bytes).digest('hex');
const hex64=v=>typeof v==='string'&&/^[0-9a-f]{64}$/.test(v);
// Frozen from the adopted deployment record before the call is prepared. It
// never accepts a transaction fingerprint: before/after cannot authenticate.
export function freezeCallContext({network,address,entryPoint,deploymentRecordSha256}) {
  assert.equal(network,'stagenet');assert.equal(entryPoint,'acceptValidity');
  assert.ok(hex64(deploymentRecordSha256),'ERR_CALL_CONTEXT_DEPLOYMENT');
  const context=Object.freeze({schema:'sherlock-sealed-call-context/1',network,address:normalizeContractAddress(address),entry_point:entryPoint,deployment_record_sha256:deploymentRecordSha256});
  contexts.add(context);return context;
}
export const isCallContext=value=>contexts.has(value);
export function assertCallContext(fingerprint,context) {
  assert.ok(contexts.has(context),'ERR_CALL_CONTEXT_UNFROZEN');
  assert.equal(normalizeContractAddress(fingerprint.address),context.address,'ERR_CALL_ADDRESS_NOT_DEPLOYMENT');
  assert.equal(fingerprint.entryPoint,context.entry_point,'ERR_CALL_ENTRY_NOT_CONTEXT');
}
// Full TransactionCostModel.serialize: runtime plus remaining cost fields.
// CostModel has no serialize. feePrices remain a separate current-price check.
const modelHex=model=>{
  assert.equal(typeof model?.serialize,'function','ERR_CALL_TRANSACTION_COST_MODEL_API');
  const bytes=model.serialize();
  assert.ok(bytes instanceof Uint8Array&&bytes.length>0&&bytes.length<=1024*1024,'ERR_CALL_TRANSACTION_COST_MODEL_BYTES');
  return Buffer.from(bytes).toString('hex');
};
// Async form of withOwnedResources: frees only after the body settles, drains
// every distinct acquisition independently and keeps the first primary error.
async function withOwnedAsync(body,{borrowed=[]}={}) {
  const excluded=new Set(borrowed),seen=new Set(),acquired=[];
  const own=value=>{
    if(value!==null&&(typeof value==='object'||typeof value==='function')&&!excluded.has(value)&&!seen.has(value)){seen.add(value);acquired.push(value);}
    return value;
  };
  let result,primary,primaryPresent=false;
  try {result=await body(own);}
  catch(error){primary=error;primaryPresent=true;}
  const cleanupErrors=[];
  for(let i=acquired.length-1;i>=0;i--) {
    try {const free=acquired[i].free;if(typeof free==='function')free.call(acquired[i]);}
    catch(error){cleanupErrors.push(error);}
  }
  if(cleanupErrors.length)throw new ProducerCleanupError(primaryPresent,primary,cleanupErrors);
  if(primaryPresent)throw primary;
  return result;
}
// One genuine call proof. Serialized model and runtime view come from the same
// owned TransactionCostModel handle of the borrowed authenticated parameters.
export async function proveWithBoundModel({transaction,provider,provingParameters,provingIdentity,track}) {
  assert.equal(typeof track,'function');
  const {sha256,block}=provingIdentity??{};
  assert.ok(hex64(sha256),'ERR_CALL_PROVING_PARAMETERS_IDENTITY');
  assert.ok(['number','bigint','string'].includes(typeof block)&&/^[0-9]+$/.test(String(block)),'ERR_CALL_PROVING_BLOCK');
  const serialized=provingParameters.serialize();assert.ok(serialized instanceof Uint8Array,'ERR_CALL_PROVING_PARAMETERS_API');
  return withOwnedAsync(async own=>{
    const costModel=own(provingParameters.transactionCostModel),modelBytes=modelHex(costModel);
    const runtime=own(costModel.runtimeCostModel);
    assert.ok(runtime!==null&&typeof runtime==='object','ERR_CALL_RUNTIME_COST_MODEL');
    const provingModel=Object.freeze({schema:'sherlock-call-proving-model/1',transaction_cost_model_hex:modelBytes,transaction_cost_model_sha256:sha(Buffer.from(modelBytes,'hex')),parameters_sha256:sha256,parameters_block:String(block),ledger_parameters_serialize_sha256:sha(serialized),fee_prices:'NOT_BOUND_SEPARATE_CURRENT_FEE_CHECK'});
    // Installed ledger-v9 rc.3 bg.js 7737-7741 Transaction.prove BORROWS:
    // _assertClass then cost_model.__wbg_ptr, no __destroy_into_raw (Rust
    // &CostModel). Runtime view stays owned/live until this proof settles,
    // then is freed once. Unbound is tracked before cleanup so a cleanup
    // failure still leaves it caller-owned.
    const unbound=await transaction.prove(provider,runtime);track(unbound);
    models.add(provingModel);return {unbound,provingModel};
  },{borrowed:[transaction,provider,provingParameters]});
}
function assertParametersModel(provingModel,parameters,missing,changed) {
  assert.ok(models.has(provingModel),'ERR_CALL_PROVING_MODEL_UNBOUND');
  assert.ok(parameters!==null&&typeof parameters==='object',missing);
  withOwnedResources(own=>{
    assert.equal(modelHex(own(parameters.transactionCostModel)),provingModel.transaction_cost_model_hex,changed);
  },{borrowed:[parameters]});
}
// Must run on the returned FINALIZED_TRANSACTION recipe before signRecipe and
// on the signed recipe before the real DUST finalize.
export function assertRecipeProvingModel(recipe,provingModel) {
  assert.equal(recipe?.type,'FINALIZED_TRANSACTION');
  assertParametersModel(provingModel,recipe.blockData?.ledgerParameters,'ERR_CALL_RECIPE_PARAMETERS_MISSING','ERR_CALL_RECIPE_COST_MODEL_CHANGED');
}
// Must run on fresh post-proof parameters before financial acceptance/send.
// Fresh fee repricing does not establish this equality.
export function assertFreshProvingModel(provingModel,freshParameters) {
  assertParametersModel(provingModel,freshParameters,'ERR_CALL_FRESH_PARAMETERS_MISSING','ERR_CALL_FRESH_COST_MODEL_CHANGED');
}
