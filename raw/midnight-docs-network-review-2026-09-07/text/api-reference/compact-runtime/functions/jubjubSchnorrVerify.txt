# jubjubSchnorrVerify

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / jubjubSchnorrVerify

# Function: jubjubSchnorrVerify()

```
function jubjubSchnorrVerify<A>(

   rtType, 

   msg, 

   verifyingKey, 

   sig): boolean;
```

Verifies a Schnorr signature over the JubJub curve.

* `rtType` / `msg`: the message as a typed Compact value
* `pk`: verifying key (a JubJubPoint / EmbeddedGroupAffine)
* `sig`: signature as returned by [jubjubSchnorrSign](/api-reference/compact-runtime/functions/jubjubSchnorrSign.md)

Returns `true` if the signature is valid (i.e. `s·G == R + c·pk`).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

## Parameters[​](#parameters "Direct link to Parameters")

### rtType[​](#rttype "Direct link to rtType")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<`A`>

### msg[​](#msg "Direct link to msg")

`A`

### verifyingKey[​](#verifyingkey "Direct link to verifyingKey")

[`JubjubPoint`](/api-reference/compact-runtime/interfaces/JubjubPoint.md)

### sig[​](#sig "Direct link to sig")

[`JubjubSchnorrSignature`](/api-reference/compact-runtime/interfaces/JubjubSchnorrSignature.md)

## Returns[​](#returns "Direct link to Returns")

`boolean`
