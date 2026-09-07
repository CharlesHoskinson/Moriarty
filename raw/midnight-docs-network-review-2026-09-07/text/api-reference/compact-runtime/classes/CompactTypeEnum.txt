# CompactTypeEnum

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / CompactTypeEnum

# Class: CompactTypeEnum

Runtime type of an enum with a given number of entries

## Implements[​](#implements "Direct link to Implements")

* [`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md)<`number`>

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new CompactTypeEnum(maxValue, length): CompactTypeEnum;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### maxValue[​](#maxvalue "Direct link to maxValue")

`number`

##### length[​](#length "Direct link to length")

`number`

#### Returns[​](#returns "Direct link to Returns")

`CompactTypeEnum`

## Properties[​](#properties "Direct link to Properties")

### length[​](#length-1 "Direct link to length")

```
readonly length: number;
```

***

### maxValue[​](#maxvalue-1 "Direct link to maxValue")

```
readonly maxValue: number;
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
fromValue(value): number;
```

Converts this type's field-aligned binary representation to its TypeScript representation destructively; (partially) consuming the input, and ignoring superflous data for chaining.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### value[​](#value "Direct link to value")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

#### Returns[​](#returns-2 "Direct link to Returns")

`number`

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md).[`fromValue`](/api-reference/compact-runtime/interfaces/CompactType.md#fromvalue)

***

### toValue()[​](#tovalue "Direct link to toValue()")

```
toValue(value): Value;
```

Converts this type's TypeScript representation to its field-aligned binary representation

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### value[​](#value-1 "Direct link to value")

`number`

#### Returns[​](#returns-3 "Direct link to Returns")

[`Value`](/api-reference/compact-runtime/type-aliases/Value.md)

#### Implementation of[​](#implementation-of-2 "Direct link to Implementation of")

[`CompactType`](/api-reference/compact-runtime/interfaces/CompactType.md).[`toValue`](/api-reference/compact-runtime/interfaces/CompactType.md#tovalue)
