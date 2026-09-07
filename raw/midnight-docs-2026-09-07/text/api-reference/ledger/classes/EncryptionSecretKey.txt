# EncryptionSecretKey

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / EncryptionSecretKey

# Class: EncryptionSecretKey

Holds the encryption secret key of a user, which may be used to determine if a given offer contains outputs addressed to this user

## Methods[​](#methods "Direct link to Methods")

### clear()[​](#clear "Direct link to clear()")

```
clear(): void;
```

Clears the encryption secret key, so that it is no longer usable nor held in memory

#### Returns[​](#returns "Direct link to Returns")

`void`

***

### test()[​](#test "Direct link to test()")

```
test<P>(offer): boolean;
```

#### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

##### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters "Direct link to Parameters")

##### offer[​](#offer "Direct link to offer")

[`ZswapOffer`](/api-reference/ledger/classes/ZswapOffer.md)<`P`>

#### Returns[​](#returns-1 "Direct link to Returns")

`boolean`

***

### yesIKnowTheSecurityImplicationsOfThis\_serialize()[​](#yesiknowthesecurityimplicationsofthis_serialize "Direct link to yesIKnowTheSecurityImplicationsOfThis_serialize()")

```
yesIKnowTheSecurityImplicationsOfThis_serialize(): Uint8Array;
```

#### Returns[​](#returns-2 "Direct link to Returns")

`Uint8Array`

***

### yesIKnowTheSecurityImplicationsOfThis\_taggedSerialize()[​](#yesiknowthesecurityimplicationsofthis_taggedserialize "Direct link to yesIKnowTheSecurityImplicationsOfThis_taggedSerialize()")

```
yesIKnowTheSecurityImplicationsOfThis_taggedSerialize(): Uint8Array;
```

#### Returns[​](#returns-3 "Direct link to Returns")

`Uint8Array`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): EncryptionSecretKey;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-4 "Direct link to Returns")

`EncryptionSecretKey`

***

### taggedDeserialize()[​](#taggeddeserialize "Direct link to taggedDeserialize()")

```
static taggedDeserialize(raw): EncryptionSecretKey;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw-1 "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-5 "Direct link to Returns")

`EncryptionSecretKey`
