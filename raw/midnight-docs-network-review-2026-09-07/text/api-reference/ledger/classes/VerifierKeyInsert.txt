# VerifierKeyInsert

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / VerifierKeyInsert

# Class: VerifierKeyInsert

An update instruction to insert a verifier key at a specific operation and version.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new VerifierKeyInsert(operation, vk): VerifierKeyInsert;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### operation[​](#operation "Direct link to operation")

`string` | `Uint8Array`<`ArrayBufferLike`>

##### vk[​](#vk "Direct link to vk")

[`ContractOperationVersionedVerifierKey`](/api-reference/ledger/classes/ContractOperationVersionedVerifierKey.md)

#### Returns[​](#returns "Direct link to Returns")

`VerifierKeyInsert`

## Properties[​](#properties "Direct link to Properties")

### operation[​](#operation-1 "Direct link to operation")

```
readonly operation: string | Uint8Array<ArrayBufferLike>;
```

***

### vk[​](#vk-1 "Direct link to vk")

```
readonly vk: ContractOperationVersionedVerifierKey;
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
