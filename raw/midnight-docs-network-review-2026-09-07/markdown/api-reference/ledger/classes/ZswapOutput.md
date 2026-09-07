# ZswapOutput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ZswapOutput

# Class: ZswapOutput\<P>

A shielded transaction output

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

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

***

### proof[​](#proof "Direct link to proof")

```
readonly proof: P;
```

The proof of this output

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
static deserialize<P>(markerP, raw): ZswapOutput<P>;
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

`ZswapOutput`<`P`>

***

### new()[​](#new "Direct link to new()")

```
static new(

   coin, 

   segment, 

   target_cpk, 

   target_epk): UnprovenOutput;
```

Creates a new output, targeted to a user's coin public key.

Optionally the output contains a ciphertext encrypted to the user's encryption public key, which may be omitted *only* if the [ShieldedCoinInfo](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md) is transferred to the recipient another way

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### coin[​](#coin "Direct link to coin")

[`ShieldedCoinInfo`](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md)

##### segment[​](#segment "Direct link to segment")

`undefined` | `number`

##### target\_cpk[​](#target_cpk "Direct link to target_cpk")

`string`

##### target\_epk[​](#target_epk "Direct link to target_epk")

`string`

#### Returns[​](#returns-3 "Direct link to Returns")

[`UnprovenOutput`](/api-reference/ledger/type-aliases/UnprovenOutput.md)

***

### newContractOwned()[​](#newcontractowned "Direct link to newContractOwned()")

```
static newContractOwned(

   coin, 

   segment, 

   contract): UnprovenOutput;
```

Creates a new output, targeted to a smart contract

A contract must *also* explicitly receive a coin created in this way for the output to be valid

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### coin[​](#coin-1 "Direct link to coin")

[`ShieldedCoinInfo`](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md)

##### segment[​](#segment-1 "Direct link to segment")

`undefined` | `number`

##### contract[​](#contract "Direct link to contract")

`string`

#### Returns[​](#returns-4 "Direct link to Returns")

[`UnprovenOutput`](/api-reference/ledger/type-aliases/UnprovenOutput.md)
