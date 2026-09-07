# EncryptionSecretKey

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / EncryptionSecretKey

# Class: EncryptionSecretKey

Holds the encryption secret key of a user, which may be used to determine if a given offer contains outputs addressed to this user

## Methods[​](#methods "Direct link to Methods")

### test()[​](#test "Direct link to test()")

```
test(offer): boolean
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### offer[​](#offer "Direct link to offer")

[`Offer`](/api-reference/zswap/classes/Offer.md)

#### Returns[​](#returns "Direct link to Returns")

`boolean`

***

### yesIKnowTheSecurityImplicationsOfThis\_serialize()[​](#yesiknowthesecurityimplicationsofthis_serialize "Direct link to yesIKnowTheSecurityImplicationsOfThis_serialize()")

```
yesIKnowTheSecurityImplicationsOfThis_serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): EncryptionSecretKey
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`EncryptionSecretKey`](/api-reference/zswap/classes/EncryptionSecretKey.md)
