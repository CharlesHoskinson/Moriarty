# QueryContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / QueryContext

# Class: QueryContext

Provides the information needed to fully process a transaction, including information about the rest of the transaction, and the state of the chain at the time of execution.

## Constructors[​](#constructors "Direct link to Constructors")

### new QueryContext()[​](#new-querycontext "Direct link to new QueryContext()")

```
new QueryContext(state, address): QueryContext
```

Construct a basic context from a contract's address and current state value

#### Parameters[​](#parameters "Direct link to Parameters")

##### state[​](#state "Direct link to state")

[`ChargedState`](/api-reference/onchain-runtime/classes/ChargedState.md)

##### address[​](#address "Direct link to address")

`string`

#### Returns[​](#returns "Direct link to Returns")

[`QueryContext`](/api-reference/onchain-runtime/classes/QueryContext.md)

## Properties[​](#properties "Direct link to Properties")

### address[​](#address-1 "Direct link to address")

```
readonly address: string;
```

The address of the contract

***

### block[​](#block "Direct link to block")

```
block: CallContext;
```

The block-level information accessible to the contract

***

### comIndices[​](#comindices "Direct link to comIndices")

```
readonly comIndices: Map<string, bigint>;
```

The commitment indices map accessible to the contract, primarily via [qualify](/api-reference/onchain-runtime/classes/QueryContext.md#qualify)

***

### effects[​](#effects "Direct link to effects")

```
effects: Effects;
```

The effects that occurred during execution against this context, should match those declared in a [Transcript](/api-reference/onchain-runtime/type-aliases/Transcript.md)

***

### state[​](#state-1 "Direct link to state")

```
readonly state: ChargedState;
```

The current contract state retained in the context

## Methods[​](#methods "Direct link to Methods")

### insertCommitment()[​](#insertcommitment "Direct link to insertCommitment()")

```
insertCommitment(comm, index): QueryContext
```

Register a given coin commitment as being accessible at a specific index, for use when receiving coins in-contract, and needing to record their index to later spend them

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### comm[​](#comm "Direct link to comm")

`string`

##### index[​](#index "Direct link to index")

`bigint`

#### Returns[​](#returns-1 "Direct link to Returns")

[`QueryContext`](/api-reference/onchain-runtime/classes/QueryContext.md)

***

### qualify()[​](#qualify "Direct link to qualify()")

```
qualify(coin): undefined | Value
```

**`Internal`**

Internal counterpart to [insertCommitment](/api-reference/onchain-runtime/classes/QueryContext.md#insertcommitment); upgrades an encoded [ShieldedCoinInfo](/api-reference/onchain-runtime/type-aliases/ShieldedCoinInfo.md) to an encoded [QualifiedShieldedCoinInfo](/api-reference/onchain-runtime/type-aliases/QualifiedShieldedCoinInfo.md) using the inserted commitments

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### coin[​](#coin "Direct link to coin")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

#### Returns[​](#returns-2 "Direct link to Returns")

`undefined` | [`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

***

### query()[​](#query "Direct link to query()")

```
query(

   ops, 

   cost_model, 

   gas_limit?): QueryResults
```

Runs a sequence of operations in gather mode, returning the results of the gather.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### ops[​](#ops "Direct link to ops")

[`Op`](/api-reference/onchain-runtime/type-aliases/Op.md)<`null`>\[]

##### cost\_model[​](#cost_model "Direct link to cost_model")

[`CostModel`](/api-reference/onchain-runtime/classes/CostModel.md)

##### gas\_limit?[​](#gas_limit "Direct link to gas_limit?")

[`RunningCost`](/api-reference/onchain-runtime/type-aliases/RunningCost.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`QueryResults`](/api-reference/onchain-runtime/classes/QueryResults.md)

***

### runTranscript()[​](#runtranscript "Direct link to runTranscript()")

```
runTranscript(transcript, cost_model): QueryContext
```

Runs a transcript in verifying mode against the current query context, outputting a new query context, with the [state](/api-reference/onchain-runtime/classes/QueryContext.md#state-1) and [effects](/api-reference/onchain-runtime/classes/QueryContext.md#effects) from after the execution.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### transcript[​](#transcript "Direct link to transcript")

[`Transcript`](/api-reference/onchain-runtime/type-aliases/Transcript.md)<[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)>

##### cost\_model[​](#cost_model-1 "Direct link to cost_model")

[`CostModel`](/api-reference/onchain-runtime/classes/CostModel.md)

#### Returns[​](#returns-4 "Direct link to Returns")

[`QueryContext`](/api-reference/onchain-runtime/classes/QueryContext.md)

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-5 "Direct link to Returns")

`string`

***

### toVmStack()[​](#tovmstack "Direct link to toVmStack()")

```
toVmStack(): VmStack
```

Converts the QueryContext to [VmStack](/api-reference/onchain-runtime/classes/VmStack.md).

#### Returns[​](#returns-6 "Direct link to Returns")

[`VmStack`](/api-reference/onchain-runtime/classes/VmStack.md)
