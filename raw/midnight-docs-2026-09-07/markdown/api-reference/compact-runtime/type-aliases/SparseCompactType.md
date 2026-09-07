# SparseCompactType

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactType

# Type Alias: SparseCompactType

```
type SparseCompactType = 

  | SparseCompactVector

  | SparseCompactStruct

  | SparseCompactContractAddress;
```

A data structure indicating the locations of contract references in a Compact struct, vector, or (the terminating case) a contract address.
