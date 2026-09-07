> For the complete documentation index, see [llms.txt](/llms.txt)

# Midnight.js

Midnight.js is a client library designed to facilitate the development of decentralized applications on the Midnight blockchain.

[Link to related documentation](/api-reference/midnight-js/packages.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version4.1.1

StatusAll

### [Release <!-- -->4.1.1](/relnotes/midnight-js/midnight-js-4-1-1)LATEST

2 June 2026

#### Artifacts

* [NPM Package](https://www.npmjs.com/search?q=midnight-ntwrk)

#### Summary

* Renamed `IndexerFormattedError.cause` to `.errors` for ES2022 compatibility (breaking)
* Applied full password policy to signing key and private state export operations
* Emitted contract state for `blockHeight`/`blockHash` configurations
* Hardened error handling in `indexer-public-data-provider`
* Added signing key validation on import
* Warned on plain HTTP/WS for non-loopback provider URLs
* Added qanet support via NIGHT/DUST faucet flow in `testkit-js`
