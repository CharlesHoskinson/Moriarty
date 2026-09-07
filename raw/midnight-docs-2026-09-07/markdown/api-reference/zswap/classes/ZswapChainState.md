# ZswapChainState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / ZswapChainState

# Class: ZswapChainState

The on-chain state of Zswap, consisting of a Merkle tree of coin commitments, a set of nullifiers, an index into the Merkle tree, and a set of valid past Merkle tree roots

## Constructors[​](#constructors "Direct link to Constructors")

### new ZswapChainState()[​](#new-zswapchainstate "Direct link to new ZswapChainState()")

```
new ZswapChainState(): ZswapChainState
```

#### Returns[​](#returns "Direct link to Returns")

[`ZswapChainState`](/api-reference/zswap/classes/ZswapChainState.md)

## Properties[​](#properties "Direct link to Properties")

### firstFree[​](#firstfree "Direct link to firstFree")

```
readonly firstFree: bigint;
```

The first free index in the coin commitment tree

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### tryApply()[​](#tryapply "Direct link to tryApply()")

```
tryApply(offer, whitelist?): [ZswapChainState, Map<string, bigint>]
```

Try to apply an [Offer](/api-reference/zswap/classes/Offer.md) to the state, returning the updated state and a map on newly inserted coin commitments to their inserted indices.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### offer[​](#offer "Direct link to offer")

[`Offer`](/api-reference/zswap/classes/Offer.md)

##### whitelist?[​](#whitelist "Direct link to whitelist?")

`Set`<`string`>

A set of contract addresses that are of interest. If set, *only* these addresses are tracked, and all other information is discarded.

#### Returns[​](#returns-3 "Direct link to Returns")

\[[`ZswapChainState`](/api-reference/zswap/classes/ZswapChainState.md), `Map`<`string`, `bigint`>]

***

### tryApplyProofErased()[​](#tryapplyprooferased "Direct link to tryApplyProofErased()")

```
tryApplyProofErased(offer, whitelist?): [ZswapChainState, Map<string, bigint>]
```

[tryApply](/api-reference/zswap/classes/ZswapChainState.md#tryapply) for [ProofErasedOffer](/api-reference/zswap/classes/ProofErasedOffer.md)s

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### offer[​](#offer-1 "Direct link to offer")

[`ProofErasedOffer`](/api-reference/zswap/classes/ProofErasedOffer.md)

##### whitelist?[​](#whitelist-1 "Direct link to whitelist?")

`Set`<`string`>

#### Returns[​](#returns-4 "Direct link to Returns")

\[[`ZswapChainState`](/api-reference/zswap/classes/ZswapChainState.md), `Map`<`string`, `bigint`>]

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): ZswapChainState
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-5 "Direct link to Returns")

[`ZswapChainState`](/api-reference/zswap/classes/ZswapChainState.md)

***

### deserializeFromLedgerState()[​](#deserializefromledgerstate "Direct link to deserializeFromLedgerState()")

```
static deserializeFromLedgerState(raw, netid): ZswapChainState
```

Given a whole ledger serialized state, deserialize only the Zswap portion

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### raw[​](#raw-1 "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-2 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-6 "Direct link to Returns")

[`ZswapChainState`](/api-reference/zswap/classes/ZswapChainState.md)
