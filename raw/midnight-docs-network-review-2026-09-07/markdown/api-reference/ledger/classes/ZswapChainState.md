# ZswapChainState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ZswapChainState

# Class: ZswapChainState

The on-chain state of Zswap, consisting of a Merkle tree of coin commitments, a set of nullifiers, an index into the Merkle tree, and a set of valid past Merkle tree roots

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ZswapChainState(): ZswapChainState;
```

#### Returns[​](#returns "Direct link to Returns")

`ZswapChainState`

## Properties[​](#properties "Direct link to Properties")

### firstFree[​](#firstfree "Direct link to firstFree")

```
readonly firstFree: bigint;
```

The first free index in the coin commitment tree

## Methods[​](#methods "Direct link to Methods")

### filter()[​](#filter "Direct link to filter()")

```
filter(contractAddress): ZswapChainState;
```

Filters the state to only include coins that are relevant to a given contract address.

#### Parameters[​](#parameters "Direct link to Parameters")

##### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

#### Returns[​](#returns-1 "Direct link to Returns")

`ZswapChainState`

***

### postBlockUpdate()[​](#postblockupdate "Direct link to postBlockUpdate()")

```
postBlockUpdate(tblock): ZswapChainState;
```

Carries out a post-block update, which does amortized bookkeeping that only needs to be done once per state change.

Typically, `postBlockUpdate` should be run after any (sequence of) (system)-transaction application(s).

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### tblock[​](#tblock "Direct link to tblock")

`Date`

#### Returns[​](#returns-2 "Direct link to Returns")

`ZswapChainState`

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-3 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-4 "Direct link to Returns")

`string`

***

### tryApply()[​](#tryapply "Direct link to tryApply()")

```
tryApply<P>(offer, whitelist?): [ZswapChainState, Map<string, bigint>];
```

Try to apply an [ZswapOffer](/api-reference/ledger/classes/ZswapOffer.md) to the state, returning the updated state and a map on newly inserted coin commitments to their inserted indices.

#### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

##### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### offer[​](#offer "Direct link to offer")

[`ZswapOffer`](/api-reference/ledger/classes/ZswapOffer.md)<`P`>

##### whitelist?[​](#whitelist "Direct link to whitelist?")

`Set`<`string`>

A set of contract addresses that are of interest. If set, *only* these addresses are tracked, and all other information is discarded.

#### Returns[​](#returns-5 "Direct link to Returns")

\[`ZswapChainState`, `Map`<`string`, `bigint`>]

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): ZswapChainState;
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-6 "Direct link to Returns")

`ZswapChainState`

***

### deserializeFromLedgerState()[​](#deserializefromledgerstate "Direct link to deserializeFromLedgerState()")

```
static deserializeFromLedgerState(raw): ZswapChainState;
```

Given a whole ledger serialized state, deserialize only the Zswap portion

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### raw[​](#raw-1 "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-7 "Direct link to Returns")

`ZswapChainState`
