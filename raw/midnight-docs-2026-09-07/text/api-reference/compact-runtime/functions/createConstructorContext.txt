# createConstructorContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / createConstructorContext

# Function: createConstructorContext()

```
function createConstructorContext<PS>(initialPrivateState, coinPublicKey): ConstructorContext<PS>;
```

Creates a new [ConstructorContext](/api-reference/compact-runtime/interfaces/ConstructorContext.md) with the given initial private state and an empty Zswap local state.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS`

## Parameters[​](#parameters "Direct link to Parameters")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

`PS`

The private state to use to execute the contract's constructor.

### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

The Zswap coin public key of the user executing the contract.

`string` | [`EncodedCoinPublicKey`](/api-reference/compact-runtime/interfaces/EncodedCoinPublicKey.md)

## Returns[​](#returns "Direct link to Returns")

[`ConstructorContext`](/api-reference/compact-runtime/interfaces/ConstructorContext.md)<`PS`>
