//// K9 genuine C2 consumer tests (source-only, uncompiled, no results claimed).
////
//// These tests call the UNCHANGED production function `pvc2/claim.issue`
//// with the genuine public VK/proof (`pvc2/k8_fixture`, frozen public
//// K6/K7 exports) and a typed `cardano/transaction.Transaction` built by
//// `pvc2/k9_consumer_builder` from the stdlib `transaction.placeholder`.
//// No gate is duplicated, stubbed or shortcut here, and there is no
//// diagnostic helper: every test is a direct `claim.issue(...)` call.
////
//// Expectations were fixed in EXPECTATIONS.json before any execution:
////   positive  claim.issue(policy, vk, proof, tx) == True
////   N1        genuine proof against the FULL coherent tampered envelope
////             whose statement_digest is the re-derived body digest, with
////             token name/mint/output rebuilt from that envelope == False
////             (first possible refusal: the pairing equation inside
////             groth16.verify_digest; every earlier conjunct is satisfied
////             by construction, so the token-name guard cannot fire first)
////   N2        same construction with the re-derived context digest
////             (seq change) == False, same first-refusal reasoning
////   N3        real A/C-swapped proof against the unchanged positive
////             envelope/transaction == False (A and C are both valid
////             positive G1 points swapped, so uncompress succeeds and the
////             pairing equation must refuse)
//// Crypto negatives use explicit `!claim.issue(...)` Bool assertions,
//// never `fail` annotations, so a False is observed rather than any
//// evaluation error being counted as a pass. No test in this module uses
//// a `fail` annotation: the issuance `expect` patterns (singleton mint
//// pair, singleton claim output, Script holder, InlineDatum) are
//// satisfied by every transaction built here, including the guard tests,
//// so each rejection below is a Bool False from an `and` conjunct, not
//// an evaluation failure.
////
//// Optional non-crypto guards (each states its first expected refusal;
//// `claim.issue` checks the pairing last, so these return False without
//// needing the pairing result to be the deciding factor):
////   wrong minted quantity (2): first refusal `qty == 1`
////   mismatched envelope token name: mint/output carry a zero32 name while
////     qty, holder, quantity and single-token checks pass; first refusal
////     `name == envelope_name(raw)`
////   wrong VK digest: envelope/mint/output rebuilt from an envelope whose
////     vk_digest is zero32; all earlier conjuncts pass; first refusal
////     `env.vk_digest == blake2b_256(vk)`
////
//// The chosen policy id is the locally chosen 28-byte export settle hash
//// (see the builder header): it is NOT a compiled C2 script hash, and no
//// compiled wrapper, ledger or Preprod result is claimed. All context
//// values are existing local fixtures, not real Preprod data.

use pvc2/claim
use pvc2/k8_fixture
use pvc2/k9_consumer_builder

test k9_positive_claim_issue_returns_true() {
  let (policy_id, _env, tx) = k9_consumer_builder.build_positive()
  claim.issue(policy_id, k8_fixture.vk, k8_fixture.proof, tx)
}

test k9_n1_body_rederived_digest_claim_issue_rejected() {
  let env =
    k9_consumer_builder.env_with_statement_digest(
      k8_fixture.n1_statement_digest,
    )
  let tx =
    k9_consumer_builder.build_for_env(
      k9_consumer_builder.chosen_policy_id,
      env,
    )
  !claim.issue(
    k9_consumer_builder.chosen_policy_id,
    k8_fixture.vk,
    k8_fixture.proof,
    tx,
  )
}

test k9_n2_context_rederived_digest_claim_issue_rejected() {
  let env =
    k9_consumer_builder.env_with_statement_digest(
      k8_fixture.n2_statement_digest,
    )
  let tx =
    k9_consumer_builder.build_for_env(
      k9_consumer_builder.chosen_policy_id,
      env,
    )
  !claim.issue(
    k9_consumer_builder.chosen_policy_id,
    k8_fixture.vk,
    k8_fixture.proof,
    tx,
  )
}

test k9_n3_proof_ac_swapped_claim_issue_rejected() {
  let (policy_id, _env, tx) = k9_consumer_builder.build_positive()
  !claim.issue(policy_id, k8_fixture.vk, k8_fixture.n3_proof, tx)
}

test k9_wrong_minted_quantity_claim_issue_rejected() {
  let env = k9_consumer_builder.positive_env()
  let raw: Data = env
  let tx =
    k9_consumer_builder.build_for_env_with_token(
      k9_consumer_builder.chosen_policy_id,
      env,
      claim.envelope_name(raw),
      2,
    )
  !claim.issue(
    k9_consumer_builder.chosen_policy_id,
    k8_fixture.vk,
    k8_fixture.proof,
    tx,
  )
}

test k9_mismatched_envelope_name_claim_issue_rejected() {
  let env = k9_consumer_builder.positive_env()
  let tx =
    k9_consumer_builder.build_for_env_with_token(
      k9_consumer_builder.chosen_policy_id,
      env,
      #"0000000000000000000000000000000000000000000000000000000000000000",
      1,
    )
  !claim.issue(
    k9_consumer_builder.chosen_policy_id,
    k8_fixture.vk,
    k8_fixture.proof,
    tx,
  )
}

test k9_wrong_vk_digest_claim_issue_rejected() {
  let env =
    k9_consumer_builder.env_with_vk_digest(
      #"0000000000000000000000000000000000000000000000000000000000000000",
    )
  let tx =
    k9_consumer_builder.build_for_env(
      k9_consumer_builder.chosen_policy_id,
      env,
    )
  !claim.issue(
    k9_consumer_builder.chosen_policy_id,
    k8_fixture.vk,
    k8_fixture.proof,
    tx,
  )
}
