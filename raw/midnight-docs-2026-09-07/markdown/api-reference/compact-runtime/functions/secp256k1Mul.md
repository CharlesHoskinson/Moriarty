# secp256k1Mul

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / secp256k1Mul

# Function: secp256k1Mul()

```
function secp256k1Mul(a, b): Secp256k1Point;
```

The Compact builtin `ecMul` function for secp256k1 points.

`multiplyUnsafe` is used, instead of `multiply`, because the latter rejects a zero scalar; the "unsafe" (variable-time) is due to non-constant time operations, which we don't guarantee anyways.

## Parameters[​](#parameters "Direct link to Parameters")

### a[​](#a "Direct link to a")

[`Secp256k1Point`](/api-reference/compact-runtime/interfaces/Secp256k1Point.md)

### b[​](#b "Direct link to b")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`Secp256k1Point`](/api-reference/compact-runtime/interfaces/Secp256k1Point.md)
