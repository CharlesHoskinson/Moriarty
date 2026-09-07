# valueToBigInt

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / valueToBigInt

# Function: valueToBigInt()

```
function valueToBigInt(x): bigint;
```

**`Internal`**

Internal conversion between field-aligned binary values and bigints within the scalar field

## Parameters[​](#parameters "Direct link to Parameters")

### x[​](#x "Direct link to x")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

`bigint`

## Throws[​](#throws "Direct link to Throws")

If the value does not encode a field element
