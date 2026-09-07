> For the complete documentation index, see [llms.txt](/llms.txt)

# Proof Server

Proof Server releases are now part of Ledger

Starting with Ledger 7.0.0, Proof Server releases are included as part of the [Ledger release](/relnotes/ledger.md). For the most recent Proof Server releases and updates, please refer to the [release notes overview](/relnotes/overview.md).

Proof Server is a component of the Midnight infrastructure that enables zero-knowledge proof generation, transaction verification, and privacy-preserving transaction processing.

[Link to related documentation](/guides/run-proof-server.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version4.0.0

StatusAll

### [Release <!-- -->4.0.0](/relnotes/proof-server/proof-server-4-0-0)UNSUPPORTED

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
