//// S2' thread validator (four-piece settlement, PROPOSED, unreviewed, uncompiled, unrun).
//// One script hash serves as the thread-token policy (genesis, burn) and the custody
//// spend script, as S2 did. Research candidate only: no funds, no Preprod use.
////
//// Parameters, in application order: the reviewed C2 hash, the VI hash, then the VS1,
//// VS2, VS3 and VS4 piece hashes. Acyclic: compile C2, apply it to VI, apply (C2, VI) to
//// VS3, compile VS1, VS2 and VS4 unparameterized, then apply all six to S2'.
//// Settle is delegated to the four pieces (`shard4.delegated`); VS3 requires VI. Revoke
//// and Close run the unchanged W1 gates here. Genesis adds the identity and
//// shard-credential guards.

use cardano/assets.{PolicyId}
use cardano/transaction.{OutputReference, Transaction}
use pvc2/shard
use pvc2/shard4
use pvc2/thread
use pvc2/types.{
  BurnThreads, Close, Genesis, Revoke, S2Mint, Settle, SettleRedeemerV2,
  ThreadDatumV2,
}

validator s2p(
  c2_hash: ByteArray,
  vi_hash: ByteArray,
  vs1_hash: ByteArray,
  vs2_hash: ByteArray,
  vs3_hash: ByteArray,
  vs4_hash: ByteArray,
) {
  mint(redeemer: S2Mint, policy_id: PolicyId, self: Transaction) {
    when redeemer is {
      Genesis { seed } ->
        shard4.genesis(
          c2_hash,
          vi_hash,
          [vs1_hash, vs2_hash, vs3_hash, vs4_hash],
          policy_id,
          seed,
          self,
        )
      BurnThreads -> thread.burn_only(policy_id, self)
    }
  }

  spend(
    datum: Option<Data>,
    redeemer: Data,
    own_ref: OutputReference,
    self: Transaction,
  ) {
    expect Some(raw) = datum
    if shard.is_settle(redeemer) {
      shard4.delegated(
        c2_hash,
        vi_hash,
        [vs1_hash, vs2_hash, vs3_hash, vs4_hash],
        own_ref,
        self,
      )
    } else {
      expect td: ThreadDatumV2 = raw
      expect r: SettleRedeemerV2 = redeemer
      when r is {
        Settle { .. } -> False
        Revoke { signer, signature, continuation } ->
          thread.revoke(td, own_ref, signer, signature, continuation, self)
        Close { reserve_output } ->
          thread.close(td, own_ref, reserve_output, self)
      }
    }
  }

  else(_) {
    fail
  }
}
