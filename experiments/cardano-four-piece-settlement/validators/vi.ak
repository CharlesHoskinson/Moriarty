//// VI authorization shard (sharded settlement, PROPOSED, unreviewed, uncompiled, unrun).
//// Withdraw-zero script, run once per settle transaction. Parameter: the reviewed C2
//// hash. It covers every Settle input of the named thread policy, checks authority shape,
//// windows and native co-authority, renders every owner-intent message, verifies every
//// signature and binds each thread's published intents digest (`shard.authorize`).
//// Any other purpose fails. Registration of its stake credential is a deployment step
//// whose witness rule is not pinned (DESIGN-VOTES D-6).

use cardano/address.{Credential}
use cardano/transaction.{Transaction}
use pvc2/shard.{AuthShardV1}

validator vi(c2_hash: ByteArray) {
  withdraw(redeemer: AuthShardV1, _account: Credential, self: Transaction) {
    shard.authorize(c2_hash, redeemer, self)
  }

  else(_) {
    fail
  }
}
