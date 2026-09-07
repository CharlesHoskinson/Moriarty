# ZswapTransient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ZswapTransient

# Class: ZswapTransient\<P>

A shielded "transient"; an output that is immediately spent within the same transaction

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

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

### inputProof[​](#inputproof "Direct link to inputProof")

```
readonly inputProof: P;
```

The input proof of this transient

***

### nullifier[​](#nullifier "Direct link to nullifier")

```
readonly nullifier: string;
```

The nullifier of the transient

***

### outputProof[​](#outputproof "Direct link to outputProof")

```
readonly outputProof: P;
```

The output proof of this transient

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
static deserialize<P>(markerP, raw): ZswapTransient<P>;
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

`ZswapTransient`<`P`>

***

### newFromContractOwnedOutput()[​](#newfromcontractownedoutput "Direct link to newFromContractOwnedOutput()")

```
static newFromContractOwnedOutput(

   coin, 

   segment, 

   output): UnprovenTransient;
```

Creates a new contract-owned transient, from a given output and its coin.

The [QualifiedShieldedCoinInfo](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md) should have an `mt_index` of `0`

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### coin[​](#coin "Direct link to coin")

[`QualifiedShieldedCoinInfo`](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`undefined` | `number`

##### output[​](#output "Direct link to output")

[`UnprovenOutput`](/api-reference/ledger/type-aliases/UnprovenOutput.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenTransient`](/api-reference/ledger/type-aliases/UnprovenTransient.md)
