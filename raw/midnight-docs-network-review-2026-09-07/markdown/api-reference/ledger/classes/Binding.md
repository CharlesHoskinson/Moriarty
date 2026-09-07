# Binding

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / Binding

# Class: Binding

A Fiat-Shamir proof of exponent binding (or ephemerally signing) an [Intent](/api-reference/ledger/classes/Intent.md).

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new Binding(data): Binding;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### data[​](#data "Direct link to data")

`String`

#### Returns[​](#returns "Direct link to Returns")

`Binding`

## Properties[​](#properties "Direct link to Properties")

### instance[​](#instance "Direct link to instance")

```
instance: "binding";
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
static deserialize(raw): Binding;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`Binding`
