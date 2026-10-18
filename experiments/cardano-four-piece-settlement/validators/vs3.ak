//// VS3 identity piece (four-piece settlement, PROPOSED, unreviewed, uncompiled, unrun).
//// Withdraw-zero script, run once per settle transaction. Parameters: the reviewed C2 hash
//// and the VI hash. It requires VI in the same transaction with the same coverage, then
//// checks per thread the claim script, the pinned shape digest, the claim envelope and the
//// statement digest built with VI's published intents digest (`shard4.vs3_identity`).
//// Any other purpose fails.

use cardano/address.{Credential}
use cardano/transaction.{Transaction}
use pvc2/shard4.{VsPieceV1}

validator vs3(c2_hash: ByteArray, vi_hash: ByteArray) {
  withdraw(redeemer: VsPieceV1, _account: Credential, self: Transaction) {
    shard4.vs3_identity(c2_hash, vi_hash, redeemer, self)
  }

  else(_) {
    fail
  }
}
