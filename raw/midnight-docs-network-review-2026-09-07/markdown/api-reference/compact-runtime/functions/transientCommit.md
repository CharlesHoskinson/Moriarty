# transientCommit

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / transientCommit

# Function: transientCommit()

```
function transientCommit<A>(

   rtType, 

   value, 

   opening): bigint;
```

The Compact builtin `transientCommit` function

This function is a circuit-efficient commitment function from arbitrary values representable in Compact, and a field element commitment opening, to field elements, which is not guaranteed to persist between upgrades. It should not be used to derive state data, but can be used for consistency checks.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

## Parameters[​](#parameters "Direct link to Parameters")

### rtType[​](#rttype "Direct link to rtType")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<`A`>

### value[​](#value "Direct link to value")

`A`

### opening[​](#opening "Direct link to opening")

`bigint`

## Returns[​](#returns "Direct link to Returns")

`bigint`

## Throws[​](#throws "Direct link to Throws")

If `opening` is out of range for field elements
