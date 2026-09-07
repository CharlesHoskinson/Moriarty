# secp256k1ScalarNeg

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / secp256k1ScalarNeg

# Function: secp256k1ScalarNeg()

```
function secp256k1ScalarNeg(x): bigint;
```

Secp256k1 scalar field negation

This function returns the negation of x in the secp256k1 scalar field. That is, a value y such that x + y = 0 (modulo SECP256K1\_SCALAR\_MODULUS). x is assumed to be in the range \[0, SECP256K1\_SCALAR\_MODULUS).

## Parameters[​](#parameters "Direct link to Parameters")

### x[​](#x "Direct link to x")

`bigint`

## Returns[​](#returns "Direct link to Returns")

`bigint`
