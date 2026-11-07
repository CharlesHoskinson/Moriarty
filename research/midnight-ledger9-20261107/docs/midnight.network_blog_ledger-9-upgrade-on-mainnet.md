<!-- source: https://midnight.network/blog/ledger-9-upgrade-on-mainnet fetched_at: 2026-11-07T20:48:48Z status: 200 untrusted -->

November 06 2026

# Ledger 9 upgrade unlocks signature support and composable contracts on mainnet

![Avatar of the content author](/_next/image?url=https%3A%2F%2Fcdn.sanity.io%2Fimages%2F330xhmya%2Fproduction%2Ff45b2fc73c3a8a5b34e9c23d1b875d47c63c77ca-512x512.png&w=128&q=75)

Midnight

Network UpdatesAnnouncements

![](/_next/image?url=https%3A%2F%2Fcdn.sanity.io%2Fimages%2F330xhmya%2Fproduction%2Fa9fca65454cf725e87cbbe4cb7b9085e14a752d2-1000x1000.png&w=640&q=75)

Sign up now so you never miss an update

subscribe

contributors

![Avatar of the content author](/_next/image?url=https%3A%2F%2Fcdn.sanity.io%2Fimages%2F330xhmya%2Fproduction%2Ff45b2fc73c3a8a5b34e9c23d1b875d47c63c77ca-512x512.png&w=128&q=75)

Midnight

The Midnight network has just executed a hard fork, upgrading mainnet from Ledger 8 to Ledger 9 on mainnet. This upgrade delivers expanded functionality to the network as well as security improvements.

During the update to Ledger 9, existing balances and deployed smart contracts migrate automatically. Existing smart contracts will carry over unchanged, but builders will need to update their apps to [Midnight.js 5](https://github.com/midnightntwrk/midnight-js/releases). For contracts not yet deployed, developers will need to compile them using the updated Compact toolchain targeting Ledger 9. For mainnet holders of cNIGHT, your DUST generation registration will happen automatically. No action is required on your part.

The engineering team is accelerating feature deployments to give developers access to these capabilities faster. Because of this accelerated pace, technical documentation for some of the newest features is still in progress and will roll out over the coming weeks. Keep an eye on the [Midnight docs for all the latest release information](https://docs.midnight.network/).

The latest updates introduce standard cryptographic primitives, contract-to-contract execution, and a bundle of performance improvements. Together, these improvements establish the necessary infrastructure for connections with other systems, application composability, and more advanced privacy functionality.

## Prime stablecoin integration and custodian signature support

Moving stablecoins between blockchain networks usually forces you to expose your financial history to the public. When you move stablecoins onto the Midnight network, you may want to do so in a way that keeps your transaction data private. To make this work safely, the network must use the exact cryptographic standards used across the broader blockchain ecosystem. The [Compact programming language](https://docs.midnight.network/compact/reference/compact-reference) now includes these native formulas. Practically, this means developers can build tools that let you move money from other blockchains onto Midnight privately.

This update introduces native support for specific signature formats called Keccak-256 and ECDSA. Keccak-256 is the standard algorithm that Ethereum uses to scramble data into fixed-length cryptographic fingerprints. ECDSA is the standard digital signature method that networks like Bitcoin and Ethereum use to prove you own a specific wallet address. Because the Midnight network processes these exact functions natively, you can deposit assets from other chains directly. The network verifies your transaction mathematically, so you never have to trust a centralized middleman to confirm your transfer.

The ledger layer also accepts both Schnorr and ECDSA signatures to authorize transactions. Accepting ECDSA directly allows the largest institutional custody providers to connect to the network.

This release also implements the BIP-44 address derivation standard. BIP-44 operates like a master key system, where a single cryptographic seed phrase generates and manages thousands of secure addresses. Custodians and wallet providers use this exact standard across the industry. Midnight network support for this standard means enterprise partners can use their existing software to create Midnight wallets. That makes it straightforward for financial institutions to mint, burn, and transfer assets immediately without buying new security hardware.

## **Contract-to-contract execution and live event tracking**

Decentralized applications rarely function in isolation. More advanced operations require composability, which is the ability for different applications to plug into one another like interlocking building blocks.

This release introduces true composability, allowing developers to build in pieces that fit together seamlessly.

* **Cross-contract calls:** One contract can now ask another contract to execute a function as part of the exact same transaction. This creates a vastly superior user experience. For example, instead of forcing a user to manually sign three separate transactions to approve a token, swap it, and deposit it into a liquidity pool, developers can combine these steps into a single user action.
* **Knowing the caller:** A contract can now ask which contract called it, and use that answer to authorize specific actions. A secure vault can now enforce rules like, "Only the approved lending contract may withdraw funds from me."
* **Events and logs:** Smart contracts can now publish short public notices, such as "payment received." When your transaction finalizes, the contract broadcasts this event. Wallets, explorers, and tracking programs listen for these events and send the organized data directly to front-end user interfaces, refreshing your screen balances in real time.

Without cross-contract calls, every team has to rebuild the exact same basic infrastructure inside its own contract. That is slower, harder to audit, and harder to upgrade. With this release, developers can create shared building blocks. A token standard, an identity check, or an escrow system can be written once, thoroughly audited, and reused by many applications.

Most importantly, this phase lays the groundwork for improvements to private DeFi. This will open up a range of applications that will be able to protect confidentiality while remaining regulatory compliant to deliver functionality that is not available on other chains.

## **Infrastructure packaging, documentation, and node operations**

There are numerous performance improvements bundled into this release with reliability upgrades, and security maintenance patches. Together all of these improvements make Midnight safer to upgrade, easier to build on, and more secure.

Most notably, all zero-knowledge proofs and their matching verification keys now move to a new version: the Zero-Knowledge Intermediate Representation (ZKIR V3). This upgrades the compiler to support the new signature formats and acts as the essential underlying packaging that enables the network's new cryptographic features.

To support these accelerated feature deployments, the engineering team is finalizing technical documentation directly in the code repository over the coming weeks. Once complete, the documentation will detail exact opcode references and transaction format specifications.

## **What this unlocks for the ecosystem**

The transition to Ledger 9 represents a significant improvement in the capabilities of the Midnight network.

By combining native ECDSA signatures, enterprise custody support, and smart contract composability, Ledger 9 unlocks:

* **Institutional custody and capital:** Direct compatibility with ECDSA and BIP-44 derivation lets regulated custodians like BitGo integrate Midnight into their existing hardware security systems, paving the way for institutional asset management.
* **Contracts that work together for modular DeFi:** By connecting modular contracts builders can assemble sophisticated decentralized applications that streamline multi-step transactions into a single click.
* **A foundation for advanced private finance:** By handling external cryptographic standards and multi-contract flows natively, Ledger 9 creates the structural runway for private cross-chain stablecoins and end-to-end shielded DeFi workflows.

These upgrades give builders the exact tools they need to create applications that are as easy to use as traditional web apps, as interconnected as standard DeFi, with additional security that benefits the overall ecosystem.

## Notes on the release process

Unlike previous upgrades, Ledger 9 has been deployed directly to mainnet, bypassing the standard Preprod deployment and soak period. This approach is used when security fixes require a more controlled release path to protect the network. Learn more in the release notes.

While mainnet cNIGHT DUST registrations replay automatically, builders on Preview and Preprod will need to re-register their tNIGHT on those networks to resume DUST generation.

Network UpdatesAnnouncements

## Share

[twitter](https://twitter.com/intent/tweet?url=https%3A%2F%2Fmidnight.network%2Fblog%2Fledger-9-upgrade-on-mainnet&text=Ledger+9+upgrade+unlocks+signature+support+and+composable+contracts+on+mainnet&via=MidnightNtwrk "Share on Twitter")[linkedin](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fmidnight.network%2Fblog%2Fledger-9-upgrade-on-mainnet "Share on LinkedIn")Copy link

[Previous PostHow shielded assets and privacy elevate on-chain security](/blog/how-shielded-assets-and-privacy-elevate-on-chain-security "Previous blog post")