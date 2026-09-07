# hasCoinCommitment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / hasCoinCommitment

# Function: hasCoinCommitment()

```
function hasCoinCommitment(

   context, 

   coinInfo, 

   recipient): boolean;
```

Checks whether a coin commitment has already been added to the current query context.

## Parameters[​](#parameters "Direct link to Parameters")

### context[​](#context "Direct link to context")

[`CircuitContext`](/api-reference/compact-runtime/interfaces/CircuitContext.md)

The current circuit context.

### coinInfo[​](#coininfo "Direct link to coinInfo")

[`EncodedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md)

The coin information to check.

### recipient[​](#recipient "Direct link to recipient")

[`EncodedRecipient`](/api-reference/compact-runtime/interfaces/EncodedRecipient.md)

The coin recipient to check.

## Returns[​](#returns "Direct link to Returns")

`boolean`
