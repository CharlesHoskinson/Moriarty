# VmResults

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / VmResults

# Class: VmResults

Represents the results of a VM call

## Properties[​](#properties "Direct link to Properties")

### events[​](#events "Direct link to events")

```
readonly events: GatherResult[];
```

The events that got emitted by this VM invocation

***

### gasCost[​](#gascost "Direct link to gasCost")

```
readonly gasCost: RunningCost;
```

The computed gas cost of running this VM invocation

***

### stack[​](#stack "Direct link to stack")

```
readonly stack: VmStack;
```

The VM stack at the end of the VM invocation

## Methods[​](#methods "Direct link to Methods")

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns "Direct link to Returns")

`string`
