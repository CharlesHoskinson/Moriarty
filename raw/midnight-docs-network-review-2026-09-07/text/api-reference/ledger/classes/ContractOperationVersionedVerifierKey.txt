# ContractOperationVersionedVerifierKey

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ContractOperationVersionedVerifierKey

# Class: ContractOperationVersionedVerifierKey

A versioned verifier key to be associated with a [ContractOperation](/api-reference/ledger/classes/ContractOperation.md).

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ContractOperationVersionedVerifierKey(version, rawVk): ContractOperationVersionedVerifierKey;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### version[​](#version "Direct link to version")

`"v3"`

##### rawVk[​](#rawvk "Direct link to rawVk")

`Uint8Array`

#### Returns[​](#returns "Direct link to Returns")

`ContractOperationVersionedVerifierKey`

## Properties[​](#properties "Direct link to Properties")

### rawVk[​](#rawvk-1 "Direct link to rawVk")

```
readonly rawVk: Uint8Array;
```

***

### version[​](#version-1 "Direct link to version")

```
readonly version: "v3";
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
