# ProofErasedOutput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / ProofErasedOutput

# Class: ProofErasedOutput

An [Output](/api-reference/zswap/classes/Output.md) with all proof information erased

Primarily for use in testing, or handling data known to be correct from external information

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
static deserialize(raw, netid): ProofErasedOutput
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`ProofErasedOutput`](/api-reference/zswap/classes/ProofErasedOutput.md)
