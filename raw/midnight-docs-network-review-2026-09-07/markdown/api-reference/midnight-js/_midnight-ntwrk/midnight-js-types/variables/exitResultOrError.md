# exitResultOrError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / exitResultOrError

# Variable: exitResultOrError

> `const` **exitResultOrError**: <`A`, `E`>(`exit`) => `A`

Unwraps an Effect `Exit` instance, returning its value if it is successful, or throwing the error contained within it.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

### E[​](#e "Direct link to E")

`E`

## Parameters[​](#parameters "Direct link to Parameters")

### exit[​](#exit "Direct link to exit")

`Exit.Exit`<`A`, `E`>

The source Effect `Exit` instance.

## Returns[​](#returns "Direct link to Returns")

`A`

The value from `exit` if it is successful, otherwise throws the error contained within it.
