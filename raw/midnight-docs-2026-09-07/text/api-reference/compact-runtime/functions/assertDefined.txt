# assertDefined

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / assertDefined

# Function: assertDefined()

```
function assertDefined<T>(t, name): asserts t is NonNullable<T>;
```

**`Internal`**

Compiler internal for asserting an object is non-nullable.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### T[​](#t "Direct link to T")

`T`

## Parameters[​](#parameters "Direct link to Parameters")

### t[​](#t-1 "Direct link to t")

`T` | `undefined`

### name[​](#name "Direct link to name")

`string`

## Returns[​](#returns "Direct link to Returns")

`asserts t is NonNullable<T>`
