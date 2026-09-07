# persistentCommit

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / persistentCommit

# Function: persistentCommit()

```
function persistentCommit<A>(

   rtType, 

   value, 

   opening): Uint8Array;
```

The Compact builtin `persistentCommit` function

This function is a non-circuit-optimised commitment function from arbitrary values representable in Compact, and a 256-bit bytestring opening, to a 256-bit bytestring. It is guaranteed to persist between upgrades. It *should* be used to derive state data, and not for consistency checks where avoidable.

Note that data containing `Opaque` elements *may* throw runtime errors, and cannot be relied upon as a consistent representation.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

## Parameters[​](#parameters "Direct link to Parameters")

### rtType[​](#rttype "Direct link to rtType")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<`A`>

### value[​](#value "Direct link to value")

`A`

### opening[​](#opening "Direct link to opening")

`Uint8Array`

## Returns[​](#returns "Direct link to Returns")

`Uint8Array`

## Throws[​](#throws "Direct link to Throws")

If `rtType` encodes a type containing Compact 'Opaque' types, or `opening` is not 32 bytes long
