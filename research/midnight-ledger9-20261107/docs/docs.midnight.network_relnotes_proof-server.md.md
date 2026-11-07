<!-- source: https://docs.midnight.network/relnotes/proof-server.md fetched_at: 2026-11-07T20:48:44Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Proof Server

Proof server releases are now part of Ledger

Starting with Ledger 7.0.0, proof server releases appear on the [Ledger page](https://docs.midnight.network/relnotes/ledger.md), not on this page. Images for 7.0.0 and later are on Docker Hub as [`midnightntwrk/proof-server`](https://hub.docker.com/r/midnightntwrk/proof-server). For the version to run on each network, see the [compatibility matrix](https://docs.midnight.network/relnotes/support-matrix.md).

The releases on this page predate that change. Their images, up to 4.0.0, are in the older `midnightnetwork/proof-server` repository.

Proof Server is a component of the Midnight infrastructure that enables zero-knowledge proof generation, transaction verification, and privacy-preserving transaction processing.

[Link to related documentation](https://docs.midnight.network/guides/run-proof-server.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version4.0.0

StatusAll

### [Release <!-- -->4.0.0](https://docs.midnight.network/relnotes/proof-server/proof-server-4-0-0.md)UNSUPPORTED

12 May 2025

#### Artifacts

* [Proof Server](https://hub.docker.com/r/midnightnetwork/proof-server/)

#### Summary

* Integrated with the new storage model, making required objects `Storable` to allow storing MPT leafs as `Sp`s.
* Added segment IDs to Zswap constructors. These should be set to `1` for fallible offers, and `0` for guaranteed offers.
* Renamed `ZswapLocalStateNoKeys` to `ZswapLocalState`, removing the existing (with keys) state.
* Switch from Pluto-Eris to BLS12-381.
* Switched to using data providers instead of direct prover keys and parameters.
* Add a data provider to fetch key material for Midnight. The source of this may be overridden with the `MIDNIGHT_PARAM_SOURCE` environment variable.
