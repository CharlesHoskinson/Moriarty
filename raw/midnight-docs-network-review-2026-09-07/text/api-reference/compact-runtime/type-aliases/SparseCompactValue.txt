# SparseCompactValue

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactValue

# Type Alias: SparseCompactValue

```
type SparseCompactValue = {

  descriptor: CompactType<unknown>;

  sparseType: SparseCompactType;

  tag: "compactValue";

};
```

A data structure indicating the locations of all contract references in a Compact value.

## Properties[​](#properties "Direct link to Properties")

### descriptor[​](#descriptor "Direct link to descriptor")

```
descriptor: CompactType<unknown>;
```

A descriptor that can be used to convert an [AlignedValue](/api-reference/compact-runtime/type-aliases/AlignedValue.md) into a TypeScript representation of the same value. This descriptor will only ever decode `struct`s or `Vector`s that contain contract addresses.

***

### sparseType[​](#sparsetype "Direct link to sparseType")

```
sparseType: SparseCompactType;
```

A data structure indicating how to navigate to the contract addresses present in the output of the above `descriptor`.

***

### tag[​](#tag "Direct link to tag")

```
tag: "compactValue";
```
