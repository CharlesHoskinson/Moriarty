# jubjubSchnorrVerifyingKey

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / jubjubSchnorrVerifyingKey

# Function: jubjubSchnorrVerifyingKey()

```
function jubjubSchnorrVerifyingKey(signingKey): JubjubPoint;
```

Derives the Schnorr verifying key (public key) from a signing key.

Equivalent to [ecMulGenerator](/api-reference/compact-runtime/functions/ecMulGenerator.md)(signingKey).

## Parameters[​](#parameters "Direct link to Parameters")

### signingKey[​](#signingkey "Direct link to signingKey")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`JubjubPoint`](/api-reference/compact-runtime/interfaces/JubjubPoint.md)
