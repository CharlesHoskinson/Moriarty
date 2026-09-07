# persistentHash

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / persistentHash

# Function: persistentHash()

```
function persistentHash(align, val): Value
```

**`Internal`**

Internal implementation of the persistent hash primitive

## Parameters[​](#parameters "Direct link to Parameters")

### align[​](#align "Direct link to align")

[`Alignment`](/api-reference/onchain-runtime/type-aliases/Alignment.md)

### val[​](#val "Direct link to val")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

## Throws[​](#throws "Direct link to Throws")

If [val](/api-reference/onchain-runtime/functions/persistentHash.md#val) does not have alignment [align](/api-reference/onchain-runtime/functions/persistentHash.md#align), or any component has a compress alignment
