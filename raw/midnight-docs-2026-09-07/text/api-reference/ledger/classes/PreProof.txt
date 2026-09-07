# PreProof

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / PreProof

# Class: PreProof

The preimage, or data required to produce, a [Proof](/api-reference/ledger/classes/Proof.md).

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new PreProof(data): PreProof;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### data[​](#data "Direct link to data")

`String`

#### Returns[​](#returns "Direct link to Returns")

`PreProof`

## Properties[​](#properties "Direct link to Properties")

### instance[​](#instance "Direct link to instance")

```
instance: "pre-proof";
```

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): PreProof;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`PreProof`
