# assertDefined

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / assertDefined

# Function: assertDefined()

> **assertDefined**<`A`>(`value`, `message?`): `asserts value is NonNullable<A>`

Asserts that the given value is non-nullable.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### A[​](#a "Direct link to A")

`A`

## Parameters[​](#parameters "Direct link to Parameters")

### value[​](#value "Direct link to value")

`A` | `null` | `undefined`

The value to test for nullability.

### message?[​](#message "Direct link to message?")

`string`

The error message to use if an error is thrown.

## Returns[​](#returns "Direct link to Returns")

`asserts value is NonNullable<A>`

## Throws[​](#throws "Direct link to Throws")

Error If the value is nullable.
