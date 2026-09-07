# QueryResults

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / QueryResults

# Class: QueryResults

The results of making a query against a specific state or context

## Properties[​](#properties "Direct link to Properties")

### context[​](#context "Direct link to context")

```
readonly context: QueryContext;
```

The context state after executing the query. This can be used to execute further queries

***

### events[​](#events "Direct link to events")

```
readonly events: GatherResult[];
```

Any events/results that occurred during or from the query

***

### gasCost[​](#gascost "Direct link to gasCost")

```
readonly gasCost: RunningCost;
```

The measured cost of executing the query

## Methods[​](#methods "Direct link to Methods")

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns "Direct link to Returns")

`string`
