# createWitnessContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / createWitnessContext

# Function: createWitnessContext()

```
function createWitnessContext<L, PS>(

   ledger, 

   privateState, 

contractAddress): WitnessContext<L, PS>;
```

**`Internal`**

Internal constructor for [WitnessContext](/api-reference/compact-runtime/interfaces/WitnessContext.md).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### L[​](#l "Direct link to L")

`L`

### PS[​](#ps "Direct link to PS")

`PS`

## Parameters[​](#parameters "Direct link to Parameters")

### ledger[​](#ledger "Direct link to ledger")

`L`

### privateState[​](#privatestate "Direct link to privateState")

`PS`

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

## Returns[​](#returns "Direct link to Returns")

[`WitnessContext`](/api-reference/compact-runtime/interfaces/WitnessContext.md)<`L`, `PS`>
