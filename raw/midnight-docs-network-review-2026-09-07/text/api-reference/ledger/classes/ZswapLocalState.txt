# ZswapLocalState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ZswapLocalState

# Class: ZswapLocalState

The local state of a user/wallet, consisting of a set of unspent coins

It also keeps track of coins that are in-flight, either expecting to spend or expecting to receive, and a local copy of the global coin commitment Merkle tree to generate proofs against.

It does not store keys internally, but accepts them as arguments to various operations.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ZswapLocalState(): ZswapLocalState;
```

Creates a new, empty state

#### Returns[​](#returns "Direct link to Returns")

`ZswapLocalState`

## Properties[​](#properties "Direct link to Properties")

### coins[​](#coins "Direct link to coins")

```
readonly coins: Set<QualifiedShieldedCoinInfo>;
```

The set of *spendable* coins of this wallet

***

### firstFree[​](#firstfree "Direct link to firstFree")

```
readonly firstFree: bigint;
```

The first free index in the internal coin commitments Merkle tree. This may be used to identify which merkle tree updates are necessary.

***

### pendingOutputs[​](#pendingoutputs "Direct link to pendingOutputs")

```
readonly pendingOutputs: Map<string, [ShieldedCoinInfo, undefined | Date]>;
```

The outputs that this wallet is expecting to receive in the future, with an optional TTL attached.

***

### pendingSpends[​](#pendingspends "Direct link to pendingSpends")

```
readonly pendingSpends: Map<string, [QualifiedShieldedCoinInfo, undefined | Date]>;
```

The spends that this wallet is expecting to be finalized on-chain in the future. Each has an optional TTL attached.

## Methods[​](#methods "Direct link to Methods")

### apply()[​](#apply "Direct link to apply()")

```
apply<P>(secretKeys, offer): ZswapLocalState;
```

Locally applies an offer to the current state, returning the updated state

#### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

##### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters "Direct link to Parameters")

##### secretKeys[​](#secretkeys "Direct link to secretKeys")

[`ZswapSecretKeys`](/api-reference/ledger/classes/ZswapSecretKeys.md)

##### offer[​](#offer "Direct link to offer")

[`ZswapOffer`](/api-reference/ledger/classes/ZswapOffer.md)<`P`>

#### Returns[​](#returns-1 "Direct link to Returns")

`ZswapLocalState`

***

### applyCollapsedUpdate()[​](#applycollapsedupdate "Direct link to applyCollapsedUpdate()")

```
applyCollapsedUpdate(update): ZswapLocalState;
```

Applies a collapsed Merkle tree update to the current local state, fast forwarding through the indices included in it, if it is a correct update.

The general flow for usage if Alice is in state A, and wants to ask Bob how to reach the new state B, is:

* Find where she left off – what's her firstFree?

* Find out where she's going – ask for Bob's firstFree.

* Find what contents she does care about – ask Bob for the filtered entries she want to include proper in her tree.

* In order, of Merkle tree indices:

  <!-- -->

  * Insert (with `apply` offers Alice cares about).
  * Skip (with this method) sections Alice does not care about, obtaining the collapsed update covering the gap from Bob. Note that `firstFree` is not included in the tree itself, and both ends of updates *are* included.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### update[​](#update "Direct link to update")

[`MerkleTreeCollapsedUpdate`](/api-reference/ledger/classes/MerkleTreeCollapsedUpdate.md)

#### Returns[​](#returns-2 "Direct link to Returns")

`ZswapLocalState`

***

### applyFailed()[​](#applyfailed "Direct link to applyFailed()")

```
applyFailed<P>(offer): ZswapLocalState;
```

Locally reverts pending outputs/spends from an offer known to have failed or which has been discarded.

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### P[​](#p-1 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### offer[​](#offer-1 "Direct link to offer")

[`ZswapOffer`](/api-reference/ledger/classes/ZswapOffer.md)<`P`>

#### Returns[​](#returns-3 "Direct link to Returns")

`ZswapLocalState`

***

### clearPending()[​](#clearpending "Direct link to clearPending()")

```
clearPending(time): ZswapLocalState;
```

Clears pending outputs / spends that have passed their TTL without being included in a block.

Note that as TTLs are *from a block perspective*, and there is some latency between the block and the wallet, the time passed in here should not be the current time, but incorporate a latency buffer.

NOTE: This API endpoint is currently non-functional and works as a no-op.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### time[​](#time "Direct link to time")

`Date`

#### Returns[​](#returns-4 "Direct link to Returns")

`ZswapLocalState`

***

### replayEvents()[​](#replayevents "Direct link to replayEvents()")

```
replayEvents(secretKeys, events): ZswapLocalState;
```

Replays observed events against the current local state. These *must* be replayed in the same order as emitted by the chain being followed.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-1 "Direct link to secretKeys")

[`ZswapSecretKeys`](/api-reference/ledger/classes/ZswapSecretKeys.md)

##### events[​](#events "Direct link to events")

[`Event`](/api-reference/ledger/classes/Event.md)\[]

#### Returns[​](#returns-5 "Direct link to Returns")

`ZswapLocalState`

***

### replayEventsWithChanges()[​](#replayeventswithchanges "Direct link to replayEventsWithChanges()")

```
replayEventsWithChanges(secretKeys, events): ZswapLocalStateWithChanges;
```

Replays observed events against the current local state, returning both the updated state and the state changes. These *must* be replayed in the same order as emitted by the chain being followed.

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-2 "Direct link to secretKeys")

[`ZswapSecretKeys`](/api-reference/ledger/classes/ZswapSecretKeys.md)

##### events[​](#events-1 "Direct link to events")

[`Event`](/api-reference/ledger/classes/Event.md)\[]

#### Returns[​](#returns-6 "Direct link to Returns")

[`ZswapLocalStateWithChanges`](/api-reference/ledger/classes/ZswapLocalStateWithChanges.md)

***

### revertTransaction()[​](#reverttransaction "Direct link to revertTransaction()")

```
revertTransaction<S, P, B>(transaction): ZswapLocalState;
```

Locally reverts all pending outputs/spends from a transaction which has been discarded.

Behaves as [applyFailed](#applyfailed) for the entire transaction.

#### Type Parameters[​](#type-parameters-2 "Direct link to Type Parameters")

##### S[​](#s "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

##### P[​](#p-2 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

##### B[​](#b "Direct link to B")

`B` *extends* [`Bindingish`](/api-reference/ledger/type-aliases/Bindingish.md)

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### transaction[​](#transaction "Direct link to transaction")

[`Transaction`](/api-reference/ledger/classes/Transaction.md)<`S`, `P`, `B`>

#### Returns[​](#returns-7 "Direct link to Returns")

`ZswapLocalState`

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-8 "Direct link to Returns")

`Uint8Array`

***

### spend()[​](#spend "Direct link to spend()")

```
spend(

   secretKeys, 

   coin, 

   segment, 

   ttl?): [ZswapLocalState, UnprovenInput];
```

Initiates a new spend of a specific coin, outputting the corresponding [ZswapInput](/api-reference/ledger/classes/ZswapInput.md), and the updated state marking this coin as in-flight.

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-3 "Direct link to secretKeys")

[`ZswapSecretKeys`](/api-reference/ledger/classes/ZswapSecretKeys.md)

##### coin[​](#coin "Direct link to coin")

[`QualifiedShieldedCoinInfo`](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`undefined` | `number`

##### ttl?[​](#ttl "Direct link to ttl?")

`Date`

#### Returns[​](#returns-9 "Direct link to Returns")

\[`ZswapLocalState`, [`UnprovenInput`](/api-reference/ledger/type-aliases/UnprovenInput.md)]

***

### spendFromOutput()[​](#spendfromoutput "Direct link to spendFromOutput()")

```
spendFromOutput(

   secretKeys, 

   coin, 

   segment, 

   output, 

   ttl?): [ZswapLocalState, UnprovenTransient];
```

Initiates a new spend of a new-yet-received output, outputting the corresponding [ZswapTransient](/api-reference/ledger/classes/ZswapTransient.md), and the updated state marking this coin as in-flight.

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-4 "Direct link to secretKeys")

[`ZswapSecretKeys`](/api-reference/ledger/classes/ZswapSecretKeys.md)

##### coin[​](#coin-1 "Direct link to coin")

[`QualifiedShieldedCoinInfo`](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md)

##### segment[​](#segment-1 "Direct link to segment")

`undefined` | `number`

##### output[​](#output "Direct link to output")

[`UnprovenOutput`](/api-reference/ledger/type-aliases/UnprovenOutput.md)

##### ttl?[​](#ttl-1 "Direct link to ttl?")

`Date`

#### Returns[​](#returns-10 "Direct link to Returns")

\[`ZswapLocalState`, [`UnprovenTransient`](/api-reference/ledger/type-aliases/UnprovenTransient.md)]

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-9 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-11 "Direct link to Returns")

`string`

***

### watchFor()[​](#watchfor "Direct link to watchFor()")

```
watchFor(coinPublicKey, coin): ZswapLocalState;
```

Adds a coin to the list of coins that are expected to be received

This should be used if an output is creating a coin for this wallet, which does not contain a ciphertext to detect it. In this case, the wallet must know the commitment ahead of time to notice the receipt.

#### Parameters[​](#parameters-10 "Direct link to Parameters")

##### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

`string`

##### coin[​](#coin-2 "Direct link to coin")

[`ShieldedCoinInfo`](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md)

#### Returns[​](#returns-12 "Direct link to Returns")

`ZswapLocalState`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): ZswapLocalState;
```

#### Parameters[​](#parameters-11 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-13 "Direct link to Returns")

`ZswapLocalState`
