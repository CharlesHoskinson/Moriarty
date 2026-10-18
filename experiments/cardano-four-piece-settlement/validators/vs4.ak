//// VS4 custody piece (four-piece settlement, PROPOSED, unreviewed, uncompiled, unrun).
//// Withdraw-zero script, run once per settle transaction, no parameter. Per covered thread
//// it checks the exact custody value before, the complete continuation (datum from the
//// claim envelope's statement digest, exact custody value after) and the tagged external
//// payouts (`shard4.vs4_custody`). Any other purpose fails.

use cardano/address.{Credential}
use cardano/transaction.{Transaction}
use pvc2/shard4.{VsPieceV1}

validator vs4 {
  withdraw(redeemer: VsPieceV1, _account: Credential, self: Transaction) {
    shard4.vs4_custody(redeemer, self)
  }

  else(_) {
    fail
  }
}
