[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Midnight Logo](/img/midnight-header-logo-dark.svg)](/)

[API reference](/api-reference)[SDKs](/sdks)[Compact](/compact)[Dev Diaries](/blog)

AI search

Search

* [Overview](/)
* [What is Midnight?](/what-is-midnight)
* [Getting started](/getting-started)
* [Examples](/examples)
* [Tutorials](/tutorials)
* [Concepts](/concepts)
* [Guides](/category/guides)
* [Tokens](/tokens)
* [AI integration](/ai-integration)
* [Compact](/compact)
* [Nodes](/nodes)
* [Release notes](/relnotes/overview)

  + [Compatibility matrix](/relnotes/support-matrix)
  + [Environments and endpoints](/relnotes/network)
  + [Compact.js](/relnotes/compact-js)
  + [Compact developer tools](/relnotes/compact-tools)
  + [Compact compiler](/relnotes/compact)
  + [DApp Connector API](/relnotes/dapp-connector-api)
  + [Ledger](/relnotes/ledger)
  + [Midnight Indexer](/relnotes/midnight-indexer)
  + [Midnight.js](/relnotes/midnight-js)
  + [Wallet API](/relnotes/midnight-wallet-api)
  + [Node](/relnotes/node)
  + [Onchain Runtime](/relnotes/onchain-runtime)
  + [Proof Server](/relnotes/proof-server)
  + [Wallet SDK](/relnotes/wallet)
* [Troubleshoot](/category/troubleshoot)
* [Glossary](/glossary)

* [Release notes](/relnotes/overview)
* Proof Server

Version: v1

Explore with… ▾

> For the complete documentation index, see [llms.txt](/llms.txt)

Proof Server
============

Proof Server releases are now part of Ledger

Starting with Ledger 7.0.0, Proof Server releases are included as part of the [Ledger release](/relnotes/ledger). For the most recent Proof Server releases and updates, please refer to the [release notes overview](/relnotes/overview).

Proof Server is a component of the Midnight infrastructure that enables zero-knowledge proof generation, transaction verification, and privacy-preserving transaction processing.

[Link to related documentation](/guides/run-proof-server)

To download the component, click the appropriate link under **Artifacts**.

---

VersionAll4.0.03.0.73.0.6

StatusAllUNSUPPORTEDDEPRECATED

### [Release 4.0.0](/relnotes/proof-server/proof-server-4-0-0)

#### Artifacts

* [Proof Server](https://hub.docker.com/r/midnightnetwork/proof-server/)

#### Summary

* Integrated with the new storage model, making required objects `Storable` to allow storing MPT leafs as `Sp`s.
* Added segment IDs to Zswap constructors. These should be set to `1` for fallible offers, and `0` for guaranteed offers.
* Renamed `ZswapLocalStateNoKeys` to `ZswapLocalState`, removing the existing (with keys) state.
* Switch from Pluto-Eris to BLS12-381.
* Switched to using data providers instead of direct prover keys and parameters.
* Add a data provider to fetch key material for Midnight. The source of this may be overridden with the `MIDNIGHT_PARAM_SOURCE` environment variable.

[Edit this page](https://github.com/midnightntwrk/midnight-docs/edit/main/docs/relnotes/proof-server.mdx)

Last updated on **Sep 10, 2026**

[Previous

Onchain Runtime](/relnotes/onchain-runtime)[Next

Wallet SDK](/relnotes/wallet)

![Midnight Logo](/img/midnight-header-logo-dark.svg)![Midnight Logo](/img/midnight-header-logo-dark.svg)

© 2026 Input Output Global, Inc. All Rights Reserved.

Resources

* [Midnight Foundation](https://midnight.network/)
* [Dev Diaries](/blog)
* [Glacier Drop](https://www.midnight.gd/)
* [Careers](https://midnight.network/careers)

Legal

* [Cookie Policy](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20cookie-policy.pdf)
* [Privacy Policy](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20%20-%20Privacy%20Notice.pdf)
* [Terms and Conditions](https://45047878.fs1.hubspotusercontent-na1.net/hubfs/45047878/Midnight%20Foundation%20-%20Website%20Terms%20of%20Use.pdf)

Social

* [![YouTube](/img/youtube.svg)](https://www.youtube.com/channel/UCy3oZ64F3FOtjZ5sZGQNgkA)
* [![X/Twitter](/img/x.svg)](https://x.com/MidnightNtwrk)
* [![Discord](/img/discord.svg)](https://discord.com/invite/midnightnetwork)
* [![LinkedIn](/img/linkedin.svg)](https://www.linkedin.com/showcase/midnight-ntwrk/)