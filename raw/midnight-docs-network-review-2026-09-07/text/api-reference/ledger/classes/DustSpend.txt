# DustSpend

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / DustSpend

# Class: DustSpend\<P>

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

## Properties[​](#properties "Direct link to Properties")

### newCommitment[​](#newcommitment "Direct link to newCommitment")

```
readonly newCommitment: bigint;
```

***

### oldNullifier[​](#oldnullifier "Direct link to oldNullifier")

```
readonly oldNullifier: bigint;
```

***

### proof[​](#proof "Direct link to proof")

```
readonly proof: P;
```

***

### vFee[​](#vfee "Direct link to vFee")

```
readonly vFee: bigint;
```

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
static deserialize<P>(markerP, raw): DustSpend<P>;
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

`DustSpend`<`P`>
