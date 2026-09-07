# secp256k1ToProjective

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / secp256k1ToProjective

# Function: secp256k1ToProjective()

```
function secp256k1ToProjective(p): WeierstrassPoint<bigint>;
```

Lift the simple affine `Secp256k1Point` representation into a noble-curves projective point. Identity maps to `Point.ZERO`; every other input is validated to lie on the curve by `fromAffine`.

## Parameters[​](#parameters "Direct link to Parameters")

### p[​](#p "Direct link to p")

[`Secp256k1Point`](/api-reference/compact-runtime/interfaces/Secp256k1Point.md)

## Returns[​](#returns "Direct link to Returns")

`WeierstrassPoint`<`bigint`>
