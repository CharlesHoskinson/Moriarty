# UnprovenAuthorizedMint

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / UnprovenAuthorizedMint

# Class: UnprovenAuthorizedMint

A request to mint a coin, authorized by the mint's recipient, without the proof for the authorization being generated

## Properties[​](#properties "Direct link to Properties")

### coin[​](#coin "Direct link to coin")

```
readonly coin: CoinInfo;
```

The coin to be minted

***

### recipient[​](#recipient "Direct link to recipient")

```
readonly recipient: string;
```

The recipient of this mint

## Methods[​](#methods "Direct link to Methods")

### erase\_proof()[​](#erase_proof "Direct link to erase_proof()")

```
erase_proof(): ProofErasedAuthorizedMint
```

#### Returns[​](#returns "Direct link to Returns")

[`ProofErasedAuthorizedMint`](/api-reference/zswap/classes/ProofErasedAuthorizedMint.md)

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): UnprovenAuthorizedMint
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenAuthorizedMint`](/api-reference/zswap/classes/UnprovenAuthorizedMint.md)
