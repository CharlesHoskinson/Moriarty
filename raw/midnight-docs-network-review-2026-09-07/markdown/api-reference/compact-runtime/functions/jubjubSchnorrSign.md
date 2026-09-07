# jubjubSchnorrSign

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / jubjubSchnorrSign

# Function: jubjubSchnorrSign()

```
function jubjubSchnorrSign<A>(

   rtType, 

   msg, 

   signingKey): JubjubSchnorrSignature;
```

Produces a Schnorr signature over the JubJub curve.

* `rtType` / `msg`: the message as a typed Compact value
* `sk`: signing key as a JubJub scalar (e.g. as returned by [jubjubSampleScalar](/api-reference/compact-runtime/functions/jubjubSampleScalar.md))

The signature scheme:

* Nonce `r` sampled uniformly at random
* Announcement `R = r·G`
* Challenge `c = PoseidonHash(R.x, R.y, pk.x, pk.y, msg...)`
* Response `s = r + c·sk` (in the JubJub scalar field)

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

## Parameters[​](#parameters "Direct link to Parameters")

### rtType[​](#rttype "Direct link to rtType")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<`A`>

### msg[​](#msg "Direct link to msg")

`A`

### signingKey[​](#signingkey "Direct link to signingKey")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`JubjubSchnorrSignature`](/api-reference/compact-runtime/interfaces/JubjubSchnorrSignature.md)
