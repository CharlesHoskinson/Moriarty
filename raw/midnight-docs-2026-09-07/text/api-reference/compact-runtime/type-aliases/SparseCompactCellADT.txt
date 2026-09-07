# SparseCompactCellADT

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactCellADT

# Type Alias: SparseCompactCellADT

```
type SparseCompactCellADT = {

  tag: "cell";

  valueType: SparseCompactValue;

};
```

A data structure indicating the locations of all contract references in a Compact `Cell` ADT.

## Properties[​](#properties "Direct link to Properties")

### tag[​](#tag "Direct link to tag")

```
tag: "cell";
```

***

### valueType[​](#valuetype "Direct link to valueType")

```
valueType: SparseCompactValue;
```

A data structure indicating the locations of all contract references in the Compact value contained in the outer `Cell` ADT.
