# UnprovenInput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / UnprovenInput

# Class: UnprovenInput

A [Input](/api-reference/zswap/classes/Input.md), before being proven

All "shielded" information in the input can still be extracted at this stage!

## Properties[​](#properties "Direct link to Properties")

### contractAddress[​](#contractaddress "Direct link to contractAddress")

```
readonly contractAddress: undefined | string;
```

The contract address receiving the input, if the sender is a contract

***

### nullifier[​](#nullifier "Direct link to nullifier")

```
readonly nullifier: string;
```

The nullifier of the input

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
static deserialize(raw, netid): UnprovenInput
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`UnprovenInput`](/api-reference/zswap/classes/UnprovenInput.md)

***

### newContractOwned()[​](#newcontractowned "Direct link to newContractOwned()")

```
static newContractOwned(

   coin, 

   segment, 

   contract, 

   state): UnprovenInput
```

Creates a new input, spending a specific coin from a smart contract, against a state which contains this coin.

Note that inputs created in this way *also* need to be authorized by the contract

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### coin[​](#coin "Direct link to coin")

[`QualifiedCoinInfo`](/api-reference/zswap/type-aliases/QualifiedCoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`number`

##### contract[​](#contract "Direct link to contract")

`string`

##### state[​](#state "Direct link to state")

[`ZswapChainState`](/api-reference/zswap/classes/ZswapChainState.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenInput`](/api-reference/zswap/classes/UnprovenInput.md)
