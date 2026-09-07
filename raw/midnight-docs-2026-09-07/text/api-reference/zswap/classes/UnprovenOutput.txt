# UnprovenOutput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / UnprovenOutput

# Class: UnprovenOutput

An [Output](/api-reference/zswap/classes/Output.md) before being proven

All "shielded" information in the output can still be extracted at this stage!

## Properties[​](#properties "Direct link to Properties")

### commitment[​](#commitment "Direct link to commitment")

```
readonly commitment: string;
```

The commitment of the output

***

### contractAddress[​](#contractaddress "Direct link to contractAddress")

```
readonly contractAddress: undefined | string;
```

The contract address receiving the output, if the recipient is a contract

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): UnprovenOutput
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`UnprovenOutput`](/api-reference/zswap/classes/UnprovenOutput.md)

***

### new()[​](#new "Direct link to new()")

```
static new(

   coin, 

   segment, 

   target_cpk, 

   target_epk): UnprovenOutput
```

Creates a new output, targeted to a user's coin public key.

Optionally the output contains a ciphertext encrypted to the user's encryption public key, which may be omitted *only* if the [CoinInfo](/api-reference/zswap/type-aliases/CoinInfo.md) is transferred to the recipient another way

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### coin[​](#coin "Direct link to coin")

[`CoinInfo`](/api-reference/zswap/type-aliases/CoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`number`

##### target\_cpk[​](#target_cpk "Direct link to target_cpk")

`string`

##### target\_epk[​](#target_epk "Direct link to target_epk")

`string`

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenOutput`](/api-reference/zswap/classes/UnprovenOutput.md)

***

### newContractOwned()[​](#newcontractowned "Direct link to newContractOwned()")

```
static newContractOwned(

   coin, 

   segment, 

   contract): UnprovenOutput
```

Creates a new output, targeted to a smart contract

A contract must *also* explicitly receive a coin created in this way for the output to be valid

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### coin[​](#coin-1 "Direct link to coin")

[`CoinInfo`](/api-reference/zswap/type-aliases/CoinInfo.md)

##### segment[​](#segment-1 "Direct link to segment")

`number`

##### contract[​](#contract "Direct link to contract")

`string`

#### Returns[​](#returns-4 "Direct link to Returns")

[`UnprovenOutput`](/api-reference/zswap/classes/UnprovenOutput.md)
