# persistentHash

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / persistentHash

# Function: persistentHash()

```
function persistentHash<A>(rtType, value): Uint8Array;
```

The Compact builtin `persistentHash` function

This function is a non-circuit-optimised hash function for mostly arbitrary data. It is guaranteed to persist between upgrades, with the exception of devnet. It *should* be used to derive state data, and not for consistency checks where avoidable.

Note that data containing `Opaque` elements *may* throw runtime errors, and cannot be relied upon as a consistent representation.

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
