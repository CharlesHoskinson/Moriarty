<!-- source: https://docs.midnight.network/relnotes/ledger.md fetched_at: 2026-11-07T20:48:42Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Ledger

The ledger in Midnight is a public record of contract states and token states, consisting of Zswap’s cryptographic commitments and a mapping of contract addresses to their respective states.

[Link to related documentation](https://docs.midnight.network/concepts/how-midnight-works/semantics.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version9.1.0.1

StatusAll

### [Release <!-- -->9.1.0.1](https://docs.midnight.network/relnotes/ledger/ledger-9-1-0-1.md)LATEST

6 November 2026

#### Artifacts

* [Ledger](https://www.npmjs.com/package/@midnightntwrk/ledger-v9)
* [Onchain Runtime](https://www.npmjs.com/package/@midnightntwrk/onchain-runtime-v4)
* [Proof Server](https://hub.docker.com/r/midnightntwrk/proof-server)
* [Proof server GitHub release](https://github.com/midnightntwrk/midnight-ledger/releases/tag/proof-server-9.0.0)
* [GitHub release](https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0)

#### Summary

First ledger 9 release: Mainnet runs it after the ledger 9 network upgrade, with proof server 9.0.0. Builder packages carry ledger 9.1.0.0

* Mainnet moved to ledger 9 at the ledger 9 network upgrade on 6 November 2026. On the same day, Preview was reset as a new chain that runs ledger 9.1.0.1. Preprod stays on ledger 8.1.3 until a later upgrade.
* Ledger 8 transactions do not decode on ledger 9. DApps need wallet SDK 2.0.0, Midnight.js 5.0.0, and indexer 4.4.101. Indexer operators must re-index from genesis.
* Every Mainnet validator and RPC node must upgrade from node 1.0.400 (or from 2.1.100) directly to node 2.1.101 before the governance upgrade. A node on 1.0.400 stops at the upgrade block.
* Adds contract-to-contract calls and event emission (first phase, unshielded data only), ECDSA signature verification, native Keccak hashing, native secp256k1, and ZKIR 3.2.0. Compile with Compact toolchain 0.36.0 (language version 0.28.0, Compact runtime 0.21.0).
* The tested upgrade rehearsal checks that a smart contract deployed before the upgrade can still be called. In an earlier devnet test, artifacts from toolchain 0.31.1 kept working with the ZK config provider set to `verify: 'require-if-present'`, and Midnight.js 5.0.0 refuses artifacts from other toolchains, such as 0.30.0, with `EraMismatchError`.
* DUST state resets once at the upgrade and NIGHT balances do not change. On Mainnet, every registration is replayed automatically to its full cap, so Mainnet users do not need to do anything. Transactions submitted during the replay (about 48 blocks) wait in the transaction pool and are included once it ends.
* A fee floor, `min_block_price`, applies from the first block after the upgrade, so fees rise slightly. Update fee estimation before the upgrade.
* Contract action subscriptions opened without an offset now deliver only new actions. Pass an explicit offset, such as height `0`, to replay history. Midnight.js 5.0.0 does this.
* Proof server 9.0.0 is the proof server for ledger 9. Pin it by image digest. Its default job capacity is now 10 instead of 0 (no limit).
* npm packages: `@midnightntwrk/ledger-v9` 1.0.0 and `@midnightntwrk/onchain-runtime-v4` 4.0.0. The on-chain runtime major version is in the package name.

## Component versions by release[​](#component-versions-by-release "Direct link to Component versions by release")

A Ledger release includes the on-chain runtime, ZKIR (the zero-knowledge intermediate representation), and the proof server. Each has its own version number. To choose versions for a DApp, see the [compatibility matrix](https://docs.midnight.network/relnotes/support-matrix.md).

Mainnet runs ledger 9 after the ledger 9 network upgrade. Preview was reset as a new ledger 9 chain on 6 November 2026. Preprod runs ledger 8.1.3 until a later upgrade. To prepare for the change, see the [ledger 9 upgrade page](https://docs.midnight.network/relnotes/ledger-9.md).

The node embeds ledger 9.1.0.1. The packages that DApp and wallet builders use carry ledger 9.1.0.0.

| Ledger  | On-chain runtime | ZKIR  | Proof server    |
| ------- | ---------------- | ----- | --------------- |
| 9.1.0.1 | 4.0.0            | 3.2.0 | 9.0.0           |
| 8.1.3   | 3.1.2            | 2.1.1 | 8.1.3           |
| 8.1.2   | 3.1.1            | 2.1.1 | None. Use 8.1.3 |
| 8.1.1   | 3.1.0            | 2.1.0 | None. Use 8.1.3 |
| 8.1.0   | 3.1.0            | 2.1.0 | 8.1.0           |
| 8.0.3   | 3.0.0            | 2.1.0 | 8.0.3           |
| 8.0.2   | 3.0.0            | 2.1.0 | 8.0.2           |
| 7.0.0   | 2.0.0            | 2.1.0 | 7.0.0           |
