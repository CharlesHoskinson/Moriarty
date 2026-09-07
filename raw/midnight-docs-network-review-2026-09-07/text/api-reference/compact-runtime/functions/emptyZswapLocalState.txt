# emptyZswapLocalState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / emptyZswapLocalState

# Function: emptyZswapLocalState()

```
function emptyZswapLocalState(coinPublicKey): EncodedZswapLocalState;
```

Constructs a new [EncodedZswapLocalState](/api-reference/compact-runtime/interfaces/EncodedZswapLocalState.md) with the given coin public key. The result can be used to create a [ConstructorContext](/api-reference/compact-runtime/interfaces/ConstructorContext.md).

## Parameters[​](#parameters "Direct link to Parameters")

### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

The Zswap coin public key of the user executing the circuit.

`string` | [`EncodedCoinPublicKey`](/api-reference/compact-runtime/interfaces/EncodedCoinPublicKey.md)

## Returns[​](#returns "Direct link to Returns")

[`EncodedZswapLocalState`](/api-reference/compact-runtime/interfaces/EncodedZswapLocalState.md)
