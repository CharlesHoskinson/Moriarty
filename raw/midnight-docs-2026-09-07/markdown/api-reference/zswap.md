> For the complete documentation index, see [llms.txt](/llms.txt)

# ZSwap API

**@midnight/zswap v4.0.0-rc**

***

# Zswap TypeScript API

This document outlines the usage of the Zswap TS API

## Network ID[​](#network-id "Direct link to Network ID")

Prior to any interaction, setNetworkId should be used to set the [NetworkId](/api-reference/zswap/enumerations/NetworkId.md) to target the correct network.

## Proof stages[​](#proof-stages "Direct link to Proof stages")

Most transaction components will be in one of three stages: `X`, `UnprovenX`, or `ProofErasedX`. The `UnprovenX` stage is *always* the first one. It is possible to transition to the `X` stage by proving an `UnprovenTransaction` through the proof server. For testing, and where proofs aren't necessary, the `ProofErasedX` stage is used, which can be reached via `eraseProof[s]` from the other two stages.

## Transaction structure[​](#transaction-structure "Direct link to Transaction structure")

A [Transaction](/api-reference/zswap/classes/Transaction.md) runs in two phases: a *guaranteed* phase, handling fee payments and fast-to-verify operations, and a *fallible* phase, handling operations which may fail atomically, separately from the guaranteed phase. It therefore contains:

* A "guaranteed" [Offer](/api-reference/zswap/classes/Offer.md)
* Optionally, a "fallible" [Offer](/api-reference/zswap/classes/Offer.md)
* Contract call information not accessible to this API

It also contains additional cryptographic glue that will be omitted in this document.

### Zswap[​](#zswap "Direct link to Zswap")

A Zswap [Offer](/api-reference/zswap/classes/Offer.md) consists of:

* A set of [Input](/api-reference/zswap/classes/Input.md)s, burning coins.
* A set of [Output](/api-reference/zswap/classes/Output.md)s, creating coins.
* A set of [Transient](/api-reference/zswap/classes/Transient.md)s, indicating a coin that is created and burnt in the same transaction.
* A mapping from [TokenType](/api-reference/zswap/type-aliases/TokenType.md)s to offer balance, positive when there are more inputs than outputs and vice versa.

[Input](/api-reference/zswap/classes/Input.md)s can be created either from a [QualifiedCoinInfo](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md) and a contract address, if the coin is contract-owned, or from a [QualifiedCoinInfo](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md) and a ZswapLocalState, if it is user-owned. Similarly, [Output](/api-reference/zswap/classes/Output.md)s can be created from a [CoinInfo](/api-reference/zswap/type-aliases/CoinInfo.md) and a contract address for contract-owned coins, or from a [CoinInfo](/api-reference/zswap/type-aliases/CoinInfo.md) and a user's public key(s), if it is user-owned. A [Transient](/api-reference/zswap/classes/Transient.md) is created similarly to a [Input](/api-reference/zswap/classes/Input.md), but directly converts an existing [Output](/api-reference/zswap/classes/Output.md).

A [QualifiedCoinInfo](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md) is a [CoinInfo](/api-reference/zswap/type-aliases/CoinInfo.md) with an index into the Merkle tree of coin commitments that can be used to find the relevant coin to spend, while a [CoinInfo](/api-reference/zswap/type-aliases/CoinInfo.md) consists of a coins [TokenType](/api-reference/zswap/type-aliases/TokenType.md), value, and a nonce.

## State Structure[​](#state-structure "Direct link to State Structure")

[ZswapChainState](/api-reference/zswap/classes/ZswapChainState.md) holds the on-chain state of Zswap, while ZswaplocalState contains the local, wallet state.
