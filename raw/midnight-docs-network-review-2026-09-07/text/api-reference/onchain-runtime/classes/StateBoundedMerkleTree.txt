# StateBoundedMerkleTree

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / StateBoundedMerkleTree

# Class: StateBoundedMerkleTree

Represents a fixed-depth Merkle tree storing hashed data, whose preimages are unknown

## Constructors[​](#constructors "Direct link to Constructors")

### new StateBoundedMerkleTree()[​](#new-stateboundedmerkletree "Direct link to new StateBoundedMerkleTree()")

```
new StateBoundedMerkleTree(height): StateBoundedMerkleTree
```

Create a blank tree with the given height

#### Parameters[​](#parameters "Direct link to Parameters")

##### height[​](#height "Direct link to height")

`number`

#### Returns[​](#returns "Direct link to Returns")

[`StateBoundedMerkleTree`](/api-reference/onchain-runtime/classes/StateBoundedMerkleTree.md)

## Properties[​](#properties "Direct link to Properties")

### height[​](#height-1 "Direct link to height")

```
readonly height: number;
```

## Methods[​](#methods "Direct link to Methods")

### collapse()[​](#collapse "Direct link to collapse()")

```
collapse(start, end): StateBoundedMerkleTree
```

**`Internal`**

Erases all but necessary hashes between, and inclusive of, `start` and `end` inidices

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### start[​](#start "Direct link to start")

`bigint`

##### end[​](#end "Direct link to end")

`bigint`

#### Returns[​](#returns-1 "Direct link to Returns")

[`StateBoundedMerkleTree`](/api-reference/onchain-runtime/classes/StateBoundedMerkleTree.md)

#### Throws[​](#throws "Direct link to Throws")

If the indices are out-of-bounds for the tree, or `end < start`

***

### findPathForLeaf()[​](#findpathforleaf "Direct link to findPathForLeaf()")

```
findPathForLeaf(leaf): undefined | AlignedValue
```

**`Internal`**

Internal implementation of the finding path primitive. Returns undefined if the leaf is not in the tree.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### leaf[​](#leaf "Direct link to leaf")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

#### Returns[​](#returns-2 "Direct link to Returns")

`undefined` | [`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

***

### pathForLeaf()[​](#pathforleaf "Direct link to pathForLeaf()")

```
pathForLeaf(index, leaf): AlignedValue
```

**`Internal`**

Internal implementation of the path construction primitive

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### index[​](#index "Direct link to index")

`bigint`

##### leaf[​](#leaf-1 "Direct link to leaf")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

#### Throws[​](#throws-1 "Direct link to Throws")

If the index is out-of-bounds for the tree

***

### rehash()[​](#rehash "Direct link to rehash()")

```
rehash(): StateBoundedMerkleTree
```

Rehashes the tree, updating all internal hashes and ensuring all node hashes are present. Necessary because the onchain runtime does not automatically rehash trees.

#### Returns[​](#returns-4 "Direct link to Returns")

[`StateBoundedMerkleTree`](/api-reference/onchain-runtime/classes/StateBoundedMerkleTree.md)

***

### root()[​](#root "Direct link to root()")

```
root(): undefined | AlignedValue
```

**`Internal`**

Internal implementation of the merkle tree root primitive. Returns undefined if the tree has not been fully hashed.

#### Returns[​](#returns-5 "Direct link to Returns")

`undefined` | [`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-6 "Direct link to Returns")

`string`

***

### update()[​](#update "Direct link to update()")

```
update(index, leaf): StateBoundedMerkleTree
```

Inserts a value into the Merkle tree, returning the updated tree

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### index[​](#index-1 "Direct link to index")

`bigint`

##### leaf[​](#leaf-2 "Direct link to leaf")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

#### Returns[​](#returns-7 "Direct link to Returns")

[`StateBoundedMerkleTree`](/api-reference/onchain-runtime/classes/StateBoundedMerkleTree.md)

#### Throws[​](#throws-2 "Direct link to Throws")

If the index is out-of-bounds for the tree
