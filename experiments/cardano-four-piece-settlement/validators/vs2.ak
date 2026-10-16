//// VS2 accounting piece (four-piece settlement, PROPOSED, unreviewed, uncompiled, unrun).
//// Withdraw-zero script, run once per settle transaction, no parameter. Per covered thread
//// it checks the recomputed post values, the Core bounds and every signer's limits on the
//// complete next snapshot (`shard4.vs2_accounting`). Any other purpose fails.

use cardano/address.{Credential}
use cardano/transaction.{Transaction}
use pvc2/shard4.{VsPieceV1}

validator vs2 {
  withdraw(redeemer: VsPieceV1, _account: Credential, self: Transaction) {
    shard4.vs2_accounting(redeemer, self)
  }

  else(_) {
    fail
  }
}
