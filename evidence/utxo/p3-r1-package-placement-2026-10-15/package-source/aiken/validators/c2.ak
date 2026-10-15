//// C2 claim validator (ABI v2, P3S1, PROPOSED, unreviewed, uncompiled). Issue verifies
//// one Groth16 proof; release and refund follow N1' (claimant signs every path).

use cardano/assets.{PolicyId}
use cardano/transaction.{OutputReference, Transaction}
use pvc2/claim
use pvc2/types.{Burn, C2Mint, EnvV2, Issue}

validator c2 {
  mint(redeemer: C2Mint, policy_id: PolicyId, self: Transaction) {
    when redeemer is {
      Issue { vk, proof } -> claim.issue(policy_id, vk, proof, self)
      Burn -> claim.burn_only(policy_id, self)
    }
  }

  spend(
    datum: Option<Data>,
    _redeemer: Data,
    own_ref: OutputReference,
    self: Transaction,
  ) {
    expect Some(raw) = datum
    expect env: EnvV2 = raw
    claim.release(own_ref, raw, env, self)
  }

  else(_) {
    fail
  }
}
