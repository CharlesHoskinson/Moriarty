# createZswapOutput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / createZswapOutput

# Function: createZswapOutput()

```
function createZswapOutput(

   circuitContext, 

   coinInfo, 

   recipient): [];
```

Adds a coin to the list of outputs produced by the circuit.

## Parameters[​](#parameters "Direct link to Parameters")

### circuitContext[​](#circuitcontext "Direct link to circuitContext")

[`CircuitContext`](/api-reference/compact-runtime/interfaces/CircuitContext.md)<`unknown`>

The current circuit context.

### coinInfo[​](#coininfo "Direct link to coinInfo")

[`EncodedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md)

The coin to produce.

### recipient[​](#recipient "Direct link to recipient")

[`EncodedRecipient`](/api-reference/compact-runtime/interfaces/EncodedRecipient.md)

The coin recipient - either a coin public key representing an end user or a contract address representing a contract.

## Returns[​](#returns "Direct link to Returns")

\[]
