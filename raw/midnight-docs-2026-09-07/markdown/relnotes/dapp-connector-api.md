> For the complete documentation index, see [llms.txt](/llms.txt)

# DApp Connector API

DApp Connector API allows decentralized applications (DApps) to request wallet access, verify authorization, and retrieve service URIs for interacting with the Midnight network.

[Link to related documentation](/api-reference/dapp-connector.md)

To download the component, click the appropriate link under **Artifacts**.

***

<!-- -->

Version4.0.1

StatusAll

### [Release <!-- -->4.0.1](/relnotes/dapp-connector-api/dapp-connector-api-4-0-1)LATEST

17 February 2026

#### Artifacts

* [NPM package](https://www.npmjs.com/package/@midnight-ntwrk/dapp-connector-api)
* [GitHub release](https://github.com/midnightntwrk/midnight-dapp-connector-api/releases/tag/v4.0.1)

#### Summary

* All transacting methods now accept the `payFees` option that was previously only available on `makeIntent`.
* Disabling fee payment supports sponsored-fee flows where a separate wallet pays transaction fees.
* Backwards-compatible: omitting the option keeps the existing fee payment behavior.
* The motivation for the option is documented in the API specification.
