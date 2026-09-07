# constructorContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.9.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / constructorContext

# Function: constructorContext()

```
function constructorContext<T>(initialPrivateState, coinPublicKey): ConstructorContext<T>;
```

Creates a new [ConstructorContext](/api-reference/compact-runtime/interfaces/ConstructorContext.md) with the given initial private state and an empty Zswap local state.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### T[​](#t "Direct link to T")

`T`

## Parameters[​](#parameters "Direct link to Parameters")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

`T`

The private state to use to execute the contract's constructor.

### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

`string`

The Zswap coin public key of the user executing the contract.

## Returns[​](#returns "Direct link to Returns")

[`ConstructorContext`](/api-reference/compact-runtime/interfaces/ConstructorContext.md)<`T`>
