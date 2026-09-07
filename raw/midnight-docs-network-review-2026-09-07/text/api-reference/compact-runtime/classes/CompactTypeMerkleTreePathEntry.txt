# CompactTypeMerkleTreePathEntry

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.9.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / CompactTypeMerkleTreePathEntry

# Class: CompactTypeMerkleTreePathEntry

Runtime type of [MerkleTreePathEntry](/api-reference/compact-runtime/interfaces/MerkleTreePathEntry.md)

## Implements[​](#implements "Direct link to Implements")

* [`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<[`MerkleTreePathEntry`](/api-reference/compact-runtime/interfaces/MerkleTreePathEntry.md)>

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new CompactTypeMerkleTreePathEntry(): CompactTypeMerkleTreePathEntry;
```

#### Returns[​](#returns "Direct link to Returns")

`CompactTypeMerkleTreePathEntry`

## Properties[​](#properties "Direct link to Properties")

### bool[​](#bool "Direct link to bool")

```
readonly bool: CompactTypeBoolean;
```

***

### digest[​](#digest "Direct link to digest")

```
readonly digest: CompactTypeMerkleTreeDigest;
```

## Methods[​](#methods "Direct link to Methods")

### alignment()[​](#alignment "Direct link to alignment()")

```
alignment(): Alignment;
```

The field-aligned binary alignment of this type.

#### Returns[​](#returns-1 "Direct link to Returns")

[`Alignment`](/api-reference/compact-runtime/type-aliases/Alignment.md)

#### Implementation of[​](#implementation-of "Direct link to Implementation of")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md).[`alignment`](/api-reference/compact-runtime/interfaces/CompactType.md#alignment)

***

### fromValue()[​](#fromvalue "Direct link to fromValue()")

```
fromValue(value): MerkleTreePathEntry;
```

Converts this type's field-aligned binary representation to its TypeScript representation destructively; (partially) consuming the input, and ignoring superflous data for chaining.

#### Parameters[​](#parameters "Direct link to Parameters")

##### value[​](#value "Direct link to value")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`MerkleTreePathEntry`](/api-reference/compact-runtime/interfaces/MerkleTreePathEntry.md)

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md).[`fromValue`](/api-reference/compact-runtime/interfaces/CompactType.md#fromvalue)

***

### toValue()[​](#tovalue "Direct link to toValue()")

```
toValue(value): Value;
```

Converts this type's TypeScript representation to its field-aligned binary representation

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### value[​](#value-1 "Direct link to value")

[`MerkleTreePathEntry`](/api-reference/compact-runtime/interfaces/MerkleTreePathEntry.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

#### Implementation of[​](#implementation-of-2 "Direct link to Implementation of")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md).[`toValue`](/api-reference/compact-runtime/interfaces/CompactType.md#tovalue)
