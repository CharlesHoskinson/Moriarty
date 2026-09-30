[Skip to main content](#content-area)

* [System Status](https://status.near-intents.org/posts/dashboard)
* [Explorer](https://explorer.near-intents.org/)
* [Support](https://t.me/near_intents)
* [Partners](https://partners.near-intents.org/)

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Learn](/learn/omni-bridge/overview)[Changelog](/changelog/overview)

[NEAR Intents home page![light logo](https://mintcdn.com/defuselabsltd/H7BXykL_JPKw4xWb/images/logo/light.svg?fit=max&auto=format&n=H7BXykL_JPKw4xWb&q=85&s=e31b086fb76c5f6228ea4020afc8db58)![dark logo](https://mintcdn.com/defuselabsltd/H7BXykL_JPKw4xWb/images/logo/dark.svg?fit=max&auto=format&n=H7BXykL_JPKw4xWb&q=85&s=24c0664a9d5a23125fbdbafce3213d1f)](/)

Search...

⌘KAsk Assistant⌘I

* [System Status](https://status.near-intents.org/posts/dashboard)
* [Explorer](https://explorer.near-intents.org/)
* [Support](https://t.me/near_intents)
* [Partners](https://partners.near-intents.org/)
* [Partners](https://partners.near-intents.org/)

Search...

Navigation

Bridges

Token Bridges

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Learn](/learn/omni-bridge/overview)[Changelog](/changelog/overview)

Bridges

Token Bridges
=============

Copy page

Bridges that route assets between NEAR Intents and external blockchains

Copy page

![NEAR Intents Token Bridges](https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/token-bridge-intro-light.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=975d16d67e787a276ceb8ee5a4ca2665)
![NEAR Intents Token Bridges](https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/token-bridge-intro-dark.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=25d1da3d9fac6d5ab8ad36b0cb71bc46)
NEAR Intents uses multiple bridges to move assets between the Verifier contract and external blockchains. Each bridge handles a different set of chains and has its own trust model. When a [withdrawal](/integration/verifier-contract/deposits-and-withdrawals/withdrawals) is executed, the protocol selects the appropriate bridge based on the destination chain.

These bridges provide the infrastructure for moving assets between external chains and NEAR Intents.

Omni Bridge
-----------

Cross-chain transfers for major EVM chains, Solana, and Bitcoin

POA Bridge
----------

Proof of Authority bridge supporting the widest chain set

HOT Bridge
----------

HOT/Omni protocol for EVM rollups, TON, Stellar, and more

---

Omni Bridge
-----------

The Omni Bridge is designed for high-throughput transfers across the most widely used chains. It supports both EVM-compatible networks and non-EVM chains like Solana and Bitcoin.
**Supported chains:** Ethereum, Base, Arbitrum, BNB, Solana, Bitcoin
**Route ID:** `omni_bridge`
**Official documentation:** [Omni Bridge docs](https://docs.near.org/chain-abstraction/omnibridge/overview)


---

POA Bridge
----------

The POA (Proof of Authority) Bridge supports the widest range of chains in the NEAR Intents ecosystem. It uses a Proof of Authority consensus model to validate cross-chain transfers, covering UTXO-based chains (Bitcoin, Litecoin, Dogecoin, BCH, Zcash) and newer L1s (Sui, Aptos, Cardano, Starknet).
**Supported chains:** Ethereum, Base, Arbitrum, Gnosis, Berachain, Bitcoin, BCH, Litecoin, Dogecoin, Solana, XRP, Zcash, Tron, Sui, Aptos, Cardano, Starknet
**Route ID:** `poa_bridge`


---

HOT Bridge
----------

The HOT Bridge routes assets through the HOT/Omni protocol, extending NEAR Intents to chains like TON and Stellar, as well as newer EVM rollups like Scroll and Monad.
**Supported chains:** BNB, Polygon, Optimism, Avalanche, Scroll, Monad, TON, Stellar, LayerX, Adi, Plasma
**Route ID:** `hot_bridge`
**Official documentation:** [HOT Bridge docs](https://docs.hotdao.ai/omni-tokens)


---

Next steps
----------

Withdrawals
-----------

Learn how withdrawals use bridges to move assets out

Supported Chains
----------------

See the full list of supported chains and tokens

Was this page helpful?

YesNo

[Manual BTC Refunds

Recover Bitcoin you sent to a bridge deposit address that never finalized on NEAR

Next](/integration/bridging/btc-deposit-refund)

⌘I

[x](https://x.com/near_intents)[github](https://github.com/defuse-protocol)[telegram](https://t.me/near_intents)

[Powered byThis documentation is built and hosted on Mintlify, a developer documentation platform](https://www.mintlify.com?utm_campaign=poweredBy&utm_medium=referral&utm_source=defuselabsltd)