# TransactionCostModel

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / TransactionCostModel

# Class: TransactionCostModel

## Properties[​](#properties "Direct link to Properties")

### inputFeeOverhead[​](#inputfeeoverhead "Direct link to inputFeeOverhead")

```
readonly inputFeeOverhead: bigint;
```

The increase in fees to expect from adding a new input to a transaction

***

### outputFeeOverhead[​](#outputfeeoverhead "Direct link to outputFeeOverhead")

```
readonly outputFeeOverhead: bigint;
```

The increase in fees to expect from adding a new output to a transaction

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
static deserialize(raw, netid): TransactionCostModel
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`TransactionCostModel`](/api-reference/zswap/classes/TransactionCostModel.md)

***

### dummyTransactionCostModel()[​](#dummytransactioncostmodel "Direct link to dummyTransactionCostModel()")

```
static dummyTransactionCostModel(): TransactionCostModel
```

A dummy cost model, for use in testing

#### Returns[​](#returns-3 "Direct link to Returns")

[`TransactionCostModel`](/api-reference/zswap/classes/TransactionCostModel.md)
