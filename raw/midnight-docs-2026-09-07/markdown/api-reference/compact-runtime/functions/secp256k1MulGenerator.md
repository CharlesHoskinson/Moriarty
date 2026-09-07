# secp256k1MulGenerator

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / secp256k1MulGenerator

# Function: secp256k1MulGenerator()

```
function secp256k1MulGenerator(b): Secp256k1Point;
```

The Compact builtin `ecMulGenerator` function for secp256k1 points.

`multiplyUnsafe` is used, instead of `multiply`, because the latter rejects a zero scalar; the "unsafe" (variable-time) is due to non-constant time operations, which we don't guarantee anyways.

## Parameters[​](#parameters "Direct link to Parameters")

### b[​](#b "Direct link to b")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`Secp256k1Point`](/api-reference/compact-runtime/interfaces/Secp256k1Point.md)
