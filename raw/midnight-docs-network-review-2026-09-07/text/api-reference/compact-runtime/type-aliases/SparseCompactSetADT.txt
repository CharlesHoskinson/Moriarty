# SparseCompactSetADT

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactSetADT

# Type Alias: SparseCompactSetADT

```
type SparseCompactSetADT = {

  tag: "set";

  valueType: SparseCompactValue;

};
```

A data structure indicating the locations of all contract references in a Compact `Set` ADT.

## Properties[​](#properties "Direct link to Properties")

### tag[​](#tag "Direct link to tag")

```
tag: "set";
```

***

### valueType[​](#valuetype "Direct link to valueType")

```
valueType: SparseCompactValue;
```

A data structure indicating the locations of all contract references in a Compact value in the outer `Set` ADT.
