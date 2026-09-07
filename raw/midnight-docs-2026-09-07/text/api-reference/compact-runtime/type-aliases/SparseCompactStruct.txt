# SparseCompactStruct

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactStruct

# Type Alias: SparseCompactStruct

```
type SparseCompactStruct = {

  elements: Record<string, SparseCompactType>;

  tag: "struct";

};
```

A data structure indicating the locations of contract references in a Compact struct.

## Properties[​](#properties "Direct link to Properties")

### elements[​](#elements "Direct link to elements")

```
elements: Record<string, SparseCompactType>;
```

A data structure indicating the locations of contract references in the elements of a Compact struct. The keys of the record correspond to fields of the Compact struct that contain contract references. We use the keys of the record to explore the elements of the corresponding CompactStruct.

***

### tag[​](#tag "Direct link to tag")

```
tag: "struct";
```
