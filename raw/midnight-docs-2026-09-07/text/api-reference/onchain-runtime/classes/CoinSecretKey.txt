# CoinSecretKey

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / CoinSecretKey

# Class: CoinSecretKey

Holds the coin secret key of a user, serialized as a hex-encoded 32-byte string

## Methods[​](#methods "Direct link to Methods")

### clear()[​](#clear "Direct link to clear()")

```
clear(): void
```

Clears the coin secret key, so that it is no longer usable nor held in memory

#### Returns[​](#returns "Direct link to Returns")

`void`

***

### yesIKnowTheSecurityImplicationsOfThis\_serialize()[​](#yesiknowthesecurityimplicationsofthis_serialize "Direct link to yesIKnowTheSecurityImplicationsOfThis_serialize()")

```
yesIKnowTheSecurityImplicationsOfThis_serialize(): Uint8Array
```

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): CoinSecretKey
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

[`CoinSecretKey`](/api-reference/onchain-runtime/classes/CoinSecretKey.md)
