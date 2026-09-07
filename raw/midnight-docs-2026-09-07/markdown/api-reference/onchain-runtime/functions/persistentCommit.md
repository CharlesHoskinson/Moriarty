# persistentCommit

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / persistentCommit

# Function: persistentCommit()

```
function persistentCommit(

   align, 

   val, 

   opening): Value
```

**`Internal`**

Internal implementation of the persistent commitment primitive

## Parameters[​](#parameters "Direct link to Parameters")

### align[​](#align "Direct link to align")

[`Alignment`](/api-reference/onchain-runtime/type-aliases/Alignment.md)

### val[​](#val "Direct link to val")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

### opening[​](#opening "Direct link to opening")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

## Throws[​](#throws "Direct link to Throws")

If [val](/api-reference/onchain-runtime/functions/persistentCommit.md#val) does not have alignment [align](/api-reference/onchain-runtime/functions/persistentCommit.md#align), [opening](/api-reference/onchain-runtime/functions/persistentCommit.md#opening) does not encode a 32-byte bytestring, or any component has a compress alignment
