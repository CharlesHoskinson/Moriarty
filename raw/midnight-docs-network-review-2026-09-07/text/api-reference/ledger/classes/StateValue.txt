# StateValue

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / StateValue

# Class: StateValue

Represents the core of a contract's state, and recursively represents each of its components.

There are different *classes* of state values:

* `null`
* Cells of [AlignedValue](/api-reference/ledger/type-aliases/AlignedValue.md)s
* Maps from [AlignedValue](/api-reference/ledger/type-aliases/AlignedValue.md)s to state values
* Bounded Merkle trees containing [AlignedValue](/api-reference/ledger/type-aliases/AlignedValue.md) leaves
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
asArray(): undefined | StateValue[];
```

#### Returns[​](#returns-1 "Direct link to Returns")

`undefined` | `StateValue`\[]

***

### asBoundedMerkleTree()[​](#asboundedmerkletree "Direct link to asBoundedMerkleTree()")

```
asBoundedMerkleTree(): undefined | StateBoundedMerkleTree;
```

#### Returns[​](#returns-2 "Direct link to Returns")

`undefined` | [`StateBoundedMerkleTree`](/api-reference/ledger/classes/StateBoundedMerkleTree.md)

***

### asCell()[​](#ascell "Direct link to asCell()")

```
asCell(): AlignedValue;
```

#### Returns[​](#returns-3 "Direct link to Returns")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

***

### asMap()[​](#asmap "Direct link to asMap()")

```
asMap(): undefined | StateMap;
```

#### Returns[​](#returns-4 "Direct link to Returns")

`undefined` | [`StateMap`](/api-reference/ledger/classes/StateMap.md)

***

### encode()[​](#encode "Direct link to encode()")

```
encode(): EncodedStateValue;
```

**`Internal`**

#### Returns[​](#returns-5 "Direct link to Returns")

[`EncodedStateValue`](/api-reference/ledger/type-aliases/EncodedStateValue.md)

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
type(): "map" | "null" | "cell" | "array" | "boundedMerkleTree";
```

#### Returns[​](#returns-8 "Direct link to Returns")

`"map"` | `"null"` | `"cell"` | `"array"` | `"boundedMerkleTree"`

***

### decode()[​](#decode "Direct link to decode()")

```
static decode(value): StateValue;
```

**`Internal`**

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### value[​](#value-1 "Direct link to value")

[`EncodedStateValue`](/api-reference/ledger/type-aliases/EncodedStateValue.md)

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

[`StateBoundedMerkleTree`](/api-reference/ledger/classes/StateBoundedMerkleTree.md)

#### Returns[​](#returns-11 "Direct link to Returns")

`StateValue`

***

### newCell()[​](#newcell "Direct link to newCell()")

```
static newCell(value): StateValue;
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### value[​](#value-2 "Direct link to value")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

#### Returns[​](#returns-12 "Direct link to Returns")

`StateValue`

***

### newMap()[​](#newmap "Direct link to newMap()")

```
static newMap(map): StateValue;
```

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### map[​](#map "Direct link to map")

[`StateMap`](/api-reference/ledger/classes/StateMap.md)

#### Returns[​](#returns-13 "Direct link to Returns")

`StateValue`

***

### newNull()[​](#newnull "Direct link to newNull()")

```
static newNull(): StateValue;
```

#### Returns[​](#returns-14 "Direct link to Returns")

`StateValue`
