# decodeZswapLocalState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / decodeZswapLocalState

# Function: decodeZswapLocalState()

```
function decodeZswapLocalState(state): ZswapLocalState;
```

Converts an [EncodedZswapLocalState](/api-reference/compact-runtime/interfaces/EncodedZswapLocalState.md) to a [ZswapLocalState](/api-reference/compact-runtime/interfaces/ZswapLocalState.md). Used when we need to use data from contract execution to construct transactions.

## Parameters[​](#parameters "Direct link to Parameters")

### state[​](#state "Direct link to state")

[`EncodedZswapLocalState`](/api-reference/compact-runtime/interfaces/EncodedZswapLocalState.md)

The encoded Zswap local state.

## Returns[​](#returns "Direct link to Returns")

[`ZswapLocalState`](/api-reference/compact-runtime/interfaces/ZswapLocalState.md)
