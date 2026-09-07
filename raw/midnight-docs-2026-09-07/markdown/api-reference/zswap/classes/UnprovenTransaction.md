# UnprovenTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / UnprovenTransaction

# Class: UnprovenTransaction

[Transaction](/api-reference/zswap/classes/Transaction.md), prior to being proven

All "shielded" information in the transaction can still be extracted at this stage!

## Constructors[​](#constructors "Direct link to Constructors")

### new UnprovenTransaction()[​](#new-unproventransaction "Direct link to new UnprovenTransaction()")

```
new UnprovenTransaction(guaranteed, fallible?): UnprovenTransaction
```

Creates the transaction from guaranteed/fallible [UnprovenOffer](/api-reference/zswap/classes/UnprovenOffer.md)s

#### Parameters[​](#parameters "Direct link to Parameters")

##### guaranteed[​](#guaranteed "Direct link to guaranteed")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

##### fallible?[​](#fallible "Direct link to fallible?")

[`UnprovenOffer`](/api-reference/zswap/classes/UnprovenOffer.md)

#### Returns[​](#returns "Direct link to Returns")

[`UnprovenTransaction`](/api-reference/zswap/classes/UnprovenTransaction.md)

## Properties[​](#properties "Direct link to Properties")

### fallibleCoins[​](#falliblecoins "Direct link to fallibleCoins")

```
readonly fallibleCoins: undefined | UnprovenOffer;
```

The fallible Zswap offer

***

### guaranteedCoins[​](#guaranteedcoins "Direct link to guaranteedCoins")

```
readonly guaranteedCoins: undefined | UnprovenOffer;
```

The guaranteed Zswap offer

***

### mint[​](#mint "Direct link to mint")

```
readonly mint: undefined | UnprovenAuthorizedMint;
```

The mint this transaction represents, if applicable

## Methods[​](#methods "Direct link to Methods")

### eraseProofs()[​](#eraseproofs "Direct link to eraseProofs()")

```
eraseProofs(): ProofErasedTransaction
```

Erases the proofs contained in this transaction

#### Returns[​](#returns-1 "Direct link to Returns")

[`ProofErasedTransaction`](/api-reference/zswap/classes/ProofErasedTransaction.md)

***

### identifiers()[​](#identifiers "Direct link to identifiers()")

```
identifiers(): string[]
```

Returns the set of identifiers contained within this transaction. Any of these *may* be used to watch for a specific transaction.

#### Returns[​](#returns-2 "Direct link to Returns")

`string`\[]

***

### merge()[​](#merge "Direct link to merge()")

```
merge(other): UnprovenTransaction
```

Merges this transaction with another

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### other[​](#other "Direct link to other")

[`UnprovenTransaction`](/api-reference/zswap/classes/UnprovenTransaction.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenTransaction`](/api-reference/zswap/classes/UnprovenTransaction.md)

#### Throws[​](#throws "Direct link to Throws")

If both transactions have contract interactions, or they spend the same coins

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-4 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-5 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): UnprovenTransaction
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-6 "Direct link to Returns")

[`UnprovenTransaction`](/api-reference/zswap/classes/UnprovenTransaction.md)

***

### fromMint()[​](#frommint "Direct link to fromMint()")

```
static fromMint(mint): UnprovenTransaction
```

Creates a minting claim transaction, the funds claimed must have been legitimately minted previously.

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### mint[​](#mint-1 "Direct link to mint")

[`UnprovenAuthorizedMint`](/api-reference/zswap/classes/UnprovenAuthorizedMint.md)

#### Returns[​](#returns-7 "Direct link to Returns")

[`UnprovenTransaction`](/api-reference/zswap/classes/UnprovenTransaction.md)
