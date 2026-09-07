# hashToCurve

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / hashToCurve

# Function: hashToCurve()

```
function hashToCurve<A>(rtType, x): JubjubPoint;
```

The Compact builtin `hashToCurve` function

This function maps arbitrary values representable in Compact to elliptic curve points in the proof system's embedded curve.

Outputs are guaranteed to have unknown discrete logarithm with respect to the group base, and any other output, but are not guaranteed to be unique (a given input can be proven correct for multiple outputs).

Inputs of different types may have the same output, if they have the same field-aligned binary representation.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

## Parameters[​](#parameters "Direct link to Parameters")

### rtType[​](#rttype "Direct link to rtType")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<`A`>

### x[​](#x "Direct link to x")

`A`

## Returns[​](#returns "Direct link to Returns")

[`JubjubPoint`](/api-reference/compact-runtime/interfaces/JubjubPoint.md)
