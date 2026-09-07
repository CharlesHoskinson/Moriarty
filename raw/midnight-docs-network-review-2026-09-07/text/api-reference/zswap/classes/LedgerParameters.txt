# LedgerParameters

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / LedgerParameters

# Class: LedgerParameters

Parameters used by the Midnight ledger, including transaction fees and bounds

## Properties[​](#properties "Direct link to Properties")

### transactionCostModel[​](#transactioncostmodel "Direct link to transactionCostModel")

```
readonly transactionCostModel: TransactionCostModel;
```

The cost model used for transaction fees contained in these parameters

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): LedgerParameters
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`LedgerParameters`](/api-reference/zswap/classes/LedgerParameters.md)

***

### dummyParameters()[​](#dummyparameters "Direct link to dummyParameters()")

```
static dummyParameters(): LedgerParameters
```

A dummy set of testing parameters

#### Returns[​](#returns-3 "Direct link to Returns")

[`LedgerParameters`](/api-reference/zswap/classes/LedgerParameters.md)
