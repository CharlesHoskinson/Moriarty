import {test} from 'node:test';
import assert from 'node:assert/strict';
import {importPinned, validateSignedRecipe, recheckFinalizedAgainstRecipe} from '/home/charl/Moriarty/.worktrees/sp05-ledger-integration-grok/experiments/moriarty-midnight-financial/ledger/providers.mjs';

// Real pinned SDK objects and signatures, synthetic UTXO references. No proof,
// wallet, finalization, submission or ledger-validity claim is made here.
const ledger = await importPinned('@midnight-ntwrk/ledger-v8');
function fixture() {
  const sk = ledger.sampleSigningKey();
  const vk = ledger.signatureVerifyingKey(sk);
  const owner = ledger.addressFromKey(vk);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const intent = ledger.Intent.new(ttl);
  const input = {value: 100n, owner: vk, type: token, intentHash: ledger.sampleIntentHash(), outputNo: 0};
  intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new([input], [{value: 90n, owner, type: token}], []);
  intent.fallibleUnshieldedOffer = ledger.UnshieldedOffer.new([], [{value: 7n, owner, type: token}], []);
  const tx = ledger.Transaction.fromParts('undeployed', undefined, undefined, intent);
  const intents = tx.intents;
  assert.equal(intents.size, 1);
  const [segment, actual] = [...intents][0];
  actual.guaranteedUnshieldedOffer = actual.guaranteedUnshieldedOffer.addSignatures([ledger.signData(sk, actual.signatureData(segment))]);
  intents.set(segment, actual);
  tx.intents = intents;
  const recipe = {type: 'UNPROVEN_TRANSACTION', baseTransaction: tx, ttl};
  assert.deepEqual(validateSignedRecipe(recipe, {vk}, {ledger, tokenType: token}), {ok: true, inputCount: 1, signed: 1});
  const copy = ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', tx.serialize());
  assert.deepEqual(copy.serialize(), tx.serialize());
  return {recipe, copy, segment, vk, token};
}
test('SDK control: signed recipe passes validation and unchanged comparison', () => {
  const {recipe, copy} = fixture();
  assert.equal(recheckFinalizedAgainstRecipe(copy, recipe).ok, true);
});
for (const change of ['recipient', 'amount']) {
  test(`SDK objects: comparison rejects changed fallible ${change}`, () => {
    const {recipe, copy, segment, vk, token} = fixture();
    const intents = copy.intents;
    const i = intents.get(segment);
    const outputs = i.fallibleUnshieldedOffer.outputs;
    if (change === 'recipient') outputs[0].owner = ledger.sampleUserAddress();
    else outputs[0].value += 1n;
    i.fallibleUnshieldedOffer = ledger.UnshieldedOffer.new([], outputs, []);
    intents.set(segment, i); copy.intents = intents;
    // The native signature covers the changed output. This independent check
    // must reject the mutation, while the application's comparison must too.
    assert.throws(() => validateSignedRecipe({baseTransaction: copy, ttl: recipe.ttl}, {vk}, {ledger, tokenType: token}), /signature verify failed/);
    assert.throws(() => recheckFinalizedAgainstRecipe(copy, recipe));
  });
}
