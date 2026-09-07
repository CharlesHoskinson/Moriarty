# ecMul

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / ecMul

# Function: ecMul()

```
function ecMul(a, b): Value
```

**`Internal`**

Internal implementation of the elliptic curve multiplication primitive

## Parameters[​](#parameters "Direct link to Parameters")

### a[​](#a "Direct link to a")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

### b[​](#b "Direct link to b")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

[`Value`](/api-reference/onchain-runtime/type-aliases/Value.md)

## Throws[​](#throws "Direct link to Throws")

If [a](/api-reference/onchain-runtime/functions/ecMul.md#a) does not encode an elliptic curve point or [b](/api-reference/onchain-runtime/functions/ecMul.md#b) does not encode a field element
