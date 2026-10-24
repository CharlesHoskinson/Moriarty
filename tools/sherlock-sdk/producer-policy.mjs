// Pure policy: no SDK, wallet, filesystem, network or timer dependencies.
import assert from 'node:assert/strict';
import {withOwnedResources} from './owned-resources.mjs';
export function canonicalTtl(value) {
  assert.ok(value instanceof Date);
  const ms=value.getTime();assert.ok(Number.isSafeInteger(ms)&&ms>=0,'ERR_PRODUCER_TTL');
  return new Date(ms-ms%1000);
}
export function assertCanonicalTtl(value) {
  assert.equal(canonicalTtl(value).getTime(),value.getTime(),'ERR_PRODUCER_TTL_PRECISION');
}
// Only callers enforcing the no-unproven-call domain may use this fee API.
// The fee-only mock never escapes this scope into a real recipe or signer.
export function exactMockFee(transaction,parameters) {
  return withOwnedResources(own=>{
    const priced=own(transaction.mockProve());
    const fee=priced.feesWithMargin(parameters,5);
    assert.equal(typeof fee,'bigint');assert.ok(fee>0n,'ERR_PRODUCER_FEE');return fee;
  },{borrowed:[transaction,parameters]});
}
export function assertRecipeParameters(recipe,parameters) {
  const actual=recipe.blockData?.ledgerParameters;
  assert.ok(actual,'ERR_PRODUCER_RECIPE_PARAMETERS');
  return withOwnedResources(own=>{
    const actualModel=own(actual.transactionCostModel);
    const expectedModel=own(parameters.transactionCostModel);
    assert.equal(typeof actualModel?.serialize,'function','ERR_PRODUCER_TRANSACTION_COST_MODEL_API');
    assert.equal(typeof expectedModel?.serialize,'function','ERR_PRODUCER_TRANSACTION_COST_MODEL_API');
    // CostModel has no serialize API. The supported parent serialization
    // includes runtimeCostModel and the remaining transaction-cost fields.
    // No runtime getter copies are needed or acquired for this comparison.
    assert.deepEqual(actualModel.serialize(),expectedModel.serialize(),'ERR_PRODUCER_TRANSACTION_COST_MODEL_CHANGED');
  },{borrowed:[recipe,actual,parameters]});
}
// Freshly fetched parameters belong to this producer scope. Original proving
// and recipe parameters are supplied as borrowed identities and stay live.
export function withFreshParameters(fresh,body,{borrowed=[]}={}) {
  return withOwnedResources(own=>body(own(fresh.params)),{borrowed});
}
