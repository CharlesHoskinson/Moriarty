<!-- source: https://docs.midnight.network/relnotes/compact.md fetched_at: 2026-11-07T20:48:45Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Compact toolchain

Compact is Midnight's smart contract programming language, designed for building secure, efficient, and adaptable decentralized applications.

The Compact toolchain includes the Compact compiler, formatter, and the Compact "fixup" tool. The compiler translates Compact smart contract code into JavaScript implementations and a representation of the zero-knowledge circuits.

[Link to related documentation](https://docs.midnight.network/compact/compilation-and-tooling/compiler-usage.md)

To install the toolchain, first install the [Compact devtools](https://docs.midnight.network/relnotes/compact-tools.md). Then install the toolchain for the network you deploy to, as listed in the [compatibility matrix](https://docs.midnight.network/relnotes/support-matrix.md):

* **Mainnet and Preview** run ledger 9 and use toolchain 0.36.0. Mainnet moved to ledger 9 in the [ledger 9 network upgrade](https://docs.midnight.network/relnotes/ledger-9.md), and Preview was reset as a new ledger 9 chain on 6 November 2026. Run `compact update 0.36` to get it.
* **Preprod** runs ledger 8 until its upgrade. Run `compact update 0.31` to get toolchain 0.31.x for Preprod.

A bare `compact update` installs the newest release, which can target a ledger version that your network does not run yet.

***

<!-- -->

Version0.36.0

StatusAll

### [Release <!-- -->0.36.0](https://docs.midnight.network/relnotes/compact/toolchain-0.36.0.md)LATEST

#### Compact language <!-- -->0.28.0

4 November 2026

#### Artifacts

* [GitHub release](https://github.com/LFDT-Minokawa/compact/releases/tag/compactc-v0.36.0)

#### Summary

Mainnet and Preview toolchain for ledger 9: ZKIR version 3 is the only backend, `--feature-zkir-v3` is removed, and circuits in a contract type cannot be `pure`; Preprod uses 0.31.x until its upgrade

* Targets ledger 9, which runs on Mainnet after the ledger 9 network upgrade and on Preview. Preprod still runs ledger 8, so use `compact update 0.31` for Preprod until its upgrade
* Language version 0.28.0 and Compact runtime 0.21.0; compiled contracts require `@midnight-ntwrk/compact-runtime` 0.21.x
* The compiler always generates ZKIR version 3; passing `--feature-zkir-v3` is now an error (breaking)
* Circuits declared in a contract type cannot be `pure`, and the arguments of every cross-contract call must be disclosed (breaking)
* Purity is removed from the Compact runtime cross-contract API (breaking)
* Recompiling a contract whose verifier keys are ZKIR version 2 changes its keys; give the deployed contract the new keys through its maintenance authority, or redeploy it (breaking)
* Contracts that make cross-contract calls must be recompiled and their keys replaced, and callers and callees upgraded together (breaking)
* Fixes several compiler internal errors and Compact runtime defects
