# keccak256

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / keccak256

# Function: keccak256()

```
function keccak256<A>(rtType, value): Uint8Array;
```

The Compact builtin `keccak256` function

Hashes `value` using Keccak-256 and returns the 32-byte digest.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

## Parameters[​](#parameters "Direct link to Parameters")

### rtType[​](#rttype "Direct link to rtType")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<`A`>

### value[​](#value "Direct link to value")

`A`

## Returns[​](#returns "Direct link to Returns")

`Uint8Array`

## Throws[​](#throws "Direct link to Throws")

If `rtType` encodes a type containing Compact 'Opaque' types
