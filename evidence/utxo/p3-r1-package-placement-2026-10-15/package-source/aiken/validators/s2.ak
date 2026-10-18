//// S2 thread validator (ABI v2, P3S1-R1 successor, PROPOSED, unreviewed, uncompiled).
//// One script hash serves as the thread-token policy (genesis, burn) and the custody
//// spend script. Research candidate only: no funds, no Preprod use, no settled-payment
//// wording until P3-12's conditions hold.
////
//// RR-02 / M7-a: S2 is parameterized by the reviewed C2 script hash. C2 takes no
//// parameter, so there is no cycle: compile C2, apply its hash to S2, then hash S2.
//// Genesis requires `claim_hash == c2_hash`, and settle re-checks it, so the S2 hash
//// (the signed intent anchor) transitively names the claim verifier script.

use cardano/assets.{PolicyId}
use cardano/transaction.{OutputReference, Transaction}
use pvc2/thread
use pvc2/types.{
  BurnThreads, Close, Genesis, Revoke, S2Mint, Settle, SettleRedeemerV2,
  ThreadDatumV2,
}

validator s2(c2_hash: ByteArray) {
  mint(redeemer: S2Mint, policy_id: PolicyId, self: Transaction) {
    when redeemer is {
      Genesis { seed } -> thread.genesis(c2_hash, policy_id, seed, self)
      BurnThreads -> thread.burn_only(policy_id, self)
    }
  }

  spend(
    datum: Option<ThreadDatumV2>,
    redeemer: SettleRedeemerV2,
    own_ref: OutputReference,
    self: Transaction,
  ) {
    expect Some(td) = datum
    when redeemer is {
      Settle { .. } -> thread.settle(c2_hash, td, own_ref, redeemer, self)
      Revoke { signer, signature, continuation } ->
        thread.revoke(td, own_ref, signer, signature, continuation, self)
      Close { reserve_output } -> thread.close(td, own_ref, reserve_output, self)
    }
  }

  else(_) {
    fail
  }
}
