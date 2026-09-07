# ecMul

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ecMul

# Function: ecMul()

```
function ecMul(a, b): Value;
```

**`Internal`**

Internal implementation of the elliptic curve multiplication primitive

## Parameters[​](#parameters "Direct link to Parameters")

### a[​](#a "Direct link to a")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

### b[​](#b "Direct link to b")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

## Returns[​](#returns "Direct link to Returns")

[`Value`](/api-reference/ledger/type-aliases/Value.md)

## Throws[​](#throws "Direct link to Throws")

If [a](#ecmul) does not encode an elliptic curve point or [b](#ecmul) does not encode a field element
