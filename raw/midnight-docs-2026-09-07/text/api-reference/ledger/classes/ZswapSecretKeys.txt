# ZswapSecretKeys

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ZswapSecretKeys

# Class: ZswapSecretKeys

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

### clear()[​](#clear "Direct link to clear()")

```
clear(): void;
```

Clears the secret keys, so that they are no longer usable nor held in memory Note: it does not clear copies of the keys - which is particularly relevant for proof preimages Note: this will cause all other operations to fail

#### Returns[​](#returns "Direct link to Returns")

`void`

***

### fromSeed()[​](#fromseed "Direct link to fromSeed()")

```
static fromSeed(seed): ZswapSecretKeys;
```

Derives secret keys from a 32-byte seed

#### Parameters[​](#parameters "Direct link to Parameters")

##### seed[​](#seed "Direct link to seed")

`Uint8Array`

#### Returns[​](#returns-1 "Direct link to Returns")

`ZswapSecretKeys`

***

### fromSeedRng()[​](#fromseedrng "Direct link to fromSeedRng()")

```
static fromSeedRng(seed): ZswapSecretKeys;
```

Derives secret keys from a 32-byte seed using deprecated implementation. Use only for compatibility purposes

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### seed[​](#seed-1 "Direct link to seed")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

`ZswapSecretKeys`
