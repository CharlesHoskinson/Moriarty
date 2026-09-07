# persistentCommit

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / persistentCommit

# Function: persistentCommit()

```
function persistentCommit(

   align, 

   val, 

   opening): Value;
```

**`Internal`**

Internal implementation of the persistent commitment primitive

## Parameters[​](#parameters "Direct link to Parameters")

### align[​](#align "Direct link to align")

[`Alignment`](/api-reference/ledger/type-aliases/Alignment.md)

### val[​](#val "Direct link to val")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

### opening[​](#opening "Direct link to opening")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

## Throws[​](#throws "Direct link to Throws")

If [val](#persistentcommit) does not have alignment [align](#persistentcommit), [opening](#persistentcommit) does not encode a 32-byte bytestring, or any component has a compress alignment
