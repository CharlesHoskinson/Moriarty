# ZswapInput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ZswapInput

# Class: ZswapInput\<P>

A shielded transaction input

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

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

***

### proof[​](#proof "Direct link to proof")

```
readonly proof: P;
```

The proof of this input

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize<P>(markerP, raw): ZswapInput<P>;
```

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### P[​](#p-1 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### markerP[​](#markerp "Direct link to markerP")

`P`\[`"instance"`]

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-2 "Direct link to Returns")

`ZswapInput`<`P`>

***

### newContractOwned()[​](#newcontractowned "Direct link to newContractOwned()")

```
static newContractOwned(

   coin, 

   segment, 

   contract, 

   state): UnprovenInput;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### coin[​](#coin "Direct link to coin")

[`QualifiedShieldedCoinInfo`](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`undefined` | `number`

##### contract[​](#contract "Direct link to contract")

`string`

##### state[​](#state "Direct link to state")

[`ZswapChainState`](/api-reference/ledger/classes/ZswapChainState.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenInput`](/api-reference/ledger/type-aliases/UnprovenInput.md)
