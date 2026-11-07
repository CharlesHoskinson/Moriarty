<!-- source: https://docs.midnight.network/relnotes/onchain-runtime/onchain-runtime-4-0-0.md fetched_at: 2026-11-07T20:48:44Z status: 200 untrusted -->

> For the complete documentation index, see [llms.txt](https://docs.midnight.network/llms.txt)

# Onchain Runtime

Onchain Runtime is a component of the Midnight infrastructure that enables zero-knowledge proof generation, transaction verification, and privacy-preserving transaction processing.

To learn more, see the [Onchain Runtime Tutorial](https://docs.midnight.network/api-reference/onchain-runtime.md)

***

12 May 2025

## Midnight 4.0.0 Onchain Runtime Release Notes[​](#midnight-400-onchain-runtime-release-notes "Direct link to Midnight 4.0.0 Onchain Runtime Release Notes")

Learn about the latest updates and improvements to the Midnight Onchain Runtime, including new features, bug fixes, and breaking changes designed to enhance functionality and stability.

### Stable Release[​](#stable-release "Direct link to Stable Release")

* **Version**: 4.0.0
* **Audience**: Developers integrating with the Midnight ecosystem
* **Environment**: Production-ready

This release (version `4.0.0`) is considered stable and production-ready. It can safely be deployed in public and production environments.

### Changelog[​](#changelog "Direct link to Changelog")

#### Breaking[​](#breaking "Direct link to Breaking")

* Integrated with the new storage model, making required objects `Storable` to allow storing MPT leafs as `Sp`s.
* Added segment IDs to Zswap constructors. These should be set to `1` for fallible offers, and `0` for guaranteed offers.
* Renamed `ZswapLocalStateNoKeys` to `ZswapLocalState`, removing the existing (with keys) state.
* Switch from Pluto-Eris to BLS12-381.
* Switched to using data providers instead of direct prover keys and parameters.

#### Features[​](#features "Direct link to Features")

* Add a data provider to fetch key material for Midnight. The source of this may be overridden with the `MIDNIGHT_PARAM_SOURCE` environment variable.

### Artifacts[​](#artifacts "Direct link to Artifacts")

* [Onchain Runtime](https://www.npmjs.com/package/@midnight-ntwrk/onchain-runtime)
