# ContractReferenceLocations

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ContractReferenceLocations

# Type Alias: ContractReferenceLocations

```
type ContractReferenceLocations = 

  | EmptyPublicLedger

  | PublicLedgerSegments;
```

A data structure indicating the locations of all contract references in a given ledger state. If it is a [EmptyPublicLedger](/api-reference/compact-runtime/type-aliases/EmptyPublicLedger.md), then no contract references are present in the ledger state. If it is a [PublicLedgerSegments](/api-reference/compact-runtime/type-aliases/PublicLedgerSegments.md), then contract references are present and can be extracted using [contractDependencies](/api-reference/compact-runtime/functions/contractDependencies.md).
