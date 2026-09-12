[Skip to main content](#content-area)

[NEAR Docs home page![light logo](https://mintcdn.com/neardocs/qO2GD-gji1aakHqN/assets/logo.svg?fit=max&auto=format&n=qO2GD-gji1aakHqN&q=85&s=fe9551b1c3085e39a97965b2886dad4a)![dark logo](https://mintcdn.com/neardocs/qO2GD-gji1aakHqN/assets/logo_rev.svg?fit=max&auto=format&n=qO2GD-gji1aakHqN&q=85&s=1455d1ccc3385495e8b52e6657ce8c84)](/)

Search docs...

⌘K

* [Wallets](https://wallet.near.org/)
* [Explorers](https://explorer.near.org/)
* [NEAR Catalog](https://nearcatalog.xyz/)

Search...

Navigation

Omni Bridge

Omni Bridge Overview

[Home](/)[Protocol](/protocol/network/architecture)[Smart Contracts](/smart-contracts/what-is)[Web3 Apps](/web3-apps/what-is)[Multi-Chain](/chain-abstraction/what-is)[Tokens & Primitives](/primitives/what-is)[Data Infra](/data-infrastructure/what-is)[RPC](/api/rpc/introduction)

[Omni Bridge](/chain-abstraction/omnibridge/overview)

Omni Bridge Overview
====================

Copy page

Learn about Omni Bridge, a multi-chain asset bridge that enables secure and efficient transfers between blockchain networks using Chain Signatures and MPC technology.

Copy page

The [Omni Bridge](https://github.com/Near-One/omni-bridge) is a multi-chain asset bridge that facilitates secure and efficient asset transfers between different blockchain networks. It solves key challenges in cross-chain communication by leveraging [Chain Signatures](/chain-abstraction/chain-signatures) and its decentralized [Multi-Party Computation (MPC) service](/chain-abstraction/chain-signatures#multi-party-computation-service) to enable trustless cross-chain asset transfers.

To learn more see [How Omni Bridge Works](/chain-abstraction/omnibridge/how-it-works).

Supported Chains
----------------

Omni Bridge launches with a hybrid architecture, utilizing different verification methods based on chain-specific requirements and technical constraints. This approach allows us to support multiple chains from day one while progressively transitioning to full Chain Signatures integration.
Currently the supported chains are:

* **Ethereum** - *(Light client + Chain Signatures)*
* **Bitcoin** - *(Light client + Chain Signatures)*
* **Zcash** - *(Light client + Chain Signatures)*
* **Solana** - *(Wormhole + Chain Signatures)*
* **Base** - *(Wormhole + Chain Signatures)*
* **BNB** - *(Wormhole + Chain Signatures)*
* **Arbitrum** - *(Wormhole + Chain Signatures)*
* **Polygon** - *(Wormhole + Chain Signatures)*

See [Omni Bridge Roadmap](/chain-abstraction/omnibridge/roadmap) for more details.

Resources
---------

* [Near-One/omni-bridge](https://github.com/Near-One/omni-bridge) - Omni Bridge repository
* [Near-One/bridge-sdk-js](https://github.com/Near-One/bridge-sdk-js) - JavaScript SDK
* [Near-One/bridge-sdk-rs](https://github.com/Near-One/bridge-sdk-rs) - Rust SDK

Was this page helpful?

YesNo

[MultiSig Voting](/chain-abstraction/chain-signatures/tutorials/multichain-dao/3-voting)[How It Works](/chain-abstraction/omnibridge/how-it-works)

[Powered byThis documentation is built and hosted on Mintlify, a developer documentation platform](https://www.mintlify.com?utm_campaign=poweredBy&utm_medium=referral&utm_source=neardocs)