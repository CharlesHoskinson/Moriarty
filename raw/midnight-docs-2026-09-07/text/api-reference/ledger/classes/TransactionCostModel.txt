# TransactionCostModel

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / TransactionCostModel

# Class: TransactionCostModel

## Properties[​](#properties "Direct link to Properties")

### baselineCost[​](#baselinecost "Direct link to baselineCost")

```
readonly baselineCost: RunningCost;
```

A baseline cost to begin with

***

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

***

### runtimeCostModel[​](#runtimecostmodel "Direct link to runtimeCostModel")

```
readonly runtimeCostModel: CostModel;
```

A cost model for calculating transaction fees

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): TransactionCostModel;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

`TransactionCostModel`

***

### initialTransactionCostModel()[​](#initialtransactioncostmodel "Direct link to initialTransactionCostModel()")

```
static initialTransactionCostModel(): TransactionCostModel;
```

The initial cost model of Midnight

#### Returns[​](#returns-3 "Direct link to Returns")

`TransactionCostModel`
