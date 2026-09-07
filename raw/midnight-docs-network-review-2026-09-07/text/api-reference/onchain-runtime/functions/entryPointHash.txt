# entryPointHash

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / entryPointHash

# Function: entryPointHash()

```
function entryPointHash(entryPoint): string
```

Computes the (hex-encoded) hash of a given contract entry point. Used in composable contracts to reference the called contract's entry point ID in-circuit.

## Parameters[​](#parameters "Direct link to Parameters")

### entryPoint[​](#entrypoint "Direct link to entryPoint")

`string` | `Uint8Array`<`ArrayBufferLike`>

## Returns[​](#returns "Direct link to Returns")

`string`
