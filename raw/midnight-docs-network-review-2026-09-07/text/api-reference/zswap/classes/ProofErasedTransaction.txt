# ProofErasedTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / ProofErasedTransaction

# Class: ProofErasedTransaction

[Transaction](/api-reference/zswap/classes/Transaction.md), with all proof information erased

Primarily for use in testing, or handling data known to be correct from external information

## Properties[​](#properties "Direct link to Properties")

### fallibleCoins[​](#falliblecoins "Direct link to fallibleCoins")

```
readonly fallibleCoins: undefined | ProofErasedOffer;
```

The fallible Zswap offer

***

### guaranteedCoins[​](#guaranteedcoins "Direct link to guaranteedCoins")

```
readonly guaranteedCoins: undefined | ProofErasedOffer;
```

The guaranteed Zswap offer

***

### mint[​](#mint "Direct link to mint")

```
readonly mint: undefined | ProofErasedAuthorizedMint;
```

The mint this transaction represents, if applicable

## Methods[​](#methods "Direct link to Methods")

### fees()[​](#fees "Direct link to fees()")

```
fees(params): bigint
```

The cost of this transaction, in the atomic unit of the base token

#### Parameters[​](#parameters "Direct link to Parameters")

##### params[​](#params "Direct link to params")

[`LedgerParameters`](/api-reference/zswap/classes/LedgerParameters.md)

#### Returns[​](#returns "Direct link to Returns")

`bigint`

***

### identifiers()[​](#identifiers "Direct link to identifiers()")

```
identifiers(): string[]
```

Returns the set of identifiers contained within this transaction. Any of these *may* be used to watch for a specific transaction.

#### Returns[​](#returns-1 "Direct link to Returns")

`string`\[]

***

### imbalances()[​](#imbalances "Direct link to imbalances()")

```
imbalances(guaranteed, fees?): Map<string, bigint>
```

For given fees, and a given section (guaranteed/fallible), what the surplus or deficit of this transaction in any token type is.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### guaranteed[​](#guaranteed "Direct link to guaranteed")

`boolean`

##### fees?[​](#fees-1 "Direct link to fees?")

`bigint`

#### Returns[​](#returns-2 "Direct link to Returns")

`Map`<`string`, `bigint`>

***

### merge()[​](#merge "Direct link to merge()")

```
merge(other): ProofErasedTransaction
```

Merges this transaction with another

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### other[​](#other "Direct link to other")

[`ProofErasedTransaction`](/api-reference/zswap/classes/ProofErasedTransaction.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`ProofErasedTransaction`](/api-reference/zswap/classes/ProofErasedTransaction.md)

#### Throws[​](#throws "Direct link to Throws")

If both transactions have contract interactions, or they spend the same coins

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-4 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-5 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): ProofErasedTransaction
```

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-6 "Direct link to Returns")

[`ProofErasedTransaction`](/api-reference/zswap/classes/ProofErasedTransaction.md)
