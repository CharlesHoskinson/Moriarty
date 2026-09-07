# LedgerParameters

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / LedgerParameters

# Class: LedgerParameters

Parameters used by the Midnight ledger, including transaction fees and bounds

## Properties[​](#properties "Direct link to Properties")

### dust[​](#dust "Direct link to dust")

```
readonly dust: DustParameters;
```

The parameters associated with DUST.

***

### feePrices[​](#feeprices "Direct link to feePrices")

```
readonly feePrices: FeePrices;
```

The fee prices for transaction

***

### transactionCostModel[​](#transactioncostmodel "Direct link to transactionCostModel")

```
readonly transactionCostModel: TransactionCostModel;
```

The cost model used for transaction fees contained in these parameters

## Methods[​](#methods "Direct link to Methods")

### maxPriceAdjustment()[​](#maxpriceadjustment "Direct link to maxPriceAdjustment()")

```
maxPriceAdjustment(): number;
```

The maximum price adjustment per block with the current parameters, as a multiplicative factor (that is: 1.1 would indicate a 10% adjustment). Will always return the positive (>1) adjustment factor. Note that negative adjustments are the additive inverse (1.1 has a corresponding 0.9 downward adjustment), *not* the multiplicative as might reasonably be assumed.

#### Returns[​](#returns "Direct link to Returns")

`number`

***

### normalizeFullness()[​](#normalizefullness "Direct link to normalizeFullness()")

```
normalizeFullness(fullness): NormalizedCost;
```

Normalizes a detailed block fullness cost to the block limits.

#### Parameters[​](#parameters "Direct link to Parameters")

##### fullness[​](#fullness "Direct link to fullness")

[`SyntheticCost`](/api-reference/ledger/type-aliases/SyntheticCost.md)

#### Returns[​](#returns-1 "Direct link to Returns")

[`NormalizedCost`](/api-reference/ledger/type-aliases/NormalizedCost.md)

#### Throws[​](#throws "Direct link to Throws")

if any of the block limits is exceeded

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-2 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-3 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): LedgerParameters;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-4 "Direct link to Returns")

`LedgerParameters`

***

### initialParameters()[​](#initialparameters "Direct link to initialParameters()")

```
static initialParameters(): LedgerParameters;
```

The initial parameters of Midnight

#### Returns[​](#returns-5 "Direct link to Returns")

`LedgerParameters`
