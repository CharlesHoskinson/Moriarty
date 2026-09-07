# StateMap

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / StateMap

# Class: StateMap

Represents a key-value map, where keys are [AlignedValue](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)s, and values are [StateValue](/api-reference/onchain-runtime/classes/StateValue.md)s.

## Constructors[​](#constructors "Direct link to Constructors")

### new StateMap()[​](#new-statemap "Direct link to new StateMap()")

```
new StateMap(): StateMap
```

#### Returns[​](#returns "Direct link to Returns")

[`StateMap`](/api-reference/onchain-runtime/classes/StateMap.md)

## Methods[​](#methods "Direct link to Methods")

### get()[​](#get "Direct link to get()")

```
get(key): undefined | StateValue
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### key[​](#key "Direct link to key")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

#### Returns[​](#returns-1 "Direct link to Returns")

`undefined` | [`StateValue`](/api-reference/onchain-runtime/classes/StateValue.md)

***

### insert()[​](#insert "Direct link to insert()")

```
insert(key, value): StateMap
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### key[​](#key-1 "Direct link to key")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

##### value[​](#value "Direct link to value")

[`StateValue`](/api-reference/onchain-runtime/classes/StateValue.md)

#### Returns[​](#returns-2 "Direct link to Returns")

[`StateMap`](/api-reference/onchain-runtime/classes/StateMap.md)

***

### keys()[​](#keys "Direct link to keys()")

```
keys(): AlignedValue[]
```

#### Returns[​](#returns-3 "Direct link to Returns")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)\[]

***

### remove()[​](#remove "Direct link to remove()")

```
remove(key): StateMap
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### key[​](#key-2 "Direct link to key")

[`AlignedValue`](/api-reference/onchain-runtime/type-aliases/AlignedValue.md)

#### Returns[​](#returns-4 "Direct link to Returns")

[`StateMap`](/api-reference/onchain-runtime/classes/StateMap.md)

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-5 "Direct link to Returns")

`string`
