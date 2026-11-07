<!-- source: https://docs.midnight.network/relnotes/node.md fetched_at: 2026-11-07T20:48:43Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Node

Node is a core component of the Midnight network, responsible for syncing, validating transactions, and maintaining the chain state.

[Link to related documentation](https://docs.midnight.network/nodes.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version2.1.101

StatusAll

### [Release <!-- -->2.1.101](https://docs.midnight.network/relnotes/node/node-2-1-101.md)LATEST

6 November 2026

#### Artifacts

* [Midnight node](https://hub.docker.com/r/midnightntwrk/midnight-node)
* [Node toolkit](https://hub.docker.com/r/midnightntwrk/midnight-node-toolkit)
* [GitHub release](https://github.com/midnightntwrk/midnight-node/releases/tag/node-2.1.101)

#### Summary

* Upgrade every Mainnet validator and RPC node from v1.0.400 or v2.1.100 directly to v2.1.101 before the governance upgrade on 6 November 2026, with block producers upgrading together. The v2.1.0 release candidates are not a supported upgrade step. A node on v1.0.400 stops at the upgrade block.
* Binary first, then runtime: the chain keeps running ledger 8 until a governance action applies the runtime upgrade and moves Mainnet to ledger 9.1.0.1. The binary upgrade is one-way, so snapshot the data directory first. The node runs the 2.1.100 runtime (`spec_version` 2001100), which the governance upgrade enacts.
* Known issue: a maintenance transaction that removes a verifier key (`VerifierKeyRemove`) is rejected with `BalanceCheckOverspend` when client packages use ledger 9.1.0.0. Inserting a verifier key and replacing the maintenance authority work, so most contracts are unaffected. Update clients to ledger 9.1.0.1 when it is published; until then, redeploy a cross-contract caller that needs the 0.36.0 fix.
* Known issue: keep the default `separate` storage layout. Setting `storage_separation=unified` on an existing v1.0.400 data directory silently stops block import. To move to `unified`, stop the node, delete `chains/` and `ledger_storage/`, and resync.
* Added the `SafeMode` and `C2MBridge` pallets. The bridge pallet ships turned off and stays off until governance enables it, with a rollout announced separately.
* Removed the `smart-contracts`, `sign-address-association`, and `sign-block-producer-metadata` subcommands. The node connects to Cardano db-sync over TLS only.
* After the upgrade, ledger 8 transactions do not decode on Mainnet. DApps and wallets need wallet SDK 2.0.0, Midnight.js 5.0.0, and indexer 4.4.101, and indexers re-index from genesis. See the ledger 9 upgrade page at /relnotes/ledger-9.
* DUST resets once at the upgrade and every Mainnet registration is replayed to its full cap, so Mainnet users do not need to do anything. Transactions submitted during the replay wait in the transaction pool and are included once it ends.
* Preview was reset on 6 November 2026 as a new chain that runs ledger 9, with no ledger 8 history. Start a Preview node on v2.1.101 with a fresh data directory. Preprod keeps running node v1.0.400 until a later upgrade.
