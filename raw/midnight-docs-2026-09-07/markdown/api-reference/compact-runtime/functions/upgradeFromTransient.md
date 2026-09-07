# upgradeFromTransient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / upgradeFromTransient

# Function: upgradeFromTransient()

```
function upgradeFromTransient(x): Uint8Array;
```

The Compact builtin `upgradeFromTransient` function

This function "upgrades" the output of a [transientHash](/api-reference/compact-runtime/functions/transientHash.md) or [transientCommit](/api-reference/compact-runtime/functions/transientCommit.md) to 256-bit byte string, which can then be used in [persistentHash](/api-reference/compact-runtime/functions/persistentHash.md) or [persistentCommit](/api-reference/compact-runtime/functions/persistentCommit.md).

## Parameters[​](#parameters "Direct link to Parameters")

### x[​](#x "Direct link to x")

`bigint`

## Returns[​](#returns "Direct link to Returns")

`Uint8Array`

## Throws[​](#throws "Direct link to Throws")

If `x` is not a valid field element
