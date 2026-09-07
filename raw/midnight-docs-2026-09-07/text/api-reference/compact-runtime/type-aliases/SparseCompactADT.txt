# SparseCompactADT

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactADT

# Type Alias: SparseCompactADT

```
type SparseCompactADT = 

  | SparseCompactCellADT

  | SparseCompactArrayLikeADT

  | SparseCompactMapADT;
```

A discriminated union describing the locations of contract references in either a Compact `Cell`, `List`, `Set`, or `Map` ADT.
