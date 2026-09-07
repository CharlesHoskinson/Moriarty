# persistentHash

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / persistentHash

# Function: persistentHash()

```
function persistentHash(align, val): Value;
```

**`Internal`**

Internal implementation of the persistent hash primitive

## Parameters[​](#parameters "Direct link to Parameters")

### align[​](#align "Direct link to align")

[`Alignment`](/api-reference/ledger/type-aliases/Alignment.md)

### val[​](#val "Direct link to val")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

## Throws[​](#throws "Direct link to Throws")

If [val](#persistenthash) does not have alignment [align](#persistenthash), or any component has a compress alignment
