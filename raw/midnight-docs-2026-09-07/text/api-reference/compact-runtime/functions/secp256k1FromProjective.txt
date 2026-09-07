# secp256k1FromProjective

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / secp256k1FromProjective

# Function: secp256k1FromProjective()

```
function secp256k1FromProjective(p): Secp256k1Point;
```

Project a noble-curves point back down to the simple affine `Secp256k1Point` representation.

## Parameters[​](#parameters "Direct link to Parameters")

### p[​](#p "Direct link to p")

`WeierstrassPoint`<`bigint`>

## Returns[​](#returns "Direct link to Returns")

[`Secp256k1Point`](/api-reference/compact-runtime/interfaces/Secp256k1Point.md)
