> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Token Bridges

> Bridges that route assets between NEAR Intents and external blockchains

<img className="block dark:hidden" src="https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/token-bridge-intro-light.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=975d16d67e787a276ceb8ee5a4ca2665" alt="NEAR Intents Token Bridges" width="1654" height="1524" data-path="images/diagrams/token-bridge-intro-light.png" />

<img className="hidden dark:block" src="https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/token-bridge-intro-dark.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=25d1da3d9fac6d5ab8ad36b0cb71bc46" alt="NEAR Intents Token Bridges" width="1654" height="1532" data-path="images/diagrams/token-bridge-intro-dark.png" />

NEAR Intents uses multiple bridges to move assets between the Verifier contract and external blockchains. Each bridge handles a different set of chains and has its own trust model. When a [withdrawal](/integration/verifier-contract/deposits-and-withdrawals/withdrawals) is executed, the protocol selects the appropriate bridge based on the destination chain.

<Tip>
  These bridges provide the infrastructure for moving assets between external chains and NEAR Intents.
</Tip>

<CardGroup cols={3}>
  <Card title="Omni Bridge" icon="bridge" href="#omni-bridge">
    Cross-chain transfers for major EVM chains, Solana, and Bitcoin
  </Card>

  <Card title="POA Bridge" icon="shield-halved" href="#poa-bridge">
    Proof of Authority bridge supporting the widest chain set
  </Card>

  <Card title="HOT Bridge" icon="fire" href="#hot-bridge">
    HOT/Omni protocol for EVM rollups, TON, Stellar, and more
  </Card>
</CardGroup>

***

## Omni Bridge

The Omni Bridge is designed for high-throughput transfers across the most widely used chains. It supports both EVM-compatible networks and non-EVM chains like Solana and Bitcoin.

**Supported chains:** Ethereum, Base, Arbitrum, BNB, Solana, Bitcoin

**Route ID:** `omni_bridge`

**Official documentation:** [Omni Bridge docs](https://docs.near.org/chain-abstraction/omnibridge/overview)

***

## POA Bridge

The POA (Proof of Authority) Bridge supports the widest range of chains in the NEAR Intents ecosystem. It uses a Proof of Authority consensus model to validate cross-chain transfers, covering UTXO-based chains (Bitcoin, Litecoin, Dogecoin, BCH, Zcash) and newer L1s (Sui, Aptos, Cardano, Starknet).

**Supported chains:** Ethereum, Base, Arbitrum, Gnosis, Berachain, Bitcoin, BCH, Litecoin, Dogecoin, Solana, XRP, Zcash, Tron, Sui, Aptos, Cardano, Starknet

**Route ID:** `poa_bridge`

***

## HOT Bridge

The HOT Bridge routes assets through the HOT/Omni protocol, extending NEAR Intents to chains like TON and Stellar, as well as newer EVM rollups like Scroll and Monad.

**Supported chains:** BNB, Polygon, Optimism, Avalanche, Scroll, Monad, TON, Stellar, LayerX, Adi, Plasma

**Route ID:** `hot_bridge`

**Official documentation:** [HOT Bridge docs](https://docs.hotdao.ai/omni-tokens)

***

## Next steps

<CardGroup cols={3}>
  <Card title="Withdrawals" icon="arrow-right-from-bracket" href="/integration/verifier-contract/deposits-and-withdrawals/withdrawals">
    Learn how withdrawals use bridges to move assets out
  </Card>

  <Card title="Supported Chains" icon="check-circle" href="/resources/chain-support">
    See the full list of supported chains and tokens
  </Card>
</CardGroup>
