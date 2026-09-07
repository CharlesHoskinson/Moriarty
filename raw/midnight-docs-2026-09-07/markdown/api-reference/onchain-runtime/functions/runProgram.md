# runProgram

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / runProgram

# Function: runProgram()

```
function runProgram(

   initial, 

   ops, 

   cost_model, 

   gas_limit?): VmResults
```

Runs a VM program against an initial stack, with an optional gas limit

## Parameters[​](#parameters "Direct link to Parameters")

### initial[​](#initial "Direct link to initial")

[`VmStack`](/api-reference/onchain-runtime/classes/VmStack.md)

### ops[​](#ops "Direct link to ops")

[`Op`](/api-reference/onchain-runtime/type-aliases/Op.md)<`null`>\[]

### cost\_model[​](#cost_model "Direct link to cost_model")

[`CostModel`](/api-reference/onchain-runtime/classes/CostModel.md)

### gas\_limit?[​](#gas_limit "Direct link to gas_limit?")

[`RunningCost`](/api-reference/onchain-runtime/type-aliases/RunningCost.md)

## Returns[​](#returns "Direct link to Returns")

[`VmResults`](/api-reference/onchain-runtime/classes/VmResults.md)
