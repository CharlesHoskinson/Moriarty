# UnprovenTransient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / UnprovenTransient

# Class: UnprovenTransient

A [Transient](/api-reference/zswap/classes/Transient.md), before being proven

All "shielded" information in the transient can still be extracted at this stage!

## Properties[​](#properties "Direct link to Properties")

### commitment[​](#commitment "Direct link to commitment")

```
readonly commitment: string;
```

The commitment of the transient

***

### contractAddress[​](#contractaddress "Direct link to contractAddress")

```
readonly contractAddress: undefined | string;
```

The contract address creating the transient, if applicable

***

### nullifier[​](#nullifier "Direct link to nullifier")

```
readonly nullifier: string;
```

The nullifier of the transient

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
static deserialize(raw, netid): UnprovenTransient
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`UnprovenTransient`](/api-reference/zswap/classes/UnprovenTransient.md)

***

### newFromContractOwnedOutput()[​](#newfromcontractownedoutput "Direct link to newFromContractOwnedOutput()")

```
static newFromContractOwnedOutput(

   coin, 

   segment, 

   output): UnprovenTransient
```

Creates a new contract-owned transient, from a given output and its coin.

The [QualifiedCoinInfo](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md) should have an `mt_index` of `0`

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### coin[​](#coin "Direct link to coin")

[`QualifiedCoinInfo`](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`number`

##### output[​](#output "Direct link to output")

[`UnprovenOutput`](/api-reference/zswap/classes/UnprovenOutput.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenTransient`](/api-reference/zswap/classes/UnprovenTransient.md)
