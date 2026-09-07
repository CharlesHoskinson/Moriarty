# SecretKeys

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / SecretKeys

# Class: SecretKeys

## Properties[​](#properties "Direct link to Properties")

### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

```
readonly coinPublicKey: string;
```

***

### coinSecretKey[​](#coinsecretkey "Direct link to coinSecretKey")

```
readonly coinSecretKey: CoinSecretKey;
```

***

### encryptionPublicKey[​](#encryptionpublickey "Direct link to encryptionPublicKey")

```
readonly encryptionPublicKey: string;
```

***

### encryptionSecretKey[​](#encryptionsecretkey "Direct link to encryptionSecretKey")

```
readonly encryptionSecretKey: EncryptionSecretKey;
```

## Methods[​](#methods "Direct link to Methods")

### fromSeed()[​](#fromseed "Direct link to fromSeed()")

```
static fromSeed(seed): SecretKeys
```

Derives secret keys from a 32-byte seed

#### Parameters[​](#parameters "Direct link to Parameters")

##### seed[​](#seed "Direct link to seed")

`Uint8Array`<`ArrayBufferLike`>

#### Returns[​](#returns "Direct link to Returns")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)

***

### fromSeedRng()[​](#fromseedrng "Direct link to fromSeedRng()")

```
static fromSeedRng(seed): SecretKeys
```

Derives secret keys from a 32-byte seed using deprecated implementation. Use only for compatibility purposes

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### seed[​](#seed-1 "Direct link to seed")

`Uint8Array`<`ArrayBufferLike`>

#### Returns[​](#returns-1 "Direct link to Returns")

[`SecretKeys`](/api-reference/zswap/classes/SecretKeys.md)
