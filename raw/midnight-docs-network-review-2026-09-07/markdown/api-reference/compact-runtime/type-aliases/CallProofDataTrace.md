# CallProofDataTrace

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / CallProofDataTrace

# Type Alias: CallProofDataTrace

```
type CallProofDataTrace = CallProofData[];
```

List of data needed to construct proofs and transactions for all circuit calls resulting from executing a root circuit. The calls are in depth-first traversal order. In other words, the first circuit to complete execution is first, and the last circuit to complete execution (the root circuit) is last.
