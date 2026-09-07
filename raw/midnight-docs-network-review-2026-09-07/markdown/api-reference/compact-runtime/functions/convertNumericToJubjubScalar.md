# convertNumericToJubjubScalar

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / convertNumericToJubjubScalar

# Function: convertNumericToJubjubScalar()

```
function convertNumericToJubjubScalar(x): bigint;
```

Conversion of a native field or unsigned integer value to a JubjubScalar

The native field is BLS12-381 scalar, which has a larger field modulus than the Jubjub scalar field. The value is converted modulo the Jubjub scalar field modulus.

## Parameters[​](#parameters "Direct link to Parameters")

### x[​](#x "Direct link to x")

`bigint`

## Returns[​](#returns "Direct link to Returns")

`bigint`
