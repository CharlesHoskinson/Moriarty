# VerifierKeyRemove

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / VerifierKeyRemove

# Class: VerifierKeyRemove

An update instruction to remove a verifier key of a specific operation and version.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new VerifierKeyRemove(operation, version): VerifierKeyRemove;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### operation[​](#operation "Direct link to operation")

`string` | `Uint8Array`<`ArrayBufferLike`>

##### version[​](#version "Direct link to version")

[`ContractOperationVersion`](/api-reference/ledger/classes/ContractOperationVersion.md)

#### Returns[​](#returns "Direct link to Returns")

`VerifierKeyRemove`

## Properties[​](#properties "Direct link to Properties")

### operation[​](#operation-1 "Direct link to operation")

```
readonly operation: string | Uint8Array<ArrayBufferLike>;
```

***

### version[​](#version-1 "Direct link to version")

```
readonly version: ContractOperationVersion;
```

## Methods[​](#methods "Direct link to Methods")

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`
