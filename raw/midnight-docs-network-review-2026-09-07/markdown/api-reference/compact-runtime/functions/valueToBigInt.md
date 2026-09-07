# valueToBigInt

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / valueToBigInt

# Function: valueToBigInt()

```
function valueToBigInt(x): bigint;
```

**`Internal`**

Internal conversion between field-aligned binary values and bigints within the scalar field

## Parameters[​](#parameters "Direct link to Parameters")

### x[​](#x "Direct link to x")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

`bigint`

## Throws[​](#throws "Direct link to Throws")

If the value does not encode a field element
