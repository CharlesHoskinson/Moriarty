# LocalState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / LocalState

# Class: LocalState

The local state of a user/wallet, consisting of a set of unspent coins

It also keeps track of coins that are in-flight, either expecting to spend or expecting to receive, and a local copy of the global coin commitment Merkle tree to generate proofs against.

## Constructors[​](#constructors "Direct link to Constructors")

### new LocalState()[​](#new-localstate "Direct link to new LocalState()")

```
new LocalState(): LocalState
```

Creates a new, empty state

#### Returns[​](#returns "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

## Properties[​](#properties "Direct link to Properties")

### coins[​](#coins "Direct link to coins")

```
readonly coins: Set<QualifiedCoinInfo>;
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
readonly pendingOutputs: Map<string, CoinInfo>;
```

The outputs that this wallet is expecting to receive in the future

***

### pendingSpends[​](#pendingspends "Direct link to pendingSpends")

```
readonly pendingSpends: Map<string, QualifiedCoinInfo>;
```

The spends that this wallet is expecting to be finalized on-chain in the future

## Methods[​](#methods "Direct link to Methods")

### apply()[​](#apply "Direct link to apply()")

```
apply(secretKeys, offer): LocalState
```

Locally applies an offer to the current state, returning the updated state

#### Parameters[​](#parameters "Direct link to Parameters")

##### secretKeys[​](#secretkeys "Direct link to secretKeys")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

##### offer[​](#offer "Direct link to offer")

[`Offer`](/api-reference/zswap/classes/Offer.md)

#### Returns[​](#returns-1 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### applyCollapsedUpdate()[​](#applycollapsedupdate "Direct link to applyCollapsedUpdate()")

```
applyCollapsedUpdate(update): LocalState
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

[`MerkleTreeCollapsedUpdate`](/api-reference/zswap/classes/MerkleTreeCollapsedUpdate.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### applyFailed()[​](#applyfailed "Direct link to applyFailed()")

```
applyFailed(offer): LocalState
```

Locally marks an offer as failed, allowing inputs used in it to be spendable once more.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### offer[​](#offer-1 "Direct link to offer")

[`Offer`](/api-reference/zswap/classes/Offer.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### applyFailedProofErased()[​](#applyfailedprooferased "Direct link to applyFailedProofErased()")

```
applyFailedProofErased(offer): LocalState
```

Locally marks an proof-erased offer as failed, allowing inputs used in it to be spendable once more.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### offer[​](#offer-2 "Direct link to offer")

[`ProofErasedOffer`](/api-reference/zswap/classes/ProofErasedOffer.md)

#### Returns[​](#returns-4 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### applyProofErased()[​](#applyprooferased "Direct link to applyProofErased()")

```
applyProofErased(secretKeys, offer): LocalState
```

Locally applies a proof-erased offer to the current state, returning the updated state

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-1 "Direct link to secretKeys")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

##### offer[​](#offer-3 "Direct link to offer")

[`ProofErasedOffer`](/api-reference/zswap/classes/ProofErasedOffer.md)

#### Returns[​](#returns-5 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### applyProofErasedTx()[​](#applyprooferasedtx "Direct link to applyProofErasedTx()")

```
applyProofErasedTx(

   secretKeys, 

   tx, 

   res): LocalState
```

Locally applies a proof-erased transaction to the current state, returning the updated state

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-2 "Direct link to secretKeys")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

##### tx[​](#tx "Direct link to tx")

[`ProofErasedTransaction`](/api-reference/zswap/classes/ProofErasedTransaction.md)

##### res[​](#res "Direct link to res")

The result type of applying this transaction against the ledger state

`"success"` | `"partialSuccess"` | `"failure"`

#### Returns[​](#returns-6 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### applySystemTx()[​](#applysystemtx "Direct link to applySystemTx()")

```
applySystemTx(secretKeys, tx): LocalState
```

Locally applies a system transaction to the current state, returning the updated state

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-3 "Direct link to secretKeys")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

##### tx[​](#tx-1 "Direct link to tx")

[`SystemTransaction`](/api-reference/zswap/classes/SystemTransaction.md)

#### Returns[​](#returns-7 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### applyTx()[​](#applytx "Direct link to applyTx()")

```
applyTx(

   secretKeys, 

   tx, 

   res): LocalState
```

Locally applies a transaction to the current state, returning the updated state

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-4 "Direct link to secretKeys")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

##### tx[​](#tx-2 "Direct link to tx")

[`Transaction`](/api-reference/zswap/classes/Transaction.md)

##### res[​](#res-1 "Direct link to res")

The result type of applying this transaction against the ledger state

`"success"` | `"partialSuccess"` | `"failure"`

#### Returns[​](#returns-8 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-9 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### spend()[​](#spend "Direct link to spend()")

```
spend(

   secretKeys, 

   coin, 

   segment): [LocalState, UnprovenInput]
```

Initiates a new spend of a specific coin, outputting the corresponding [UnprovenInput](/api-reference/zswap/classes/UnprovenInput.md), and the updated state marking this coin as in-flight.

#### Parameters[​](#parameters-9 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-5 "Direct link to secretKeys")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

##### coin[​](#coin "Direct link to coin")

[`QualifiedCoinInfo`](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`number`

#### Returns[​](#returns-10 "Direct link to Returns")

\[[`LocalState`](/api-reference/zswap/classes/LocalState.md), [`UnprovenInput`](/api-reference/zswap/classes/UnprovenInput.md)]

***

### spendFromOutput()[​](#spendfromoutput "Direct link to spendFromOutput()")

```
spendFromOutput(

   secretKeys, 

   coin, 

   segment, 

   output): [LocalState, UnprovenTransient]
```

Initiates a new spend of a new-yet-received output, outputting the corresponding [UnprovenTransient](/api-reference/zswap/classes/UnprovenTransient.md), and the updated state marking this coin as in-flight.

#### Parameters[​](#parameters-10 "Direct link to Parameters")

##### secretKeys[​](#secretkeys-6 "Direct link to secretKeys")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

##### coin[​](#coin-1 "Direct link to coin")

[`QualifiedCoinInfo`](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md)

##### segment[​](#segment-1 "Direct link to segment")

`number`

##### output[​](#output "Direct link to output")

[`UnprovenOutput`](/api-reference/zswap/classes/UnprovenOutput.md)

#### Returns[​](#returns-11 "Direct link to Returns")

\[[`LocalState`](/api-reference/zswap/classes/LocalState.md), [`UnprovenTransient`](/api-reference/zswap/classes/UnprovenTransient.md)]

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-11 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-12 "Direct link to Returns")

`string`

***

### watchFor()[​](#watchfor "Direct link to watchFor()")

```
watchFor(coinPublicKey, coin): LocalState
```

Adds a coin to the list of coins that are expected to be received

This should be used if an output is creating a coin for this wallet, which does not contain a ciphertext to detect it. In this case, the wallet must know the commitment ahead of time to notice the receipt.

#### Parameters[​](#parameters-12 "Direct link to Parameters")

##### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

`string`

##### coin[​](#coin-2 "Direct link to coin")

[`CoinInfo`](/api-reference/zswap/type-aliases/CoinInfo.md)

#### Returns[​](#returns-13 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): LocalState
```

#### Parameters[​](#parameters-13 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-14 "Direct link to Returns")

[`LocalState`](/api-reference/zswap/classes/LocalState.md)
