# SparseCompactMapADT

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / SparseCompactMapADT

# Type Alias: SparseCompactMapADT

```
type SparseCompactMapADT = {

  keyType?: SparseCompactValue;

  tag: "map";

  valueType?:   | SparseCompactADT

     | SparseCompactValue;

};
```

A data structure indicating the locations of all contract references in a Compact `Map` ADT.

## Properties[​](#properties "Direct link to Properties")

### keyType?[​](#keytype "Direct link to keyType?")

```
optional keyType: SparseCompactValue;
```

A data structure indicating the locations of all contract references in the Compact values that are the keys of the outer `Map` ADT.

***

### tag[​](#tag "Direct link to tag")

```
tag: "map";
```

***

### valueType?[​](#valuetype "Direct link to valueType?")

```
optional valueType: 

  | SparseCompactADT

  | SparseCompactValue;
```

A data structure indicating the locations of all contract references in the Compact entities that are the values of the outer `Map` ADT. Since the values of a `Map` ADT may be either Compact values or other `Map` ADTs, we take the union of the corresponding data structures.
