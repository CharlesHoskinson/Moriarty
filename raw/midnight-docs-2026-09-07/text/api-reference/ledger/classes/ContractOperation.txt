# ContractOperation

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ContractOperation

# Class: ContractOperation

An individual operation, or entry point of a contract, consisting primarily of a ZK verifier keys, potentially for different versions of the proving system.

Only the latest available version is exposed to this API.

Note that the serialized form of the key is checked on initialization

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ContractOperation(): ContractOperation;
```

#### Returns[​](#returns "Direct link to Returns")

`ContractOperation`

## Properties[​](#properties "Direct link to Properties")

### verifierKey[​](#verifierkey "Direct link to verifierKey")

```
verifierKey: Uint8Array;
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

#### Parameters[​](#parameters "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): ContractOperation;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`ContractOperation`
