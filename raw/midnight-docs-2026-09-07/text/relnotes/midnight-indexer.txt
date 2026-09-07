> For the complete documentation index, see [llms.txt](/llms.txt)

# Midnight Indexer

The Midnight Indexer is a set of components designed to optimize the flow of blockchain data from a Midnight node to end-user applications. It retrieves history of blocks, processes them, stores indexed data efficiently, and provides a GraphQL API for queries and subscriptions.

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version4.3.3

StatusAll

### [Release <!-- -->4.3.3](/relnotes/midnight-indexer/midnight-indexer-4-3-3)LATEST

4 June 2026

#### Artifacts

* [Chain Indexer Docker image](https://hub.docker.com/r/midnightntwrk/chain-indexer)
* [Indexer API Docker image](https://hub.docker.com/r/midnightntwrk/indexer-api)
* [Wallet Indexer Docker image](https://hub.docker.com/r/midnightntwrk/wallet-indexer)
* [Indexer Standalone Docker image](https://hub.docker.com/r/midnightntwrk/indexer-standalone)
* [SPO Indexer Docker image](https://hub.docker.com/r/midnightntwrk/spo-indexer)

#### Summary

* Added per-connection and per-client subscription quotas to GraphQL WebSocket
* Introduced `@beta` GraphQL directive for in-flight API fields
* Added per-tree end indexes (`zswapEndIndex`, `dustCommitmentEndIndex`, `dustGenerationEndIndex`) to `Block`
* Added lazy `transaction` reference on nullifier event types
* Tightened shielded nullifier transactions input validation
* Renamed dust nullifier byte fields with `LeBytes` suffix (breaking, `@beta` only)
* Fixed `dustGenerations` subscription to deliver dtime updates on fresh subscriptions
