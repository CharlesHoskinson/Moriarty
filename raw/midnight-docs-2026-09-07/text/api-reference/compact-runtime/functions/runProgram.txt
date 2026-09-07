# runProgram

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / runProgram

# Function: runProgram()

```
function runProgram(

   initial, 

   ops, 

   cost_model, 

   gas_limit?): VmResults;
```

Runs a VM program against an initial stack, with an optional gas limit

## Parameters[​](#parameters "Direct link to Parameters")

### initial[​](#initial "Direct link to initial")

[`VmStack`](/api-reference/compact-runtime/classes/VmStack.md)

### ops[​](#ops "Direct link to ops")

[`Op`](/api-reference/compact-runtime/type-aliases/Op.md)<`null`>\[]

### cost\_model[​](#cost_model "Direct link to cost_model")

[`CostModel`](/api-reference/compact-runtime/classes/CostModel.md)

### gas\_limit?[​](#gas_limit "Direct link to gas_limit?")

[`RunningCost`](/api-reference/compact-runtime/type-aliases/RunningCost.md)

## Returns[​](#returns "Direct link to Returns")

[`VmResults`](/api-reference/compact-runtime/classes/VmResults.md)
