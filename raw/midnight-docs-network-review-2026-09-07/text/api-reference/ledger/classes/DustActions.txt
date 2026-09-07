# DustActions

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / DustActions

# Class: DustActions\<S, P>

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### S[​](#s "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

### P[​](#p "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new DustActions<S, P>(

   markerS, 

   markerP, 

   ctime, 

   spends?, 

registrations?): DustActions<S, P>;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### markerS[​](#markers "Direct link to markerS")

`S`\[`"instance"`]

##### markerP[​](#markerp "Direct link to markerP")

`P`\[`"instance"`]

##### ctime[​](#ctime "Direct link to ctime")

`Date`

##### spends?[​](#spends "Direct link to spends?")

[`DustSpend`](/api-reference/ledger/classes/DustSpend.md)<`P`>\[]

##### registrations?[​](#registrations "Direct link to registrations?")

[`DustRegistration`](/api-reference/ledger/classes/DustRegistration.md)<`S`>\[]

#### Returns[​](#returns "Direct link to Returns")

`DustActions`<`S`, `P`>

## Properties[​](#properties "Direct link to Properties")

### ctime[​](#ctime-1 "Direct link to ctime")

```
ctime: Date;
```

***

### registrations[​](#registrations-1 "Direct link to registrations")

```
registrations: DustRegistration<S>[];
```

***

### spends[​](#spends-1 "Direct link to spends")

```
spends: DustSpend<P>[];
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
static deserialize<S, P>(

   markerS, 

   markerP, 

raw): DustActions<S, P>;
```

#### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

##### S[​](#s-1 "Direct link to S")

`S` *extends* [`Signaturish`](/api-reference/ledger/type-aliases/Signaturish.md)

##### P[​](#p-1 "Direct link to P")

`P` *extends* [`Proofish`](/api-reference/ledger/type-aliases/Proofish.md)

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### markerS[​](#markers-1 "Direct link to markerS")

`S`\[`"instance"`]

##### markerP[​](#markerp-1 "Direct link to markerP")

`P`\[`"instance"`]

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`DustActions`<`S`, `P`>
