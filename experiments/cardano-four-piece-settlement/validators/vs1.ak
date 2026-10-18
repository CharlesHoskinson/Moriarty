//// VS1 structure piece (four-piece settlement, PROPOSED, unreviewed, uncompiled, unrun).
//// Withdraw-zero script, run once per settle transaction, no parameter. It checks exact
//// coverage, global output allocation once, and per thread the structural shape, own-policy
//// mint, top-up and reserve floor (`shard4.vs1_structure`). Any other purpose fails.

use cardano/address.{Credential}
use cardano/transaction.{Transaction}
use pvc2/shard4.{VsPieceV1}

validator vs1 {
  withdraw(redeemer: VsPieceV1, _account: Credential, self: Transaction) {
    shard4.vs1_structure(redeemer, self)
  }

  else(_) {
    fail
  }
}
