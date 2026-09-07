# DustSecretKey

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / DustSecretKey

# Class: DustSecretKey

A secret key for the Dust, used to derive Dust UTxO nonces and prove credentials to spend Dust UTxOs

## Properties[​](#properties "Direct link to Properties")

### publicKey[​](#publickey "Direct link to publicKey")

```
publicKey: bigint;
```

## Methods[​](#methods "Direct link to Methods")

### clear()[​](#clear "Direct link to clear()")

```
clear(): void;
```

Clears the dust secret key, so that it is no longer usable nor held in memory

#### Returns[​](#returns "Direct link to Returns")

`void`

***

### fromBigint()[​](#frombigint "Direct link to fromBigint()")

```
static fromBigint(bigint): DustSecretKey;
```

Temporary method to create an instance of DustSecretKey from a bigint (its natural representation)

#### Parameters[​](#parameters "Direct link to Parameters")

##### bigint[​](#bigint "Direct link to bigint")

`bigint`

#### Returns[​](#returns-1 "Direct link to Returns")

`DustSecretKey`

***

### fromSeed()[​](#fromseed "Direct link to fromSeed()")

```
static fromSeed(seed): DustSecretKey;
```

Create an instance of DustSecretKey from a seed.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### seed[​](#seed "Direct link to seed")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

`DustSecretKey`
