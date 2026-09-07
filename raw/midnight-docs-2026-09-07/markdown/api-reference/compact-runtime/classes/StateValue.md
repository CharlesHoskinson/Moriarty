# StateValue

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / StateValue

# Class: StateValue

Represents the core of a contract's state, and recursively represents each of its components.

There are different *classes* of state values:

* `null`
* Cells of [AlignedValue](/api-reference/compact-runtime/type-aliases/AlignedValue.md)s
* Maps from [AlignedValue](/api-reference/compact-runtime/type-aliases/AlignedValue.md)s to state values
* Bounded Merkle trees containing [AlignedValue](/api-reference/compact-runtime/type-aliases/AlignedValue.md) leaves
* Short (<= 15 element) arrays of state values

State values are *immutable*, any operations that mutate states will return a new state instead.

## Methods[​](#methods "Direct link to Methods")

### arrayPush()[​](#arraypush "Direct link to arrayPush()")

```
arrayPush(value): StateValue;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### value[​](#value "Direct link to value")

`StateValue`

#### Returns[​](#returns "Direct link to Returns")

`StateValue`

***

### asArray()[​](#asarray "Direct link to asArray()")

```
asArray(): StateValue[] | undefined;
```

#### Returns[​](#returns-1 "Direct link to Returns")

`StateValue`\[] | `undefined`

***

### asBoundedMerkleTree()[​](#asboundedmerkletree "Direct link to asBoundedMerkleTree()")

```
asBoundedMerkleTree(): StateBoundedMerkleTree | undefined;
```

#### Returns[​](#returns-2 "Direct link to Returns")

[`StateBoundedMerkleTree`](/api-reference/compact-runtime/classes/StateBoundedMerkleTree.md) | `undefined`

***

### asCell()[​](#ascell "Direct link to asCell()")

```
asCell(): AlignedValue;
```

#### Returns[​](#returns-3 "Direct link to Returns")

[`AlignedValue`](/api-reference/compact-runtime/type-aliases/AlignedValue.md)

***

### asMap()[​](#asmap "Direct link to asMap()")

```
asMap(): StateMap | undefined;
```

#### Returns[​](#returns-4 "Direct link to Returns")

[`StateMap`](/api-reference/compact-runtime/classes/StateMap.md) | `undefined`

***

### encode()[​](#encode "Direct link to encode()")

```
encode(): EncodedStateValue;
```

**`Internal`**

#### Returns[​](#returns-5 "Direct link to Returns")

[`EncodedStateValue`](/api-reference/compact-runtime/type-aliases/EncodedStateValue.md)

***

### logSize()[​](#logsize "Direct link to logSize()")

```
logSize(): number;
```

#### Returns[​](#returns-6 "Direct link to Returns")

`number`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-7 "Direct link to Returns")

`string`

***

### type()[​](#type "Direct link to type()")

```
type(): "cell" | "map" | "array" | "null" | "boundedMerkleTree";
```

#### Returns[​](#returns-8 "Direct link to Returns")

`"cell"` | `"map"` | `"array"` | `"null"` | `"boundedMerkleTree"`

***

### decode()[​](#decode "Direct link to decode()")

```
static decode(value): StateValue;
```

**`Internal`**

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### value[​](#value-1 "Direct link to value")

[`EncodedStateValue`](/api-reference/compact-runtime/type-aliases/EncodedStateValue.md)

#### Returns[​](#returns-9 "Direct link to Returns")

`StateValue`

***

### newArray()[​](#newarray "Direct link to newArray()")

```
static newArray(): StateValue;
```

#### Returns[​](#returns-10 "Direct link to Returns")

`StateValue`

***

### newBoundedMerkleTree()[​](#newboundedmerkletree "Direct link to newBoundedMerkleTree()")

```
static newBoundedMerkleTree(tree): StateValue;
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### tree[​](#tree "Direct link to tree")

[`StateBoundedMerkleTree`](/api-reference/compact-runtime/classes/StateBoundedMerkleTree.md)

#### Returns[​](#returns-11 "Direct link to Returns")

`StateValue`

***

### newCell()[​](#newcell "Direct link to newCell()")

```
static newCell(value): StateValue;
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### value[​](#value-2 "Direct link to value")

[`AlignedValue`](/api-reference/compact-runtime/type-aliases/AlignedValue.md)

#### Returns[​](#returns-12 "Direct link to Returns")

`StateValue`

***

### newMap()[​](#newmap "Direct link to newMap()")

```
static newMap(map): StateValue;
```

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### map[​](#map "Direct link to map")

[`StateMap`](/api-reference/compact-runtime/classes/StateMap.md)

#### Returns[​](#returns-13 "Direct link to Returns")

`StateValue`

***

### newNull()[​](#newnull "Direct link to newNull()")

```
static newNull(): StateValue;
```

#### Returns[​](#returns-14 "Direct link to Returns")

`StateValue`
