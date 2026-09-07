# SparseCompactContractAddress

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactContractAddress

# Type Alias: SparseCompactContractAddress

```
type SparseCompactContractAddress = {

  tag: "contractAddress";

};
```

A data structure indicating that the current CompactValue being explored is a contract reference. When this type is recognized, the current CompactValue should be a [ContractAddress](/api-reference/compact-runtime/type-aliases/ContractAddress.md), and the address is added to the dependency set.

## Properties[​](#properties "Direct link to Properties")

### tag[​](#tag "Direct link to tag")

```
tag: "contractAddress";
```
