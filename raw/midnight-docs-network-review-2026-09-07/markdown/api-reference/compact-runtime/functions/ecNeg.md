# ecNeg

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ecNeg

# Function: ecNeg()

```
function ecNeg(a): JubjubPoint;
```

The Compact builtin `ecNeg` function

This function negates an elliptic curve point. On the JubJub twisted Edwards curve, the negation of (x, y) is (-x, y).

## Parameters[​](#parameters "Direct link to Parameters")

### a[​](#a "Direct link to a")

[`JubjubPoint`](/api-reference/compact-runtime/interfaces/JubjubPoint.md)

## Returns[​](#returns "Direct link to Returns")

[`JubjubPoint`](/api-reference/compact-runtime/interfaces/JubjubPoint.md)
