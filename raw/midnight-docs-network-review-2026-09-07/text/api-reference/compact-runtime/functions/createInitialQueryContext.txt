# createInitialQueryContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / createInitialQueryContext

# Function: createInitialQueryContext()

```
function createInitialQueryContext(

   contractState, 

   contractAddress, 

   time, 

   parentBlockHash?, 

   caller?): QueryContext;
```

**`Internal`**

## Parameters[​](#parameters "Direct link to Parameters")

### contractState[​](#contractstate "Direct link to contractState")

[`ContractState`](/api-reference/compact-runtime/classes/ContractState.md) | [`StateValue`](/api-reference/compact-runtime/classes/StateValue.md) | [`ChargedState`](/api-reference/compact-runtime/classes/ChargedState.md)

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

### time[​](#time "Direct link to time")

`number`

### parentBlockHash?[​](#parentblockhash "Direct link to parentBlockHash?")

`string`

### caller?[​](#caller "Direct link to caller?")

[`PublicAddress`](/api-reference/compact-runtime/type-aliases/PublicAddress.md)

## Returns[​](#returns "Direct link to Returns")

[`QueryContext`](/api-reference/compact-runtime/classes/QueryContext.md)
