<!-- source: https://docs.midnight.network/relnotes/midnight-indexer.md fetched_at: 2026-11-07T20:48:43Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Midnight Indexer

The Midnight Indexer is a set of components designed to optimize the flow of blockchain data from a Midnight node to end-user applications. It retrieves history of blocks, processes them, stores indexed data efficiently, and provides a GraphQL API for queries and subscriptions.

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version4.4.101

StatusAll

### [Release <!-- -->4.4.101](https://docs.midnight.network/relnotes/midnight-indexer/midnight-indexer-4-4-101.md)LATEST

6 November 2026

Mainnet and Preview, for the ledger 9 network upgrade and the Preview reset. Preprod stays on the 4.3.x line.

#### Artifacts

* [Chain Indexer Docker image](https://hub.docker.com/r/midnightntwrk/chain-indexer)
* [Indexer API Docker image](https://hub.docker.com/r/midnightntwrk/indexer-api)
* [Wallet Indexer Docker image](https://hub.docker.com/r/midnightntwrk/wallet-indexer)
* [Indexer Standalone Docker image](https://hub.docker.com/r/midnightntwrk/indexer-standalone)
* [SPO Indexer Docker image](https://hub.docker.com/r/midnightntwrk/spo-indexer)

#### Summary

Indexer line for the ledger 9 network upgrade on Mainnet. Requires a full re-index from genesis.

* Indexes the chain across the switch from ledger 8 to ledger 9 and follows the chain across the upgrade
* Requires a full re-index from genesis; wipe the indexer and ledger databases together
* 4.3.800 to 4.4.101 is not an upgrade path
* Serves post-upgrade state for contracts untouched since the switch (breaking)
* A contract action subscription without an offset delivers only new actions; pass an offset such as height 0 to replay history (breaking)
* Reports DUST capacity correctly after the DUST reset; required by wallet SDK 2.0.0
* Pairs with node 2.1.101 and ledger 9.1.0.1
