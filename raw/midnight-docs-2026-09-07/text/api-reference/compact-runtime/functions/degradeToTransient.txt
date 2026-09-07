# degradeToTransient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / degradeToTransient

# Function: degradeToTransient()

```
function degradeToTransient(x): bigint;
```

The Compact builtin `degradeToTransient` function

This function "degrades" the output of a [persistentHash](/api-reference/compact-runtime/functions/persistentHash.md) or [persistentCommit](/api-reference/compact-runtime/functions/persistentCommit.md) to a field element, which can then be used in [transientHash](/api-reference/compact-runtime/functions/transientHash.md) or [transientCommit](/api-reference/compact-runtime/functions/transientCommit.md).

## Parameters[​](#parameters "Direct link to Parameters")

### x[​](#x "Direct link to x")

`Uint8Array`

## Returns[​](#returns "Direct link to Returns")

`bigint`

## Throws[​](#throws "Direct link to Throws")

If `x` is not 32 bytes long
