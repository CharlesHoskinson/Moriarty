# entryPointHash

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / entryPointHash

# Function: entryPointHash()

```
function entryPointHash(entryPoint): string;
```

Computes the (hex-encoded) hash of a given contract entry point. Used in composable contracts to reference the called contract's entry point ID in-circuit.

## Parameters[​](#parameters "Direct link to Parameters")

### entryPoint[​](#entrypoint "Direct link to entryPoint")

`string` | `Uint8Array`<`ArrayBufferLike`>

## Returns[​](#returns "Direct link to Returns")

`string`
