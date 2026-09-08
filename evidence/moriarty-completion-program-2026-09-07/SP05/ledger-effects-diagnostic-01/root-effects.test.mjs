import {test} from 'node:test';
import assert from 'node:assert/strict';
import {recheckFinalizedAgainstRecipe} from '/home/charl/Moriarty/.worktrees/sp05-ledger-integration-grok/experiments/moriarty-midnight-financial/ledger/providers.mjs';

// Controlled objects exercise the candidate's existing public helper contract.
// They are not signed/proven ledger transactions or operational authority.
function intent(segment, owner = 'approved-recipient') {
  return {
    ttl: 1700000100,
    guaranteedUnshieldedOffer: {
      inputs: [], outputs: [{segment, outputNo: 0, type: 'asset-A', value: 12n, owner}], signatures: [],
    },
    fallibleUnshieldedOffer: {
      inputs: [], outputs: [{segment, outputNo: 1, type: 'asset-B', value: 7n, owner}], signatures: [],
    },
  };
}
function fixture() {
  const baseTransaction = {intents: new Map([[1, intent(1)]])};
  const balancingTransaction = {intents: new Map([[2, intent(2)]])};
  const recipe = {baseTransaction, balancingTransaction};
  const finalized = {intents: new Map([...structuredClone(baseTransaction.intents), ...structuredClone(balancingTransaction.intents)])};
  return {recipe, finalized};
}
test('control: unchanged base and balancing effects are accepted', () => {
  const {recipe, finalized} = fixture();
  assert.equal(recheckFinalizedAgainstRecipe(finalized, recipe).ok, true);
});
for (const [name, mutate] of [
  ['fallible recipient', f => {f.intents.get(1).fallibleUnshieldedOffer.outputs[0].owner = 'unapproved-recipient';}],
  ['fallible amount', f => {f.intents.get(1).fallibleUnshieldedOffer.outputs[0].value = 700n;}],
  ['balancing recipient', f => {f.intents.get(2).guaranteedUnshieldedOffer.outputs[0].owner = 'unapproved-recipient';}],
  ['additional intent', f => {f.intents.set(3, intent(3, 'unapproved-recipient'));}],
  ['missing balancing intent', f => {f.intents.delete(2);}],
  ['missing expiration', f => {delete f.intents.get(1).ttl;}],
]) {
  test(`reject changed finalized ${name}`, () => {
    const {recipe, finalized} = fixture();
    mutate(finalized);
    assert.throws(() => recheckFinalizedAgainstRecipe(finalized, recipe));
  });
}
