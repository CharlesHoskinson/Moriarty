> For the complete documentation index, see [llms.txt](/llms.txt)

# Ledger

The ledger in Midnight is a public record of contract states and token states, consisting of Zswap’s cryptographic commitments and a mapping of contract addresses to their respective states.

[Link to related documentation](/concepts/how-midnight-works/semantics.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version8.1.1

StatusAll

### [Release <!-- -->8.1.1](/relnotes/ledger/ledger-8-1-1)LATEST

31 July 2026

#### Artifacts

* [Ledger](https://www.npmjs.com/package/@midnightntwrk/ledger-v8)
* [GitHub release](https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-8.1.1)

#### Summary

* No protocol, serialization, or API behavior changes for well-formed transactions.
* Added test coverage for array handling.
* Clippy 1.97 lint cleanups across the workspace.
* npm packages now publish under the `@midnightntwrk` scope; update dependencies such as `@midnight-ntwrk/ledger-v8` to `@midnightntwrk/ledger-v8`.
