# runProgram

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / runProgram

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

[`VmStack`](/api-reference/ledger/classes/VmStack.md)

### ops[​](#ops "Direct link to ops")

[`Op`](/api-reference/ledger/type-aliases/Op.md)<`null`>\[]

### cost\_model[​](#cost_model "Direct link to cost_model")

[`CostModel`](/api-reference/ledger/classes/CostModel.md)

### gas\_limit?[​](#gas_limit "Direct link to gas_limit?")

[`RunningCost`](/api-reference/ledger/type-aliases/RunningCost.md)

## Returns[​](#returns "Direct link to Returns")

[`VmResults`](/api-reference/ledger/classes/VmResults.md)
