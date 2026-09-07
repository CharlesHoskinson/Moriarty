# createZswapInput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / createZswapInput

# Function: createZswapInput()

```
function createZswapInput(circuitContext, qualifiedShieldedCoinInfo): [];
```

Adds a coin to the list of inputs consumed by the circuit.

## Parameters[​](#parameters "Direct link to Parameters")

### circuitContext[​](#circuitcontext "Direct link to circuitContext")

[`CircuitContext`](/api-reference/compact-runtime/interfaces/CircuitContext.md)

The current circuit context.

### qualifiedShieldedCoinInfo[​](#qualifiedshieldedcoininfo "Direct link to qualifiedShieldedCoinInfo")

[`EncodedQualifiedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedQualifiedShieldedCoinInfo.md)

The input to consume.

## Returns[​](#returns "Direct link to Returns")

\[]
